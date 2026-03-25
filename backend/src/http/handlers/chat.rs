use axum::{
    extract::State,
    Json,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::{
    services::chat_service::{self, ChatServiceError},
    state::AppState,
};

#[derive(Deserialize)]
pub struct PromptRequest {
    pub prompt: String,
    pub model: String,
}

#[derive(Serialize)]
pub struct PromptResponse {
    pub content: String,
    pub model: String,
    pub provider: &'static str,
    pub finish_reason: Option<String>,
}

pub async fn complete_prompt(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<PromptRequest>,
) -> Response {
    let authorization = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    match chat_service::complete_prompt(
        &state.http_client,
        payload.prompt,
        payload.model,
        authorization,
    )
    .await
    {
        Ok(completion) => (
            StatusCode::OK,
            Json(PromptResponse {
                content: completion.content,
                model: completion.model,
                provider: completion.provider,
                finish_reason: completion.finish_reason,
            }),
        )
            .into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn chat_options() -> StatusCode {
    StatusCode::NO_CONTENT
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

struct ApiError {
    status: StatusCode,
    message: String,
}

impl From<ChatServiceError> for ApiError {
    fn from(value: ChatServiceError) -> Self {
        match value {
            ChatServiceError::InvalidPrompt | ChatServiceError::InvalidModel => Self {
                status: StatusCode::BAD_REQUEST,
                message: value.to_string(),
            },
            ChatServiceError::MissingApiKey => Self {
                status: StatusCode::BAD_REQUEST,
                message: value.to_string(),
            },
            ChatServiceError::Provider(error) => Self {
                status: StatusCode::BAD_GATEWAY,
                message: error.to_string(),
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(ErrorResponse { error: self.message })).into_response()
    }
}
