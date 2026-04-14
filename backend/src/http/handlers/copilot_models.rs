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
    services::copilot_model_service::{self, CopilotModelServiceError},
    state::AppState,
};

#[derive(Deserialize)]
pub struct CreateCopilotModelRequest {
    pub model_id: String,
}

pub async fn list_copilot_models(
    State(state): State<AppState>,
    session: AuthSession,
) -> Response {
    match copilot_model_service::list_models(&state.db_pool, session.user_id).await {
        Ok(models) => (StatusCode::OK, Json(models)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn create_copilot_model(
    State(state): State<AppState>,
    session: AuthSession,
    Json(payload): Json<CreateCopilotModelRequest>,
) -> Response {
    match copilot_model_service::create_model(
        &state.db_pool,
        session.user_id,
        payload.model_id,
    )
    .await
    {
        Ok(model) => (StatusCode::CREATED, Json(model)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn delete_copilot_model(
    State(state): State<AppState>,
    session: AuthSession,
    Path(id): Path<String>,
) -> Response {
    let id = match parse_id(id) {
        Ok(id) => id,
        Err(error) => return error.into_response(),
    };

    match copilot_model_service::delete_model(&state.db_pool, session.user_id, id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

struct ApiError {
    status: StatusCode,
    message: String,
}

impl From<CopilotModelServiceError> for ApiError {
    fn from(value: CopilotModelServiceError) -> Self {
        match value {
            CopilotModelServiceError::InvalidModelId => Self {
                status: StatusCode::BAD_REQUEST,
                message: value.to_string(),
            },
            CopilotModelServiceError::ModelNotFound => Self {
                status: StatusCode::NOT_FOUND,
                message: value.to_string(),
            },
            CopilotModelServiceError::DuplicateModel => Self {
                status: StatusCode::CONFLICT,
                message: value.to_string(),
            },
            CopilotModelServiceError::Storage(_) => Self {
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

fn parse_id(id: String) -> Result<Uuid, ApiError> {
    Uuid::parse_str(&id).map_err(|_| ApiError {
        status: StatusCode::BAD_REQUEST,
        message: "Enter a valid ID.".to_string(),
    })
}
