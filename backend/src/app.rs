use axum::Router;
use reqwest::Client;

use crate::state::AppState;

pub fn build_router() -> Router {
    crate::http::routes::router().with_state(AppState {
        http_client: Client::new(),
    })
}
