use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use futures_util::stream::BoxStream;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::chat::contracts::{ChatRole, PromptMessage};

use super::adapter::{ProviderAdapter, ProviderError, ProviderStreamEvent};
use super::openai_compat_stream::parse_openai_stream;

const OPENAI_CHAT_COMPLETIONS_URL: &str = "https://api.openai.com/v1/chat/completions";
const OPENAI_AUTH_URL: &str = "https://auth.openai.com/oauth/token";
const OPENAI_CLIENT_ID: &str = "app_EMoamEEZ73f0CkXaXp7hrann";
const EXPIRY_BUFFER_SECS: i64 = 60;

struct CachedApiToken {
    token: String,
    expires_at: chrono::DateTime<chrono::Utc>,
}

pub struct OpenAiAdapter {
    token_cache: Arc<RwLock<HashMap<String, CachedApiToken>>>,
}

impl OpenAiAdapter {
    pub fn new() -> Self {
        Self {
            token_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn resolve_api_token(
        &self,
        client: &Client,
        provided_token: &str,
    ) -> Result<String, ProviderError> {
        if provided_token.starts_with("sk-") {
            return Ok(provided_token.to_string());
        }

        {
            let cache = self.token_cache.read().await;
            if let Some(cached) = cache.get(provided_token) {
                let now = chrono::Utc::now();
                if cached.expires_at - chrono::Duration::seconds(EXPIRY_BUFFER_SECS) > now {
                    return Ok(cached.token.clone());
                }
            }
        }

        let id_token = refresh_id_token(client, provided_token).await?;
        let exchanged = exchange_id_token_for_api_key(client, &id_token).await?;

        let expires_in = exchanged.expires_in.unwrap_or(3600).max(1);
        let expires_at = chrono::Utc::now() + chrono::Duration::seconds(expires_in);

        let mut cache = self.token_cache.write().await;
        cache.insert(
            provided_token.to_string(),
            CachedApiToken {
                token: exchanged.access_token.clone(),
                expires_at,
            },
        );

        Ok(exchanged.access_token)
    }
}

#[derive(Serialize)]
struct OpenAiRequest<'a> {
    model: &'a str,
    stream: bool,
    messages: Vec<OpenAiRequestMessage<'a>>,
}

#[derive(Serialize)]
struct OpenAiRequestMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct RefreshTokenResponse {
    id_token: Option<String>,
}

#[derive(Deserialize)]
struct ApiKeyExchangeResponse {
    access_token: String,
    expires_in: Option<i64>,
}

#[async_trait]
impl ProviderAdapter for OpenAiAdapter {
    fn name(&self) -> &str {
        "openai"
    }

    fn endpoint(&self) -> &str {
        OPENAI_CHAT_COMPLETIONS_URL
    }

    async fn stream_prompt(
        &self,
        client: &Client,
        messages: &[PromptMessage],
        model: &str,
        api_key: &str,
    ) -> Result<BoxStream<'static, Result<ProviderStreamEvent, ProviderError>>, ProviderError> {
        let resolved_api_token = self.resolve_api_token(client, api_key).await?;

        let payload = OpenAiRequest {
            model,
            stream: true,
            messages: messages
                .iter()
                .map(|message| OpenAiRequestMessage {
                    role: chat_role_as_provider_role(&message.role),
                    content: &message.content,
                })
                .collect(),
        };

        let response = client
            .post(OPENAI_CHAT_COMPLETIONS_URL)
            .bearer_auth(resolved_api_token)
            .json(&payload)
            .send()
            .await
            .map_err(ProviderError::Http)?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(ProviderError::Other(
                format!("OpenAI completion request failed ({status}): {body}").into(),
            ));
        }

        Ok(parse_openai_stream(
            response,
            model.to_string(),
            self.name().to_string(),
        ))
    }
}

fn chat_role_as_provider_role(role: &ChatRole) -> &'static str {
    match role {
        ChatRole::System => "system",
        ChatRole::User => "user",
        ChatRole::Assistant => "assistant",
        ChatRole::Tool => "tool",
    }
}

async fn refresh_id_token(client: &Client, refresh_token: &str) -> Result<String, ProviderError> {
    let response = client
        .post(OPENAI_AUTH_URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[
            ("grant_type", "refresh_token"),
            ("client_id", OPENAI_CLIENT_ID),
            ("refresh_token", refresh_token),
        ])
        .send()
        .await
        .map_err(ProviderError::Http)?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(ProviderError::Other(
            format!("OpenAI refresh token exchange failed ({status}): {body}").into(),
        ));
    }

    let payload = response
        .json::<RefreshTokenResponse>()
        .await
        .map_err(ProviderError::Http)?;

    payload.id_token.ok_or_else(|| {
        ProviderError::Other("OpenAI refresh response did not include an id_token".into())
    })
}

async fn exchange_id_token_for_api_key(
    client: &Client,
    id_token: &str,
) -> Result<ApiKeyExchangeResponse, ProviderError> {
    let response = client
        .post(OPENAI_AUTH_URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[
            ("grant_type", "urn:ietf:params:oauth:grant-type:token-exchange"),
            ("client_id", OPENAI_CLIENT_ID),
            ("requested_token", "openai-api-key"),
            ("subject_token", id_token),
            (
                "subject_token_type",
                "urn:ietf:params:oauth:token-type:id_token",
            ),
        ])
        .send()
        .await
        .map_err(ProviderError::Http)?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(ProviderError::Other(
            format!("OpenAI API key exchange failed ({status}): {body}").into(),
        ));
    }

    response
        .json::<ApiKeyExchangeResponse>()
        .await
        .map_err(ProviderError::Http)
}
