use crate::db::models::server_metrics::ServerMetric;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct ServerMetricResponseDto {
    pub timestamp: Option<i64>,
    pub server_id: i64,
    pub cpu: f64,
    pub cpu_model: String,
    pub cpu_cores: i64,
    pub cpu_physical_cores: i64,
    pub cpu_speed: f64,
    pub os: String,
    pub distro: String,
    pub kernel: String,
    pub arch: String,
    pub mem_used: f64,
    pub mem_used_gb: f64,
    pub mem_total: f64,
    pub uptime: i64,
    pub disk_used: f64,
    pub total_disk: f64,
    pub network_in: f64,
    pub network_out: f64,
}

impl From<ServerMetric> for ServerMetricResponseDto {
    fn from(m: ServerMetric) -> Self {
        Self {
            timestamp: m.timestamp,
            server_id: m.server_id,
            cpu: m.cpu,
            cpu_model: m.cpu_model,
            cpu_cores: m.cpu_cores,
            cpu_physical_cores: m.cpu_physical_cores,
            cpu_speed: m.cpu_speed,
            os: m.os,
            distro: m.distro,
            kernel: m.kernel,
            arch: m.arch,
            mem_used: m.mem_used,
            mem_used_gb: m.mem_used_gb,
            mem_total: m.mem_total,
            uptime: m.uptime,
            disk_used: m.disk_used,
            total_disk: m.total_disk,
            network_in: m.network_in,
            network_out: m.network_out,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct ContainerMetricResponseDto {
    pub server_id: i64,
    pub application_id: i64,
    pub compose_id: i64,
    pub container_id: String,
    pub container_name: String,
    pub cpu_percent: f64,
    pub memory_used_mb: f64,
    pub memory_limit_mb: f64,
    pub net_rx_kbps: f64,
    pub net_tx_kbps: f64,
    pub timestamp: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct MonitoringStatusResponseDto {
    pub status: String,
    pub service: String,
    pub endpoints: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct RotateMonitoringTokenDto {
    pub organization_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct MonitoringTokenResponseDto {
    pub server_id: i64,
    pub organization_id: i64,
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object, ts_rs::TS)]
pub struct MonitoringAgentStatusDto {
    pub server_id: i64,
    pub organization_id: i64,
    pub last_seen_at: Option<i64>,
    pub agent_version: Option<String>,
    pub state: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object, ts_rs::TS)]
pub struct MonitoringPolicyDto {
    pub organization_id: i64,
    pub desired_agent_version: Option<String>,
    pub retention_days: i64,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct UpdateMonitoringPolicyDto {
    pub desired_agent_version: Option<String>,
    pub retention_days: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct MonitoringActionDto {
    pub action: String,
    pub success: bool,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct CreateMaintenanceWindowDto {
    pub server_id: Option<i64>,
    pub starts_at: i64,
    pub ends_at: i64,
    pub reason: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object, ts_rs::TS)]
pub struct MaintenanceWindowDto {
    pub id: i64,
    pub organization_id: i64,
    pub server_id: Option<i64>,
    pub starts_at: i64,
    pub ends_at: i64,
    pub reason: String,
    pub created_at: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct SilenceAlertDto {
    pub until: i64,
}
