use async_trait::async_trait;
use futures_util::stream::BoxStream;
use reqwest::Client;
use serde_json::Value;

use crate::chat::contracts::PromptMessage;

/// The normalized event shape every provider adapter must produce.
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

/// A unified error type for provider failures.
#[derive(Debug)]
pub enum ProviderError {
    Http(reqwest::Error),
    InvalidPayload(serde_json::Error),
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(error) => write!(f, "provider HTTP error: {error}"),
            Self::InvalidPayload(error) => write!(f, "provider returned invalid payload: {error}"),
            Self::Other(error) => write!(f, "provider error: {error}"),
        }
    }
}

impl std::error::Error for ProviderError {}

/// The contract every provider adapter implements.
#[async_trait]
pub trait ProviderAdapter: Send + Sync {
    /// A stable identifier for this provider (e.g. "openrouter", "anthropic").
    fn name(&self) -> &str;

    /// The base endpoint URL used by this adapter.
    fn endpoint(&self) -> &str;

    /// Open a streaming completion and return a normalized event stream.
    async fn stream_prompt(
        &self,
        client: &Client,
        messages: &[PromptMessage],
        model: &str,
        api_key: &str,
    ) -> Result<BoxStream<'static, Result<ProviderStreamEvent, ProviderError>>, ProviderError>;
}
