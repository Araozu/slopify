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
    services::copilot_token_service::{self, CopilotTokenServiceError},
    state::AppState,
};

#[derive(Deserialize)]
pub struct CreateCopilotTokenRequest {
    pub name: String,
    pub github_token: String,
}

#[derive(Deserialize)]
pub struct UpdateCopilotTokenRequest {
    pub name: Option<String>,
    pub github_token: Option<String>,
}

pub async fn list_copilot_tokens(
    State(state): State<AppState>,
    session: AuthSession,
) -> Response {
    match copilot_token_service::list_tokens(&state.db_pool, session.user_id).await {
        Ok(tokens) => (StatusCode::OK, Json(tokens)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn create_copilot_token(
    State(state): State<AppState>,
    session: AuthSession,
    Json(payload): Json<CreateCopilotTokenRequest>,
) -> Response {
    match copilot_token_service::create_token(
        &state.db_pool,
        session.user_id,
        payload.name,
        payload.github_token,
    )
    .await
    {
        Ok(token) => (StatusCode::CREATED, Json(token)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn update_copilot_token(
    State(state): State<AppState>,
    session: AuthSession,
    Path(token_id): Path<String>,
    Json(payload): Json<UpdateCopilotTokenRequest>,
) -> Response {
    let token_id = match parse_token_id(token_id) {
        Ok(id) => id,
        Err(error) => return error.into_response(),
    };

    match copilot_token_service::update_token(
        &state.db_pool,
        session.user_id,
        token_id,
        payload.name,
        payload.github_token,
    )
    .await
    {
        Ok(token) => (StatusCode::OK, Json(token)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn delete_copilot_token(
    State(state): State<AppState>,
    session: AuthSession,
    Path(token_id): Path<String>,
) -> Response {
    let token_id = match parse_token_id(token_id) {
        Ok(id) => id,
        Err(error) => return error.into_response(),
    };

    match copilot_token_service::delete_token(&state.db_pool, session.user_id, token_id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

struct ApiError {
    status: StatusCode,
    message: String,
}

impl From<CopilotTokenServiceError> for ApiError {
    fn from(value: CopilotTokenServiceError) -> Self {
        match value {
            CopilotTokenServiceError::InvalidName | CopilotTokenServiceError::InvalidToken => {
                Self {
                    status: StatusCode::BAD_REQUEST,
                    message: value.to_string(),
                }
            }
            CopilotTokenServiceError::TokenNotFound => Self {
                status: StatusCode::NOT_FOUND,
                message: value.to_string(),
            },
            CopilotTokenServiceError::DuplicateName => Self {
                status: StatusCode::CONFLICT,
                message: value.to_string(),
            },
            CopilotTokenServiceError::Storage(_) => Self {
                status: StatusCode::INTERNAL_SERVER_ERROR,
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
