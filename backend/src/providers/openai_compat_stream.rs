use async_stream::try_stream;
use futures_util::{stream::BoxStream, StreamExt};
use reqwest::Response;
use serde::Deserialize;

use super::adapter::{ProviderError, ProviderStreamEvent};
use super::sse_utils::{extract_sse_data, pop_sse_frame_bytes};

#[derive(Deserialize)]
pub(super) struct StreamChunk {
    pub model: Option<String>,
    pub choices: Vec<StreamChoice>,
}

#[derive(Deserialize)]
pub(super) struct StreamChoice {
    pub delta: Option<StreamDelta>,
    pub finish_reason: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct StreamDelta {
    pub content: Option<String>,
    pub reasoning: Option<String>,
    pub reasoning_content: Option<String>,
}

/// Consume an OpenAI-compatible streaming response and produce a normalized event stream.
pub fn parse_openai_stream(
    response: Response,
    model_hint: String,
    provider_name: String,
) -> BoxStream<'static, Result<ProviderStreamEvent, ProviderError>> {
    let mut byte_stream = response.bytes_stream();

    let stream = try_stream! {
        let mut buffer = Vec::<u8>::new();
        let mut final_model = model_hint.clone();
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

                let event: StreamChunk = serde_json::from_str(&data)?;

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

                    if let Some(content) = delta.content.filter(|v| !v.is_empty()) {
                        yield ProviderStreamEvent::TextDelta(content);
                    }

                    let reasoning = delta
                        .reasoning
                        .or(delta.reasoning_content)
                        .filter(|v| !v.is_empty());
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

    Box::pin(stream)
}
