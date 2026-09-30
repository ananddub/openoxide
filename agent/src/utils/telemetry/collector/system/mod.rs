use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, Networks, RefreshKind, System};
use tracing::debug;

use super::{bytes_to_gb, bytes_to_mb};

/// One live host sample: vitals plus the static host identity fields the
/// panel's `HostInfo` wire message needs.
#[derive(Debug, Clone)]
pub struct ServerMetric {
    pub timestamp: String,
    pub cpu: f64,
    pub cpu_model: String,
    pub cpu_cores: i32,
    pub cpu_physical_cores: i32,
    pub cpu_speed: f64,
    pub os: String,
    pub distro: String,
    pub kernel: String,
    pub arch: String,
    pub mem_used: f64,
    pub mem_used_gb: f64,
    pub mem_total: f64,
    pub uptime: u64,
    pub disk_used: f64,
    pub total_disk: f64,
    pub network_in: f64,
    pub network_out: f64,
}

pub struct SystemCollector {
    system: System,
    disks: Disks,
    networks: Networks,
    host: HostInfo,
}

struct HostInfo {
    cpu_model: String,
    cpu_cores: i32,
    cpu_physical_cores: i32,
    cpu_speed: f64,
    os: String,
    distro: String,
    kernel: String,
    arch: String,
}

impl SystemCollector {
    pub fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::nothing().with_cpu_usage().with_frequency())
                .with_memory(MemoryRefreshKind::nothing().with_ram()),
        );

        let cpus = system.cpus();
        let logical_cores = cpus.len();
        let cpu_model = cpus
            .first()
            .map(|c| c.brand().trim().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let cpu_speed = cpus
            .first()
            .map(|c| c.frequency() as f64 / 1000.0)
            .unwrap_or(0.0);

        let host = HostInfo {
            cpu_model,
            cpu_cores: logical_cores as i32,
            cpu_physical_cores: system.physical_core_count().unwrap_or(logical_cores) as i32,
            cpu_speed,
            os: System::name().unwrap_or_else(|| "unknown".to_string()),
            distro: System::long_os_version().unwrap_or_else(|| "unknown".to_string()),
            kernel: System::kernel_version().unwrap_or_else(|| "unknown".to_string()),
            arch: System::cpu_arch(),
        };

        Self {
            system,
            disks: Disks::new_with_refreshed_list(),
            networks: Networks::new_with_refreshed_list(),
            host,
        }
    }

    pub async fn sample(&mut self) -> ServerMetric {
        self.system.refresh_cpu_usage();
        tokio::time::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
        self.system.refresh_cpu_usage();
        self.system.refresh_memory();
        self.disks.refresh(true);
        self.networks.refresh(true);

        let cpu = self.system.global_cpu_usage() as f64;

        let mem_total_bytes = self.system.total_memory() as f64;
        let mem_used_bytes = self.system.used_memory() as f64;
        let mem_percent = if mem_total_bytes > 0.0 {
            (mem_used_bytes / mem_total_bytes) * 100.0
        } else {
            0.0
        };

        // Report the host root filesystem. Summing every mount double-counts
 // one device; match the standard `df /`. When the agent runs in a
        // container the host root is bind-mounted at `/host` (see the agent
        // compose), so prefer that, then `/`, then the largest device.
        let disk_for = |mount: &str| {
            self.disks
                .list()
                .iter()
                .find(|d| d.mount_point() == std::path::Path::new(mount))
                .map(|d| (d.total_space() - d.available_space(), d.total_space()))
        };
        let (disk_used_bytes, disk_total_bytes) = disk_for("/host")
            .or_else(|| disk_for("/"))
            .or_else(|| {
                self.disks
                    .list()
                    .iter()
                    .filter(|d| !d.is_removable())
                    .max_by_key(|d| d.total_space())
                    .map(|d| (d.total_space() - d.available_space(), d.total_space()))
            })
            .unwrap_or((0, 0));

        let disk_percent = if disk_total_bytes > 0 {
            (disk_used_bytes as f64 / disk_total_bytes as f64) * 100.0
        } else {
            0.0
        };

        let (net_in, net_out) = self
            .networks
            .list()
            .values()
            .fold((0u64, 0u64), |(rx, tx), d| {
                (rx + d.total_received(), tx + d.total_transmitted())
            });

        let metric = ServerMetric {
            timestamp: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            cpu,
            cpu_model: self.host.cpu_model.clone(),
            cpu_cores: self.host.cpu_cores,
            cpu_physical_cores: self.host.cpu_physical_cores,
            cpu_speed: self.host.cpu_speed,
            os: self.host.os.clone(),
            distro: self.host.distro.clone(),
            kernel: self.host.kernel.clone(),
            arch: self.host.arch.clone(),
            mem_used: mem_percent,
            mem_used_gb: bytes_to_gb(mem_used_bytes),
            mem_total: bytes_to_gb(mem_total_bytes),
            uptime: System::uptime(),
            disk_used: disk_percent,
            total_disk: bytes_to_gb(disk_total_bytes as f64),
            network_in: bytes_to_mb(net_in as f64),
            network_out: bytes_to_mb(net_out as f64),
        };

        debug!(
            cpu = format!("{:.1}%", metric.cpu),
            mem = format!("{:.1}%", metric.mem_used),
            disk = format!("{:.1}%", metric.disk_used),
            "sampled host metrics"
        );

        metric
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_units() {
        assert!((bytes_to_gb(1_073_741_824.0) - 1.0).abs() < f64::EPSILON);
        assert!((bytes_to_mb(1_048_576.0) - 1.0).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn sample_reports_plausible_values() {
        let mut collector = SystemCollector::new();
        let metric = collector.sample().await;

        assert!(metric.cpu >= 0.0 && metric.cpu <= 100.0);
        assert!(metric.mem_used >= 0.0 && metric.mem_used <= 100.0);
        assert!(metric.disk_used >= 0.0 && metric.disk_used <= 100.0);
        assert!(metric.cpu_cores > 0);
        assert!(!metric.timestamp.is_empty());
    }
}
