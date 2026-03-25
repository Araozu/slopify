use reqwest::Client;

#[derive(Clone)]
pub struct AppState {
    pub http_client: Client,
}
