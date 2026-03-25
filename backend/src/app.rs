use axum::Router;
use reqwest::Client;
use tower_http::services::{ServeDir, ServeFile};

use crate::{config::AppConfig, state::AppState};

pub fn build_router(config: &AppConfig) -> Router {
    let static_dir = config.static_dir().to_string();
    let index_file = format!("{static_dir}/index.html");

    crate::http::routes::router()
        .fallback_service(ServeDir::new(static_dir).not_found_service(ServeFile::new(index_file)))
        .with_state(AppState {
            http_client: Client::new(),
        })
}
