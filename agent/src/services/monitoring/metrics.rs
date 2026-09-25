use tokio::sync::Mutex;
use tokio_stream::StreamExt;
use tonic::{Response, Status};

use crate::api::proto::{
    ContainerMetricPoint, ContainerMetricsResponse, GetContainerMetricsRequest, GetMetricsRequest,
    HostInfo, ServerMetricPoint, ServerMetricsResponse,
};
use crate::docker::api::DockerApi;
use crate::docker::json_lines::JsonAccumulator;
use crate::docker::types::ContainerSummary;
use crate::telemetry::collector::bytes_to_mb;
use crate::telemetry::collector::system::{ServerMetric, SystemCollector};

/// Every metric is sampled live at request time — the agent stores no
/// history. Server vitals come from live sysinfo samples (two when the panel
/// asks for a trend); container vitals come straight from Docker's one-shot
/// stats API (the same multiplexed request `docker stats --no-stream` makes).
pub struct MetricsService;

impl MetricsService {
    pub async fn server(
        collector: &Mutex<SystemCollector>,
        request: GetMetricsRequest,
    ) -> Result<Response<ServerMetricsResponse>, Status> {
        let first = {
            let mut collector = collector.lock().await;
            collector.sample().await
        };
        // History is never stored; when the panel asks for a trend it gets a
        // second live sample a moment later, which is the minimum that makes
        // a sparkline drawable.
        let mut metrics = vec![server_point(&first)];
        if request.limit > 1 {
            let host = host_info(&first);
            tokio::time::sleep(std::time::Duration::from_millis(1200)).await;
            let second = {
                let mut collector = collector.lock().await;
                collector.sample().await
            };
            metrics.push(server_point(&second));
            return Ok(Response::new(ServerMetricsResponse {
                host: Some(host),
                metrics,
            }));
        }
        Ok(Response::new(ServerMetricsResponse {
            host: Some(host_info(&first)),
            metrics,
        }))
    }

    pub async fn containers(
        docker: &DockerApi,
        request: GetContainerMetricsRequest,
    ) -> Result<Response<ContainerMetricsResponse>, Status> {
        let summaries: Vec<ContainerSummary> = docker
            .get_json("/containers/json")
            .await
            .map_err(|error| Status::internal(format!("docker container list: {error}")))?;
        let matched: Vec<ContainerSummary> = summaries
            .into_iter()
            .filter(|summary| {
                summary
                    .display_name()
                    .as_str()
                    .contains(request.app_name.as_str())
            })
            .collect();
        if matched.is_empty() {
            return Ok(Response::new(ContainerMetricsResponse { metrics: vec![] }));
        }

        // One one-shot stats call per matched container, in parallel —
        // exactly what `docker stats --no-stream` does (joined-id
        // multiplexing was removed from Docker 29).
        let samples: Vec<Option<super::hub::RawStats>> =
            futures::future::join_all(matched.iter().map(|summary| {
                let docker = docker.clone();
                let id = summary.id.to_string();
                async move {
                    let mut chunks = docker
                        .get_stream(&format!("/containers/{id}/stats?stream=false"))
                        .await
                        .ok()?;
                    let mut accumulator = JsonAccumulator::new();
                    while let Some(chunk) = chunks.next().await {
                        let Ok(chunk) = chunk else { break };
                        if let Some(doc) = accumulator
                            .push::<super::hub::RawStats>(&chunk)
                            .into_iter()
                            .next()
                        {
                            return Some(doc);
                        }
                    }
                    None
                }
            }))
            .await;

        let now = chrono::Utc::now().timestamp();
        let mut metrics = Vec::new();
        for (summary, doc) in matched.iter().zip(samples) {
            let Some(doc) = doc else { continue };
            let (application_id, compose_id) = (
                label_id(summary, "com.openoxide.application-id"),
                label_id(summary, "com.openoxide.compose-id"),
            );
            let name = {
                let from_summary = summary.display_name().to_string();
                if from_summary.is_empty() {
                    doc.name.trim_start_matches('/').to_string()
                } else {
                    from_summary
                }
            };
            let (rx, tx_bytes) = doc.stats.network_bytes();
            let (read, write) = doc.stats.block_io_bytes();
            metrics.push(ContainerMetricPoint {
                id: 0,
                timestamp: now,
                container_id: doc.id,
                name,
                cpu_perc: doc.stats.cpu_percent(),
                mem_perc: doc.stats.memory.used_percent(),
                mem_used_mb: bytes_to_mb(doc.stats.memory.used_bytes() as f64),
                mem_total_mb: bytes_to_mb(doc.stats.memory.limit as f64),
                net_in_mb: bytes_to_mb(rx as f64),
                net_out_mb: bytes_to_mb(tx_bytes as f64),
                block_read_mb: bytes_to_mb(read as f64),
                block_write_mb: bytes_to_mb(write as f64),
                application_id,
                compose_id,
            });
        }
        Ok(Response::new(ContainerMetricsResponse { metrics }))
    }
}

fn host_info(row: &ServerMetric) -> HostInfo {
    HostInfo {
        cpu_model: row.cpu_model.clone(),
        cpu_cores: row.cpu_cores,
        cpu_physical_cores: row.cpu_physical_cores,
        cpu_speed: row.cpu_speed,
        os: row.os.clone(),
        distro: row.distro.clone(),
        kernel: row.kernel.clone(),
        arch: row.arch.clone(),
    }
}

fn server_point(row: &ServerMetric) -> ServerMetricPoint {
    ServerMetricPoint {
        timestamp: unix_timestamp(&row.timestamp),
        cpu: row.cpu,
        mem_used_gb: row.mem_used_gb,
        mem_total_gb: row.mem_total,
        // The sample keeps disk usage as a percentage of total; the wire
        // contract is GB, so derive the used amount.
        disk_used_gb: row.total_disk * row.disk_used / 100.0,
        disk_total_gb: row.total_disk,
        network_in: row.network_in,
        network_out: row.network_out,
        uptime: row.uptime,
    }
}

fn label_id(summary: &ContainerSummary, key: &str) -> i64 {
    summary
        .labels
        .get(key)
        .and_then(|value| value.parse().ok())
        .unwrap_or(0)
}

/// The sample carries an RFC3339 string; the wire format carries unix
/// seconds, so the conversion happens exactly here, at the boundary.
fn unix_timestamp(rfc3339: &str) -> i64 {
    chrono::DateTime::parse_from_rfc3339(rfc3339)
        .map(|value| value.timestamp())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamps_convert_at_the_boundary() {
        assert_eq!(unix_timestamp("2026-08-31T12:00:00Z"), 1_788_177_600);
        assert_eq!(unix_timestamp("not a timestamp"), 0);
    }
}
