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
    services::thread_service::{self, ThreadServiceError},
    state::AppState,
};

#[derive(Deserialize)]
pub struct CreateThreadRequest {
    pub title: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateThreadRequest {
    pub title: String,
}

pub async fn list_threads(State(state): State<AppState>, session: AuthSession) -> Response {
    match thread_service::list_threads(&state.db_pool, session.user_id).await {
        Ok(threads) => (StatusCode::OK, Json(threads)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn create_thread(
    State(state): State<AppState>,
    session: AuthSession,
    Json(payload): Json<CreateThreadRequest>,
) -> Response {
    match thread_service::create_thread(&state.db_pool, session.user_id, payload.title).await {
        Ok(thread) => (StatusCode::CREATED, Json(thread)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn thread_options() -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn update_thread(
    State(state): State<AppState>,
    session: AuthSession,
    Path(thread_id): Path<String>,
    Json(payload): Json<UpdateThreadRequest>,
) -> Response {
    let thread_id = match parse_thread_id(thread_id) {
        Ok(thread_id) => thread_id,
        Err(error) => return error.into_response(),
    };

    match thread_service::update_thread_title(&state.db_pool, session.user_id, thread_id, &payload.title)
        .await
    {
        Ok(thread) => (StatusCode::OK, Json(thread)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn delete_thread(
    State(state): State<AppState>,
    session: AuthSession,
    Path(thread_id): Path<String>,
) -> Response {
    let thread_id = match parse_thread_id(thread_id) {
        Ok(thread_id) => thread_id,
        Err(error) => return error.into_response(),
    };

    match thread_service::delete_thread(&state.db_pool, session.user_id, thread_id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

struct ApiError {
    status: StatusCode,
    message: String,
}

impl From<ThreadServiceError> for ApiError {
    fn from(value: ThreadServiceError) -> Self {
        match value {
            ThreadServiceError::NotFound => Self {
                status: StatusCode::NOT_FOUND,
                message: value.to_string(),
            },
            ThreadServiceError::Storage(_) => Self {
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

fn parse_thread_id(thread_id: String) -> Result<Uuid, ApiError> {
    Uuid::parse_str(&thread_id).map_err(|_| ApiError {
        status: StatusCode::BAD_REQUEST,
        message: "Enter a valid thread ID.".to_string(),
    })
}
