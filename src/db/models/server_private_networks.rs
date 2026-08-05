use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct ServerPrivateNetwork {
    pub server_id: i64,
    pub connection_mode: String,
    pub provider: Option<String>,
    pub private_host: Option<String>,
    pub tunnel_address: Option<String>,
    pub public_key: Option<String>,
    pub endpoint: Option<String>,
    pub listen_port: Option<i64>,
    pub persistent_keepalive: Option<i64>,
    pub status: String,
    pub last_handshake_at: Option<i64>,
    pub config_version: i64,
    pub created_at: i64,
    pub updated_at: i64,
}
