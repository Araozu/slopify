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
    services::openrouter_key_service::{self, OpenRouterKeyServiceError},
    state::AppState,
};

#[derive(Deserialize)]
pub struct CreateOpenRouterKeyRequest {
    pub name: String,
    pub api_key: String,
}

#[derive(Deserialize)]
pub struct UpdateOpenRouterKeyRequest {
    pub name: Option<String>,
    pub api_key: Option<String>,
}

pub async fn list_openrouter_keys(
    State(state): State<AppState>,
    session: AuthSession,
) -> Response {
    match openrouter_key_service::list_keys(&state.db_pool, session.user_id).await {
        Ok(keys) => (StatusCode::OK, Json(keys)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn create_openrouter_key(
    State(state): State<AppState>,
    session: AuthSession,
    Json(payload): Json<CreateOpenRouterKeyRequest>,
) -> Response {
    match openrouter_key_service::create_key(
        &state.db_pool,
        session.user_id,
        payload.name,
        payload.api_key,
    )
    .await
    {
        Ok(key) => (StatusCode::CREATED, Json(key)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn update_openrouter_key(
    State(state): State<AppState>,
    session: AuthSession,
    Path(key_id): Path<String>,
    Json(payload): Json<UpdateOpenRouterKeyRequest>,
) -> Response {
    let key_id = match parse_key_id(key_id) {
        Ok(key_id) => key_id,
        Err(error) => return error.into_response(),
    };

    match openrouter_key_service::update_key(
        &state.db_pool,
        session.user_id,
        key_id,
        payload.name,
        payload.api_key,
    )
    .await
    {
        Ok(key) => (StatusCode::OK, Json(key)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn delete_openrouter_key(
    State(state): State<AppState>,
    session: AuthSession,
    Path(key_id): Path<String>,
) -> Response {
    let key_id = match parse_key_id(key_id) {
        Ok(key_id) => key_id,
        Err(error) => return error.into_response(),
    };

    match openrouter_key_service::delete_key(&state.db_pool, session.user_id, key_id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

struct ApiError {
    status: StatusCode,
    message: String,
}

impl From<OpenRouterKeyServiceError> for ApiError {
    fn from(value: OpenRouterKeyServiceError) -> Self {
        match value {
            OpenRouterKeyServiceError::InvalidName
            | OpenRouterKeyServiceError::InvalidApiKey => Self {
                status: StatusCode::BAD_REQUEST,
                message: value.to_string(),
            },
            OpenRouterKeyServiceError::KeyNotFound => Self {
                status: StatusCode::NOT_FOUND,
                message: value.to_string(),
            },
            OpenRouterKeyServiceError::DuplicateName => Self {
                status: StatusCode::CONFLICT,
                message: value.to_string(),
            },
            OpenRouterKeyServiceError::Storage(_) => Self {
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

fn parse_key_id(key_id: String) -> Result<Uuid, ApiError> {
    Uuid::parse_str(&key_id).map_err(|_| ApiError {
        status: StatusCode::BAD_REQUEST,
        message: "Enter a valid key ID.".to_string(),
    })
}
