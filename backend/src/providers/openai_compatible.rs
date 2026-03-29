use async_stream::try_stream;
use futures_util::{StreamExt, stream::BoxStream};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::chat::contracts::{ChatRole, PromptMessage};

const OPENROUTER_API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";

#[derive(Debug, Clone)]
pub enum ProviderStreamEvent {
    TextDelta(String),
    ReasoningDelta(String),
    Completed {
        model: String,
        finish_reason: Option<String>,
        vendor_metadata: Value,
    },
}

#[derive(Debug)]
pub enum OpenAiCompatibleError {
    Http(reqwest::Error),
    InvalidChunk(serde_json::Error),
}

impl std::fmt::Display for OpenAiCompatibleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(error) => write!(f, "failed to call OpenAI-compatible endpoint: {error}"),
            Self::InvalidChunk(error) => write!(f, "provider stream returned invalid JSON: {error}"),
        }
    }
}

impl std::error::Error for OpenAiCompatibleError {}

impl From<reqwest::Error> for OpenAiCompatibleError {
    fn from(value: reqwest::Error) -> Self {
        Self::Http(value)
    }
}

impl From<serde_json::Error> for OpenAiCompatibleError {
    fn from(value: serde_json::Error) -> Self {
        Self::InvalidChunk(value)
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

pub async fn stream_prompt(
    client: &Client,
    messages: &[PromptMessage],
    model: &str,
    api_key: &str,
) -> Result<BoxStream<'static, Result<ProviderStreamEvent, OpenAiCompatibleError>>, OpenAiCompatibleError> {
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
        .post(OPENROUTER_API_URL)
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
        let mut buffer = String::new();
        let mut final_model = model_name.clone();
        let mut finish_reason = None;
        let mut is_done = false;

        while !is_done {
            let Some(chunk) = byte_stream.next().await else {
                break;
            };
            let chunk = chunk?;
            buffer.push_str(&String::from_utf8_lossy(&chunk));

            while let Some(frame) = pop_sse_frame(&mut buffer) {
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

        yield ProviderStreamEvent::Completed {
            model: final_model,
            finish_reason,
            vendor_metadata: serde_json::json!({
                "provider": "openrouter",
                "streamed": true
            }),
        };
    };

    Ok(Box::pin(stream))
}

fn chat_role_as_provider_role(role: &ChatRole) -> &'static str {
    match role {
        ChatRole::System => "system",
        ChatRole::User => "user",
        ChatRole::Assistant => "assistant",
        ChatRole::Tool => "tool",
    }
}

fn pop_sse_frame(buffer: &mut String) -> Option<String> {
    let (frame_end, delimiter_len) = if let Some(index) = buffer.find("\r\n\r\n") {
        (index, 4)
    } else {
        (buffer.find("\n\n")?, 2)
    };
    let frame = buffer[..frame_end].to_string();
    buffer.drain(..frame_end + delimiter_len);
    Some(frame)
}

fn extract_sse_data(frame: &str) -> Option<String> {
    let mut lines = Vec::new();

    for line in frame.lines() {
        if let Some(rest) = line.strip_prefix("data:") {
            lines.push(rest.trim_start().to_string());
        }
    }

    if lines.is_empty() {
        None
    } else {
        Some(lines.join("\n"))
    }
}
