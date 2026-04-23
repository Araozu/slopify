use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ZenApiKey {
    pub id: String,
    pub name: String,
    pub api_key: String,
}
