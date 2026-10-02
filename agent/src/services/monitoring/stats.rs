use std::{pin::Pin, sync::Arc};

use tokio_stream::{Stream, StreamExt};
use tonic::{Response, Status};

use crate::api::proto::StatsFrame;
use crate::docker::api::DockerApi;
use crate::docker::json_lines::JsonAccumulator;
use crate::docker::types::{ContainerStats, ContainerSummary};
use serde::Deserialize;

pub type StatsStream = Pin<Box<dyn Stream<Item = Result<StatsFrame, Status>> + Send + 'static>>;

/// Raw stats document: CLI-relevant extras (`name`, `id`, pids) layered over
/// the metric fields `ContainerStats` already models.
#[derive(Debug, Deserialize)]
pub struct RawStats {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "pids_stats", default)]
    pub pids: PidsStats,
    #[serde(flatten)]
    pub stats: ContainerStats,
}

#[derive(Debug, Default, Deserialize)]
pub struct PidsStats {
    #[serde(default)]
    pub current: u64,
}

/// Streams container stats straight from Docker with nothing held in
/// between: one live `/stats` connection per requested container while the
/// panel watches (or a one-shot `stream=false` sample per container), each
/// document parsed and forwarded the instant it arrives. The agent queues no
/// frames and merges nothing — when the panel stops reading, Docker's own
/// flow control pauses the socket and the agent simply waits.
pub struct StatsService;

impl StatsService {
    pub async fn stream(
        docker: Arc<DockerApi>,
        request: crate::api::proto::StreamStatsRequest,
    ) -> Result<Response<StatsStream>, Status> {
        let ids = if request.container_ids.is_empty() {
            let summaries: Vec<ContainerSummary> = docker
                .get_json("/containers/json")
                .await
                .map_err(|error| Status::unavailable(error.to_string()))?;
            summaries.into_iter().map(|s| s.id.to_string()).collect()
        } else {
            request.container_ids
        };

        if request.stream {
            let feeds: Vec<_> = ids
                .into_iter()
                .map(|id| container_feed(docker.clone(), id))
                .collect();
            let stream = async_stream::stream! {
                let mut merged = futures::stream::select_all(feeds);
                while let Some(frame) = merged.next().await {
                    yield Ok(frame);
                }
            };
            return Ok(Response::new(Box::pin(stream)));
        }

        let stream = async_stream::stream! {
            for id in ids {
                if let Some(frame) = one_shot(&docker, &id).await {
                    yield Ok(frame);
                }
            }
        };
        Ok(Response::new(Box::pin(stream)))
    }
}

/// One container's live `/stats` feed rendered to CLI-shaped JSON. The
/// writable-layer size is inspected once when the feed opens because the
/// stats API never carries `size_rw`.
fn container_feed(
    docker: Arc<DockerApi>,
    id: String,
) -> futures::stream::BoxStream<'static, StatsFrame> {
    Box::pin(async_stream::stream! {
        let size_rw = inspect_size_rw(&docker, &id).await;
        let mut chunks = match docker
            .get_stream(&format!("/containers/{id}/stats?stream=true"))
            .await
        {
            Ok(chunks) => chunks,
            Err(error) => {
                tracing::debug!(%error, "stats feed could not open");
                return;
            }
        };
        let mut accumulator = JsonAccumulator::new();
        while let Some(chunk) = chunks.next().await {
            match chunk {
                Ok(chunk) => {
                    for doc in accumulator.push::<RawStats>(&chunk) {
                        yield StatsFrame {
                            json_line: to_cli_line(&doc, size_rw),
                        };
                    }
                }
                Err(error) => {
                    tracing::debug!(%error, "stats feed error");
                    return;
                }
            }
        }
        // Dropping `chunks` releases the dedicated Docker connection.
    })
}

/// A single live sample of one container, exactly what `docker stats
/// --no-stream` collects.
async fn one_shot(docker: &DockerApi, id: &str) -> Option<StatsFrame> {
    let mut chunks = docker
        .get_stream(&format!("/containers/{id}/stats?stream=false"))
        .await
        .ok()?;
    let mut accumulator = JsonAccumulator::new();
    while let Some(chunk) = chunks.next().await {
        let Ok(chunk) = chunk else { break };
        if let Some(doc) = accumulator.push::<RawStats>(&chunk).into_iter().next() {
            let size_rw = inspect_size_rw(docker, id).await;
            return Some(StatsFrame {
                json_line: to_cli_line(&doc, size_rw),
            });
        }
    }
    None
}

/// Inspects a container's writable-layer size (stats API omits `size_rw`).
async fn inspect_size_rw(docker: &DockerApi, id: &str) -> u64 {
    docker
        .get_json::<serde_json::Value>(&format!("/containers/{id}/json?size=1"))
        .await
        .ok()
        .and_then(|doc| doc.get("SizeRw").and_then(|v| v.as_u64()))
        .unwrap_or(0)
}

/// Renders one API stats document as the JSON object produced by
/// `docker stats --format {{json .}}`.
fn to_cli_line(doc: &RawStats, size_rw: u64) -> String {
    let name = doc.name.trim_start_matches('/');
    let short_id = doc.id.get(..12).unwrap_or(&doc.id);
    let (rx, tx_bytes) = doc.stats.network_bytes();
    let (read, write) = doc.stats.block_io_bytes();
    serde_json::json!({
        "BlockIO": format!("{} / {}", human(read, false), human(write, false)),
        "CPUPerc": format!("{:.2}%", doc.stats.cpu_percent()),
        "Container": short_id,
        "ID": doc.id,
        "MemPerc": format!("{:.2}%", doc.stats.memory.used_percent()),
        "MemUsage": format!(
            "{} / {}",
            human(doc.stats.memory.used_bytes(), true),
            human(doc.stats.memory.limit, true)
        ),
        "Name": name,
        "NetIO": format!("{} / {}", human(rx, false), human(tx_bytes, false)),
        "PIDs": doc.pids.current.to_string(),
        // Writable-layer disk usage. Docker's own `docker stats` shows this as
        // `SIZE`; there is no percentage (containers have no disk quota), so
        // consumers render it as a byte value only.
        "SizeRw": human(size_rw, true),
    })
    .to_string()
}

/// Mirrors Docker's own formatters: binary units for memory (MiB/GiB), SI for
/// net and block IO (kB/MB). Sub-unit values print as integer bytes.
fn human(bytes: u64, binary: bool) -> String {
    let divisor = if binary { 1024.0 } else { 1000.0 };
    let units: &[&str] = if binary {
        &["B", "KiB", "MiB", "GiB", "TiB", "PiB"]
    } else {
        &["B", "kB", "MB", "GB", "TB", "PB"]
    };
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= divisor && unit + 1 < units.len() {
        value /= divisor;
        unit += 1;
    }
    if unit == 0 {
        format!("{}B", value as u64)
    } else {
        format!("{value:.2}{}", units[unit])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_uses_binary_units_and_net_uses_si() {
        assert_eq!(human(23_620_000, true), "22.53MiB");
        assert_eq!(human(4_100, false), "4.10kB");
        assert_eq!(human(999, false), "999B");
        assert_eq!(human(0, true), "0B");
    }

    #[test]
    fn cli_line_matches_docker_json_shape() {
        let doc: RawStats = serde_json::from_str(
            r#"{
                "name": "/test-app",
                "id": "9c40c7061e00a1b2c3d4e5f60718293a4b5c6d7e",
                "pids_stats": { "current": 7 },
                "cpu_stats": { "cpu_usage": { "total_usage": 200 }, "system_cpu_usage": 1000, "online_cpus": 2 },
                "precpu_stats": { "cpu_usage": { "total_usage": 100 }, "system_cpu_usage": 500, "online_cpus": 2 },
                "memory_stats": { "usage": 1048576, "limit": 4194304, "stats": {} },
                "networks": { "eth0": { "rx_bytes": 2000, "tx_bytes": 1000 } },
                "blkio_stats": { "io_service_bytes_recursive": [ { "op": "Read", "value": 3000 }, { "op": "Write", "value": 4000 } ] }
            }"#,
        )
        .expect("valid stats json");

        let line = to_cli_line(&doc, 40_960);
        let value: serde_json::Value = serde_json::from_str(&line).expect("valid JSON line");
        assert_eq!(value["Name"], "test-app");
        assert_eq!(value["Container"], "9c40c7061e00");
        assert_eq!(value["PIDs"], "7");
        assert_eq!(value["CPUPerc"], "40.00%");
        assert_eq!(value["MemUsage"], "1.00MiB / 4.00MiB");
        assert_eq!(value["MemPerc"], "25.00%");
        assert_eq!(value["NetIO"], "2.00kB / 1.00kB");
        assert_eq!(value["BlockIO"], "3.00kB / 4.00kB");
        assert_eq!(value["SizeRw"], "40.00KiB");
    }

    #[test]
    fn missing_pids_and_name_still_render() {
        let doc: RawStats = serde_json::from_str(
            r#"{ "cpu_stats": {}, "precpu_stats": {}, "memory_stats": {}, "networks": {}, "blkio_stats": {} }"#,
        )
        .expect("valid stats json");
        let value: serde_json::Value =
            serde_json::from_str(&to_cli_line(&doc, 0)).expect("valid JSON line");
        assert_eq!(value["PIDs"], "0");
        assert_eq!(value["Name"], "");
        assert_eq!(value["CPUPerc"], "0.00%");
    }
}
