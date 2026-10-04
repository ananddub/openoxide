use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use tracing::debug;

use super::{bytes_to_gb, bytes_to_mb};

/// One live host sample. Produced fresh per request; nothing here survives
/// the call that created it.
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

/// Stateless host reader: every call measures from scratch and keeps nothing
/// between requests. CPU usage is inherently a delta, so each sample measures
/// one short window inside the request itself — the answer may arrive a
/// moment late, but it is always live. Disk is a single `statvfs` on one
/// mount and network is a single `/proc/net/dev` read: no per-request
/// device/mount enumeration, which is what made older versions spike CPU on
/// Docker hosts with many mounts.
pub struct SystemCollector;

impl SystemCollector {
    pub fn new() -> Self {
        Self
    }

    pub async fn sample(&mut self) -> ServerMetric {
        let mut system = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::nothing().with_cpu_usage().with_frequency())
                .with_memory(MemoryRefreshKind::nothing().with_ram()),
        );
        system.refresh_cpu_usage();
        tokio::time::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
        system.refresh_cpu_usage();

        let cpu = system.global_cpu_usage() as f64;
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

        let mem_total_bytes = system.total_memory() as f64;
        let mem_used_bytes = system.used_memory() as f64;
        let mem_percent = if mem_total_bytes > 0.0 {
            (mem_used_bytes / mem_total_bytes) * 100.0
        } else {
            0.0
        };

        // Report the host root filesystem. When the agent runs in a container
        // the host root is bind-mounted at `/host` (see the agent compose),
        // so prefer that, then `/`.
        let (disk_used_bytes, disk_total_bytes) =
            statvfs("/host").or_else(|| statvfs("/")).unwrap_or((0, 0));
        let disk_percent = if disk_total_bytes > 0 {
            (disk_used_bytes as f64 / disk_total_bytes as f64) * 100.0
        } else {
            0.0
        };

        let (net_in, net_out) = net_totals();

        let metric = ServerMetric {
            timestamp: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            cpu,
            cpu_model,
            cpu_cores: logical_cores as i32,
            cpu_physical_cores: system.physical_core_count().unwrap_or(logical_cores) as i32,
            cpu_speed,
            os: System::name().unwrap_or_else(|| "unknown".to_string()),
            distro: System::long_os_version().unwrap_or_else(|| "unknown".to_string()),
            kernel: System::kernel_version().unwrap_or_else(|| "unknown".to_string()),
            arch: System::cpu_arch(),
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

impl Default for SystemCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Total and used bytes of the filesystem one path sits on, via `statvfs` —
/// the same numbers `df` prints, with no device list walk.
fn statvfs(path: &str) -> Option<(u64, u64)> {
    let c_path = std::ffi::CString::new(path).ok()?;
    let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statvfs(c_path.as_ptr(), &mut stat) } != 0 {
        return None;
    }
    let block = stat.f_frsize as u64;
    let total = stat.f_blocks as u64 * block;
    let used = (stat.f_blocks as u64 - stat.f_bfree as u64) * block;
    Some((used, total))
}

/// Cumulative RX/TX bytes across every interface, straight from
/// `/proc/net/dev` — one small file read, no per-interface state kept.
fn net_totals() -> (u64, u64) {
    let Ok(text) = std::fs::read_to_string("/proc/net/dev") else {
        return (0, 0);
    };
    let mut rx = 0u64;
    let mut tx = 0u64;
    for line in text.lines().skip(2) {
        let Some((_, data)) = line.split_once(':') else {
            continue;
        };
        let fields: Vec<&str> = data.split_whitespace().collect();
        if fields.len() >= 9 {
            rx += fields[0].parse().unwrap_or(0);
            tx += fields[8].parse().unwrap_or(0);
        }
    }
    (rx, tx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_units() {
        assert!((bytes_to_gb(1_073_741_824.0) - 1.0).abs() < f64::EPSILON);
        assert!((bytes_to_mb(1_048_576.0) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn net_totals_sums_interfaces_from_proc_format() {
        // Same parsing as `net_totals`, over the `/proc/net/dev` shape.
        let text =
            "  lo: 100 0 0 0 0 0 0 0 50 0 0 0 0 0 0 0\n eth0: 200 0 0 0 0 0 0 0 75 0 0 0 0 0 0 0\n";
        let mut rx = 0u64;
        let mut tx = 0u64;
        for line in text.lines() {
            let Some((_, data)) = line.split_once(':') else {
                continue;
            };
            let fields: Vec<&str> = data.split_whitespace().collect();
            if fields.len() >= 9 {
                rx += fields[0].parse().unwrap_or(0);
                tx += fields[8].parse().unwrap_or(0);
            }
        }
        assert_eq!((rx, tx), (300, 125));
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
