use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatRole {
    System,
    User,
    Assistant,
    Tool,
}

impl ChatRole {
    pub fn parse(input: &str) -> Option<Self> {
        match input {
            "system" => Some(Self::System),
            "user" => Some(Self::User),
            "assistant" => Some(Self::Assistant),
            "tool" => Some(Self::Tool),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptMessage {
    pub role: ChatRole,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatMessageStatus {
    Streaming,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderDescriptor {
    pub provider: String,
    pub model: String,
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ClientChatPart {
    Text { text: String },
    Reasoning { text: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientChatMessageMetadata {
    pub finish_reason: Option<String>,
    pub vendor_metadata: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientChatMessage {
    pub id: String,
    pub conversation_id: String,
    pub role: ChatRole,
    pub status: ChatMessageStatus,
    pub parts: Vec<ClientChatPart>,
    pub provider: ProviderDescriptor,
    pub created_at: String,
    pub metadata: ClientChatMessageMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientChatError {
    pub code: String,
    pub message: String,
    pub retryable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientChatEvent {
    MessageStarted {
        message: ClientChatMessage,
    },
    TextDelta {
        message_id: String,
        delta: String,
    },
    ReasoningDelta {
        message_id: String,
        delta: String,
    },
    MessageCompleted {
        message: ClientChatMessage,
    },
    MessageFailed {
        message_id: String,
        error: ClientChatError,
    },
}
