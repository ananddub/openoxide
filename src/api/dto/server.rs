use crate::utils::setup::{PortAvailability, ServerAudit, SetupOutcome, SetupStep, ToolState};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Serialize, Object)]
pub struct ServerConnectionResponseDto {
    pub connected: bool,
    pub reused_sessions: usize,
    pub max_pool_size: usize,
    pub connections: usize,
    pub active_channels: usize,
    pub max_channels_per_session: usize,
}
