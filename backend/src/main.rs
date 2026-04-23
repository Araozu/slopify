mod auth;
mod app;
mod chat;
mod config;
mod copilot_models;
mod copilot_tokens;
mod db;
mod http;
mod openrouter_keys;
mod openrouter_models;
mod providers;
mod services;
mod state;
mod storage;
mod system_prompts;
mod threads;
mod zen_keys;

use config::AppConfig;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=info".parse().unwrap()),
        )
        .init();

    let config = AppConfig::from_env();
    let pool = db::pool::create_pool(config.database_url(), config.database_max_connections())
        .await
        .expect("failed to connect to postgres");
    let listener = tokio::net::TcpListener::bind(config.bind_address())
        .await
        .expect("failed to bind TCP listener");

    tracing::info!("backend listening on http://{}", config.bind_address());

    axum::serve(listener, app::build_router(&config, pool))
        .await
        .expect("backend server exited unexpectedly");
}
