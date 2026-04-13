use async_stream::try_stream;
use async_trait::async_trait;
use futures_util::{StreamExt, stream::BoxStream};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::chat::contracts::{ChatRole, PromptMessage};

use super::adapter::{ProviderAdapter, ProviderError, ProviderStreamEvent};
use super::sse_utils::{extract_sse_data, pop_sse_frame_bytes};

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

#[derive(Deserialize)]
struct AnthropicStreamEvent {
    #[serde(rename = "type")]
    event_type: String,
    #[serde(default)]
    delta: Option<AnthropicDelta>,
    #[serde(default)]
    message: Option<AnthropicMessage>,
    #[serde(default)]
    content_block: Option<AnthropicContentBlock>,
}

#[derive(Deserialize)]
struct AnthropicDelta {
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    thinking: Option<String>,
    #[serde(default)]
    stop_reason: Option<String>,
}

#[derive(Deserialize)]
struct AnthropicMessage {
    #[serde(default)]
    model: Option<String>,
}

#[derive(Deserialize)]
struct AnthropicContentBlock {
    #[serde(rename = "type")]
    block_type: String,
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

        let endpoint = self.endpoint.clone();
        let provider_name = self.name().to_string();
        let response = client
            .post(&endpoint)
            .header("x-api-key", api_key)
            .header("anthropic-version", ANTHROPIC_API_VERSION)
            .header("content-type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(ProviderError::Http)?
            .error_for_status()
            .map_err(ProviderError::Http)?;
        let mut byte_stream = response.bytes_stream();
        let model_name = model.to_string();

        let stream = try_stream! {
            let mut buffer = Vec::<u8>::new();
            let mut final_model = model_name.clone();
            let mut finish_reason: Option<String> = None;
            let mut current_block_type: Option<String> = None;
            let mut saw_message_stop = false;

            'stream: loop {
                let Some(chunk) = byte_stream.next().await else {
                    break;
                };
                let chunk = chunk.map_err(ProviderError::Http)?;
                buffer.extend_from_slice(&chunk);

                while let Some(frame_bytes) = pop_sse_frame_bytes(&mut buffer) {
                    let frame = String::from_utf8(frame_bytes).map_err(|e| {
                        ProviderError::Other(Box::new(e))
                    })?;
                    let Some(data) = extract_sse_data(&frame) else {
                        continue;
                    };

                    let event: AnthropicStreamEvent =
                        serde_json::from_str(&data).map_err(ProviderError::InvalidPayload)?;

                    match event.event_type.as_str() {
                        "message_start" => {
                            if let Some(msg) = event.message {
                                if let Some(m) = msg.model {
                                    final_model = m;
                                }
                            }
                        }
                        "content_block_start" => {
                            if let Some(block) = event.content_block {
                                current_block_type = Some(block.block_type);
                            }
                        }
                        "content_block_delta" => {
                            if let Some(delta) = event.delta {
                                if let Some(text) = delta.text.filter(|t| !t.is_empty()) {
                                    match current_block_type.as_deref() {
                                        Some("thinking") => {
                                            yield ProviderStreamEvent::ReasoningDelta(text);
                                        }
                                        _ => {
                                            yield ProviderStreamEvent::TextDelta(text);
                                        }
                                    }
                                }
                                if let Some(thinking) = delta.thinking.filter(|t| !t.is_empty()) {
                                    yield ProviderStreamEvent::ReasoningDelta(thinking);
                                }
                            }
                        }
                        "content_block_stop" => {
                            current_block_type = None;
                        }
                        "message_delta" => {
                            if let Some(delta) = event.delta {
                                if delta.stop_reason.is_some() {
                                    finish_reason = delta.stop_reason;
                                }
                            }
                        }
                        "message_stop" => {
                            saw_message_stop = true;
                            break 'stream;
                        }
                        _ => {}
                    }
                }
            }

            if !saw_message_stop {
                Err(ProviderError::Other(
                    "stream ended before message_stop".into(),
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

fn chat_role_as_anthropic_role(role: &ChatRole) -> &'static str {
    match role {
        ChatRole::User => "user",
        ChatRole::Assistant => "assistant",
        // System and Tool are handled before this point.
        ChatRole::System | ChatRole::Tool => unreachable!(),
    }
}
