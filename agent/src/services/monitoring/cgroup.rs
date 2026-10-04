//! Container CPU/RAM/network/IO/pids read straight from cgroup v2 and procfs,
//! bypassing Docker's `/stats` endpoint entirely.
//!
//! dockerd's stats collector is what spikes the daemon: every `/stats` read —
//! streaming or one-shot — spins up a per-container sampler that itself walks
//! these same cgroup files. Reading them directly costs the daemon nothing.
//! The only Docker call left is the (cheap, cached) container inventory and the
//! one-time `State.Pid` lookup used to locate a container's cgroup and network
//! namespace.
//!
//! Two host views are required and both are mounted into the agent container:
//!   * `/sys/fs/cgroup`  — the host cgroup root (the `docker-<id>.scope` trees).
//!   * `/host/proc`      — the host procfs (via `/:/host:ro`), for per-container
//!     network counters at `/host/proc/<pid>/net/dev` and the cgroup path at
//!     `/host/proc/<pid>/cgroup`.
//!
//! The synthesized [`RawStats`] is fed to the existing `to_cli_line`, so the
//! gRPC frame the panel and browser parse is unchanged.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};

use serde_json::Value;

use crate::docker::{
    api::DockerApi,
    types::{
        BlkioEntry, BlkioStats, ContainerStats, CpuStats, CpuUsage, MemoryStats, NetworkStats,
    },
};

use super::stats::{PidsStats, RawStats};

/// Carry-over needed to turn the cumulative `cpu.stat` counter into a rate,
/// plus the resolved pid/cgroup path so later rounds skip the Docker inspect.
/// One small entry per watched container; nothing grows without bound.
struct Tracked {
    usage_usec: u64,
    wall: Instant,
    cgroup_rel: String,
    pid: u32,
}

pub struct CgroupCollector {
    docker: Arc<DockerApi>,
    cgroup_root: PathBuf,
    proc_root: PathBuf,
    host_mem_total: Option<u64>,
    tracked: HashMap<String, Tracked>,
}

impl CgroupCollector {
    pub fn new(docker: Arc<DockerApi>) -> Self {
        let cgroup_root = std::env::var("CGROUP_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/sys/fs/cgroup"));
        let proc_root = std::env::var("PROC_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/host/proc"));
        Self {
            docker,
            cgroup_root,
            proc_root,
            host_mem_total: None,
            tracked: HashMap::new(),
        }
    }

    /// One live sample. `None` when the container cannot be resolved to a
    /// running pid/cgroup this round (it is dropped and retried next tick).
    pub async fn sample(&mut self, full_id: &str, name: &str) -> Option<RawStats> {
        let (pid, cgroup_rel) = match self.resolve(full_id).await {
            Some(pair) => pair,
            None => {
                self.tracked.remove(full_id);
                return None;
            }
        };
        let scope = self.cgroup_root.join(cgroup_rel.trim_start_matches('/'));
        let usage_usec = read_cpu_usage(&scope).await?;

        // CPU% over the wall window since the last sample. The API formula is
        // `(cpu_delta / system_delta) * cores * 100`; feeding it a synthetic
        // doc whose `system_delta` is the elapsed wall time (µs) and whose
        // `cores` is 1 reproduces "100% = one core busy", matching `docker
        // stats`. First sample has no prior, so it reports 0.
        let (precpu_usage, system_usec) = match self.tracked.get(full_id) {
            Some(prev) => (
                prev.usage_usec,
                prev.wall
                    .elapsed()
                    .as_micros()
                    .try_into()
                    .unwrap_or(u64::MAX),
            ),
            None => (usage_usec, 0),
        };
        self.tracked.insert(
            full_id.to_string(),
            Tracked {
                usage_usec,
                wall: Instant::now(),
                cgroup_rel,
                pid,
            },
        );

        let mem_current = read_u64(&scope.join("memory.current")).await.unwrap_or(0);
        let mem_limit = self.memory_limit(&scope).await;
        let inactive_file = read_memory_inactive(&scope).await;
        let pids = read_u64(&scope.join("pids.current")).await.unwrap_or(0);
        let (blk_read, blk_write) = read_io(&scope).await;
        let (net_rx, net_tx) = read_net_dev(&self.proc_root, pid).await;

        let mut memory_stats = HashMap::new();
        memory_stats.insert("inactive_file".to_string(), inactive_file);
        let mut networks = HashMap::new();
        networks.insert(
            "eth".to_string(),
            NetworkStats {
                rx_bytes: net_rx,
                tx_bytes: net_tx,
            },
        );

        Some(RawStats {
            name: name.to_string(),
            id: full_id.to_string(),
            pids: PidsStats { current: pids },
            stats: ContainerStats {
                cpu: CpuStats {
                    usage: CpuUsage {
                        total: usage_usec,
                        percpu: Vec::new(),
                    },
                    system_usage: system_usec,
                    online_cpus: 1,
                },
                precpu: CpuStats {
                    usage: CpuUsage {
                        total: precpu_usage,
                        percpu: Vec::new(),
                    },
                    system_usage: 0,
                    online_cpus: 1,
                },
                memory: MemoryStats {
                    usage: mem_current,
                    limit: mem_limit,
                    stats: memory_stats,
                },
                networks,
                blkio: BlkioStats {
                    io_service_bytes: Some(vec![
                        BlkioEntry {
                            op: "Read".to_string(),
                            value: blk_read,
                        },
                        BlkioEntry {
                            op: "Write".to_string(),
                            value: blk_write,
                        },
                    ]),
                },
            },
        })
    }

    /// Resolve a container's init pid and cgroup-relative path. Cached entries
    /// are reused while `/host/proc/<pid>` still exists; a recreated container
    /// has a dead pid and is re-inspected (the only Docker call, once).
    async fn resolve(&mut self, full_id: &str) -> Option<(u32, String)> {
        if let Some(tracked) = self.tracked.get(full_id)
            && proc_path_exists(&self.proc_root, &tracked.pid.to_string()).await
        {
            return Some((tracked.pid, tracked.cgroup_rel.clone()));
        }
        let value: Value = self
            .docker
            .get_json(&format!("/containers/{full_id}"))
            .await
            .ok()?;
        let pid = value.get("State")?.get("Pid")?.as_u64()? as u32;
        if pid == 0 {
            return None; // not running
        }
        let cgroup_rel = read_proc_cgroup(&self.proc_root, pid).await?;
        Some((pid, cgroup_rel))
    }

    /// Per-container limit from `memory.max`; when unlimited fall back to the
    /// host total so the percentage stays meaningful.
    async fn memory_limit(&mut self, scope: &Path) -> u64 {
        match read_trimmed(&scope.join("memory.max")).await.as_deref() {
            Some("max") | None => {
                if self.host_mem_total.is_none() {
                    self.host_mem_total = read_host_mem_total(&self.proc_root).await;
                }
                self.host_mem_total.unwrap_or(0)
            }
            Some(value) => value.parse().unwrap_or(0),
        }
    }
}

async fn read_trimmed(path: &Path) -> Option<String> {
    tokio::fs::read_to_string(path)
        .await
        .ok()
        .map(|value| value.trim().to_string())
}

async fn read_u64(path: &Path) -> Option<u64> {
    read_trimmed(path).await?.parse().ok()
}

/// `usage_usec` from cgroup v2 `cpu.stat` (cumulative CPU across all cores).
async fn read_cpu_usage(scope: &Path) -> Option<u64> {
    let text = tokio::fs::read_to_string(scope.join("cpu.stat"))
        .await
        .ok()?;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("usage_usec ") {
            return rest.trim().parse().ok();
        }
    }
    None
}

async fn read_memory_inactive(scope: &Path) -> u64 {
    let Ok(text) = tokio::fs::read_to_string(scope.join("memory.stat")).await else {
        return 0;
    };
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("inactive_file ")
            && let Ok(value) = rest.trim().parse::<u64>()
        {
            return value;
        }
    }
    0
}

/// `rbytes=`/`wbytes=` summed across devices in cgroup v2 `io.stat`.
async fn read_io(scope: &Path) -> (u64, u64) {
    let Ok(text) = tokio::fs::read_to_string(scope.join("io.stat")).await else {
        return (0, 0);
    };
    let (mut read, mut write) = (0u64, 0u64);
    for line in text.lines() {
        for field in line.split_whitespace().skip(1) {
            if let Some(value) = field.strip_prefix("rbytes=") {
                read += value.parse().unwrap_or(0);
            } else if let Some(value) = field.strip_prefix("wbytes=") {
                write += value.parse().unwrap_or(0);
            }
        }
    }
    (read, write)
}

/// Cumulative rx/tx bytes summed over the container's interfaces, skipping the
/// loopback device. `/proc/<pid>/net/dev` reflects that pid's network
/// namespace, so the init pid is enough. Columns after `iface:` are
/// rx(bytes,packets,errs,drop,fifo,frame,compressed,multicast) then
/// tx(same); so rx is index 0 and tx is index 8.
async fn read_net_dev(proc_root: &Path, pid: u32) -> (u64, u64) {
    let path = proc_root.join(pid.to_string()).join("net/dev");
    let Ok(text) = tokio::fs::read_to_string(&path).await else {
        return (0, 0);
    };
    let (mut rx, mut tx) = (0u64, 0u64);
    for line in text.lines().skip(2) {
        let Some((iface, rest)) = line.split_once(':') else {
            continue;
        };
        if iface.trim() == "lo" {
            continue;
        }
        let columns: Vec<&str> = rest.split_whitespace().collect();
        if columns.len() >= 9 {
            rx += columns[0].parse().unwrap_or(0);
            tx += columns[8].parse().unwrap_or(0);
        }
    }
    (rx, tx)
}

async fn proc_path_exists(proc_root: &Path, pid: &str) -> bool {
    tokio::fs::try_exists(proc_root.join(pid).join("net/dev"))
        .await
        .unwrap_or(false)
}

/// The cgroup-v2 unified path (the `0::<path>` line) relative to the host's
/// cgroup root, e.g. `/system.slice/docker-<id>.scope`.
async fn read_proc_cgroup(proc_root: &Path, pid: u32) -> Option<String> {
    let text = tokio::fs::read_to_string(proc_root.join(pid.to_string()).join("cgroup"))
        .await
        .ok()?;
    for line in text.lines() {
        if let Some((hierarchy, path)) = line.split_once("::")
            && hierarchy == "0"
        {
            return Some(path.to_string());
        }
    }
    None
}

/// Host total RAM in bytes (used when a container is memory-unlimited). Prefer
/// the mounted host procfs, fall back to the agent's own (un-namespaced) view.
async fn read_host_mem_total(proc_root: &Path) -> Option<u64> {
    for path in [proc_root.join("meminfo"), PathBuf::from("/proc/meminfo")] {
        if let Ok(text) = tokio::fs::read_to_string(&path).await {
            for line in text.lines() {
                if let Some(rest) = line.strip_prefix("MemTotal:")
                    && let Some(kb) = rest.split_whitespace().next()
                    && let Ok(kb) = kb.parse::<u64>()
                {
                    return Some(kb * 1024);
                }
            }
        }
    }
    None
}
