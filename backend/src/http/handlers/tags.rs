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
    services::tag_service::{self, TagServiceError},
    state::AppState,
};

#[derive(Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Deserialize)]
pub struct AddTagToThreadRequest {
    pub tag_id: String,
}

pub async fn list_tags(State(state): State<AppState>, session: AuthSession) -> Response {
    match tag_service::list_tags(&state.db_pool, session.user_id).await {
        Ok(tags) => (StatusCode::OK, Json(tags)).into_response(),
        Err(e) => ApiError::from(e).into_response(),
    }
}

pub async fn create_tag(
    State(state): State<AppState>,
    session: AuthSession,
    Json(payload): Json<CreateTagRequest>,
) -> Response {
    let name = payload.name.trim().to_string();
    if name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "Tag name is required." })),
        )
            .into_response();
    }
    let color = payload.color.unwrap_or_else(|| "#6366f1".to_string());
    match tag_service::create_tag(&state.db_pool, session.user_id, &name, &color).await {
        Ok(tag) => (StatusCode::CREATED, Json(tag)).into_response(),
        Err(e) => ApiError::from(e).into_response(),
    }
}

pub async fn delete_tag(
    State(state): State<AppState>,
    session: AuthSession,
    Path(tag_id): Path<String>,
) -> Response {
    let tag_id = match Uuid::parse_str(&tag_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "Invalid tag ID." })),
            )
                .into_response();
        }
    };
    match tag_service::delete_tag(&state.db_pool, session.user_id, tag_id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => ApiError::from(e).into_response(),
    }
}

pub async fn add_tag_to_thread(
    State(state): State<AppState>,
    session: AuthSession,
    Path(thread_id): Path<String>,
    Json(payload): Json<AddTagToThreadRequest>,
) -> Response {
    let thread_id = match Uuid::parse_str(&thread_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "Invalid thread ID." })),
            )
                .into_response();
        }
    };
    let tag_id = match Uuid::parse_str(&payload.tag_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "Invalid tag ID." })),
            )
                .into_response();
        }
    };
    match tag_service::add_tag_to_thread(&state.db_pool, session.user_id, thread_id, tag_id).await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => ApiError::from(e).into_response(),
    }
}

pub async fn remove_tag_from_thread(
    State(state): State<AppState>,
    session: AuthSession,
    Path((thread_id, tag_id)): Path<(String, String)>,
) -> Response {
    let thread_id = match Uuid::parse_str(&thread_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "Invalid thread ID." })),
            )
                .into_response();
        }
    };
    let tag_id = match Uuid::parse_str(&tag_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "Invalid tag ID." })),
            )
                .into_response();
        }
    };
    match tag_service::remove_tag_from_thread(
        &state.db_pool,
        session.user_id,
        thread_id,
        tag_id,
    )
    .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => ApiError::from(e).into_response(),
    }
}

struct ApiError {
    status: StatusCode,
    message: String,
}

impl From<TagServiceError> for ApiError {
    fn from(value: TagServiceError) -> Self {
        match value {
            TagServiceError::NotFound => Self {
                status: StatusCode::NOT_FOUND,
                message: value.to_string(),
            },
            TagServiceError::Storage(_) => Self {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: value.to_string(),
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(serde_json::json!({ "error": self.message }))).into_response()
    }
}
