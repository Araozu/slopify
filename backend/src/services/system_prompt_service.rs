use sqlx::{PgPool, error::ErrorKind};
use uuid::Uuid;

use crate::{
    storage::system_prompts as system_prompt_storage,
    system_prompts::contracts::SystemPrompt,
};

const MAX_NAME_LENGTH: usize = 100;
const MAX_CONTENT_LENGTH: usize = 32_000;

#[derive(Debug)]
pub enum SystemPromptServiceError {
    InvalidName,
    InvalidContent,
    NotFound,
    DuplicateName,
    Storage(sqlx::Error),
}

impl std::fmt::Display for SystemPromptServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidName => {
                write!(f, "Enter a name between 1 and {MAX_NAME_LENGTH} characters.")
            }
            Self::InvalidContent => {
                write!(
                    f,
                    "Enter content between 1 and {MAX_CONTENT_LENGTH} characters."
                )
            }
            Self::NotFound => write!(f, "System prompt not found."),
            Self::DuplicateName => write!(f, "You already have a system prompt with that name."),
            Self::Storage(error) => write!(f, "failed to access system prompt data: {error}"),
        }
    }
}

impl std::error::Error for SystemPromptServiceError {}

impl From<sqlx::Error> for SystemPromptServiceError {
    fn from(value: sqlx::Error) -> Self {
        if let sqlx::Error::Database(database_error) = &value
            && matches!(database_error.kind(), ErrorKind::UniqueViolation)
        {
            return Self::DuplicateName;
        }

        Self::Storage(value)
    }
}

pub async fn list_system_prompts(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<SystemPrompt>, SystemPromptServiceError> {
    let records = system_prompt_storage::list_system_prompts(pool, user_id).await?;
    Ok(records.into_iter().map(map_system_prompt).collect())
}

pub async fn create_system_prompt(
    pool: &PgPool,
    user_id: Uuid,
    name: String,
    content: String,
) -> Result<SystemPrompt, SystemPromptServiceError> {
    let normalized_name = normalize_name(name)?;
    let normalized_content = normalize_content(content)?;
    let record = system_prompt_storage::create_system_prompt(
        pool,
        Uuid::new_v4(),
        user_id,
        &normalized_name,
        &normalized_content,
    )
    .await?;

    Ok(map_system_prompt(record))
}

pub async fn update_system_prompt(
    pool: &PgPool,
    user_id: Uuid,
    prompt_id: Uuid,
    name: Option<String>,
    content: Option<String>,
) -> Result<SystemPrompt, SystemPromptServiceError> {
    let normalized_name = name.map(normalize_name).transpose()?;
    let normalized_content = content.map(normalize_content).transpose()?;

    let current = system_prompt_storage::find_system_prompt_by_id(pool, prompt_id, user_id)
        .await?
        .ok_or(SystemPromptServiceError::NotFound)?;

    let updated = system_prompt_storage::update_system_prompt(
        pool,
        prompt_id,
        user_id,
        normalized_name.as_deref().unwrap_or(&current.name),
        normalized_content.as_deref().unwrap_or(&current.content),
    )
    .await?
    .ok_or(SystemPromptServiceError::NotFound)?;

    Ok(map_system_prompt(updated))
}

pub async fn delete_system_prompt(
    pool: &PgPool,
    user_id: Uuid,
    prompt_id: Uuid,
) -> Result<(), SystemPromptServiceError> {
    let deleted = system_prompt_storage::delete_system_prompt(pool, prompt_id, user_id).await?;

    if !deleted {
        return Err(SystemPromptServiceError::NotFound);
    }

    Ok(())
}

pub async fn get_system_prompt_content(
    pool: &PgPool,
    user_id: Uuid,
    prompt_id: Uuid,
) -> Result<String, SystemPromptServiceError> {
    let record = system_prompt_storage::find_system_prompt_by_id(pool, prompt_id, user_id)
        .await?
        .ok_or(SystemPromptServiceError::NotFound)?;

    Ok(record.content)
}

fn normalize_name(name: String) -> Result<String, SystemPromptServiceError> {
    let normalized = name.trim().to_string();
    if normalized.is_empty() || normalized.len() > MAX_NAME_LENGTH {
        return Err(SystemPromptServiceError::InvalidName);
    }
    Ok(normalized)
}

fn normalize_content(content: String) -> Result<String, SystemPromptServiceError> {
    let normalized = content.trim().to_string();
    if normalized.is_empty() || normalized.len() > MAX_CONTENT_LENGTH {
        return Err(SystemPromptServiceError::InvalidContent);
    }
    Ok(normalized)
}

fn map_system_prompt(
    record: system_prompt_storage::SystemPromptRecord,
) -> SystemPrompt {
    SystemPrompt {
        id: record.id.to_string(),
        name: record.name,
        content: record.content,
    }
}
