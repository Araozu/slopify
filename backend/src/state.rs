use std::sync::Arc;

use reqwest::Client;
use sqlx::PgPool;

use crate::providers::registry::ProviderRegistry;

#[derive(Clone)]
pub struct AppState {
    pub http_client: Client,
    pub db_pool: PgPool,
    pub providers: Arc<ProviderRegistry>,
}
