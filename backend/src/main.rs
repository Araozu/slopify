mod app;
mod chat;
mod config;
mod db;
mod http;
mod providers;
mod services;
mod state;
mod storage;
mod threads;

use config::AppConfig;

#[tokio::main]
async fn main() {
    let config = AppConfig::from_env();
    let pool = db::pool::create_pool(config.database_url(), config.database_max_connections())
        .await
        .expect("failed to connect to postgres");
    let listener = tokio::net::TcpListener::bind(config.bind_address())
        .await
        .expect("failed to bind TCP listener");

    println!("backend listening on http://{}", config.bind_address());

    axum::serve(listener, app::build_router(&config, pool))
        .await
        .expect("backend server exited unexpectedly");
}
