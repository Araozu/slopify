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
    services::go_key_service::{self, GoKeyServiceError},
    state::AppState,
};

#[derive(Deserialize)]
pub struct CreateGoKeyRequest {
    pub name: String,
    pub api_key: String,
}

#[derive(Deserialize)]
pub struct UpdateGoKeyRequest {
    pub name: Option<String>,
    pub api_key: Option<String>,
}

pub async fn list_go_keys(
    State(state): State<AppState>,
    session: AuthSession,
) -> Response {
    match go_key_service::list_keys(&state.db_pool, session.user_id).await {
        Ok(keys) => (StatusCode::OK, Json(keys)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn create_go_key(
    State(state): State<AppState>,
    session: AuthSession,
    Json(payload): Json<CreateGoKeyRequest>,
) -> Response {
    match go_key_service::create_key(
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

pub async fn update_go_key(
    State(state): State<AppState>,
    session: AuthSession,
    Path(key_id): Path<String>,
    Json(payload): Json<UpdateGoKeyRequest>,
) -> Response {
    let key_id = match parse_key_id(key_id) {
        Ok(key_id) => key_id,
        Err(error) => return error.into_response(),
    };

    match go_key_service::update_key(
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

pub async fn delete_go_key(
    State(state): State<AppState>,
    session: AuthSession,
    Path(key_id): Path<String>,
) -> Response {
    let key_id = match parse_key_id(key_id) {
        Ok(key_id) => key_id,
        Err(error) => return error.into_response(),
    };

    match go_key_service::delete_key(&state.db_pool, session.user_id, key_id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

struct ApiError {
    status: StatusCode,
    message: String,
}

impl From<GoKeyServiceError> for ApiError {
    fn from(value: GoKeyServiceError) -> Self {
        match value {
            GoKeyServiceError::InvalidName | GoKeyServiceError::InvalidApiKey => Self {
                status: StatusCode::BAD_REQUEST,
                message: value.to_string(),
            },
            GoKeyServiceError::KeyNotFound => Self {
                status: StatusCode::NOT_FOUND,
                message: value.to_string(),
            },
            GoKeyServiceError::DuplicateName => Self {
                status: StatusCode::CONFLICT,
                message: value.to_string(),
            },
            GoKeyServiceError::Storage(_) => Self {
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
