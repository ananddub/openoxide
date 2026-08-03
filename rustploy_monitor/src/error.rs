use hyper::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MonitorError {
    #[error("Docker API error: {0}")]
    Docker(#[from] DockerError),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),

    #[error("gRPC error: {0}")]
    Grpc(#[from] tonic::Status),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Error)]
pub enum DockerError {
    #[error("Could not connect to Docker socket at {path}: {source}")]
    SocketConnect {
        path: String,
        source: std::io::Error,
    },

    #[error("Handshake failed: {0}")]
    Handshake(String),

    #[error("Request to {uri} failed: {message}")]
    RequestFailed { uri: String, message: String },

    #[error("Resource not found at {uri}: {detail}")]
    NotFound { uri: String, detail: String },

    #[error("Docker API returned HTTP {status} for {uri}: {detail}")]
    ApiStatus {
        status: StatusCode,
        uri: String,
        detail: String,
    },

    #[error("Response body decode error from {path}: {source}")]
    JsonDecode {
        path: String,
        source: serde_json::Error,
    },

    #[error("Response exceeded max payload size ({max_bytes} bytes)")]
    PayloadTooLarge { max_bytes: usize },

    #[error("Stream error: {0}")]
    Stream(String),
}
