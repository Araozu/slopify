use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, error::ErrorKind};
use uuid::Uuid;

use crate::{
    openai_tokens::contracts::{OpenAiAuthType, OpenAiToken},
    storage::openai_tokens as openai_token_storage,
};

const MAX_NAME_LENGTH: usize = 80;
const MAX_TOKEN_LENGTH: usize = 4096;
const OPENAI_CLIENT_ID: &str = "app_EMoamEEZ73f0CkXaXp7hrann";
const OPENAI_ISSUER: &str = "https://auth.openai.com";
const OPENAI_USERCODE_URL: &str = "https://auth.openai.com/api/accounts/deviceauth/usercode";
const OPENAI_DEVICE_POLL_URL: &str = "https://auth.openai.com/api/accounts/deviceauth/token";
const OPENAI_OAUTH_TOKEN_URL: &str = "https://auth.openai.com/oauth/token";
const OPENAI_DEVICE_CALLBACK_URI: &str = "https://auth.openai.com/deviceauth/callback";
const DEFAULT_DEVICE_EXPIRY_SECONDS: u64 = 15 * 60;

#[derive(Debug)]
pub enum OpenAiTokenServiceError {
    InvalidName,
    InvalidToken,
    TokenNotFound,
    DuplicateName,
    Storage(sqlx::Error),
    DeviceFlowFailed(String),
}

impl std::fmt::Display for OpenAiTokenServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidName => {
                write!(f, "Enter a name between 1 and {MAX_NAME_LENGTH} characters.")
            }
            Self::InvalidToken => {
                write!(f, "Enter a token between 1 and {MAX_TOKEN_LENGTH} characters.")
            }
            Self::TokenNotFound => write!(f, "OpenAI token not found."),
            Self::DuplicateName => write!(f, "You already have a token with that name."),
            Self::Storage(error) => write!(f, "failed to access OpenAI token data: {error}"),
            Self::DeviceFlowFailed(msg) => write!(f, "OpenAI device flow failed: {msg}"),
        }
    }
}

impl std::error::Error for OpenAiTokenServiceError {}

impl From<sqlx::Error> for OpenAiTokenServiceError {
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
) -> Result<Vec<OpenAiToken>, OpenAiTokenServiceError> {
    let tokens = openai_token_storage::list_tokens(pool, user_id).await?;
    tokens
        .into_iter()
        .map(map_token)
        .collect::<Result<Vec<_>, _>>()
}

pub async fn create_token(
    pool: &PgPool,
    user_id: Uuid,
    name: String,
    token: String,
) -> Result<OpenAiToken, OpenAiTokenServiceError> {
    let normalized_name = normalize_name(name)?;
    let normalized_token = normalize_token(token)?;
    let auth_type = infer_auth_type(&normalized_token);

    let record = openai_token_storage::create_token(
        pool,
        Uuid::new_v4(),
        user_id,
        &normalized_name,
        auth_type_as_db_value(&auth_type),
        &normalized_token,
    )
    .await?;

    map_token(record)
}

pub async fn delete_token(
    pool: &PgPool,
    user_id: Uuid,
    token_id: Uuid,
) -> Result<(), OpenAiTokenServiceError> {
    let deleted = openai_token_storage::delete_token(pool, token_id, user_id).await?;

    if !deleted {
        return Err(OpenAiTokenServiceError::TokenNotFound);
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_auth_id: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

#[derive(Deserialize)]
struct DeviceCodeRawResponse {
    device_auth_id: String,
    #[serde(alias = "usercode")]
    user_code: String,
    interval: serde_json::Value,
    expires_in: Option<u64>,
}

pub async fn initiate_device_code(
    http_client: &Client,
) -> Result<DeviceCodeResponse, OpenAiTokenServiceError> {
    let response = http_client
        .post(OPENAI_USERCODE_URL)
        .header("Accept", "application/json")
        .json(&serde_json::json!({
            "client_id": OPENAI_CLIENT_ID,
        }))
        .send()
        .await
        .map_err(|e| OpenAiTokenServiceError::DeviceFlowFailed(e.to_string()))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(OpenAiTokenServiceError::DeviceFlowFailed(format!(
            "OpenAI returned {status}: {body}"
        )));
    }

    let payload = response
        .json::<DeviceCodeRawResponse>()
        .await
        .map_err(|e| OpenAiTokenServiceError::DeviceFlowFailed(e.to_string()))?;

    Ok(DeviceCodeResponse {
        device_auth_id: payload.device_auth_id,
        user_code: payload.user_code,
        verification_uri: format!("{OPENAI_ISSUER}/codex/device"),
        expires_in: payload.expires_in.unwrap_or(DEFAULT_DEVICE_EXPIRY_SECONDS),
        interval: normalize_interval(payload.interval).unwrap_or(5),
    })
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum DevicePollResult {
    Pending,
    SlowDown,
    Complete { token: OpenAiToken },
    Expired,
}

#[derive(Deserialize)]
struct PollSuccessResponse {
    authorization_code: String,
    code_verifier: String,
}

#[derive(Deserialize)]
struct OAuthCodeExchangeResponse {
    refresh_token: Option<String>,
}

pub async fn poll_device_code(
    http_client: &Client,
    pool: &PgPool,
    user_id: Uuid,
    device_auth_id: &str,
    user_code: &str,
    name: &str,
) -> Result<DevicePollResult, OpenAiTokenServiceError> {
    let normalized_name = normalize_name(name.to_string())?;

    let response = http_client
        .post(OPENAI_DEVICE_POLL_URL)
        .header("Accept", "application/json")
        .json(&serde_json::json!({
            "device_auth_id": device_auth_id,
            "user_code": user_code,
        }))
        .send()
        .await
        .map_err(|e| OpenAiTokenServiceError::DeviceFlowFailed(e.to_string()))?;

    if response.status().is_success() {
        let payload = response
            .json::<PollSuccessResponse>()
            .await
            .map_err(|e| OpenAiTokenServiceError::DeviceFlowFailed(e.to_string()))?;

        let refresh_token = exchange_authorization_code(
            http_client,
            &payload.authorization_code,
            &payload.code_verifier,
        )
        .await?;

        let record = openai_token_storage::create_token(
            pool,
            Uuid::new_v4(),
            user_id,
            &normalized_name,
            auth_type_as_db_value(&OpenAiAuthType::OAuthRefreshToken),
            &refresh_token,
        )
        .await?;

        return Ok(DevicePollResult::Complete {
            token: map_token(record)?,
        });
    }

    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    let error_code = parse_error_code(&body);

    if status == reqwest::StatusCode::FORBIDDEN || status == reqwest::StatusCode::NOT_FOUND {
        return Ok(match error_code.as_deref() {
            Some("slow_down") => DevicePollResult::SlowDown,
            Some("expired_token") => DevicePollResult::Expired,
            _ => DevicePollResult::Pending,
        });
    }

    Err(OpenAiTokenServiceError::DeviceFlowFailed(format!(
        "poll failed ({status}): {body}"
    )))
}

async fn exchange_authorization_code(
    http_client: &Client,
    authorization_code: &str,
    code_verifier: &str,
) -> Result<String, OpenAiTokenServiceError> {
    let response = http_client
        .post(OPENAI_OAUTH_TOKEN_URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[
            ("grant_type", "authorization_code"),
            ("client_id", OPENAI_CLIENT_ID),
            ("code", authorization_code),
            ("code_verifier", code_verifier),
            ("redirect_uri", OPENAI_DEVICE_CALLBACK_URI),
        ])
        .send()
        .await
        .map_err(|e| OpenAiTokenServiceError::DeviceFlowFailed(e.to_string()))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(OpenAiTokenServiceError::DeviceFlowFailed(format!(
            "code exchange failed ({status}): {body}"
        )));
    }

    let payload = response
        .json::<OAuthCodeExchangeResponse>()
        .await
        .map_err(|e| OpenAiTokenServiceError::DeviceFlowFailed(e.to_string()))?;

    payload.refresh_token.ok_or_else(|| {
        OpenAiTokenServiceError::DeviceFlowFailed(
            "code exchange response did not include a refresh_token".to_string(),
        )
    })
}

fn parse_error_code(body: &str) -> Option<String> {
    let value = serde_json::from_str::<serde_json::Value>(body).ok()?;

    if let Some(code) = value.get("error").and_then(|v| v.as_str()) {
        return Some(code.to_string());
    }

    value
        .get("error")
        .and_then(|v| v.get("code"))
        .and_then(|v| v.as_str())
        .map(|v| v.to_string())
}

fn normalize_name(name: String) -> Result<String, OpenAiTokenServiceError> {
    let normalized = name.trim().to_string();

    if normalized.is_empty() || normalized.len() > MAX_NAME_LENGTH {
        return Err(OpenAiTokenServiceError::InvalidName);
    }

    Ok(normalized)
}

fn normalize_token(token: String) -> Result<String, OpenAiTokenServiceError> {
    let normalized = token.trim().to_string();

    if normalized.is_empty() || normalized.len() > MAX_TOKEN_LENGTH {
        return Err(OpenAiTokenServiceError::InvalidToken);
    }

    Ok(normalized)
}

fn infer_auth_type(token: &str) -> OpenAiAuthType {
    if token.starts_with("sk-") {
        OpenAiAuthType::ApiKey
    } else {
        OpenAiAuthType::OAuthRefreshToken
    }
}

fn auth_type_as_db_value(auth_type: &OpenAiAuthType) -> &'static str {
    match auth_type {
        OpenAiAuthType::ApiKey => "api_key",
        OpenAiAuthType::OAuthRefreshToken => "oauth_refresh_token",
    }
}

fn auth_type_from_db_value(value: &str) -> Result<OpenAiAuthType, OpenAiTokenServiceError> {
    match value {
        "api_key" => Ok(OpenAiAuthType::ApiKey),
        "oauth_refresh_token" => Ok(OpenAiAuthType::OAuthRefreshToken),
        _ => Err(OpenAiTokenServiceError::DeviceFlowFailed(
            "stored token has unsupported auth_type".to_string(),
        )),
    }
}

fn normalize_interval(value: serde_json::Value) -> Option<u64> {
    match value {
        serde_json::Value::Number(number) => number.as_u64(),
        serde_json::Value::String(raw) => raw.trim().parse::<u64>().ok(),
        _ => None,
    }
}

fn map_token(record: openai_token_storage::OpenAiTokenRecord) -> Result<OpenAiToken, OpenAiTokenServiceError> {
    Ok(OpenAiToken {
        id: record.id.to_string(),
        name: record.name,
        auth_type: auth_type_from_db_value(&record.auth_type)?,
        token: record.token,
    })
}
