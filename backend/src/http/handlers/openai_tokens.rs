use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    http::auth::AuthSession,
    services::openai_token_service::{self, OpenAiTokenServiceError},
    state::AppState,
};

#[derive(Deserialize)]
pub struct CreateOpenAiTokenRequest {
    pub name: String,
    pub token: String,
}

#[derive(Deserialize)]
pub struct PollDeviceCodeRequest {
    pub device_auth_id: String,
    pub user_code: String,
    pub name: String,
}

pub async fn list_openai_tokens(State(state): State<AppState>, session: AuthSession) -> Response {
    match openai_token_service::list_tokens(&state.db_pool, session.user_id).await {
        Ok(tokens) => (StatusCode::OK, Json(tokens)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn create_openai_token(
    State(state): State<AppState>,
    session: AuthSession,
    Json(payload): Json<CreateOpenAiTokenRequest>,
) -> Response {
    match openai_token_service::create_token(&state.db_pool, session.user_id, payload.name, payload.token)
        .await
    {
        Ok(token) => (StatusCode::CREATED, Json(token)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn delete_openai_token(
    State(state): State<AppState>,
    session: AuthSession,
    Path(token_id): Path<String>,
) -> Response {
    let token_id = match parse_token_id(token_id) {
        Ok(id) => id,
        Err(error) => return error.into_response(),
    };

    match openai_token_service::delete_token(&state.db_pool, session.user_id, token_id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn initiate_device_code(State(state): State<AppState>, _session: AuthSession) -> Response {
    match openai_token_service::initiate_device_code(&state.http_client).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn poll_device_code(
    State(state): State<AppState>,
    session: AuthSession,
    Json(payload): Json<PollDeviceCodeRequest>,
) -> Response {
    match openai_token_service::poll_device_code(
        &state.http_client,
        &state.db_pool,
        session.user_id,
        &payload.device_auth_id,
        &payload.user_code,
        &payload.name,
    )
    .await
    {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

struct ApiError {
    status: StatusCode,
    message: String,
}

impl From<OpenAiTokenServiceError> for ApiError {
    fn from(value: OpenAiTokenServiceError) -> Self {
        match value {
            OpenAiTokenServiceError::InvalidName | OpenAiTokenServiceError::InvalidToken => Self {
                status: StatusCode::BAD_REQUEST,
                message: value.to_string(),
            },
            OpenAiTokenServiceError::TokenNotFound => Self {
                status: StatusCode::NOT_FOUND,
                message: value.to_string(),
            },
            OpenAiTokenServiceError::DuplicateName => Self {
                status: StatusCode::CONFLICT,
                message: value.to_string(),
            },
            OpenAiTokenServiceError::Storage(_) => Self {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: value.to_string(),
            },
            OpenAiTokenServiceError::DeviceFlowFailed(_) => Self {
                status: StatusCode::BAD_GATEWAY,
                message: value.to_string(),
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(serde_json::json!({
                "error": self.message,
            })),
        )
            .into_response()
    }
}

fn parse_token_id(token_id: String) -> Result<Uuid, ApiError> {
    Uuid::parse_str(&token_id).map_err(|_| ApiError {
        status: StatusCode::BAD_REQUEST,
        message: "Enter a valid token ID.".to_string(),
    })
}
