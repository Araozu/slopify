use reqwest::Client;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub http_client: Client,
    pub db_pool: PgPool,
}
