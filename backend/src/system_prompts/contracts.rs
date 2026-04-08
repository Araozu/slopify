use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SystemPrompt {
    pub id: String,
    pub name: String,
    pub content: String,
}
