use async_trait::async_trait;
use futures_util::stream::BoxStream;
use reqwest::Client;
use serde::Serialize;

use crate::chat::contracts::{ChatRole, PromptMessage};

use super::adapter::{ProviderAdapter, ProviderError, ProviderStreamEvent};
use super::anthropic_stream::parse_anthropic_stream;

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_API_VERSION: &str = "2023-06-01";

pub struct AnthropicAdapter {
    endpoint: String,
}

impl AnthropicAdapter {
    pub fn new() -> Self {
        Self {
            endpoint: ANTHROPIC_API_URL.to_string(),
        }
    }
}

#[derive(Serialize)]
struct AnthropicRequest<'a> {
    model: &'a str,
    max_tokens: u32,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<&'a str>,
    messages: Vec<AnthropicRequestMessage<'a>>,
}

#[derive(Serialize)]
struct AnthropicRequestMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[async_trait]
impl ProviderAdapter for AnthropicAdapter {
    fn name(&self) -> &str {
        "anthropic"
    }

    fn endpoint(&self) -> &str {
        &self.endpoint
    }

    async fn stream_prompt(
        &self,
        client: &Client,
        messages: &[PromptMessage],
        model: &str,
        api_key: &str,
    ) -> Result<BoxStream<'static, Result<ProviderStreamEvent, ProviderError>>, ProviderError> {
        // Anthropic does not support Tool messages; fail fast.
        if messages.iter().any(|m| matches!(m.role, ChatRole::Tool)) {
            return Err(ProviderError::Other(
                "Anthropic adapter does not support Tool messages".into(),
            ));
        }

        // Concatenate all system messages in order into a single system prompt.
        let system_parts: Vec<&str> = messages
            .iter()
            .filter(|m| matches!(m.role, ChatRole::System))
            .map(|m| m.content.as_str())
            .collect();
        let system_prompt = if system_parts.is_empty() {
            None
        } else {
            Some(system_parts.join("\n\n"))
        };

        let api_messages: Vec<AnthropicRequestMessage<'_>> = messages
            .iter()
            .filter(|m| !matches!(m.role, ChatRole::System))
            .map(|m| AnthropicRequestMessage {
                role: chat_role_as_anthropic_role(&m.role),
                content: &m.content,
            })
            .collect();

        let payload = AnthropicRequest {
            model,
            max_tokens: 8192,
            stream: true,
            system: system_prompt.as_deref(),
            messages: api_messages,
        };

        let provider_name = self.name().to_string();
        let response = client
            .post(&self.endpoint)
            .header("x-api-key", api_key)
            .header("anthropic-version", ANTHROPIC_API_VERSION)
            .header("content-type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(ProviderError::Http)?
            .error_for_status()
            .map_err(ProviderError::Http)?;

        Ok(parse_anthropic_stream(
            response,
            model.to_string(),
            provider_name,
        ))
    }
}

fn chat_role_as_anthropic_role(role: &ChatRole) -> &'static str {
    match role {
        ChatRole::User => "user",
        ChatRole::Assistant => "assistant",
        // System messages are filtered out before reaching here.
        // Tool messages are rejected before reaching here.
        // Defensive fallback to "user" in case of future refactoring.
        ChatRole::System | ChatRole::Tool => "user",
    }
}
