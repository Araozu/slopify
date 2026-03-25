use axum::{
    Json,
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::services::chat_service::{self, ChatServiceError};

const ACCESS_CONTROL_ALLOW_HEADERS: &str = "content-type";
const ACCESS_CONTROL_ALLOW_METHODS: &str = "POST, OPTIONS";
const ACCESS_CONTROL_ALLOW_ORIGIN: &str = "*";

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

pub async fn complete_prompt(Json(payload): Json<PromptRequest>) -> Response {
    let client = Client::new();

    match chat_service::complete_prompt(&client, payload.prompt, payload.model).await {
        Ok(completion) => with_cors(
            StatusCode::OK,
            Json(PromptResponse {
                content: completion.content,
                model: completion.model,
                provider: completion.provider,
                finish_reason: completion.finish_reason,
            }),
        ),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn chat_options() -> Response {
    with_cors(StatusCode::NO_CONTENT, ())
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
            ChatServiceError::Provider(error) => Self {
                status: StatusCode::BAD_GATEWAY,
                message: error.to_string(),
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        with_cors(self.status, Json(ErrorResponse { error: self.message }))
    }
}

fn with_cors(status: StatusCode, body: impl IntoResponse) -> Response {
    let mut response = (status, body).into_response();
    let headers = response.headers_mut();
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static(ACCESS_CONTROL_ALLOW_ORIGIN),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static(ACCESS_CONTROL_ALLOW_METHODS),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static(ACCESS_CONTROL_ALLOW_HEADERS),
    );
    response
}
