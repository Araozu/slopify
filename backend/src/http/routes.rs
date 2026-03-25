use axum::{routing::get, Router};

use crate::http::handlers::{health, streams};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/api/v1/streams/hello", get(streams::hello_stream))
}
