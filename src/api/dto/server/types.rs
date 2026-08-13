use crate::utils::setup::{PortAvailability, ServerAudit, SetupOutcome, SetupStep, ToolState};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Default, Deserialize, Object)]
pub struct ServerConnectionDto {
    pub host_key_fingerprint: Option<String>,
    pub sudo_password: Option<String>,
    pub pool_size: Option<usize>,
}

#[derive(Clone, Debug, Default, Deserialize, Object)]
pub struct TestDirectConnectionDto {
    pub ip_address: String,
    pub port: Option<u16>,
    pub username: String,
    pub ssh_key_id: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Object)]
pub struct SetupServerDto {
    #[serde(default)]
    pub install_dependencies: bool,
    pub advertise_addr: Option<String>,
    pub acme_email: Option<String>,
    pub host_key_fingerprint: Option<String>,
    pub sudo_password: Option<String>,
    pub pool_size: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Object)]
pub struct ToolStateDto {
    pub installed: bool,
    pub version: Option<String>,
}
impl From<ToolState> for ToolStateDto {
    fn from(v: ToolState) -> Self {
        Self {
            installed: v.installed,
            version: v.version,
        }
    }
}

#[derive(Clone, Debug, Serialize, Object)]
pub struct PortAvailabilityDto {
    pub port: u16,
    pub available: bool,
}
impl From<PortAvailability> for PortAvailabilityDto {
    fn from(v: PortAvailability) -> Self {
        Self {
            port: v.port,
            available: v.available,
        }
    }
}

#[derive(Clone, Debug, Serialize, Object)]
pub struct ServerAuditDto {
    pub os_id: String,
    pub architecture: String,
    pub docker: ToolStateDto,
    pub git: ToolStateDto,
    pub rclone: ToolStateDto,
    pub nixpacks: ToolStateDto,
    pub railpack: ToolStateDto,
    pub buildpacks: ToolStateDto,
    pub swarm_active: bool,
    pub network_exists: bool,
    pub base_directory_exists: bool,
    pub docker_group_member: bool,
    pub ports: Vec<PortAvailabilityDto>,
}
impl From<ServerAudit> for ServerAuditDto {
    fn from(v: ServerAudit) -> Self {
        Self {
            os_id: v.os_id,
            architecture: v.architecture,
            docker: v.docker.into(),
            git: v.git.into(),
            rclone: v.rclone.into(),
            nixpacks: v.nixpacks.into(),
            railpack: v.railpack.into(),
            buildpacks: v.buildpacks.into(),
            swarm_active: v.swarm_active,
            network_exists: v.network_exists,
            base_directory_exists: v.base_directory_exists,
            docker_group_member: v.docker_group_member,
            ports: v.ports.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Object)]
pub struct SetupOutcomeDto {
    pub completed: Vec<String>,
    pub audit: ServerAuditDto,
}
impl From<SetupOutcome> for SetupOutcomeDto {
    fn from(v: SetupOutcome) -> Self {
        Self {
            completed: v
                .completed
                .into_iter()
                .map(|s| {
                    match s {
                        SetupStep::Dependencies => "dependencies",
                        SetupStep::BuildTools => "build-tools",
                        SetupStep::Directories => "directories",
                        SetupStep::Swarm => "swarm",
                        SetupStep::Network => "network",
                        SetupStep::TraefikConfig => "traefik-config",
                        SetupStep::Traefik => "traefik",
                        SetupStep::Monitoring => "monitoring",
                    }
                    .into()
                })
                .collect(),
            audit: v.audit.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Object, ts_rs::TS)]
pub struct ServerConnectionResponseDto {
    pub connected: bool,
    pub reused_sessions: usize,
    pub max_pool_size: usize,
    pub connections: usize,
    pub active_channels: usize,
    pub max_channels_per_session: usize,
}

// Remote Server DTOs
use crate::db::models::servers::Server;

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateRemoteServerDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub ip_address: String,
    #[serde(default = "default_port")]
    pub port: i64,
    #[serde(default = "default_username")]
    pub username: String,
    #[serde(default)]
    pub server_type: RemoteServerTypeDto,
    pub ssh_key_id: Option<i64>,
    pub build_memory_limit: Option<String>,
    pub build_cpu_limit: Option<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct TestRemoteServerConnectionDto {
    #[validate(length(min = 1, max = 255))]
    pub ip_address: String,
    #[serde(default = "default_port")]
    pub port: i64,
    #[serde(default = "default_username")]
    pub username: String,
    pub ssh_key_id: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchRemoteServerDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    pub ip_address: Option<String>,
    pub port: Option<i64>,
    pub username: Option<String>,
    pub server_status: Option<RemoteServerStatusDto>,
    pub server_type: Option<RemoteServerTypeDto>,
    pub enable_docker_cleanup: Option<i64>,
    pub log_cleanup_cron: Option<String>,
    pub command: Option<String>,
    pub metrics_config: Option<String>,
    pub ssh_key_id: Option<i64>,
    pub build_memory_limit: Option<String>,
    pub build_cpu_limit: Option<String>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct RemoteServerResponseDto {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub ip_address: String,
    pub port: i64,
    pub username: String,
    pub app_name: String,
    pub server_status: String,
    pub server_type: String,
    pub enable_docker_cleanup: i64,
    pub log_cleanup_cron: Option<String>,
    pub command: String,
    pub metrics_config: String,
    pub ssh_key_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
    pub build_memory_limit: Option<String>,
    pub build_cpu_limit: Option<String>,
}

impl From<Server> for RemoteServerResponseDto {
    fn from(value: Server) -> Self {
        Self {
            id: value.id.expect("persisted server must have an id"),
            name: value.name,
            description: value.description,
            ip_address: value.ip_address,
            port: value.port,
            username: value.username,
            app_name: value.app_name,
            server_status: value.server_status,
            server_type: value.server_type,
            enable_docker_cleanup: value.enable_docker_cleanup,
            log_cleanup_cron: value.log_cleanup_cron,
            command: value.command,
            metrics_config: value.metrics_config,
            ssh_key_id: value.ssh_key_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            build_memory_limit: value.build_memory_limit,
            build_cpu_limit: value.build_cpu_limit,
        }
    }
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct RemoteServerActionResponseDto {
    pub server: RemoteServerResponseDto,
    pub action: RemoteServerAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, poem_openapi::Enum)]
#[serde(rename_all = "kebab-case")]
#[oai(rename_all = "kebab-case")]
pub enum RemoteServerAction {
    Activate,
    Deactivate,
    TestConnection,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct MigrateServerDependenciesDto {
    #[validate(range(min = 1))]
    pub target_server_id: i64,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct ServerDependencyMigrationDto {
    pub migration_id: String,
    pub status: String,
    pub source_server_id: i64,
    pub target_server_id: i64,
    pub applications: i64,
    pub build_assignments: i64,
    pub compose_projects: i64,
    pub databases: i64,
    pub certificates: i64,
    pub schedules: i64,
    pub queued_applications: i64,
    pub queued_compose_projects: i64,
    pub error: Option<String>,
}

impl TryFrom<crate::repository::ServerMigration> for ServerDependencyMigrationDto {
    type Error = sqlx::Error;

    fn try_from(value: crate::repository::ServerMigration) -> Result<Self, Self::Error> {
        let applications: Vec<i64> = serde_json::from_str(&value.application_ids)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let build: Vec<i64> = serde_json::from_str(&value.build_application_ids)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let compose: Vec<i64> = serde_json::from_str(&value.compose_ids)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let certificates: Vec<i64> = serde_json::from_str(&value.certificate_ids)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let schedules: Vec<i64> = serde_json::from_str(&value.schedule_ids)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(Self {
            migration_id: value.id,
            status: value.status,
            source_server_id: value.source_server_id,
            target_server_id: value.target_server_id,
            applications: applications.len() as i64,
            build_assignments: build.len() as i64,
            compose_projects: compose.len() as i64,
            databases: 0,
            certificates: certificates.len() as i64,
            schedules: schedules.len() as i64,
            queued_applications: value.queued_applications,
            queued_compose_projects: value.queued_compose_projects,
            error: value.error,
        })
    }
}

fn default_port() -> i64 {
    22
}

fn default_username() -> String {
    "root".into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, poem_openapi::Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RemoteServerTypeDto {
    Deploy,
    Build,
}

impl Default for RemoteServerTypeDto {
    fn default() -> Self {
        Self::Deploy
    }
}

impl RemoteServerTypeDto {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Deploy => "DEPLOY",
            Self::Build => "BUILD",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, poem_openapi::Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RemoteServerStatusDto {
    Active,
    Inactive,
}

impl RemoteServerStatusDto {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "ACTIVE",
            Self::Inactive => "INACTIVE",
        }
    }
}

// Server Management & Private Network DTOs
use poem_openapi::Enum;

#[derive(Debug, Clone, Serialize, Deserialize, Object, ts_rs::TS)]
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
    pub labels: std::collections::BTreeMap<String, String>,
    pub cleanup_policy: ServerCleanupPolicyDto,
    pub gpu_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Object, ts_rs::TS)]
pub struct ServerManagementDto {
    pub server_id: i64,
    pub maintenance_mode: bool,
    pub maintenance_message: Option<String>,
    pub labels: std::collections::BTreeMap<String, String>,
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

#[derive(Debug, Clone, Serialize, Object, ts_rs::TS)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, ts_rs::TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServerConnectionModeDto {
    DirectSsh,
    ManagedWireguard,
    ExternalPrivateNetwork,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, ts_rs::TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrivateNetworkProviderDto {
    Wireguard,
    Tailscale,
    Zerotier,
    Netbird,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, ts_rs::TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrivateNetworkStatusDto {
    Disabled,
    Configuring,
    Active,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, ts_rs::TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrivateNetworkHealthStatusDto {
    Unknown,
    Healthy,
    Degraded,
    Unreachable,
    ConfigDrift,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, ts_rs::TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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

#[derive(Debug, Clone, Serialize, Object, ts_rs::TS)]
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

#[derive(Debug, Clone, Serialize, Object, ts_rs::TS)]
pub struct PrivateNetworkHealthDto {
    pub status: PrivateNetworkHealthStatusDto,
    pub interface_exists: bool,
    pub private_ssh_reachable: bool,
    pub latest_handshake_at: Option<i64>,
    pub checked_at: i64,
    pub error: Option<String>,
}

#[cfg(test)]
mod private_network_json_tests {
    use super::{PrivateNetworkStatusDto, ServerConnectionModeDto, UpdatePrivateNetworkDto};

    #[test]
    fn accepts_frontend_enum_casing() {
        let input: UpdatePrivateNetworkDto = serde_json::from_value(serde_json::json!({
            "connection_mode": "MANAGED_WIREGUARD",
            "provider": "WIREGUARD",
            "private_host": "10.77.2.2",
            "tunnel_address": "10.77.2.0/24",
            "public_key": null,
            "endpoint": "panel.example.com:51820",
            "listen_port": 51820,
            "persistent_keepalive": 25,
            "dns_name": null,
            "routes": []
        }))
        .unwrap();
        assert_eq!(
            input.connection_mode,
            ServerConnectionModeDto::ManagedWireguard
        );
    }

    #[test]
    fn serializes_response_enums_for_frontend() {
        assert_eq!(
            serde_json::to_string(&PrivateNetworkStatusDto::Configuring).unwrap(),
            "\"CONFIGURING\""
        );
    }
}
