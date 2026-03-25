use std::env;

pub struct AppConfig {
    host: String,
    port: u16,
    database_url: String,
    database_max_connections: u32,
    static_dir: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("APP_PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(4000);
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let database_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .ok()
            .and_then(|value| value.parse::<u32>().ok())
            .unwrap_or(5);
        let static_dir = env::var("STATIC_DIR").unwrap_or_else(|_| "../frontend/build".to_string());

        Self {
            host,
            port,
            database_url,
            database_max_connections,
            static_dir,
        }
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn database_max_connections(&self) -> u32 {
        self.database_max_connections
    }

    pub fn static_dir(&self) -> &str {
        &self.static_dir
    }
}
