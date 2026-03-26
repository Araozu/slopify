use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use crate::{
    services::thread_service::{self, ThreadServiceError},
    state::AppState,
};

#[derive(Deserialize)]
pub struct CreateThreadRequest {
    pub title: Option<String>,
}

pub async fn list_threads(State(state): State<AppState>) -> Response {
    match thread_service::list_threads(&state.db_pool).await {
        Ok(threads) => (StatusCode::OK, Json(threads)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn create_thread(
    State(state): State<AppState>,
    Json(payload): Json<CreateThreadRequest>,
) -> Response {
    match thread_service::create_thread(&state.db_pool, payload.title).await {
        Ok(thread) => (StatusCode::CREATED, Json(thread)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn thread_options() -> StatusCode {
    StatusCode::NO_CONTENT
}

struct ApiError {
    status: StatusCode,
    message: String,
}

impl From<ThreadServiceError> for ApiError {
    fn from(value: ThreadServiceError) -> Self {
        match value {
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
