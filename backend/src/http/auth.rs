use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde::Serialize;

use crate::{services::auth_service, state::AppState};

pub const SESSION_COOKIE_NAME: &str = "slopify_session";

#[derive(Clone)]
pub struct AuthSession {
    pub user_id: uuid::Uuid,
}

impl<S> FromRequestParts<S> for AuthSession
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let cookies = parts
            .headers
            .get(axum::http::header::COOKIE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default();

        let session_token = cookies
            .split(';')
            .filter_map(|cookie| Cookie::parse_encoded(cookie.trim()).ok())
            .find(|cookie| cookie.name() == SESSION_COOKIE_NAME)
            .map(|cookie| cookie.value().to_string())
            .ok_or_else(AuthApiError::unauthorized)?;

        let session = auth_service::authenticate_session(&app_state.db_pool, &session_token)
            .await
            .map_err(AuthApiError::from)?;

        Ok(Self {
            user_id: session.user_id,
        })
    }
}

#[derive(Debug)]
pub struct AuthApiError {
    status: StatusCode,
    message: String,
}

impl AuthApiError {
    pub fn unauthorized() -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            message: "Authentication required.".to_string(),
        }
    }
}

impl From<auth_service::AuthServiceError> for AuthApiError {
    fn from(value: auth_service::AuthServiceError) -> Self {
        match value {
            auth_service::AuthServiceError::InvalidEmail
            | auth_service::AuthServiceError::InvalidName
            | auth_service::AuthServiceError::InvalidPassword => Self {
                status: StatusCode::BAD_REQUEST,
                message: value.to_string(),
            },
            auth_service::AuthServiceError::InvalidCredentials
            | auth_service::AuthServiceError::SessionNotFound => Self {
                status: StatusCode::UNAUTHORIZED,
                message: value.to_string(),
            },
            auth_service::AuthServiceError::EmailTaken => Self {
                status: StatusCode::CONFLICT,
                message: value.to_string(),
            },
            auth_service::AuthServiceError::Storage(_)
            | auth_service::AuthServiceError::PasswordHash(_) => Self {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: value.to_string(),
            },
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AuthApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            axum::Json(ErrorResponse {
                error: self.message,
            }),
        )
            .into_response()
    }
}

pub fn build_session_cookie(token: &str) -> Cookie<'static> {
    Cookie::build((SESSION_COOKIE_NAME, token.to_string()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build()
}

pub fn clear_session_cookie() -> Cookie<'static> {
    let mut cookie = Cookie::build((SESSION_COOKIE_NAME, ""))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();
    cookie.make_removal();
    cookie
}
