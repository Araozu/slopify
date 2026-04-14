use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use futures_util::stream::BoxStream;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{debug, error, info};

use crate::chat::contracts::{ChatRole, PromptMessage};

use super::adapter::{ProviderAdapter, ProviderError, ProviderStreamEvent};
use super::openai_compat_stream::parse_openai_stream;

const COMPLETIONS_URL: &str = "https://api.githubcopilot.com/chat/completions";
const TOKEN_EXCHANGE_URL: &str = "https://api.github.com/copilot_internal/v2/token";

/// Safety margin: refresh the cached token 60 s before it actually expires.
const EXPIRY_BUFFER_SECS: i64 = 60;

struct CachedApiToken {
    token: String,
    expires_at: chrono::DateTime<chrono::Utc>,
}

pub struct GithubCopilotAdapter {
    /// Cache of short-lived Copilot API tokens keyed by the OAuth token they
    /// were exchanged from.
    token_cache: Arc<RwLock<HashMap<String, CachedApiToken>>>,
}

impl GithubCopilotAdapter {
    pub fn new() -> Self {
        Self {
            token_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Return a valid Copilot API token, using the cache when possible.
    async fn resolve_api_token(
        &self,
        client: &Client,
        oauth_token: &str,
    ) -> Result<String, ProviderError> {
        let token_preview = &oauth_token[..oauth_token.len().min(8)];

        // Check cache first (read lock).
        {
            let cache = self.token_cache.read().await;
            if let Some(cached) = cache.get(oauth_token) {
                let now = chrono::Utc::now();
                if cached.expires_at - chrono::Duration::seconds(EXPIRY_BUFFER_SECS) > now {
                    debug!(token_prefix = token_preview, "copilot: using cached API token");
                    return Ok(cached.token.clone());
                }
                debug!(token_prefix = token_preview, "copilot: cached token expired, re-exchanging");
            }
        }

        // Cache miss or expired — exchange the OAuth token.
        info!(token_prefix = token_preview, "copilot: exchanging OAuth token for API token");
        let api_token = exchange_oauth_token(client, oauth_token).await?;
        let token_string = api_token.token.clone();
        info!(
            expires_at = %api_token.expires_at,
            "copilot: token exchange successful"
        );

        let mut cache = self.token_cache.write().await;
        cache.insert(
            oauth_token.to_string(),
            CachedApiToken {
                token: api_token.token,
                expires_at: api_token.expires_at,
            },
        );

        Ok(token_string)
    }
}

#[derive(Deserialize)]
struct TokenExchangeResponse {
    token: String,
    #[serde(deserialize_with = "chrono::serde::ts_seconds::deserialize")]
    expires_at: chrono::DateTime<chrono::Utc>,
}

async fn exchange_oauth_token(
    client: &Client,
    oauth_token: &str,
) -> Result<TokenExchangeResponse, ProviderError> {
    debug!("copilot: requesting token from {TOKEN_EXCHANGE_URL}");

    let response = client
        .get(TOKEN_EXCHANGE_URL)
        .header("Authorization", format!("token {oauth_token}"))
        .header("Accept", "application/json")
        .header("User-Agent", "slopify/0.1")
        .send()
        .await
        .map_err(|e| {
            error!(error = %e, "copilot: token exchange request failed");
            ProviderError::Http(e)
        })?;

    let status = response.status();
    debug!(status = %status, "copilot: token exchange response status");

    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        error!(status = %status, body = %body, "copilot: token exchange returned non-2xx");
        return Err(ProviderError::Other(
            format!("token exchange failed ({status}): {body}").into(),
        ));
    }

    let body_text = response.text().await.map_err(|e| {
        error!(error = %e, "copilot: failed to read token exchange body");
        ProviderError::Http(e)
    })?;

    debug!(body_len = body_text.len(), "copilot: token exchange body received");

    let parsed: TokenExchangeResponse = serde_json::from_str(&body_text).map_err(|e| {
        error!(
            error = %e,
            body_preview = %&body_text[..body_text.len().min(500)],
            "copilot: failed to parse token exchange response"
        );
        ProviderError::InvalidPayload(e)
    })?;

    Ok(parsed)
}

#[derive(Serialize)]
struct CopilotRequest<'a> {
    model: &'a str,
    stream: bool,
    messages: Vec<CopilotRequestMessage<'a>>,
}

#[derive(Serialize)]
struct CopilotRequestMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[async_trait]
impl ProviderAdapter for GithubCopilotAdapter {
    fn name(&self) -> &str {
        "github-copilot"
    }

    fn endpoint(&self) -> &str {
        COMPLETIONS_URL
    }

    async fn stream_prompt(
        &self,
        client: &Client,
        messages: &[PromptMessage],
        model: &str,
        api_key: &str,
    ) -> Result<BoxStream<'static, Result<ProviderStreamEvent, ProviderError>>, ProviderError> {
        info!(model = model, messages = messages.len(), "copilot: starting stream_prompt");

        let copilot_token = self.resolve_api_token(client, api_key).await?;

        let payload = CopilotRequest {
            model,
            stream: true,
            messages: messages
                .iter()
                .map(|m| CopilotRequestMessage {
                    role: chat_role_str(&m.role),
                    content: &m.content,
                })
                .collect(),
        };

        debug!(
            url = COMPLETIONS_URL,
            model = model,
            "copilot: sending completion request"
        );

        let response = client
            .post(COMPLETIONS_URL)
            .bearer_auth(&copilot_token)
            .header("Content-Type", "application/json")
            .header("Copilot-Integration-Id", "vscode-chat")
            .header("Editor-Version", "vscode/1.100.0")
            .header("Editor-Plugin-Version", "copilot-chat/0.30.0")
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                error!(error = %e, "copilot: completion request failed to send");
                ProviderError::Http(e)
            })?;

        let status = response.status();
        debug!(status = %status, "copilot: completion response status");

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "copilot: completion returned non-2xx");
            return Err(ProviderError::Other(
                format!("completion request failed ({status}): {body}").into(),
            ));
        }

        info!("copilot: streaming response started");

        Ok(parse_openai_stream(
            response,
            model.to_string(),
            self.name().to_string(),
        ))
    }
}

fn chat_role_str(role: &ChatRole) -> &'static str {
    match role {
        ChatRole::System => "system",
        ChatRole::User => "user",
        ChatRole::Assistant => "assistant",
        ChatRole::Tool => "tool",
    }
}
