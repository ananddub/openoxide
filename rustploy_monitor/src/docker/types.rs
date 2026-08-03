use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

/// Type-safe Container Identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContainerId(String);

impl ContainerId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn short(&self) -> &str {
        self.0.get(..12).unwrap_or(&self.0)
    }
}

impl fmt::Display for ContainerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ContainerId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for ContainerId {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

/// Type-safe Container Name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContainerName(String);

impl ContainerName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ContainerName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ContainerName {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for ContainerName {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

/// Container entry from `GET /containers/json`.
#[derive(Debug, Deserialize)]
pub struct ContainerSummary {
    #[serde(rename = "Id")]
    pub id: ContainerId,
    #[serde(rename = "Names", default)]
    pub names: Vec<String>,
}

impl ContainerSummary {
    pub fn display_name(&self) -> ContainerName {
        let name = self
            .names
            .first()
            .map(|n| n.trim_start_matches('/').to_string())
            .unwrap_or_else(|| self.id.short().to_string());
        ContainerName::new(name)
    }

    pub fn short_id(&self) -> &str {
        self.id.short()
    }
}

/// Raw metrics payload from `GET /containers/{id}/stats`.
#[derive(Debug, Deserialize)]
pub struct ContainerStats {
    #[serde(rename = "cpu_stats", default)]
    pub cpu: CpuStats,
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
    #[serde(rename = "system_cpu_usage", default)]
    pub system_usage: u64,
    #[serde(rename = "online_cpus", default)]
    pub online_cpus: u64,
}

#[derive(Debug, Default, Deserialize)]
pub struct CpuUsage {
    #[serde(rename = "total_usage", default)]
    pub total: u64,
    #[serde(rename = "percpu_usage", default)]
    pub percpu: Vec<u64>,
}

#[derive(Debug, Default, Deserialize)]
pub struct MemoryStats {
    #[serde(default)]
    pub usage: u64,
    #[serde(default)]
    pub limit: u64,
    #[serde(default)]
    pub stats: HashMap<String, u64>,
}

impl MemoryStats {
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
    pub fn cpu_percent(&self) -> f64 {
        let cpu_delta = self.cpu.usage.total.saturating_sub(self.precpu.usage.total) as f64;
        let system_delta = self.cpu.system_usage.saturating_sub(self.precpu.system_usage) as f64;

        if system_delta <= 0.0 || cpu_delta <= 0.0 {
            return 0.0;
        }

        (cpu_delta / system_delta) * self.core_count() * 100.0
    }

    fn core_count(&self) -> f64 {
        if self.cpu.online_cpus > 0 {
            return self.cpu.online_cpus as f64;
        }
        if !self.cpu.usage.percpu.is_empty() {
            return self.cpu.usage.percpu.len() as f64;
        }
        1.0
    }

    pub fn network_bytes(&self) -> (u64, u64) {
        self.networks
            .values()
            .fold((0, 0), |(rx, tx), n| (rx + n.rx_bytes, tx + n.tx_bytes))
    }

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

/// Direct cgroup v2 metrics for one container.
#[derive(Debug, Default, Clone)]
pub struct CgroupSample {
    pub cpu_usage_usec: u64,
    pub memory_usage: u64,
    pub memory_cache: u64,
    pub memory_limit: u64,
    pub io_read_bytes: u64,
    pub io_write_bytes: u64,
}

/// Container stream sample event with type-safe IDs.
#[derive(Clone, Debug)]
pub struct ContainerSample {
    pub container_id: ContainerId,
    pub name: ContainerName,
    pub stats: Arc<ContainerStats>,
}
