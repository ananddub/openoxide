use std::pin::Pin;

use serde_json::json;
use tokio_stream::Stream;
use tonic::{Response, Status};

use super::hub::{RawStats, StatsHub};
use crate::api::proto::{StatsFrame, StreamStatsRequest};

pub type StatsStream = Pin<Box<dyn Stream<Item = Result<StatsFrame, Status>> + Send + 'static>>;

/// Streams live container stats from the agent's single shared stats feed:
/// every listener receives the same multiplexed documents, whatever the
/// number of panel consumers. The emitted JSON shape matches
/// `docker stats --format {{json .}}` exactly.
pub struct StatsService;

impl StatsService {
    pub async fn stream(
        hub: StatsHub,
        request: StreamStatsRequest,
    ) -> Result<Response<StatsStream>, Status> {
        let mut rx = hub.subscribe().await;
        let ids = request.container_ids;
        let stream = async_stream::stream! {
            if request.stream {
                while let Ok(doc) = rx.recv().await {
                    if !ids.is_empty() && !id_matches(&ids, &doc.id) {
                        continue;
                    }
                    let size_rw = hub.size_rw(&doc.id).await;
                    yield Ok(StatsFrame {
                        json_line: to_cli_line(&doc, size_rw),
                    });
                }
            } else if ids.is_empty() {
                // One-shot, all containers: the hub cycles every running
                // container once per second, so stop when a repeat arrives.
                let mut seen: Vec<String> = Vec::new();
                while let Ok(doc) = rx.recv().await {
                    if seen.iter().any(|id| doc_id_matches(id, &doc.id)) {
                        break;
                    }
                    seen.push(doc.id.clone());
                    let size_rw = hub.size_rw(&doc.id).await;
                    yield Ok(StatsFrame {
                        json_line: to_cli_line(&doc, size_rw),
                    });
                }
            } else {
                // One-shot, named containers: one sample each.
                let mut wanted = ids;
                while !wanted.is_empty() {
                    match rx.recv().await {
                        Ok(doc) => {
                            let matched = doc_id_matches_at(&wanted, &doc.id);
                            wanted.retain(|id| !doc_id_matches(id, &doc.id));
                            if matched {
                                let size_rw = hub.size_rw(&doc.id).await;
                                yield Ok(StatsFrame {
                                    json_line: to_cli_line(&doc, size_rw),
                                });
                            }
                        }
                        Err(_) => break, // feed ended
                    }
                }
            }
        };
        Ok(Response::new(Box::pin(stream)))
    }
}

/// Stats documents carry the full 64-char id while requests may name
/// containers by their 12-char short id, so match by prefix either way.
fn id_matches(ids: &[String], doc_id: &str) -> bool {
    doc_id_matches_at(ids, doc_id)
}

fn doc_id_matches_at(ids: &[String], doc_id: &str) -> bool {
    ids.iter().any(|id| doc_id_matches(id, doc_id))
}

fn doc_id_matches(id: &str, doc_id: &str) -> bool {
    doc_id == id || doc_id.starts_with(id) || id.starts_with(doc_id.get(..12).unwrap_or(doc_id))
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
        let value: serde_json::Value =
            serde_json::from_str(&to_cli_line(&doc, 0)).expect("valid JSON line");
        assert_eq!(value["PIDs"], "0");
        assert_eq!(value["Name"], "");
        assert_eq!(value["CPUPerc"], "0.00%");
    }
}
