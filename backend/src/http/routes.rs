use axum::{
    http::{Method, header},
    Router,
    routing::{get, post},
};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    http::handlers::{chat, health, streams},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health_check))
        .route(
            "/api/v1/chat/completions",
            post(chat::complete_prompt).options(chat::chat_options),
        )
        .route("/api/v1/streams/hello", get(streams::hello_stream))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::POST, Method::OPTIONS])
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]),
        )
}
