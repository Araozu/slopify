use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenAiAuthType {
    ApiKey,
    OAuthRefreshToken,
}

#[derive(Debug, Clone, Serialize)]
pub struct OpenAiToken {
    pub id: String,
    pub name: String,
    pub auth_type: OpenAiAuthType,
    pub token: String,
}
