use auto_di::singleton;

pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub secret_key: String,
    pub socket_path: String,
    pub build_memory_limit: String,
    pub build_cpu_limit: String,
    /// Shared secret accepted only by monitoring-agent ingestion endpoints.
    /// Empty disables agent ingestion rather than silently accepting anonymous data.
    pub metrics_token: String,
}

#[singleton]
impl Config {
    pub fn new() -> Result<Self, &'static str> {
        let database_url =
            std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:///db.sqlite3".to_string());
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "4000".to_string())
            .parse::<u16>()
            .expect("PORT must be a number");
        let secret_key =
            std::env::var("SECRET_KEY").unwrap_or_else(|_| "your_secret_key_here".to_string());
        let socket_path =
            std::env::var("SOCKET_PATH").unwrap_or_else(|_| "/var/run/docker.sock".to_string());
        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let build_memory_limit =
            std::env::var("BUILD_MEMORY_LIMIT").unwrap_or_else(|_| "4G".to_string());
        let build_cpu_limit = std::env::var("BUILD_CPU_LIMIT").unwrap_or_else(|_| "4".to_string());
        let metrics_token = std::env::var("METRICS_TOKEN").unwrap_or_default();
        Ok(Config {
            database_url,
            port,
            host,
            secret_key,
            socket_path,
            build_memory_limit,
            build_cpu_limit,
            metrics_token,
        })
    }
}
