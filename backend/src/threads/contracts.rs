use serde::Serialize;

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
    pub content: String,
    pub timestamp: String,
}
