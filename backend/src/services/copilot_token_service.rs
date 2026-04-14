use sqlx::{PgPool, error::ErrorKind};
use uuid::Uuid;

use crate::{
    copilot_tokens::contracts::CopilotToken,
    storage::copilot_tokens as copilot_token_storage,
};

const MAX_NAME_LENGTH: usize = 80;
const MAX_TOKEN_LENGTH: usize = 512;

#[derive(Debug)]
pub enum CopilotTokenServiceError {
    InvalidName,
    InvalidToken,
    TokenNotFound,
    DuplicateName,
    Storage(sqlx::Error),
}

impl std::fmt::Display for CopilotTokenServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidName => {
                write!(f, "Enter a name between 1 and {MAX_NAME_LENGTH} characters.")
            }
            Self::InvalidToken => {
                write!(f, "Enter a GitHub token between 1 and {MAX_TOKEN_LENGTH} characters.")
            }
            Self::TokenNotFound => write!(f, "Copilot token not found."),
            Self::DuplicateName => write!(f, "You already have a token with that name."),
            Self::Storage(error) => write!(f, "failed to access Copilot token data: {error}"),
        }
    }
}

impl std::error::Error for CopilotTokenServiceError {}

impl From<sqlx::Error> for CopilotTokenServiceError {
    fn from(value: sqlx::Error) -> Self {
        if let sqlx::Error::Database(database_error) = &value
            && matches!(database_error.kind(), ErrorKind::UniqueViolation)
        {
            return Self::DuplicateName;
        }

        Self::Storage(value)
    }
}

pub async fn list_tokens(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<CopilotToken>, CopilotTokenServiceError> {
    let tokens = copilot_token_storage::list_tokens(pool, user_id).await?;
    Ok(tokens.into_iter().map(map_token).collect())
}

pub async fn create_token(
    pool: &PgPool,
    user_id: Uuid,
    name: String,
    github_token: String,
) -> Result<CopilotToken, CopilotTokenServiceError> {
    let normalized_name = normalize_name(name)?;
    let normalized_token = normalize_token(github_token)?;
    let record = copilot_token_storage::create_token(
        pool,
        Uuid::new_v4(),
        user_id,
        &normalized_name,
        &normalized_token,
    )
    .await?;

    Ok(map_token(record))
}

pub async fn update_token(
    pool: &PgPool,
    user_id: Uuid,
    token_id: Uuid,
    name: Option<String>,
    github_token: Option<String>,
) -> Result<CopilotToken, CopilotTokenServiceError> {
    let normalized_name = name.map(normalize_name).transpose()?;
    let normalized_token = github_token.map(normalize_token).transpose()?;

    let current = copilot_token_storage::find_token_by_id(pool, token_id, user_id)
        .await?
        .ok_or(CopilotTokenServiceError::TokenNotFound)?;

    let updated = copilot_token_storage::update_token(
        pool,
        token_id,
        user_id,
        normalized_name.as_deref().unwrap_or(&current.name),
        normalized_token.as_deref().unwrap_or(&current.github_token),
    )
    .await?
    .ok_or(CopilotTokenServiceError::TokenNotFound)?;

    Ok(map_token(updated))
}

pub async fn delete_token(
    pool: &PgPool,
    user_id: Uuid,
    token_id: Uuid,
) -> Result<(), CopilotTokenServiceError> {
    let deleted = copilot_token_storage::delete_token(pool, token_id, user_id).await?;

    if !deleted {
        return Err(CopilotTokenServiceError::TokenNotFound);
    }

    Ok(())
}

fn normalize_name(name: String) -> Result<String, CopilotTokenServiceError> {
    let normalized = name.trim().to_string();

    if normalized.is_empty() || normalized.len() > MAX_NAME_LENGTH {
        return Err(CopilotTokenServiceError::InvalidName);
    }

    Ok(normalized)
}

fn normalize_token(token: String) -> Result<String, CopilotTokenServiceError> {
    let normalized = token.trim().to_string();

    if normalized.is_empty() || normalized.len() > MAX_TOKEN_LENGTH {
        return Err(CopilotTokenServiceError::InvalidToken);
    }

    Ok(normalized)
}

fn map_token(record: copilot_token_storage::CopilotTokenRecord) -> CopilotToken {
    CopilotToken {
        id: record.id.to_string(),
        name: record.name,
        github_token: record.github_token,
    }
}
