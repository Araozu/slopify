use sqlx::{PgPool, error::ErrorKind};
use uuid::Uuid;

use crate::{
    openrouter_keys::contracts::OpenRouterApiKey,
    storage::openrouter_keys as openrouter_key_storage,
};

const MAX_NAME_LENGTH: usize = 80;
const MAX_KEY_LENGTH: usize = 512;

#[derive(Debug)]
pub enum OpenRouterKeyServiceError {
    InvalidName,
    InvalidApiKey,
    KeyNotFound,
    DuplicateName,
    Storage(sqlx::Error),
}

impl std::fmt::Display for OpenRouterKeyServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidName => {
                write!(f, "Enter a name between 1 and {MAX_NAME_LENGTH} characters.")
            }
            Self::InvalidApiKey => {
                write!(f, "Enter an API key between 1 and {MAX_KEY_LENGTH} characters.")
            }
            Self::KeyNotFound => write!(f, "OpenRouter API key not found."),
            Self::DuplicateName => write!(f, "You already have a key with that name."),
            Self::Storage(error) => write!(f, "failed to access OpenRouter key data: {error}"),
        }
    }
}

impl std::error::Error for OpenRouterKeyServiceError {}

impl From<sqlx::Error> for OpenRouterKeyServiceError {
    fn from(value: sqlx::Error) -> Self {
        if let sqlx::Error::Database(database_error) = &value
            && matches!(database_error.kind(), ErrorKind::UniqueViolation)
        {
            return Self::DuplicateName;
        }

        Self::Storage(value)
    }
}

pub async fn list_keys(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<OpenRouterApiKey>, OpenRouterKeyServiceError> {
    let keys = openrouter_key_storage::list_keys(pool, user_id).await?;
    Ok(keys.into_iter().map(map_key).collect())
}

pub async fn create_key(
    pool: &PgPool,
    user_id: Uuid,
    name: String,
    api_key: String,
) -> Result<OpenRouterApiKey, OpenRouterKeyServiceError> {
    let normalized_name = normalize_name(name)?;
    let normalized_api_key = normalize_api_key(api_key)?;
    let key = openrouter_key_storage::create_key(
        pool,
        Uuid::new_v4(),
        user_id,
        &normalized_name,
        &normalized_api_key,
    )
    .await?;

    Ok(map_key(key))
}

pub async fn update_key(
    pool: &PgPool,
    user_id: Uuid,
    key_id: Uuid,
    name: Option<String>,
    api_key: Option<String>,
) -> Result<OpenRouterApiKey, OpenRouterKeyServiceError> {
    let normalized_name = name.map(normalize_name).transpose()?;
    let normalized_api_key = api_key.map(normalize_api_key).transpose()?;

    let current_key = openrouter_key_storage::find_key_by_id(pool, key_id, user_id)
        .await?
        .ok_or(OpenRouterKeyServiceError::KeyNotFound)?;

    let updated_key = openrouter_key_storage::update_key(
        pool,
        key_id,
        user_id,
        normalized_name.as_deref().unwrap_or(&current_key.name),
        normalized_api_key.as_deref().unwrap_or(&current_key.api_key),
    )
    .await?
    .ok_or(OpenRouterKeyServiceError::KeyNotFound)?;

    Ok(map_key(updated_key))
}

pub async fn delete_key(
    pool: &PgPool,
    user_id: Uuid,
    key_id: Uuid,
) -> Result<(), OpenRouterKeyServiceError> {
    let deleted = openrouter_key_storage::delete_key(pool, key_id, user_id).await?;

    if !deleted {
        return Err(OpenRouterKeyServiceError::KeyNotFound);
    }

    Ok(())
}

fn normalize_name(name: String) -> Result<String, OpenRouterKeyServiceError> {
    let normalized = name.trim().to_string();

    if normalized.is_empty() || normalized.len() > MAX_NAME_LENGTH {
        return Err(OpenRouterKeyServiceError::InvalidName);
    }

    Ok(normalized)
}

fn normalize_api_key(api_key: String) -> Result<String, OpenRouterKeyServiceError> {
    let normalized = api_key.trim().to_string();

    if normalized.is_empty() || normalized.len() > MAX_KEY_LENGTH {
        return Err(OpenRouterKeyServiceError::InvalidApiKey);
    }

    Ok(normalized)
}

fn map_key(record: openrouter_key_storage::OpenRouterApiKeyRecord) -> OpenRouterApiKey {
    OpenRouterApiKey {
        id: record.id.to_string(),
        name: record.name,
        api_key: record.api_key,
    }
}
