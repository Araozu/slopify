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
    services::system_prompt_service::{self, SystemPromptServiceError},
    state::AppState,
};

#[derive(Deserialize)]
pub struct CreateSystemPromptRequest {
    pub name: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdateSystemPromptRequest {
    pub name: Option<String>,
    pub content: Option<String>,
}

pub async fn list_system_prompts(
    State(state): State<AppState>,
    session: AuthSession,
) -> Response {
    match system_prompt_service::list_system_prompts(&state.db_pool, session.user_id).await {
        Ok(prompts) => (StatusCode::OK, Json(prompts)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn create_system_prompt(
    State(state): State<AppState>,
    session: AuthSession,
    Json(payload): Json<CreateSystemPromptRequest>,
) -> Response {
    match system_prompt_service::create_system_prompt(
        &state.db_pool,
        session.user_id,
        payload.name,
        payload.content,
    )
    .await
    {
        Ok(prompt) => (StatusCode::CREATED, Json(prompt)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn update_system_prompt(
    State(state): State<AppState>,
    session: AuthSession,
    Path(prompt_id): Path<String>,
    Json(payload): Json<UpdateSystemPromptRequest>,
) -> Response {
    let prompt_id = match parse_prompt_id(prompt_id) {
        Ok(id) => id,
        Err(error) => return error.into_response(),
    };

    match system_prompt_service::update_system_prompt(
        &state.db_pool,
        session.user_id,
        prompt_id,
        payload.name,
        payload.content,
    )
    .await
    {
        Ok(prompt) => (StatusCode::OK, Json(prompt)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn delete_system_prompt(
    State(state): State<AppState>,
    session: AuthSession,
    Path(prompt_id): Path<String>,
) -> Response {
    let prompt_id = match parse_prompt_id(prompt_id) {
        Ok(id) => id,
        Err(error) => return error.into_response(),
    };

    match system_prompt_service::delete_system_prompt(&state.db_pool, session.user_id, prompt_id)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

struct ApiError {
    status: StatusCode,
    message: String,
}

impl From<SystemPromptServiceError> for ApiError {
    fn from(value: SystemPromptServiceError) -> Self {
        match value {
            SystemPromptServiceError::InvalidName | SystemPromptServiceError::InvalidContent => {
                Self {
                    status: StatusCode::BAD_REQUEST,
                    message: value.to_string(),
                }
            }
            SystemPromptServiceError::NotFound => Self {
                status: StatusCode::NOT_FOUND,
                message: value.to_string(),
            },
            SystemPromptServiceError::DuplicateName => Self {
                status: StatusCode::CONFLICT,
                message: value.to_string(),
            },
            SystemPromptServiceError::Storage(_) => Self {
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

fn parse_prompt_id(prompt_id: String) -> Result<Uuid, ApiError> {
    Uuid::parse_str(&prompt_id).map_err(|_| ApiError {
        status: StatusCode::BAD_REQUEST,
        message: "Enter a valid system prompt ID.".to_string(),
    })
}
