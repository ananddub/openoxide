use std::collections::BTreeMap;

use poem_openapi::{Enum, Object};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct ServerCleanupPolicyDto {
    pub containers: bool,
    pub images: bool,
    pub networks: bool,
    pub volumes: bool,
    pub packages: bool,
}

impl Default for ServerCleanupPolicyDto {
    fn default() -> Self {
        Self {
            containers: true,
            images: true,
            networks: true,
            volumes: false,
            packages: false,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Object)]
pub struct UpdateServerManagementDto {
    pub maintenance_mode: bool,
    pub maintenance_message: Option<String>,
    pub labels: BTreeMap<String, String>,
    pub cleanup_policy: ServerCleanupPolicyDto,
    pub gpu_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Object)]
pub struct ServerManagementDto {
    pub server_id: i64,
    pub maintenance_mode: bool,
    pub maintenance_message: Option<String>,
    pub labels: BTreeMap<String, String>,
    pub cleanup_policy: ServerCleanupPolicyDto,
    pub gpu_enabled: bool,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Object)]
pub struct ServerActionResultDto {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, Serialize, Object)]
pub struct ServerCleanupExecutionDto {
    pub id: i64,
    pub server_id: i64,
    pub status: String,
    pub policy: String,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub started_at: i64,
    pub finished_at: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Object)]
pub struct ServerActionConnectionDto {
    pub sudo_password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Object)]
pub struct ServerBackupDto {
    pub remote_path: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServerConnectionModeDto {
    DirectSsh,
    ManagedWireguard,
    ExternalPrivateNetwork,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrivateNetworkProviderDto {
    Wireguard,
    Tailscale,
    Zerotier,
    Netbird,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrivateNetworkStatusDto {
    Disabled,
    Configuring,
    Active,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrivateNetworkHealthStatusDto {
    Unknown,
    Healthy,
    Degraded,
    Unreachable,
    ConfigDrift,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrivateNetworkRotationStateDto {
    Idle,
    Rotating,
    RollingBack,
    Failed,
}

#[derive(Debug, Clone, Deserialize, Object)]
pub struct UpdatePrivateNetworkDto {
    pub connection_mode: ServerConnectionModeDto,
    pub provider: Option<PrivateNetworkProviderDto>,
    pub private_host: Option<String>,
    pub tunnel_address: Option<String>,
    pub public_key: Option<String>,
    pub endpoint: Option<String>,
    pub listen_port: Option<u16>,
    pub persistent_keepalive: Option<u16>,
    pub dns_name: Option<String>,
    pub routes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Object)]
pub struct ServerPrivateNetworkDto {
    pub server_id: i64,
    pub connection_mode: ServerConnectionModeDto,
    pub provider: Option<PrivateNetworkProviderDto>,
    pub private_host: Option<String>,
    pub tunnel_address: Option<String>,
    pub public_key: Option<String>,
    pub endpoint: Option<String>,
    pub listen_port: Option<u16>,
    pub persistent_keepalive: Option<u16>,
    pub status: PrivateNetworkStatusDto,
    pub last_handshake_at: Option<i64>,
    pub config_version: i64,
    pub dns_name: Option<String>,
    pub routes: Vec<String>,
    pub health_status: PrivateNetworkHealthStatusDto,
    pub health_error: Option<String>,
    pub last_health_check_at: Option<i64>,
    pub consecutive_failures: i64,
    pub rotation_state: PrivateNetworkRotationStateDto,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Object)]
pub struct PrivateNetworkHealthDto {
    pub status: PrivateNetworkHealthStatusDto,
    pub interface_exists: bool,
    pub private_ssh_reachable: bool,
    pub latest_handshake_at: Option<i64>,
    pub checked_at: i64,
    pub error: Option<String>,
}
