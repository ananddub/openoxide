use auto_di::resolve;
use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    ServerAudit as ServerAuditProto, ServerConnectionInfo, ServerPortAvailability,
    ServerSetupOutcome, ServerSetupStep, ServerToolState,
};
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
        ExecError::CommandFailed { .. } => {
            ConnectError::new(ErrorCode::Unavailable, error.to_string())
        }
        ExecError::Ssh(_) => ConnectError::new(ErrorCode::Unavailable, error.to_string()),
        ExecError::StreamCancelled => {
            ConnectError::new(ErrorCode::DeadlineExceeded, error.to_string())
        }
        ExecError::Timeout { .. } => {
            ConnectError::new(ErrorCode::DeadlineExceeded, error.to_string())
        }
        _ => ConnectError::new(ErrorCode::Internal, error.to_string()),
    }
}

/// Kick the one-shot lifecycle reconciliation after an explicit connection
/// check. The HTTP response is not held open while the agent is contacted.
pub fn trigger_monitoring_refresh(server_id: i64) {
    tokio::spawn(async move {
        let monitoring = match resolve::<
            crate::services::observability::monitoring::service::MonitoringService,
        >()
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

// ── Proto mappings for the typed ServerController RPCs ─────────────────

/// Placeholder connection report emitted by the probe RPCs: the panel does
/// not expose the live SSH session pool counters over ConnectRPC yet.
pub fn placeholder_connection_info() -> ServerConnectionInfo {
    ServerConnectionInfo {
        connected: true,
        ..Default::default()
    }
}

fn tool_state_to_proto(state: ToolState) -> ServerToolState {
    ServerToolState {
        installed: state.installed,
        version: state.version,
    }
}

fn port_to_proto(port: PortAvailability) -> ServerPortAvailability {
    ServerPortAvailability {
        port: u32::from(port.port),
        available: port.available,
    }
}

pub fn audit_to_proto(audit: ServerAudit) -> ServerAuditProto {
    ServerAuditProto {
        os_id: audit.os_id,
        architecture: audit.architecture,
        docker: ::buffa::MessageField::some(tool_state_to_proto(audit.docker)),
        git: ::buffa::MessageField::some(tool_state_to_proto(audit.git)),
        rclone: ::buffa::MessageField::some(tool_state_to_proto(audit.rclone)),
        rsync: ::buffa::MessageField::some(tool_state_to_proto(audit.rsync)),
        nixpacks: ::buffa::MessageField::some(tool_state_to_proto(audit.nixpacks)),
        railpack: ::buffa::MessageField::some(tool_state_to_proto(audit.railpack)),
        buildpacks: ::buffa::MessageField::some(tool_state_to_proto(audit.buildpacks)),
        swarm_active: audit.swarm_active,
        network_exists: audit.network_exists,
        base_directory_exists: audit.base_directory_exists,
        docker_group_member: audit.docker_group_member,
        ports: audit.ports.into_iter().map(port_to_proto).collect(),
    }
}

fn step_to_proto(step: SetupStep) -> ServerSetupStep {
    match step {
        SetupStep::Dependencies => ServerSetupStep::Dependencies,
        SetupStep::BuildTools => ServerSetupStep::BuildTools,
        SetupStep::Directories => ServerSetupStep::Directories,
        SetupStep::Swarm => ServerSetupStep::Swarm,
        SetupStep::Network => ServerSetupStep::Network,
        SetupStep::TraefikConfig => ServerSetupStep::TraefikConfig,
        SetupStep::Traefik => ServerSetupStep::Traefik,
        SetupStep::Agent => ServerSetupStep::Agent,
    }
}

pub fn outcome_to_proto(outcome: SetupOutcome) -> ServerSetupOutcome {
    ServerSetupOutcome {
        completed: outcome
            .completed
            .into_iter()
            .map(|step| ::buffa::EnumValue::Known(step_to_proto(step)))
            .collect(),
        audit: ::buffa::MessageField::some(audit_to_proto(outcome.audit)),
    }
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
