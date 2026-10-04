use std::{pin::Pin, sync::Arc, time::Duration};

use tokio_stream::Stream;
use tonic::{Response, Status};

use super::cgroup::CgroupCollector;
use crate::api::proto::StatsFrame;
use crate::docker::{
    api::DockerApi,
    types::{ContainerStats, ContainerSummary},
};
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

/// Streams container stats read directly from cgroup v2 and procfs — dockerd's
/// `/stats` sampler is never touched, so the daemon does no per-container stats
/// work no matter how many views are open. The container listing is re-read
/// every round, so created/deleted containers join/leave on their own; a
/// recreated container is re-resolved to its new pid/cgroup on its first round.
/// CPU% needs two cumulative samples, so the first frame per container reads
/// 0 and every later frame is the live rate. The panel fans the single stream
/// out to browser views, so a host runs exactly one sampler set for all tabs.
pub struct StatsService;

/// Cadence of the relay. Each round does a handful of tiny cgroup/procfs file
/// reads per watched container — cheap enough to stay live at CLI-refresh speed
/// without the daemon doing any stats work in between.
const RELAY_INTERVAL: Duration = Duration::from_secs(3);

/// How a watched target selects containers. Exact id/name forms are
/// accepted from the panel; `prefix*` selectors keep matching containers
/// that were recreated after the stream opened.
enum Selector {
    Exact(String),
    NamePrefix(String),
    IdPrefix(String),
}

impl Selector {
    fn parse(raw: &str) -> Self {
        match raw.strip_suffix('*') {
            Some(prefix) if !prefix.is_empty() => {
                if prefix.bytes().all(|byte| byte.is_ascii_hexdigit()) {
                    Self::IdPrefix(prefix.to_lowercase())
                } else {
                    Self::NamePrefix(prefix.to_lowercase())
                }
            }
            _ => Self::Exact(raw.to_owned()),
        }
    }

    fn matches(&self, full_id: &str, short_id: &str, name: &str) -> bool {
        match self {
            Self::Exact(target) => {
                target.as_str() == full_id
                    || target.as_str() == short_id
                    || target.as_str() == name
                    || (target.len() >= 8 && full_id.starts_with(target.as_str()))
            }
            Self::NamePrefix(prefix) => {
                name.to_lowercase().starts_with(prefix.as_str())
                    || full_id.starts_with(prefix.as_str())
            }
            Self::IdPrefix(prefix) => full_id.starts_with(prefix.as_str()),
        }
    }
}

impl StatsService {
    pub async fn stream(
        docker: Arc<DockerApi>,
        request: crate::api::proto::StreamStatsRequest,
    ) -> Result<Response<StatsStream>, Status> {
        if request.stream {
            let selectors: Vec<Selector> = request
                .container_ids
                .iter()
                .map(|target| Selector::parse(target))
                .collect();
            let all = selectors.is_empty();
            let mut collector = CgroupCollector::new(Arc::clone(&docker));
            let stream = async_stream::stream! {
                loop {
                    let summaries: Vec<ContainerSummary> =
                        match docker.get_json("/containers/json").await {
                            Ok(summaries) => summaries,
                            Err(error) => {
                                tracing::debug!(%error, "container listing failed");
                                Vec::new()
                            }
                        };
                    for summary in &summaries {
                        if !all {
                            let full = summary.id.as_str();
                            let short = summary.id.short();
                            let name = summary.display_name();
                            if !selectors
                                .iter()
                                .any(|selector| selector.matches(full, short, name.as_str()))
                            {
                                continue;
                            }
                        }
                        let name = summary.display_name();
                        match collector.sample(summary.id.as_str(), name.as_str()).await {
                            Some(doc) => yield Ok(StatsFrame {
                                json_line: to_cli_line(&doc),
                            }),
                            None => continue,
                        }
                    }
                    tokio::time::sleep(RELAY_INTERVAL).await;
                }
            };
            return Ok(Response::new(Box::pin(stream)));
        }

        // One-shot snapshot: prime each container's CPU window, pause so a real
        // delta accrues, then emit. `container_ids` are exact ids/names (the
        // panel strips `prefix*` selectors before a non-stream call).
        let summaries: Vec<ContainerSummary> = docker
            .get_json("/containers/json")
            .await
            .map_err(|error| Status::unavailable(error.to_string()))?;
        let wanted: Vec<(String, String)> = if request.container_ids.is_empty() {
            summaries
                .iter()
                .map(|s| (s.id.to_string(), s.display_name().to_string()))
                .collect()
        } else {
            summaries
                .iter()
                .filter(|s| {
                    let full = s.id.as_str();
                    let short = s.id.short();
                    let name = s.display_name();
                    request
                        .container_ids
                        .iter()
                        .any(|target| target == full || target == short || target == name.as_str())
                })
                .map(|s| (s.id.to_string(), s.display_name().to_string()))
                .collect()
        };
        let mut collector = CgroupCollector::new(Arc::clone(&docker));
        let stream = async_stream::stream! {
            for (id, name) in &wanted {
                collector.sample(id, name).await;
            }
            tokio::time::sleep(Duration::from_millis(300)).await;
            for (id, name) in &wanted {
                if let Some(doc) = collector.sample(id, name).await {
                    yield Ok(StatsFrame {
                        json_line: to_cli_line(&doc),
                    });
                }
            }
        };
        Ok(Response::new(Box::pin(stream)))
    }
}

/// Renders one API stats document as the JSON object produced by
/// `docker stats --format {{json .}}`. Writable-layer disk size (`SizeRw`) is
/// deliberately absent: computing it makes the daemon stat-walk every
/// container's overlay layer, and consumers only need CPU/RAM/Net.
fn to_cli_line(doc: &RawStats) -> String {
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

        let line = to_cli_line(&doc);
        let value: serde_json::Value = serde_json::from_str(&line).expect("valid JSON line");
        assert_eq!(value["Name"], "test-app");
        assert_eq!(value["Container"], "9c40c7061e00");
        assert_eq!(value["PIDs"], "7");
        assert_eq!(value["CPUPerc"], "40.00%");
        assert_eq!(value["MemUsage"], "1.00MiB / 4.00MiB");
        assert_eq!(value["MemPerc"], "25.00%");
        assert_eq!(value["NetIO"], "2.00kB / 1.00kB");
        assert_eq!(value["BlockIO"], "3.00kB / 4.00kB");
        assert!(value.get("SizeRw").is_none());
    }

    #[test]
    fn missing_pids_and_name_still_render() {
        let doc: RawStats = serde_json::from_str(
            r#"{ "cpu_stats": {}, "precpu_stats": {}, "memory_stats": {}, "networks": {}, "blkio_stats": {} }"#,
        )
        .expect("valid stats json");
        let value: serde_json::Value =
            serde_json::from_str(&to_cli_line(&doc)).expect("valid JSON line");
        assert_eq!(value["PIDs"], "0");
        assert_eq!(value["Name"], "");
        assert_eq!(value["CPUPerc"], "0.00%");
    }
}
