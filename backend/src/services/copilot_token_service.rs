use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, error::ErrorKind};
use uuid::Uuid;

use crate::{
    copilot_tokens::contracts::CopilotToken,
    storage::copilot_tokens as copilot_token_storage,
};

const MAX_NAME_LENGTH: usize = 80;

const GITHUB_DEVICE_CODE_URL: &str = "https://github.com/login/device/code";
const GITHUB_OAUTH_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

/// VS Code Copilot Chat's well-known public OAuth client ID.
const COPILOT_CLIENT_ID: &str = "Iv1.b507a08c87ecfe98";

#[derive(Debug)]
pub enum CopilotTokenServiceError {
    InvalidName,
    TokenNotFound,
    DuplicateName,
    Storage(sqlx::Error),
    DeviceFlowFailed(String),
}

impl std::fmt::Display for CopilotTokenServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidName => {
                write!(f, "Enter a name between 1 and {MAX_NAME_LENGTH} characters.")
            }
            Self::TokenNotFound => write!(f, "Copilot token not found."),
            Self::DuplicateName => write!(f, "You already have a token with that name."),
            Self::Storage(error) => write!(f, "failed to access Copilot token data: {error}"),
            Self::DeviceFlowFailed(msg) => write!(f, "device flow failed: {msg}"),
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

// ---------------------------------------------------------------------------
// CRUD (unchanged)
// ---------------------------------------------------------------------------

pub async fn list_tokens(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<CopilotToken>, CopilotTokenServiceError> {
    let tokens = copilot_token_storage::list_tokens(pool, user_id).await?;
    Ok(tokens.into_iter().map(map_token).collect())
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

// ---------------------------------------------------------------------------
// OAuth device code flow
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

/// Initiate the GitHub OAuth device code flow with `scope=copilot`.
pub async fn initiate_device_code(
    http_client: &Client,
) -> Result<DeviceCodeResponse, CopilotTokenServiceError> {
    let response = http_client
        .post(GITHUB_DEVICE_CODE_URL)
        .header("Accept", "application/json")
        .json(&serde_json::json!({
            "client_id": COPILOT_CLIENT_ID,
            "scope": "copilot",
        }))
        .send()
        .await
        .map_err(|e| CopilotTokenServiceError::DeviceFlowFailed(e.to_string()))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(CopilotTokenServiceError::DeviceFlowFailed(format!(
            "GitHub returned {status}: {body}"
        )));
    }

    response
        .json::<DeviceCodeResponse>()
        .await
        .map_err(|e| CopilotTokenServiceError::DeviceFlowFailed(e.to_string()))
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum DevicePollResult {
    /// User hasn't authorized yet.
    Pending,
    /// GitHub asked us to slow down.
    SlowDown,
    /// Authorization complete — token was persisted.
    Complete { token: CopilotToken },
    /// The device code expired.
    Expired,
}

#[derive(Deserialize)]
struct OAuthTokenResponse {
    access_token: Option<String>,
    error: Option<String>,
}

/// Make a single poll attempt against GitHub's OAuth token endpoint. If the
/// user has authorized, persist the OAuth token and return `Complete`.
pub async fn poll_device_code(
    http_client: &Client,
    pool: &PgPool,
    user_id: Uuid,
    device_code: &str,
    name: &str,
) -> Result<DevicePollResult, CopilotTokenServiceError> {
    let response = http_client
        .post(GITHUB_OAUTH_TOKEN_URL)
        .header("Accept", "application/json")
        .json(&serde_json::json!({
            "client_id": COPILOT_CLIENT_ID,
            "device_code": device_code,
            "grant_type": "urn:ietf:params:oauth:grant-type:device_code",
        }))
        .send()
        .await
        .map_err(|e| CopilotTokenServiceError::DeviceFlowFailed(e.to_string()))?;

    let data: OAuthTokenResponse = response
        .json()
        .await
        .map_err(|e| CopilotTokenServiceError::DeviceFlowFailed(e.to_string()))?;

    if let Some(access_token) = data.access_token {
        let normalized_name = normalize_name(name.to_string())?;
        let record = copilot_token_storage::create_token(
            pool,
            Uuid::new_v4(),
            user_id,
            &normalized_name,
            &access_token,
        )
        .await?;

        return Ok(DevicePollResult::Complete {
            token: map_token(record),
        });
    }

    match data.error.as_deref() {
        Some("authorization_pending") => Ok(DevicePollResult::Pending),
        Some("slow_down") => Ok(DevicePollResult::SlowDown),
        Some("expired_token") => Ok(DevicePollResult::Expired),
        Some(other) => Err(CopilotTokenServiceError::DeviceFlowFailed(
            other.to_string(),
        )),
        None => Err(CopilotTokenServiceError::DeviceFlowFailed(
            "unexpected response from GitHub".to_string(),
        )),
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn normalize_name(name: String) -> Result<String, CopilotTokenServiceError> {
    let normalized = name.trim().to_string();

    if normalized.is_empty() || normalized.len() > MAX_NAME_LENGTH {
        return Err(CopilotTokenServiceError::InvalidName);
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
