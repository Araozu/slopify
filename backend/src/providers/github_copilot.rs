use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use futures_util::stream::BoxStream;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::chat::contracts::{ChatRole, PromptMessage};

use super::adapter::{ProviderAdapter, ProviderError, ProviderStreamEvent};
use super::openai_compat_stream::parse_openai_stream;

const COMPLETIONS_URL: &str = "https://api.githubcopilot.com/chat/completions";
const TOKEN_URL: &str = "https://api.github.com/copilot_internal/v2/token";

/// How many seconds before actual expiry we treat a cached token as stale.
const EXPIRY_SAFETY_MARGIN_SECS: u64 = 60;

#[derive(Clone)]
struct CachedCopilotToken {
    token: String,
    /// Unix timestamp (seconds) at which the token expires.
    expires_at: u64,
}

pub struct GithubCopilotAdapter {
    completions_endpoint: String,
    token_endpoint: String,
    /// Keyed by a hash of the user's GitHub token.
    token_cache: Arc<RwLock<HashMap<u64, CachedCopilotToken>>>,
}

impl GithubCopilotAdapter {
    pub fn new() -> Self {
        Self {
            completions_endpoint: COMPLETIONS_URL.to_string(),
            token_endpoint: TOKEN_URL.to_string(),
            token_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[derive(Deserialize)]
struct CopilotTokenResponse {
    token: String,
    expires_at: serde_json::Value,
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

fn hash_token(token: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    token.hash(&mut hasher);
    hasher.finish()
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

impl GithubCopilotAdapter {
    async fn get_copilot_token(
        &self,
        client: &Client,
        github_token: &str,
    ) -> Result<String, ProviderError> {
        let key = hash_token(github_token);
        let now = now_unix();

        {
            let cache = self.token_cache.read().await;
            if let Some(cached) = cache.get(&key) {
                if now + EXPIRY_SAFETY_MARGIN_SECS < cached.expires_at {
                    return Ok(cached.token.clone());
                }
            }
        }

        let response = client
            .get(&self.token_endpoint)
            .bearer_auth(github_token)
            .header("Accept", "application/json")
            .header("Editor-Version", "vscode/1.95.0")
            .header("Editor-Plugin-Version", "copilot/1.236.0")
            .send()
            .await
            .map_err(ProviderError::Http)?
            .error_for_status()
            .map_err(ProviderError::Http)?;

        let body: CopilotTokenResponse = response
            .json()
            .await
            .map_err(ProviderError::Http)?;

        // expires_at may be a number or an RFC3339 string depending on the API version.
        let expires_at = parse_expires_at(&body.expires_at).unwrap_or(now + 1800);

        let mut cache = self.token_cache.write().await;
        cache.insert(
            key,
            CachedCopilotToken {
                token: body.token.clone(),
                expires_at,
            },
        );

        Ok(body.token)
    }
}

fn parse_expires_at(value: &serde_json::Value) -> Option<u64> {
    match value {
        serde_json::Value::Number(n) => n.as_u64(),
        serde_json::Value::String(s) => {
            // Parse RFC3339 / ISO 8601 manually via chrono if available,
            // or fall back to parsing as a Unix timestamp string.
            s.parse::<u64>().ok().or_else(|| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(|dt| dt.timestamp() as u64)
            })
        }
        _ => None,
    }
}

#[async_trait]
impl ProviderAdapter for GithubCopilotAdapter {
    fn name(&self) -> &str {
        "github-copilot"
    }

    fn endpoint(&self) -> &str {
        &self.completions_endpoint
    }

    async fn stream_prompt(
        &self,
        client: &Client,
        messages: &[PromptMessage],
        model: &str,
        api_key: &str,
    ) -> Result<BoxStream<'static, Result<ProviderStreamEvent, ProviderError>>, ProviderError> {
        let copilot_token = self.get_copilot_token(client, api_key).await?;

        let payload = CopilotRequest {
            model,
            stream: true,
            messages: messages
                .iter()
                .map(|m| CopilotRequestMessage {
                    role: chat_role_as_copilot_role(&m.role),
                    content: &m.content,
                })
                .collect(),
        };

        let response = client
            .post(&self.completions_endpoint)
            .bearer_auth(&copilot_token)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Editor-Version", "vscode/1.95.0")
            .header("Editor-Plugin-Version", "copilot/1.236.0")
            .header("Openai-Organization", "github-copilot")
            .header("Copilot-Integration-Id", "vscode-chat")
            .json(&payload)
            .send()
            .await
            .map_err(ProviderError::Http)?
            .error_for_status()
            .map_err(ProviderError::Http)?;

        Ok(parse_openai_stream(
            response,
            model.to_string(),
            self.name().to_string(),
        ))
    }
}

fn chat_role_as_copilot_role(role: &ChatRole) -> &'static str {
    match role {
        ChatRole::System => "system",
        ChatRole::User => "user",
        ChatRole::Assistant => "assistant",
        ChatRole::Tool => "tool",
    }
}
