use reqwest::Client;

use crate::providers::openai_compatible::{self, ProviderChatCompletion};

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
pub struct PromptCompletion {
    pub content: String,
    pub model: String,
    pub provider: &'static str,
    pub finish_reason: Option<String>,
}

pub async fn complete_prompt(
    client: &Client,
    prompt: String,
    model: String,
    authorization: Option<&str>,
) -> Result<PromptCompletion, ChatServiceError> {
    let trimmed_prompt = prompt.trim();
    if trimmed_prompt.is_empty() {
        return Err(ChatServiceError::InvalidPrompt);
    }

    let trimmed_model = model.trim();
    if trimmed_model.is_empty() {
        return Err(ChatServiceError::InvalidModel);
    }

    let api_key = authorization
        .and_then(|value| value.strip_prefix("Bearer "))
        .or(authorization.and_then(|value| value.strip_prefix("bearer ")))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or(ChatServiceError::MissingApiKey)?;

    let ProviderChatCompletion {
        content,
        model,
        finish_reason,
    } = openai_compatible::complete_prompt(client, trimmed_prompt, trimmed_model, api_key).await?;

    Ok(PromptCompletion {
        content,
        model,
        provider: "openrouter",
        finish_reason,
    })
}
