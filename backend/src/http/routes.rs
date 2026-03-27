use axum::{
    http::{header, Method},
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    http::handlers::{auth, chat, health, streams, threads},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health_check))
        .nest("/api", api_router())
}

fn api_router() -> Router<AppState> {
    Router::new()
        .route("/v1/auth/register", post(auth::register))
        .route("/v1/auth/login", post(auth::login))
        .route("/v1/auth/logout", post(auth::logout))
        .route("/v1/auth/me", get(auth::me))
        .route(
            "/v1/chat/completions",
            post(chat::complete_prompt)
                .options(chat::chat_options)
                .layer(chat_cors_layer()),
        )
        .route(
            "/v1/threads",
            get(threads::list_threads)
                .post(threads::create_thread)
                .options(threads::thread_options)
                .layer(chat_cors_layer()),
        )
        .route("/v1/streams/hello", get(streams::hello_stream))
}

fn chat_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::COOKIE])
}
