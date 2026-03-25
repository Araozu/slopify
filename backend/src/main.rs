mod app;
mod chat;
mod config;
mod http;

use config::AppConfig;

#[tokio::main]
async fn main() {
    let config = AppConfig::from_env();
    let listener = tokio::net::TcpListener::bind(config.bind_address())
        .await
        .expect("failed to bind TCP listener");

    println!("backend listening on http://{}", config.bind_address());

    axum::serve(listener, app::build_router())
        .await
        .expect("backend server exited unexpectedly");
}
