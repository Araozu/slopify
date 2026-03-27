use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct OpenRouterApiKey {
    pub id: String,
    pub name: String,
    pub api_key: String,
}
