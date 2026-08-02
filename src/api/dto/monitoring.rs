use crate::db::models::server_metrics::ServerMetric;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct IngestSystemMetricDto {
    pub cpu: f64,
    pub cpu_model: Option<String>,
    pub cpu_cores: Option<i64>,
    pub cpu_physical_cores: Option<i64>,
    pub cpu_speed: Option<f64>,
    pub os: Option<String>,
    pub distro: Option<String>,
    pub kernel: Option<String>,
    pub arch: Option<String>,
    pub mem_used: f64,
    pub mem_used_gb: f64,
    pub mem_total: f64,
    pub uptime: i64,
    pub disk_used: f64,
    pub total_disk: f64,
    pub network_in: f64,
    pub network_out: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct IngestContainerMetricDto {
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

/// A batch of container metrics from one agent poll.
///
/// Agents stream stats per second per container; sending each as its own
/// request would mean N requests a second, so they are flushed together.
#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct IngestContainerMetricBatchDto {
    pub metrics: Vec<IngestContainerMetricDto>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct ServerMetricResponseDto {
    pub timestamp: Option<i64>,
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
pub struct MetricIngestResponseDto {
    pub success: bool,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct MonitoringStatusResponseDto {
    pub status: String,
    pub service: String,
    pub endpoints: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct ContainerMetricSseEventDto {
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
pub struct ContainerLogSseEventDto {
    pub container_id: String,
    pub log_line: String,
    pub timestamp: i64,
    pub is_stderr: bool,
}
