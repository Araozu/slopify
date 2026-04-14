use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopilotModel {
    pub id: String,
    pub model_id: String,
}
