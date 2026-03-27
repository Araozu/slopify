use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;

use crate::{
    http::auth::{AuthApiError, AuthSession, build_session_cookie, clear_session_cookie},
    services::auth_service,
    state::AppState,
};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn register(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<RegisterRequest>,
) -> Response {
    let result = auth_service::register_user(
        &state.db_pool,
        payload.email,
        payload.password,
        payload.name,
    )
    .await
    .map_err(AuthApiError::from);

    respond_with_session(jar, result)
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<LoginRequest>,
) -> Response {
    let result = auth_service::login_user(&state.db_pool, payload.email, payload.password)
        .await
        .map_err(AuthApiError::from);

    respond_with_session(jar, result)
}

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
    jar_session: Result<AuthSession, AuthApiError>,
) -> Response {
    if let Ok(session) = jar_session {
        let _ = auth_service::delete_sessions_for_user(&state.db_pool, session.user_id).await;
    }

    (StatusCode::NO_CONTENT, jar.add(clear_session_cookie())).into_response()
}

pub async fn me(State(state): State<AppState>, session: AuthSession) -> Response {
    match auth_service::get_user(&state.db_pool, session.user_id).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(error) => AuthApiError::from(error).into_response(),
    }
}

fn respond_with_session(
    jar: CookieJar,
    result: Result<auth_service::AuthenticatedUser, AuthApiError>,
) -> Response {
    match result {
        Ok(authenticated_user) => (
            StatusCode::OK,
            jar.add(build_session_cookie(&authenticated_user.session_token)),
            Json(authenticated_user.user),
        )
            .into_response(),
        Err(error) => error.into_response(),
    }
}
