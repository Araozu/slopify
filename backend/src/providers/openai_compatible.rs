use async_stream::try_stream;
use async_trait::async_trait;
use futures_util::{StreamExt, stream::BoxStream};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::chat::contracts::{ChatRole, PromptMessage};

use super::adapter::{ProviderAdapter, ProviderError, ProviderStreamEvent};
use super::sse_utils::{extract_sse_data, pop_sse_frame_bytes};

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

#[derive(Deserialize)]
struct OpenAiCompatibleStreamChunk {
    model: Option<String>,
    choices: Vec<OpenAiCompatibleStreamChoice>,
}

#[derive(Deserialize)]
struct OpenAiCompatibleStreamChoice {
    delta: Option<OpenAiCompatibleStreamDelta>,
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct OpenAiCompatibleStreamDelta {
    content: Option<String>,
    reasoning: Option<String>,
    reasoning_content: Option<String>,
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

        let endpoint = self.endpoint.clone();
        let provider_name = self.name().to_string();
        let response = client
            .post(&endpoint)
            .bearer_auth(api_key)
            .header("HTTP-Referer", "https://github.com/Araozu/slopify")
            .header("X-Title", "Slopify")
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;
        let mut byte_stream = response.bytes_stream();
        let model_name = model.to_string();

        let stream = try_stream! {
            let mut buffer = Vec::<u8>::new();
            let mut final_model = model_name.clone();
            let mut finish_reason = None;
            let mut is_done = false;

            while !is_done {
                let Some(chunk) = byte_stream.next().await else {
                    break;
                };
                let chunk = chunk?;
                buffer.extend_from_slice(&chunk);

                while let Some(frame_bytes) = pop_sse_frame_bytes(&mut buffer) {
                    let frame = String::from_utf8(frame_bytes).map_err(|e| {
                        ProviderError::Other(Box::new(e))
                    })?;
                    let Some(data) = extract_sse_data(&frame) else {
                        continue;
                    };

                    if data == "[DONE]" {
                        is_done = true;
                        break;
                    }

                    let event: OpenAiCompatibleStreamChunk = serde_json::from_str(&data)?;

                    if let Some(model) = event.model {
                        final_model = model;
                    }

                    for choice in event.choices {
                        if finish_reason.is_none() && choice.finish_reason.is_some() {
                            finish_reason = choice.finish_reason.clone();
                        }

                        let Some(delta) = choice.delta else {
                            continue;
                        };

                        if let Some(content) = delta.content.filter(|value| !value.is_empty()) {
                            yield ProviderStreamEvent::TextDelta(content);
                        }

                        let reasoning = delta
                            .reasoning
                            .or(delta.reasoning_content)
                            .filter(|value| !value.is_empty());
                        if let Some(reasoning) = reasoning {
                            yield ProviderStreamEvent::ReasoningDelta(reasoning);
                        }
                    }
                }
            }

            if !is_done {
                Err(ProviderError::Other(
                    "stream ended before [DONE] terminator".into(),
                ))?;
            }

            yield ProviderStreamEvent::Completed {
                model: final_model,
                finish_reason,
                vendor_metadata: serde_json::json!({
                    "provider": provider_name,
                    "streamed": true
                }),
            };
        };

        Ok(Box::pin(stream))
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
