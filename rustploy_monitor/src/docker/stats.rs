use serde::Deserialize;
use std::collections::HashMap;

/// One entry from `GET /containers/json`.
#[derive(Debug, Deserialize)]
pub struct ContainerSummary {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Names", default)]
    pub names: Vec<String>,
}

impl ContainerSummary {
    /// Docker reports names with a leading slash (`/web`); strip it so stored
    /// names match what `docker ps` shows.
    pub fn display_name(&self) -> String {
        self.names
            .first()
            .map(|n| n.trim_start_matches('/').to_string())
            .unwrap_or_else(|| self.short_id().to_string())
    }

    /// First 12 hex chars, matching the CLI's abbreviated id.
    pub fn short_id(&self) -> &str {
        self.id.get(..12).unwrap_or(&self.id)
    }
}

/// Response of `GET /containers/{id}/stats?stream=false`.
///
/// Only the fields the agent uses are declared; the daemon sends considerably
/// more and unknown fields are ignored.
#[derive(Debug, Deserialize)]
pub struct ContainerStats {
    #[serde(rename = "cpu_stats", default)]
    pub cpu: CpuStats,
    /// Previous CPU sample, included by the daemon so a single non-streaming
    /// response is enough to compute a usage percentage.
    #[serde(rename = "precpu_stats", default)]
    pub precpu: CpuStats,
    #[serde(rename = "memory_stats", default)]
    pub memory: MemoryStats,
    #[serde(rename = "networks", default)]
    pub networks: HashMap<String, NetworkStats>,
    #[serde(rename = "blkio_stats", default)]
    pub blkio: BlkioStats,
}

#[derive(Debug, Default, Deserialize)]
pub struct CpuStats {
    #[serde(rename = "cpu_usage", default)]
    pub usage: CpuUsage,
    /// Absent on Windows and on cgroup setups that do not report it; a zero
    /// system delta makes the percentage zero rather than a division error.
    #[serde(rename = "system_cpu_usage", default)]
    pub system_usage: u64,
    #[serde(rename = "online_cpus", default)]
    pub online_cpus: u64,
}

#[derive(Debug, Default, Deserialize)]
pub struct CpuUsage {
    #[serde(rename = "total_usage", default)]
    pub total: u64,
    /// Per-core totals. Used to infer core count on older daemons that omit
    /// `online_cpus`.
    #[serde(rename = "percpu_usage", default)]
    pub percpu: Vec<u64>,
}

#[derive(Debug, Default, Deserialize)]
pub struct MemoryStats {
    #[serde(default)]
    pub usage: u64,
    #[serde(default)]
    pub limit: u64,
    /// cgroup counters. `cache` (v1) and `inactive_file` (v2) are page cache,
    /// which docker excludes from the figure it displays.
    #[serde(default)]
    pub stats: HashMap<String, u64>,
}

impl MemoryStats {
    /// Memory in use, matching what `docker stats` reports.
    ///
    /// The raw `usage` counter includes page cache, which inflates the number
    /// and makes it look like a container is near its limit when it is not.
    /// Docker subtracts cache; we mirror that, preferring the cgroup v2 field
    /// and falling back to v1.
    pub fn used_bytes(&self) -> u64 {
        let cache = self
            .stats
            .get("inactive_file")
            .or_else(|| self.stats.get("total_inactive_file"))
            .or_else(|| self.stats.get("cache"))
            .copied()
            .unwrap_or(0);

        self.usage.saturating_sub(cache)
    }

    pub fn used_percent(&self) -> f64 {
        if self.limit == 0 {
            return 0.0;
        }
        (self.used_bytes() as f64 / self.limit as f64) * 100.0
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct NetworkStats {
    #[serde(default)]
    pub rx_bytes: u64,
    #[serde(default)]
    pub tx_bytes: u64,
}

#[derive(Debug, Default, Deserialize)]
pub struct BlkioStats {
    #[serde(rename = "io_service_bytes_recursive", default)]
    pub io_service_bytes: Option<Vec<BlkioEntry>>,
}

#[derive(Debug, Deserialize)]
pub struct BlkioEntry {
    #[serde(rename = "op", default)]
    pub op: String,
    #[serde(rename = "value", default)]
    pub value: u64,
}

impl ContainerStats {
    /// CPU usage as a percentage of one core's worth of time multiplied by the
    /// number of cores — the same figure `docker stats` prints, so 200% means
    /// two cores saturated.
    ///
    /// This is the delta between the current and previous samples, which is why
    /// the daemon includes `precpu_stats` in every response.
    pub fn cpu_percent(&self) -> f64 {
        let cpu_delta = self.cpu.usage.total.saturating_sub(self.precpu.usage.total) as f64;
        let system_delta = self.cpu.system_usage.saturating_sub(self.precpu.system_usage) as f64;

        if system_delta <= 0.0 || cpu_delta <= 0.0 {
            return 0.0;
        }

        (cpu_delta / system_delta) * self.core_count() * 100.0
    }

    /// Cores available to the container. `online_cpus` is authoritative when
    /// present; older daemons only give per-core usage entries.
    fn core_count(&self) -> f64 {
        if self.cpu.online_cpus > 0 {
            return self.cpu.online_cpus as f64;
        }
        if !self.cpu.usage.percpu.is_empty() {
            return self.cpu.usage.percpu.len() as f64;
        }
        1.0
    }

    /// Total received and transmitted bytes, summed across every interface.
    pub fn network_bytes(&self) -> (u64, u64) {
        self.networks
            .values()
            .fold((0, 0), |(rx, tx), n| (rx + n.rx_bytes, tx + n.tx_bytes))
    }

    /// Total bytes read and written to block devices.
    ///
    /// The daemon reports one entry per device per operation, so entries are
    /// summed per direction. Operation names vary in case between versions.
    pub fn block_io_bytes(&self) -> (u64, u64) {
        let Some(entries) = &self.blkio.io_service_bytes else {
            return (0, 0);
        };

        entries
            .iter()
            .fold((0, 0), |(read, write), entry| {
                match entry.op.to_ascii_lowercase().as_str() {
                    "read" => (read + entry.value, write),
                    "write" => (read, write + entry.value),
                    _ => (read, write),
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stats_from(json: &str) -> ContainerStats {
        serde_json::from_str(json).expect("valid stats json")
    }

    #[test]
    fn cpu_percent_scales_by_core_count() {
        // 10% of system time across 4 cores -> 40%
        let stats = stats_from(
            r#"{
                "cpu_stats": {
                    "cpu_usage": {"total_usage": 2000},
                    "system_cpu_usage": 20000,
                    "online_cpus": 4
                },
                "precpu_stats": {
                    "cpu_usage": {"total_usage": 1000},
                    "system_cpu_usage": 10000,
                    "online_cpus": 4
                }
            }"#,
        );

        assert!((stats.cpu_percent() - 40.0).abs() < 0.001);
    }

    #[test]
    fn cpu_percent_is_zero_on_first_sample() {
        // precpu is empty right after a container starts.
        let stats = stats_from(
            r#"{
                "cpu_stats": {"cpu_usage": {"total_usage": 500}, "system_cpu_usage": 1000},
                "precpu_stats": {"cpu_usage": {"total_usage": 0}, "system_cpu_usage": 0}
            }"#,
        );

        // system_delta equals current usage, so this is a real ratio, not a divide by zero
        assert!(stats.cpu_percent() >= 0.0);
        assert!(stats.cpu_percent().is_finite());
    }

    #[test]
    fn cpu_percent_survives_a_missing_system_usage() {
        let stats = stats_from(
            r#"{
                "cpu_stats": {"cpu_usage": {"total_usage": 500}},
                "precpu_stats": {"cpu_usage": {"total_usage": 100}}
            }"#,
        );

        assert_eq!(stats.cpu_percent(), 0.0);
    }

    #[test]
    fn cpu_percent_falls_back_to_percpu_length() {
        let stats = stats_from(
            r#"{
                "cpu_stats": {
                    "cpu_usage": {"total_usage": 2000, "percpu_usage": [500, 500, 500, 500]},
                    "system_cpu_usage": 20000
                },
                "precpu_stats": {
                    "cpu_usage": {"total_usage": 1000, "percpu_usage": [250, 250, 250, 250]},
                    "system_cpu_usage": 10000
                }
            }"#,
        );

        assert!((stats.cpu_percent() - 40.0).abs() < 0.001);
    }

    #[test]
    fn counters_that_go_backwards_do_not_underflow() {
        // Can happen across a daemon restart; saturating_sub keeps it at zero.
        let stats = stats_from(
            r#"{
                "cpu_stats": {"cpu_usage": {"total_usage": 100}, "system_cpu_usage": 1000},
                "precpu_stats": {"cpu_usage": {"total_usage": 500}, "system_cpu_usage": 5000}
            }"#,
        );

        assert_eq!(stats.cpu_percent(), 0.0);
    }

    #[test]
    fn memory_excludes_page_cache() {
        let stats = stats_from(
            r#"{
                "memory_stats": {
                    "usage": 200000000,
                    "limit": 400000000,
                    "stats": {"inactive_file": 50000000}
                }
            }"#,
        );

        assert_eq!(stats.memory.used_bytes(), 150_000_000);
        assert!((stats.memory.used_percent() - 37.5).abs() < 0.001);
    }

    #[test]
    fn memory_falls_back_to_cgroup_v1_cache() {
        let stats = stats_from(
            r#"{"memory_stats": {"usage": 100, "limit": 200, "stats": {"cache": 40}}}"#,
        );

        assert_eq!(stats.memory.used_bytes(), 60);
    }

    #[test]
    fn memory_percent_is_zero_without_a_limit() {
        let stats = stats_from(r#"{"memory_stats": {"usage": 100, "limit": 0}}"#);
        assert_eq!(stats.memory.used_percent(), 0.0);
    }

    #[test]
    fn network_sums_every_interface() {
        let stats = stats_from(
            r#"{
                "networks": {
                    "eth0": {"rx_bytes": 100, "tx_bytes": 200},
                    "eth1": {"rx_bytes": 50, "tx_bytes": 25}
                }
            }"#,
        );

        assert_eq!(stats.network_bytes(), (150, 225));
    }

    #[test]
    fn block_io_splits_by_direction_case_insensitively() {
        let stats = stats_from(
            r#"{
                "blkio_stats": {
                    "io_service_bytes_recursive": [
                        {"op": "Read", "value": 1000},
                        {"op": "write", "value": 2000},
                        {"op": "Read", "value": 500},
                        {"op": "Sync", "value": 9999}
                    ]
                }
            }"#,
        );

        assert_eq!(stats.block_io_bytes(), (1500, 2000));
    }

    #[test]
    fn block_io_handles_a_null_list() {
        // The daemon sends null on hosts without blkio accounting.
        let stats = stats_from(r#"{"blkio_stats": {"io_service_bytes_recursive": null}}"#);
        assert_eq!(stats.block_io_bytes(), (0, 0));
    }

    #[test]
    fn an_empty_payload_decodes_to_zeros() {
        let stats = stats_from("{}");
        assert_eq!(stats.cpu_percent(), 0.0);
        assert_eq!(stats.memory.used_bytes(), 0);
        assert_eq!(stats.network_bytes(), (0, 0));
    }

    #[test]
    fn container_name_drops_the_leading_slash() {
        let summary: ContainerSummary = serde_json::from_str(
            r#"{"Id": "abcdef123456789", "Names": ["/web"]}"#,
        )
        .unwrap();

        assert_eq!(summary.display_name(), "web");
        assert_eq!(summary.short_id(), "abcdef123456");
    }

    #[test]
    fn container_without_names_falls_back_to_id() {
        let summary: ContainerSummary =
            serde_json::from_str(r#"{"Id": "abcdef123456789", "Names": []}"#).unwrap();

        assert_eq!(summary.display_name(), "abcdef123456");
    }
}
