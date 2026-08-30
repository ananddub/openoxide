use connectrpc::ConnectError;
use serde::Deserialize;

pub fn map_error(error: String) -> ConnectError {
    tracing::warn!(%error, "networking operation failed");
    ConnectError::invalid_argument(error)
}

// ── Helper DTOs for deserializing ResourceRequest payloads ────────────

#[derive(Debug, Deserialize)]
pub struct RootNetworkDto {
    pub server_id: Option<i64>,
}
