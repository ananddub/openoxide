use crate::api::proto::{
    ContainerMetricPoint, ContainerMetricsResponse, GetContainerMetricsRequest, GetMetricsRequest,
    HostInfo, ServerMetricPoint, ServerMetricsResponse,
};
use crate::store::Store;
use tonic::{Response, Status};
use tracing::debug;

const DEFAULT_LIMIT: i64 = 50;
const MAX_LIMIT: i64 = 1000;

pub struct MetricsService;

impl MetricsService {
    pub async fn server(
        store: &Store,
        request: GetMetricsRequest,
    ) -> Result<Response<ServerMetricsResponse>, Status> {
        let limit = clamp_limit(request.limit as i64);
        let rows = store
            .get_last_n_server_metrics(limit)
            .await
            .map_err(|error| Status::internal(error.to_string()))?;
        debug!(
            requested = limit,
            returned = rows.len(),
            "serving server metrics gRPC"
        );

        // Host facts come from the newest sample; every point keeps only the
        // values that actually change between samples.
        let host = rows.iter().rev().next().map(|row| HostInfo {
            cpu_model: row.cpu_model.clone(),
            cpu_cores: row.cpu_cores,
            cpu_physical_cores: row.cpu_physical_cores,
            cpu_speed: row.cpu_speed,
            os: row.os.clone(),
            distro: row.distro.clone(),
            kernel: row.kernel.clone(),
            arch: row.arch.clone(),
        });
        let metrics = rows
            .into_iter()
            .map(|row| ServerMetricPoint {
                timestamp: unix_timestamp(&row.timestamp),
                cpu: row.cpu,
                mem_used_gb: row.mem_used_gb,
                mem_total_gb: row.mem_total,
                // The store keeps disk usage as a percentage of total; the
                // wire contract is GB, so derive the used amount.
                disk_used_gb: row.total_disk * row.disk_used / 100.0,
                disk_total_gb: row.total_disk,
                network_in: row.network_in,
                network_out: row.network_out,
                uptime: row.uptime,
            })
            .collect();
        Ok(Response::new(ServerMetricsResponse { host, metrics }))
    }

    pub async fn containers(
        store: &Store,
        request: GetContainerMetricsRequest,
    ) -> Result<Response<ContainerMetricsResponse>, Status> {
        let limit = clamp_limit(request.limit as i64);
        let rows = store
            .get_last_n_container_metrics(&request.app_name, limit)
            .await
            .map_err(|error| Status::internal(error.to_string()))?;
        debug!(container = %request.app_name, requested = limit, returned = rows.len(), "serving container metrics gRPC");

        let metrics = rows
            .into_iter()
            .map(|row| ContainerMetricPoint {
                id: row.id.unwrap_or(0),
                timestamp: unix_timestamp(&row.timestamp),
                container_id: row.container_id,
                name: row.name,
                cpu_perc: row.cpu_perc,
                mem_perc: row.mem_perc,
                mem_used_mb: row.mem_used_mb,
                mem_total_mb: row.mem_total_mb,
                net_in_mb: row.net_in_mb,
                net_out_mb: row.net_out_mb,
                block_read_mb: row.block_read_mb,
                block_write_mb: row.block_write_mb,
                application_id: row.application_id.unwrap_or(0),
                compose_id: row.compose_id.unwrap_or(0),
            })
            .collect();
        Ok(Response::new(ContainerMetricsResponse { metrics }))
    }
}

/// The in-agent store persists RFC3339 strings; the wire format carries unix
/// seconds, so the conversion happens exactly here, at the boundary.
fn unix_timestamp(rfc3339: &str) -> i64 {
    chrono::DateTime::parse_from_rfc3339(rfc3339)
        .map(|value| value.timestamp())
        .unwrap_or_default()
}

fn clamp_limit(raw: i64) -> i64 {
    if raw <= 0 {
        DEFAULT_LIMIT
    } else {
        raw.min(MAX_LIMIT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limit_defaults_and_is_capped() {
        assert_eq!(clamp_limit(0), DEFAULT_LIMIT);
        assert_eq!(clamp_limit(-5), DEFAULT_LIMIT);
        assert_eq!(clamp_limit(10), 10);
        assert_eq!(clamp_limit(50_000), MAX_LIMIT);
    }

    #[test]
    fn timestamps_convert_at_the_boundary() {
        assert_eq!(unix_timestamp("2026-08-31T12:00:00Z"), 1_788_177_600);
        assert_eq!(unix_timestamp("not a timestamp"), 0);
    }
}
