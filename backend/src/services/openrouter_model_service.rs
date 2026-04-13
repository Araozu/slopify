use sqlx::{PgPool, error::ErrorKind};
use uuid::Uuid;

use crate::{
    openrouter_models::contracts::OpenRouterModel,
    storage::openrouter_models as openrouter_model_storage,
};

#[derive(Debug)]
pub enum OpenRouterModelServiceError {
    InvalidModelId,
    ModelNotFound,
    DuplicateModel,
    Storage(sqlx::Error),
}

impl std::fmt::Display for OpenRouterModelServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidModelId => write!(f, "Enter a valid model ID."),
            Self::ModelNotFound => write!(f, "OpenRouter model not found."),
            Self::DuplicateModel => write!(f, "You already have this model saved."),
            Self::Storage(error) => write!(f, "failed to access OpenRouter model data: {error}"),
        }
    }
}

impl std::error::Error for OpenRouterModelServiceError {}

impl From<sqlx::Error> for OpenRouterModelServiceError {
    fn from(value: sqlx::Error) -> Self {
        if let sqlx::Error::Database(database_error) = &value
            && matches!(database_error.kind(), ErrorKind::UniqueViolation)
        {
            return Self::DuplicateModel;
        }

        Self::Storage(value)
    }
}

pub async fn list_models(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<OpenRouterModel>, OpenRouterModelServiceError> {
    let models = openrouter_model_storage::list_models(pool, user_id).await?;
    Ok(models.into_iter().map(map_model).collect())
}

pub async fn create_model(
    pool: &PgPool,
    user_id: Uuid,
    model_id: String,
) -> Result<OpenRouterModel, OpenRouterModelServiceError> {
    let normalized_model_id = normalize_model_id(model_id)?;
    
    // Check if it already exists to be idempotent or just return error
    if let Some(existing) = openrouter_model_storage::find_model_by_model_id(pool, user_id, &normalized_model_id).await? {
        return Ok(map_model(existing));
    }

    let model = openrouter_model_storage::create_model(
        pool,
        Uuid::new_v4(),
        user_id,
        &normalized_model_id,
    )
    .await?;

    Ok(map_model(model))
}

pub async fn delete_model(
    pool: &PgPool,
    user_id: Uuid,
    id: Uuid,
) -> Result<(), OpenRouterModelServiceError> {
    let deleted = openrouter_model_storage::delete_model(pool, id, user_id).await?;

    if !deleted {
        return Err(OpenRouterModelServiceError::ModelNotFound);
    }

    Ok(())
}

fn normalize_model_id(model_id: String) -> Result<String, OpenRouterModelServiceError> {
    let normalized = model_id.trim().to_string();

    if normalized.is_empty() {
        return Err(OpenRouterModelServiceError::InvalidModelId);
    }

    Ok(normalized)
}

fn map_model(record: openrouter_model_storage::OpenRouterModelRecord) -> OpenRouterModel {
    OpenRouterModel {
        id: record.id.to_string(),
        model_id: record.model_id,
    }
}
