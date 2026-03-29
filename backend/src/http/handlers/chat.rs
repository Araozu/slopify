use axum::{
    extract::{Path, State},
    Json,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    http::auth::AuthSession,
    services::{
        chat_service::{self, ChatServiceError},
        thread_service::{self, ThreadServiceError},
    },
    state::AppState,
};

#[derive(Deserialize)]
pub struct PromptRequest {
    pub prompt: String,
    pub model: String,
    pub thread_id: Option<Uuid>,
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
    session: AuthSession,
    headers: HeaderMap,
    Json(payload): Json<PromptRequest>,
) -> Response {
    let authorization = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    if let Some(thread_id) = payload.thread_id {
        if let Err(error) = thread_service::save_message(
            &state.db_pool,
            session.user_id,
            thread_id,
            "user",
            &payload.prompt,
        )
        .await
        {
            return ApiError::from(error).into_response();
        }
    }

    match chat_service::complete_prompt(
        &state.http_client,
        payload.prompt.clone(),
        payload.model.clone(),
        authorization,
    )
    .await
    {
        Ok(completion) => {
            if let Some(thread_id) = payload.thread_id {
                if let Err(error) = thread_service::save_message(
                    &state.db_pool,
                    session.user_id,
                    thread_id,
                    "assistant",
                    &completion.content,
                )
                .await
                {
                    return ApiError::from(error).into_response();
                }

                if let Err(error) = thread_service::update_thread_model(
                    &state.db_pool,
                    session.user_id,
                    thread_id,
                    &completion.model,
                )
                .await
                {
                    return ApiError::from(error).into_response();
                }
            }

            (
                StatusCode::OK,
                Json(PromptResponse {
                    content: completion.content,
                    model: completion.model,
                    provider: completion.provider,
                    finish_reason: completion.finish_reason,
                }),
            )
                .into_response()
        }
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn list_thread_messages(
    State(state): State<AppState>,
    session: AuthSession,
    Path(thread_id): Path<Uuid>,
) -> Response {
    match thread_service::list_messages(&state.db_pool, session.user_id, thread_id).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
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
                status: StatusCode::UNAUTHORIZED,
                message: value.to_string(),
            },
            ChatServiceError::Provider(error) => Self {
                status: StatusCode::BAD_GATEWAY,
                message: error.to_string(),
            },
        }
    }
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
        (self.status, Json(ErrorResponse { error: self.message })).into_response()
    }
}
