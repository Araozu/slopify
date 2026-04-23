use async_trait::async_trait;
use futures_util::stream::BoxStream;
use reqwest::Client;
use serde::Serialize;

use crate::chat::contracts::{ChatRole, PromptMessage};

use super::adapter::{ProviderAdapter, ProviderError, ProviderStreamEvent};
use super::anthropic_stream::parse_anthropic_stream;
use super::openai_compat_stream::parse_openai_stream;

const ZEN_BASE_URL: &str = "https://opencode.ai/zen/v1";
const ANTHROPIC_API_VERSION: &str = "2023-06-01";

pub struct OpenCodeZenAdapter;

impl OpenCodeZenAdapter {
    pub fn new() -> Self {
        Self
    }
}

// ── Anthropic wire format ────────────────────────────────────────────────────

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

// ── OpenAI Chat Completions wire format ─────────────────────────────────────

#[derive(Serialize)]
struct ChatCompletionsRequest<'a> {
    model: &'a str,
    stream: bool,
    messages: Vec<ChatCompletionsMessage<'a>>,
}

#[derive(Serialize)]
struct ChatCompletionsMessage<'a> {
    role: &'a str,
    content: &'a str,
}

// ── Adapter ──────────────────────────────────────────────────────────────────

#[async_trait]
impl ProviderAdapter for OpenCodeZenAdapter {
    fn name(&self) -> &str {
        "opencode-zen"
    }

    fn endpoint(&self) -> &str {
        ZEN_BASE_URL
    }

    async fn stream_prompt(
        &self,
        client: &Client,
        messages: &[PromptMessage],
        model: &str,
        api_key: &str,
    ) -> Result<BoxStream<'static, Result<ProviderStreamEvent, ProviderError>>, ProviderError> {
        if model.starts_with("claude-") {
            stream_anthropic(client, messages, model, api_key, self.name()).await
        } else {
            stream_chat_completions(client, messages, model, api_key, self.name()).await
        }
    }
}

async fn stream_anthropic(
    client: &Client,
    messages: &[PromptMessage],
    model: &str,
    api_key: &str,
    provider_name: &str,
) -> Result<BoxStream<'static, Result<ProviderStreamEvent, ProviderError>>, ProviderError> {
    if messages.iter().any(|m| matches!(m.role, ChatRole::Tool)) {
        return Err(ProviderError::Other(
            "Zen Anthropic path does not support Tool messages".into(),
        ));
    }

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
            role: chat_role_to_anthropic(&m.role),
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

    let url = format!("{ZEN_BASE_URL}/messages");
    let response = client
        .post(&url)
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
        provider_name.to_string(),
    ))
}

async fn stream_chat_completions(
    client: &Client,
    messages: &[PromptMessage],
    model: &str,
    api_key: &str,
    provider_name: &str,
) -> Result<BoxStream<'static, Result<ProviderStreamEvent, ProviderError>>, ProviderError> {
    let api_messages: Vec<ChatCompletionsMessage<'_>> = messages
        .iter()
        .map(|m| ChatCompletionsMessage {
            role: chat_role_to_openai(&m.role),
            content: &m.content,
        })
        .collect();

    let payload = ChatCompletionsRequest {
        model,
        stream: true,
        messages: api_messages,
    };

    let url = format!("{ZEN_BASE_URL}/chat/completions");
    let response = client
        .post(&url)
        .bearer_auth(api_key)
        .header("content-type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(ProviderError::Http)?
        .error_for_status()
        .map_err(ProviderError::Http)?;

    Ok(parse_openai_stream(
        response,
        model.to_string(),
        provider_name.to_string(),
    ))
}

fn chat_role_to_anthropic(role: &ChatRole) -> &'static str {
    match role {
        ChatRole::User => "user",
        ChatRole::Assistant => "assistant",
        ChatRole::System | ChatRole::Tool => "user",
    }
}

fn chat_role_to_openai(role: &ChatRole) -> &'static str {
    match role {
        ChatRole::User => "user",
        ChatRole::Assistant => "assistant",
        ChatRole::System => "system",
        ChatRole::Tool => "tool",
    }
}
