use reqwest::Client;
use serde::{Deserialize, Serialize};

const OPENROUTER_API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";
// FIXME: Move this API key to runtime configuration before shipping this anywhere real.
const OPENROUTER_API_KEY: &str = "sk-or-v1-fixme-hardcoded-openrouter-key";

#[derive(Debug, Clone)]
pub struct ProviderChatCompletion {
    pub content: String,
    pub model: String,
    pub finish_reason: Option<String>,
}

#[derive(Debug)]
pub enum OpenAiCompatibleError {
    Http(reqwest::Error),
    EmptyResponse,
}

impl std::fmt::Display for OpenAiCompatibleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(error) => write!(f, "failed to call OpenAI-compatible endpoint: {error}"),
            Self::EmptyResponse => write!(f, "provider returned an empty completion"),
        }
    }
}

impl std::error::Error for OpenAiCompatibleError {}

impl From<reqwest::Error> for OpenAiCompatibleError {
    fn from(value: reqwest::Error) -> Self {
        Self::Http(value)
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
struct OpenAiCompatibleResponse {
    model: Option<String>,
    choices: Vec<OpenAiCompatibleChoice>,
}

#[derive(Deserialize)]
struct OpenAiCompatibleChoice {
    finish_reason: Option<String>,
    message: OpenAiCompatibleMessage,
}

#[derive(Deserialize)]
struct OpenAiCompatibleMessage {
    content: Option<OpenAiCompatibleMessageContent>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum OpenAiCompatibleMessageContent {
    Text(String),
    Parts(Vec<OpenAiCompatibleMessagePart>),
}

#[derive(Deserialize)]
struct OpenAiCompatibleMessagePart {
    text: Option<String>,
}

impl OpenAiCompatibleMessageContent {
    fn into_text(self) -> String {
        match self {
            Self::Text(content) => content,
            Self::Parts(parts) => parts
                .into_iter()
                .filter_map(|part| part.text)
                .collect::<Vec<_>>()
                .join(""),
        }
    }
}

pub async fn complete_prompt(
    client: &Client,
    prompt: &str,
    model: &str,
) -> Result<ProviderChatCompletion, OpenAiCompatibleError> {
    let payload = OpenAiCompatibleRequest {
        model,
        stream: false,
        messages: vec![OpenAiCompatibleRequestMessage {
            role: "user",
            content: prompt,
        }],
    };

    let response = client
        .post(OPENROUTER_API_URL)
        .bearer_auth(OPENROUTER_API_KEY)
        .header("HTTP-Referer", "https://github.com/Araozu/slopify")
        .header("X-Title", "Slopify")
        .json(&payload)
        .send()
        .await?
        .error_for_status()?
        .json::<OpenAiCompatibleResponse>()
        .await?;

    let choice = response
        .choices
        .into_iter()
        .next()
        .ok_or(OpenAiCompatibleError::EmptyResponse)?;
    let content = choice
        .message
        .content
        .map(OpenAiCompatibleMessageContent::into_text)
        .unwrap_or_default();

    if content.is_empty() {
        return Err(OpenAiCompatibleError::EmptyResponse);
    }

    Ok(ProviderChatCompletion {
        content,
        model: response.model.unwrap_or_else(|| model.to_string()),
        finish_reason: choice.finish_reason,
    })
}
