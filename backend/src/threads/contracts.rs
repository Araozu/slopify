use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct Thread {
    pub id: String,
    pub title: String,
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub id: String,
    pub role: String,
    pub status: String,
    pub parts: Vec<MessagePart>,
    pub provider: Value,
    pub metadata: Value,
    pub content: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MessagePart {
    Text { text: String },
    Reasoning { text: String },
}
