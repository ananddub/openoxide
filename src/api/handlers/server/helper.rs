use auto_di::resolve;
use connectrpc::{ConnectError, ErrorCode};
use serde::{Deserialize, Serialize};

use crate::utils::exec::{ExecError, SshHostKey};
use crate::utils::setup::{PortAvailability, ServerAudit, SetupOutcome, SetupStep, ToolState};

pub fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("server or SSH key not found"),
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => {
            tracing::warn!(error = %e.message(), "foreign key violation");
            ConnectError::not_found("ssh key not found")
        }
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            tracing::warn!(error = %e.message(), "unique constraint violation");
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        other => {
            tracing::error!(error = %other, "server database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

pub fn map_exec_error(error: ExecError) -> ConnectError {
    tracing::error!(error=%error, "remote server command failed");
    match error {
        ExecError::CommandFailed { .. } => ConnectError::new(ErrorCode::Unavailable, error.to_string()),
        ExecError::Ssh(_) => ConnectError::new(ErrorCode::Unavailable, error.to_string()),
        ExecError::StreamCancelled => ConnectError::new(ErrorCode::DeadlineExceeded, error.to_string()),
        ExecError::Timeout { .. } => ConnectError::new(ErrorCode::DeadlineExceeded, error.to_string()),
        _ => ConnectError::new(ErrorCode::Internal, error.to_string()),
    }
}

/// Kick the one-shot lifecycle reconciliation after an explicit connection
/// check. The HTTP response is not held open while the agent is contacted.
pub fn trigger_monitoring_refresh(server_id: i64) {
    tokio::spawn(async move {
        let monitoring =
            match resolve::<crate::services::observability::monitoring::monitoring_service::MonitoringService>()
                .await
            {
                Ok(service) => service,
                Err(error) => {
                    tracing::debug!(server_id, %error, "monitoring refresh service unavailable");
                    return;
                }
            };
        let reconciler = match resolve::<
            crate::services::observability::monitoring::reconciler::MonitoringReconciler,
        >()
        .await
        {
            Ok(reconciler) => reconciler,
            Err(error) => {
                tracing::debug!(server_id, %error, "monitoring reconciler unavailable");
                return;
            }
        };
        match monitoring.container_states(server_id).await {
            Ok(states) => {
                if let Err(error) = reconciler.apply_snapshot(server_id, &states).await {
                    tracing::warn!(server_id, %error, "explicit monitoring refresh reconciliation failed");
                }
            }
            Err(error) => {
                tracing::debug!(server_id, %error, "agent unavailable during explicit monitoring refresh")
            }
        }
    });
}

pub fn host_key_policy(server_id: i64, host_key: &Option<String>) -> SshHostKey {
    match host_key
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        Some(fingerprint) => SshHostKey::PinnedSha256(fingerprint.to_owned()),
        None => {
            tracing::warn!(
                server_id,
                "SSH host key verification disabled; provide host_key_fingerprint"
            );
            SshHostKey::InsecureAcceptAny
        }
    }
}

// ── Helper DTOs for the generic Resource endpoints (moved from the retired
//    api::dto layer; JSON payload contract must not change) ─────────────

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ServerConnectionDto {
    pub host_key_fingerprint: Option<String>,
    pub sudo_password: Option<String>,
    pub pool_size: Option<usize>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct TestDirectConnectionDto {
    pub ip_address: String,
    pub port: Option<u16>,
    pub username: String,
    pub ssh_key_id: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct SetupServerDto {
    #[serde(default)]
    pub install_dependencies: bool,
    pub advertise_addr: Option<String>,
    pub acme_email: Option<String>,
    pub host_key_fingerprint: Option<String>,
    pub sudo_password: Option<String>,
    pub pool_size: Option<usize>,
}

#[derive(Clone, Debug, Serialize)]
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

#[derive(Clone, Debug, Serialize)]
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

#[derive(Clone, Debug, Serialize)]
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

#[derive(Clone, Debug, Serialize)]
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
                        SetupStep::Agent => "agent",
                    }
                    .into()
                })
                .collect(),
            audit: v.audit.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ServerConnectionResponseDto {
    pub connected: bool,
    pub reused_sessions: usize,
    pub max_pool_size: usize,
    pub connections: usize,
    pub active_channels: usize,
    pub max_channels_per_session: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_host_key_fingerprint_disables_pinning() {
        assert!(matches!(
            host_key_policy(1, &None),
            SshHostKey::InsecureAcceptAny
        ));
        assert!(matches!(
            host_key_policy(1, &Some(String::new())),
            SshHostKey::InsecureAcceptAny
        ));
        assert!(matches!(
            host_key_policy(1, &Some("   ".to_owned())),
            SshHostKey::InsecureAcceptAny
        ));
    }

    #[test]
    fn non_empty_host_key_fingerprint_is_trimmed_and_pinned() {
        match host_key_policy(1, &Some(" SHA256:abc ".to_owned())) {
            SshHostKey::PinnedSha256(value) => assert_eq!(value, "SHA256:abc"),
            SshHostKey::InsecureAcceptAny => panic!("expected pinned host key"),
        }
    }
}
