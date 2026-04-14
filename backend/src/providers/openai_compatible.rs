use async_trait::async_trait;
use futures_util::stream::BoxStream;
use reqwest::Client;
use serde::Serialize;

use crate::chat::contracts::{ChatRole, PromptMessage};

use super::adapter::{ProviderAdapter, ProviderError, ProviderStreamEvent};
use super::openai_compat_stream::parse_openai_stream;

const OPENROUTER_API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";

pub struct OpenRouterAdapter {
    endpoint: String,
}

impl OpenRouterAdapter {
    pub fn new() -> Self {
        Self {
            endpoint: OPENROUTER_API_URL.to_string(),
        }
    }
}

impl From<reqwest::Error> for ProviderError {
    fn from(value: reqwest::Error) -> Self {
        Self::Http(value)
    }
}

impl From<serde_json::Error> for ProviderError {
    fn from(value: serde_json::Error) -> Self {
        Self::InvalidPayload(value)
    }
}

#[derive(Serialize)]
struct OpenAiCompatibleRequest<'a> {
    model: &'a str,
    stream: bool,
    messages: Vec<OpenAiCompatibleRequestMessage<'a>>,
}

#[derive(Serialize)]
struct OpenAiCompatibleRequestMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[async_trait]
impl ProviderAdapter for OpenRouterAdapter {
    fn name(&self) -> &str {
        "openrouter"
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
        let payload = OpenAiCompatibleRequest {
            model,
            stream: true,
            messages: messages
                .iter()
                .map(|message| OpenAiCompatibleRequestMessage {
                    role: chat_role_as_provider_role(&message.role),
                    content: &message.content,
                })
                .collect(),
        };

        let response = client
            .post(&self.endpoint)
            .bearer_auth(api_key)
            .header("HTTP-Referer", "https://github.com/Araozu/slopify")
            .header("X-Title", "Slopify")
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

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
