use futures_util::{StreamExt, stream::BoxStream};
use reqwest::Client;
use serde_json::Value;

use crate::{
    chat::contracts::PromptMessage,
    providers::openai_compatible::{self, ProviderStreamEvent},
};

#[derive(Debug)]
pub enum ChatServiceError {
    InvalidPrompt,
    InvalidModel,
    MissingApiKey,
    Provider(openai_compatible::OpenAiCompatibleError),
}

impl std::fmt::Display for ChatServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPrompt => write!(f, "prompt is required"),
            Self::InvalidModel => write!(f, "model is required"),
            Self::MissingApiKey => write!(f, "an authorization bearer token is required"),
            Self::Provider(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for ChatServiceError {}

impl From<openai_compatible::OpenAiCompatibleError> for ChatServiceError {
    fn from(value: openai_compatible::OpenAiCompatibleError) -> Self {
        Self::Provider(value)
    }
}

#[derive(Debug, Clone)]
pub enum ChatServiceStreamEvent {
    TextDelta(String),
    ReasoningDelta(String),
    Completed {
        model: String,
        finish_reason: Option<String>,
        vendor_metadata: Value,
    },
}

pub async fn stream_prompt(
    client: &Client,
    prompt: String,
    messages: Vec<PromptMessage>,
    model: String,
    authorization: Option<&str>,
) -> Result<BoxStream<'static, Result<ChatServiceStreamEvent, ChatServiceError>>, ChatServiceError> {
    let (_trimmed_prompt, trimmed_model, api_key) = validate_request(&prompt, &model, authorization)?;
    let stream = openai_compatible::stream_prompt(client, &messages, trimmed_model, api_key).await?;

    let mapped_stream = stream.map(|event| match event {
        Ok(ProviderStreamEvent::TextDelta(delta)) => Ok(ChatServiceStreamEvent::TextDelta(delta)),
        Ok(ProviderStreamEvent::ReasoningDelta(delta)) => {
            Ok(ChatServiceStreamEvent::ReasoningDelta(delta))
        }
        Ok(ProviderStreamEvent::Completed {
            model,
            finish_reason,
            vendor_metadata,
        }) => Ok(ChatServiceStreamEvent::Completed {
            model,
            finish_reason,
            vendor_metadata,
        }),
        Err(error) => Err(ChatServiceError::Provider(error)),
    });

    Ok(Box::pin(mapped_stream))
}

fn validate_request<'a>(
    prompt: &'a str,
    model: &'a str,
    authorization: Option<&'a str>,
) -> Result<(&'a str, &'a str, &'a str), ChatServiceError> {
    let trimmed_prompt = prompt.trim();
    if trimmed_prompt.is_empty() {
        return Err(ChatServiceError::InvalidPrompt);
    }

    let trimmed_model = model.trim();
    if trimmed_model.is_empty() {
        return Err(ChatServiceError::InvalidModel);
    }

    let api_key = authorization
        .map(str::trim)
        .and_then(|value| {
            let (scheme, token) = value.split_once(' ')?;
            scheme.eq_ignore_ascii_case("Bearer").then_some(token.trim())
        })
        .filter(|value| !value.is_empty())
        .ok_or(ChatServiceError::MissingApiKey)?;

    Ok((trimmed_prompt, trimmed_model, api_key))
}
