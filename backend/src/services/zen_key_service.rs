use sqlx::{PgPool, error::ErrorKind};
use uuid::Uuid;

use crate::{
    storage::zen_keys as zen_key_storage,
    zen_keys::contracts::ZenApiKey,
};

const MAX_NAME_LENGTH: usize = 80;
const MAX_KEY_LENGTH: usize = 512;

#[derive(Debug)]
pub enum ZenKeyServiceError {
    InvalidName,
    InvalidApiKey,
    KeyNotFound,
    DuplicateName,
    Storage(sqlx::Error),
}

impl std::fmt::Display for ZenKeyServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidName => {
                write!(f, "Enter a name between 1 and {MAX_NAME_LENGTH} characters.")
            }
            Self::InvalidApiKey => {
                write!(f, "Enter an API key between 1 and {MAX_KEY_LENGTH} characters.")
            }
            Self::KeyNotFound => write!(f, "Zen API key not found."),
            Self::DuplicateName => write!(f, "You already have a key with that name."),
            Self::Storage(error) => write!(f, "failed to access Zen key data: {error}"),
        }
    }
}

impl std::error::Error for ZenKeyServiceError {}

impl From<sqlx::Error> for ZenKeyServiceError {
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
) -> Result<Vec<ZenApiKey>, ZenKeyServiceError> {
    let keys = zen_key_storage::list_keys(pool, user_id).await?;
    Ok(keys.into_iter().map(map_key).collect())
}

pub async fn create_key(
    pool: &PgPool,
    user_id: Uuid,
    name: String,
    api_key: String,
) -> Result<ZenApiKey, ZenKeyServiceError> {
    let normalized_name = normalize_name(name)?;
    let normalized_api_key = normalize_api_key(api_key)?;
    let key = zen_key_storage::create_key(
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
) -> Result<ZenApiKey, ZenKeyServiceError> {
    let normalized_name = name.map(normalize_name).transpose()?;
    let normalized_api_key = api_key.map(normalize_api_key).transpose()?;

    let current_key = zen_key_storage::find_key_by_id(pool, key_id, user_id)
        .await?
        .ok_or(ZenKeyServiceError::KeyNotFound)?;

    let updated_key = zen_key_storage::update_key(
        pool,
        key_id,
        user_id,
        normalized_name.as_deref().unwrap_or(&current_key.name),
        normalized_api_key.as_deref().unwrap_or(&current_key.api_key),
    )
    .await?
    .ok_or(ZenKeyServiceError::KeyNotFound)?;

    Ok(map_key(updated_key))
}

pub async fn delete_key(
    pool: &PgPool,
    user_id: Uuid,
    key_id: Uuid,
) -> Result<(), ZenKeyServiceError> {
    let deleted = zen_key_storage::delete_key(pool, key_id, user_id).await?;

    if !deleted {
        return Err(ZenKeyServiceError::KeyNotFound);
    }

    Ok(())
}

fn normalize_name(name: String) -> Result<String, ZenKeyServiceError> {
    let normalized = name.trim().to_string();

    if normalized.is_empty() || normalized.len() > MAX_NAME_LENGTH {
        return Err(ZenKeyServiceError::InvalidName);
    }

    Ok(normalized)
}

fn normalize_api_key(api_key: String) -> Result<String, ZenKeyServiceError> {
    let normalized = api_key.trim().to_string();

    if normalized.is_empty() || normalized.len() > MAX_KEY_LENGTH {
        return Err(ZenKeyServiceError::InvalidApiKey);
    }

    Ok(normalized)
}

fn map_key(record: zen_key_storage::ZenApiKeyRecord) -> ZenApiKey {
    ZenApiKey {
        id: record.id.to_string(),
        name: record.name,
        api_key: record.api_key,
    }
}
