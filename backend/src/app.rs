use axum::Router;

use crate::http;

pub fn build_router() -> Router {
    Router::new().merge(http::routes::router())
}
