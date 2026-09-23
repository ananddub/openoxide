use std::pin::Pin;

use serde::Deserialize;
use serde_json::json;
use tokio::sync::mpsc;
use tokio_stream::{Stream, StreamExt};
use tonic::{Response, Status};

use crate::api::proto::{StatsFrame, StreamStatsRequest};
use crate::docker::api::DockerApi;
use crate::docker::json_lines::JsonAccumulator;
use crate::docker::types::{ContainerStats, ContainerSummary};

pub type StatsStream =
    Pin<Box<dyn Stream<Item = Result<StatsFrame, Status>> + Send + 'static>>;

/// Streams live container stats over the agent's own Docker API connection.
/// The panel used to spawn `docker stats --format {{json .}}` CLI processes
/// (~11 MB RSS each, one per browser consumer); this service emits the exact
/// same JSON document shape from the in-process API stream instead.
pub struct StatsService;

/// Raw stats document: CLI-relevant extras (`name`, `id`, pids) layered over
/// the metric fields `ContainerStats` already models. Shared with the
/// one-shot container metrics sampling in `metrics.rs`.
#[derive(Debug, Deserialize)]
pub(crate) struct RawStats {
    #[serde(default)]
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) id: String,
    #[serde(rename = "pids_stats", default)]
    pids: PidsStats,
    #[serde(flatten)]
    pub(crate) stats: ContainerStats,
}

#[derive(Debug, Default, Deserialize)]
struct PidsStats {
    #[serde(default)]
    current: u64,
}

impl StatsService {
    pub async fn stream(
        docker: DockerApi,
        request: StreamStatsRequest,
    ) -> Result<Response<StatsStream>, Status> {
        let ids = if request.container_ids.is_empty() {
            docker
                .get_json::<Vec<ContainerSummary>>("/containers/json")
                .await
                .map_err(|error| Status::internal(format!("docker container list: {error}")))?
                .into_iter()
                .map(|container| container.id.to_string())
                .collect()
        } else {
            request.container_ids
        };

        // Docker's stats endpoint accepts multiple containers in one request
        // (`/containers/{id1}/{id2}/.../stats`), so the whole fan-out is one
        // connection, one stream and one pump task. Each document carries its
        // own `id`/`name`, so consumers can still tell containers apart.
        let (tx, rx) = mpsc::channel::<String>(64);
        if ids.is_empty() {
            drop(tx); // no containers: the stream closes immediately
        } else {
            tokio::spawn(async move {
                pump(docker, ids, request.stream, tx).await;
            });
        }

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx)
                .map(|json_line| Ok(StatsFrame { json_line })),
        )))
    }
}

async fn pump(docker: DockerApi, ids: Vec<String>, stream: bool, tx: mpsc::Sender<String>) {
    let suffix = if stream { "true" } else { "false" };
    // Docker's stats API never carries the writable-layer size (`size_rw` is
    // absent on this daemon, and `?size=1` is ignored on the stats endpoint —
    // the CLI gets it from inspect). One inspect round at stream open is
    // cheap and the values drift slowly; failed lookups just report 0.
    let sizes: Vec<(String, u64)> = futures::future::join_all(ids.iter().map(|id| {
        let docker = docker.clone();
        async move {
            let size_rw = docker
                .get_json::<serde_json::Value>(&format!("/containers/{id}/json?size=1"))
                .await
                .ok()
                .and_then(|doc| doc.get("SizeRw").and_then(|v| v.as_u64()))
                .unwrap_or(0);
            (id.clone(), size_rw)
        }
    }))
    .await
    .into_iter()
    .collect();
    let mut chunks = match docker
        .get_stream(&format!(
            "/containers/{}/stats?stream={suffix}",
            ids.join("/")
        ))
        .await
    {
        Ok(chunks) => chunks,
        Err(error) => {
            tracing::warn!(count = ids.len(), %error, "could not open container stats stream");
            return;
        }
    };

    // With `stream=false` Docker emits one sample per container and closes;
    // we still stop ourselves once every requested id has been seen so the
    // `--no-stream` contract holds even if the daemon keeps the connection.
    let mut pending: Vec<&str> = sizes.iter().map(|(id, _)| id.as_str()).collect();
    let mut accumulator = JsonAccumulator::new();
    while let Some(Ok(chunk)) = chunks.next().await {
        for doc in accumulator.push::<RawStats>(&chunk) {
            let size_rw = find_size(&sizes, &doc.id);
            if tx.send(to_cli_line(&doc, size_rw)).await.is_err() {
                return; // panel consumer went away
            }
            if !stream {
                let short = doc.id.get(..12).unwrap_or(&doc.id);
                pending.retain(|id| {
                    *id != doc.id
                        && *id != short
                        && !doc.id.starts_with(id)
                });
                if pending.is_empty() {
                    return;
                }
            }
        }
    }
}

/// Stats documents carry the full 64-char id while the request may name
/// containers by their 12-char short id, so match by prefix either way.
fn find_size(sizes: &[(String, u64)], doc_id: &str) -> u64 {
    sizes
        .iter()
        .find(|(id, _)| {
            doc_id == id
                || doc_id.starts_with(id.as_str())
                || id.starts_with(doc_id.get(..12).unwrap_or(doc_id))
        })
        .map(|(_, size)| *size)
        .unwrap_or(0)
}

/// Renders one API stats document as the JSON object produced by
/// `docker stats --format {{json .}}`.
fn to_cli_line(doc: &RawStats, size_rw: u64) -> String {
    let name = doc.name.trim_start_matches('/');
    let short_id = doc.id.get(..12).unwrap_or(&doc.id);
    let (rx, tx_bytes) = doc.stats.network_bytes();
    let (read, write) = doc.stats.block_io_bytes();
    json!({
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
        let value: serde_json::Value = serde_json::from_str(&to_cli_line(&doc, 0))
            .expect("valid JSON line");
        assert_eq!(value["PIDs"], "0");
        assert_eq!(value["Name"], "");
        assert_eq!(value["CPUPerc"], "0.00%");
    }
}
