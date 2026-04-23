use async_stream::try_stream;
use futures_util::{StreamExt, stream::BoxStream};
use reqwest::Response;
use serde::Deserialize;

use super::adapter::{ProviderError, ProviderStreamEvent};
use super::sse_utils::{extract_sse_data, pop_sse_frame_bytes};

#[derive(Deserialize)]
pub(super) struct AnthropicStreamEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(default)]
    pub delta: Option<AnthropicDelta>,
    #[serde(default)]
    pub message: Option<AnthropicMessage>,
    #[serde(default)]
    pub content_block: Option<AnthropicContentBlock>,
}

#[derive(Deserialize)]
pub(super) struct AnthropicDelta {
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub thinking: Option<String>,
    #[serde(default)]
    pub stop_reason: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct AnthropicMessage {
    #[serde(default)]
    pub model: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct AnthropicContentBlock {
    #[serde(rename = "type")]
    pub block_type: String,
}

/// Consume an Anthropic-format streaming response and produce a normalized event stream.
pub fn parse_anthropic_stream(
    response: Response,
    model_hint: String,
    provider_name: String,
) -> BoxStream<'static, Result<ProviderStreamEvent, ProviderError>> {
    let mut byte_stream = response.bytes_stream();

    let stream = try_stream! {
        let mut buffer = Vec::<u8>::new();
        let mut final_model = model_hint.clone();
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

    Box::pin(stream)
}
