use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::contracts::User,
    storage::{sessions as session_storage, users as user_storage},
};

const MIN_PASSWORD_LENGTH: usize = 8;

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user: User,
    pub session_token: String,
}

#[derive(Debug)]
pub struct AuthenticatedSession {
    pub user_id: Uuid,
}

#[derive(Debug)]
pub enum AuthServiceError {
    InvalidEmail,
    InvalidName,
    InvalidPassword,
    InvalidCredentials,
    EmailTaken,
    SessionNotFound,
    Storage(sqlx::Error),
    PasswordHash(argon2::password_hash::Error),
}

impl std::fmt::Display for AuthServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidEmail => write!(f, "Enter a valid email address."),
            Self::InvalidName => write!(f, "Enter a name between 2 and 60 characters."),
            Self::InvalidPassword => write!(
                f,
                "Enter a password with at least {MIN_PASSWORD_LENGTH} characters."
            ),
            Self::InvalidCredentials => write!(f, "Invalid email or password."),
            Self::EmailTaken => write!(f, "That email is already registered."),
            Self::SessionNotFound => write!(f, "Your session has expired."),
            Self::Storage(error) => write!(f, "failed to access auth data: {error}"),
            Self::PasswordHash(error) => write!(f, "failed to secure password: {error}"),
        }
    }
}

impl std::error::Error for AuthServiceError {}

impl From<sqlx::Error> for AuthServiceError {
    fn from(value: sqlx::Error) -> Self {
        Self::Storage(value)
    }
}

impl From<argon2::password_hash::Error> for AuthServiceError {
    fn from(value: argon2::password_hash::Error) -> Self {
        Self::PasswordHash(value)
    }
}

pub async fn register_user(
    pool: &PgPool,
    email: String,
    password: String,
    name: String,
) -> Result<AuthenticatedUser, AuthServiceError> {
    let normalized_email = normalize_email(email)?;
    let normalized_name = normalize_name(name)?;
    validate_password(&password)?;

    if user_storage::find_user_by_email(pool, &normalized_email)
        .await?
        .is_some()
    {
        return Err(AuthServiceError::EmailTaken);
    }

    let password_hash = hash_password(&password)?;
    let user = user_storage::create_user(
        pool,
        Uuid::new_v4(),
        &normalized_email,
        &password_hash,
        &normalized_name,
    )
    .await?;

    create_authenticated_user(pool, user.id).await
}

pub async fn login_user(
    pool: &PgPool,
    email: String,
    password: String,
) -> Result<AuthenticatedUser, AuthServiceError> {
    let normalized_email = normalize_email(email)?;
    let user = user_storage::find_user_by_email(pool, &normalized_email)
        .await?
        .ok_or(AuthServiceError::InvalidCredentials)?;

    verify_password(&password, &user.password_hash)?;

    create_authenticated_user(pool, user.id).await
}

pub async fn get_user(pool: &PgPool, user_id: Uuid) -> Result<User, AuthServiceError> {
    let user = user_storage::find_user_by_id(pool, user_id)
        .await?
        .ok_or(AuthServiceError::SessionNotFound)?;

    Ok(map_user(user))
}

pub async fn authenticate_session(
    pool: &PgPool,
    token: &str,
) -> Result<AuthenticatedSession, AuthServiceError> {
    let session = session_storage::find_session_by_token(pool, token)
        .await?
        .ok_or(AuthServiceError::SessionNotFound)?;

    Ok(AuthenticatedSession {
        user_id: session.user_id,
    })
}

pub async fn delete_sessions_for_user(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<(), AuthServiceError> {
    session_storage::delete_sessions_for_user(pool, user_id).await?;
    Ok(())
}

async fn create_authenticated_user(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<AuthenticatedUser, AuthServiceError> {
    let user = user_storage::find_user_by_id(pool, user_id)
        .await?
        .ok_or(AuthServiceError::InvalidCredentials)?;
    let session_token = generate_session_token()?;

    session_storage::create_session(pool, Uuid::new_v4(), user.id, &session_token).await?;

    Ok(AuthenticatedUser {
        user: map_user(user),
        session_token,
    })
}

fn normalize_email(email: String) -> Result<String, AuthServiceError> {
    let normalized = email.trim().to_lowercase();

    if normalized.len() < 3 || !normalized.contains('@') {
        return Err(AuthServiceError::InvalidEmail);
    }

    Ok(normalized)
}

fn normalize_name(name: String) -> Result<String, AuthServiceError> {
    let normalized = name.trim().to_string();

    if !(2..=60).contains(&normalized.len()) {
        return Err(AuthServiceError::InvalidName);
    }

    Ok(normalized)
}

fn validate_password(password: &str) -> Result<(), AuthServiceError> {
    if password.trim().len() < MIN_PASSWORD_LENGTH {
        return Err(AuthServiceError::InvalidPassword);
    }

    Ok(())
}

fn hash_password(password: &str) -> Result<String, AuthServiceError> {
    let salt = SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

fn verify_password(password: &str, hash: &str) -> Result<(), AuthServiceError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|_| AuthServiceError::InvalidCredentials)?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AuthServiceError::InvalidCredentials)
}

fn generate_session_token() -> Result<String, AuthServiceError> {
    Ok(format!(
        "{}{}",
        Uuid::new_v4().simple(),
        Uuid::new_v4().simple()
    ))
}

fn map_user(user: user_storage::UserRecord) -> User {
    User {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
    }
}
