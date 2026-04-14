use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CopilotToken {
    pub id: String,
    pub name: String,
    pub github_token: String,
}
