use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    MonitoringAgentAction as MonitoringAgentActionProto,
    MonitoringAgentState as ProtoMonitoringAgentState,
    MonitoringAgentStatus as MonitoringAgentStatusProto,
    MonitoringContainerMetric as MonitoringContainerMetricProto,
    MonitoringContainerState as MonitoringContainerStateProto,
    MonitoringContainerStats as MonitoringContainerStatsProto,
    MonitoringMaintenanceWindow as MonitoringMaintenanceWindowProto,
    MonitoringServerMetric as MonitoringServerMetricProto,
};
use crate::services::observability::monitoring::agent_auth::MonitoringAgentAuth;
use crate::services::observability::monitoring::lifecycle::{
    MaintenanceWindowDto, MonitoringActionDto,
};
use crate::services::observability::monitoring::proto;
use crate::services::observability::monitoring::service::{ContainerMetric, ServerMetric};

pub fn internal(error: sqlx::Error) -> ConnectError {
    tracing::error!(error = %error, "monitoring operation failed");
    ConnectError::new(ErrorCode::Internal, error.to_string())
}

pub async fn verify_server_organization(
    auth: &MonitoringAgentAuth,
    server_id: i64,
    organization_id: i64,
) -> Result<(), ConnectError> {
    let bound = auth
        .resolve_server_organization(server_id)
        .await
        .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
    if bound == Some(organization_id) {
        Ok(())
    } else {
        Err(ConnectError::not_found("monitoring server not found"))
    }
}

pub fn verify_claim_organization(
    claim_organization_id: i64,
    organization_id: i64,
) -> Result<(), ConnectError> {
    if claim_organization_id == organization_id {
        Ok(())
    } else {
        Err(ConnectError::new(
            ErrorCode::PermissionDenied,
            "organization does not match authenticated scope",
        ))
    }
}

pub fn server_metric_to_proto(metric: ServerMetric) -> MonitoringServerMetricProto {
    MonitoringServerMetricProto {
        timestamp: metric.timestamp,
        server_id: metric.server_id,
        cpu: metric.cpu,
        cpu_model: metric.cpu_model,
        cpu_cores: metric.cpu_cores,
        cpu_physical_cores: metric.cpu_physical_cores,
        cpu_speed: metric.cpu_speed,
        os: metric.os,
        distro: metric.distro,
        kernel: metric.kernel,
        arch: metric.arch,
        mem_used: metric.mem_used,
        mem_used_gb: metric.mem_used_gb,
        mem_total: metric.mem_total,
        uptime: metric.uptime,
        disk_used: metric.disk_used,
        total_disk: metric.total_disk,
        network_in: metric.network_in,
        network_out: metric.network_out,
    }
}

fn stats_from_json(value: &str) -> MonitoringContainerStatsProto {
    let pick = |payload: &serde_json::Value, key: &str| {
        payload
            .get(key)
            .and_then(|v| v.as_f64())
            .unwrap_or_default()
    };
    let payload: serde_json::Value = serde_json::from_str(value).unwrap_or_default();
    MonitoringContainerStatsProto {
        cpu_percent: pick(&payload, "cpu_percent"),
        memory_percent: pick(&payload, "memory_percent"),
        memory_used_mb: pick(&payload, "memory_used_mb"),
        memory_limit_mb: pick(&payload, "memory_limit_mb"),
        net_rx_kbps: pick(&payload, "net_rx_kbps"),
        net_tx_kbps: pick(&payload, "net_tx_kbps"),
        block_read_mb: pick(&payload, "block_read_mb"),
        block_write_mb: pick(&payload, "block_write_mb"),
    }
}

pub fn container_metric_to_proto(metric: ContainerMetric) -> MonitoringContainerMetricProto {
    MonitoringContainerMetricProto {
        id: metric.id,
        timestamp: metric.timestamp,
        container_id: metric.container_id,
        container_name: metric.container_name,
        stats: ::buffa::MessageField::some(stats_from_json(&metric.metrics_json)),
        server_id: metric.server_id,
        application_id: metric.application_id,
        compose_id: metric.compose_id,
    }
}

pub fn container_state_to_proto(
    state: proto::ContainerStatePoint,
) -> MonitoringContainerStateProto {
    MonitoringContainerStateProto {
        container_id: state.container_id,
        name: state.name,
        state: state.state,
        status: state.status,
        application_id: state.application_id,
        compose_id: state.compose_id,
        compose_project: state.compose_project,
        compose_service: state.compose_service,
    }
}

pub fn agent_status_to_proto(
    server_id: i64,
    organization_id: i64,
    last_seen_at: Option<i64>,
    agent_version: Option<String>,
) -> MonitoringAgentStatusProto {
    let state = match last_seen_at {
        Some(ts) if chrono::Utc::now().timestamp() - ts <= 30 => ProtoMonitoringAgentState::Online,
        Some(_) => ProtoMonitoringAgentState::Offline,
        None => ProtoMonitoringAgentState::NeverSeen,
    };
    MonitoringAgentStatusProto {
        server_id,
        organization_id,
        last_seen_at,
        agent_version,
        state: ::buffa::EnumValue::Known(state),
    }
}

pub fn agent_action_to_proto(action: MonitoringActionDto) -> MonitoringAgentActionProto {
    MonitoringAgentActionProto {
        action: action.action,
        success: action.success,
        message: action.message,
    }
}

pub fn maintenance_window_to_proto(
    window: MaintenanceWindowDto,
) -> MonitoringMaintenanceWindowProto {
    MonitoringMaintenanceWindowProto {
        id: window.id,
        organization_id: window.organization_id,
        server_id: window.server_id,
        starts_at: window.starts_at,
        ends_at: window.ends_at,
        reason: window.reason,
        created_at: window.created_at,
    }
}
