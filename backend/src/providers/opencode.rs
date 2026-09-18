use async_trait::async_trait;
use futures_util::stream::BoxStream;
use reqwest::Client;
use serde::Serialize;

use crate::chat::contracts::{ChatRole, PromptMessage};

use super::adapter::{ProviderAdapter, ProviderError, ProviderStreamEvent};
use super::anthropic_stream::parse_anthropic_stream;
use super::openai_compat_stream::parse_openai_stream;

const ZEN_BASE_URL: &str = "https://opencode.ai/zen/v1";
const GO_BASE_URL: &str = "https://opencode.ai/zen/go/v1";
const ANTHROPIC_API_VERSION: &str = "2023-06-01";

/// Shared adapter for the OpenCode gateways (Zen pay-as-you-go and Go
/// subscription). Both expose the same two wire formats — Anthropic Messages
/// for select model families, OpenAI Chat Completions for the rest — but
/// differ in base URL, credential, and which families use Messages.
pub struct OpenCodeAdapter {
    name: &'static str,
    base_url: &'static str,
    config_prefix: &'static str,
    messages_prefixes: &'static [&'static str],
}

impl OpenCodeAdapter {
    pub fn zen() -> Self {
        Self {
            name: "opencode-zen",
            base_url: ZEN_BASE_URL,
            config_prefix: "opencode/",
            messages_prefixes: &["claude-", "qwen"],
        }
    }

    pub fn go() -> Self {
        Self {
            name: "opencode-go",
            base_url: GO_BASE_URL,
            config_prefix: "opencode-go/",
            messages_prefixes: &["qwen", "minimax"],
        }
    }

    /// Canonical wire ID: gateway model IDs are bare (e.g. `kimi-k3`), but
    /// users coming from OpenCode config may type the `opencode[-go]/<id>`
    /// form. Both prefixes are stripped; they are disjoint so order is safe.
    fn wire_model<'a>(&self, model: &'a str) -> &'a str {
        let stripped = strip_prefix_ci(model, self.config_prefix);
        if self.config_prefix == "opencode/" {
            strip_prefix_ci(stripped, "opencode-go/")
        } else {
            strip_prefix_ci(stripped, "opencode/")
        }
    }

    fn is_messages_model(&self, wire_model: &str) -> bool {
        let normalized = wire_model.to_lowercase();
        self.messages_prefixes
            .iter()
            .any(|prefix| normalized.starts_with(prefix))
    }
}

fn strip_prefix_ci<'a>(value: &'a str, prefix: &str) -> &'a str {
    if value.len() >= prefix.len() && value[..prefix.len()].eq_ignore_ascii_case(prefix) {
        &value[prefix.len()..]
    } else {
        value
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
impl ProviderAdapter for OpenCodeAdapter {
    fn name(&self) -> &str {
        self.name
    }

    fn endpoint(&self) -> &str {
        self.base_url
    }

    async fn stream_prompt(
        &self,
        client: &Client,
        messages: &[PromptMessage],
        model: &str,
        api_key: &str,
    ) -> Result<BoxStream<'static, Result<ProviderStreamEvent, ProviderError>>, ProviderError> {
        let wire_model = self.wire_model(model);
        if self.is_messages_model(wire_model) {
            self.stream_anthropic(client, messages, wire_model, api_key)
                .await
        } else {
            self.stream_chat_completions(client, messages, wire_model, api_key)
                .await
        }
    }
}

impl OpenCodeAdapter {
    async fn stream_anthropic(
        &self,
        client: &Client,
        messages: &[PromptMessage],
        model: &str,
        api_key: &str,
    ) -> Result<BoxStream<'static, Result<ProviderStreamEvent, ProviderError>>, ProviderError> {
        if messages.iter().any(|m| matches!(m.role, ChatRole::Tool)) {
            return Err(ProviderError::Other(
                format!("{} Messages path does not support Tool messages", self.name).into(),
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

        let mut merged: Vec<(&'static str, String)> = Vec::new();
        for m in messages
            .iter()
            .filter(|m| !matches!(m.role, ChatRole::System))
        {
            let role = chat_role_to_anthropic(&m.role);
            match merged.last_mut() {
                Some((last_role, last_content)) if *last_role == role => {
                    last_content.push_str("\n\n");
                    last_content.push_str(&m.content);
                }
                _ => merged.push((role, m.content.clone())),
            }
        }
        let api_messages: Vec<AnthropicRequestMessage<'_>> = merged
            .iter()
            .map(|(role, content)| AnthropicRequestMessage {
                role,
                content: content.as_str(),
            })
            .collect();

        let payload = AnthropicRequest {
            model,
            max_tokens: 8192,
            stream: true,
            system: system_prompt.as_deref(),
            messages: api_messages,
        };

        let url = format!("{}/messages", self.base_url);
        let response = client
            .post(&url)
            .header("x-api-key", api_key)
            .header("anthropic-version", ANTHROPIC_API_VERSION)
            .header("content-type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(ProviderError::Http)?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(ProviderError::Other(
                format!("{} Messages request failed ({status}): {body}", self.name).into(),
            ));
        }

        Ok(parse_anthropic_stream(
            response,
            model.to_string(),
            self.name.to_string(),
        ))
    }

    async fn stream_chat_completions(
        &self,
        client: &Client,
        messages: &[PromptMessage],
        model: &str,
        api_key: &str,
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

        let url = format!("{}/chat/completions", self.base_url);
        let response = client
            .post(&url)
            .bearer_auth(api_key)
            .header("content-type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(ProviderError::Http)?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(ProviderError::Other(
                format!("{} Chat Completions request failed ({status}): {body}", self.name).into(),
            ));
        }

        Ok(parse_openai_stream(
            response,
            model.to_string(),
            self.name.to_string(),
        ))
    }
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
        // Chat Completions tool messages require tool_call_id, which this app
        // never produces; downgrade to user text instead of sending a 400.
        ChatRole::Tool => "user",
    }
}
