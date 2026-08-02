use std::collections::HashMap;
use std::time::Instant;

use tracing::{debug, info, warn};

use crate::docker::api::DockerApi;
use crate::docker::cgroup::{CgroupReader, CgroupSample};
use crate::docker::stats::ContainerSummary;
use crate::filter::ContainerFilter;
use crate::store::ContainerMetricRow;

/// Previous sample for one container, needed to turn cgroup's absolute
/// counters into rates.
struct Previous {
    sample: CgroupSample,
    at: Instant,
}

/// Polls every container's metrics straight from cgroup v2.
///
/// The docker API's stats endpoint needs one connection per container and makes
/// dockerd do the work; at thousands of containers that is the bottleneck.
/// Reading cgroup files costs ~35 µs per container and scales linearly, so this
/// is the path that holds up at density.
///
/// The tradeoff versus the API: cgroup exposes no network counters (those live
/// in the container's network namespace), so net I/O is reported as zero here.
/// Containers that need it use the streaming API path instead.
pub struct CgroupCollector {
    reader: CgroupReader,
    filter: ContainerFilter,
    /// Last sample per container, for CPU rate calculation.
    previous: HashMap<String, Previous>,
    /// Container id -> display name, refreshed alongside the container list.
    names: HashMap<String, String>,
}

impl CgroupCollector {
    /// Returns `None` when cgroup v2 is not mounted, in which case the caller
    /// falls back to the API path.
    pub fn new(filter: ContainerFilter) -> Option<Self> {
        let reader = CgroupReader::discover()?;
        Some(Self {
            reader,
            filter,
            previous: HashMap::new(),
            names: HashMap::new(),
        })
    }

    /// Refreshes the container list from the daemon.
    ///
    /// This is the only daemon call in the cgroup path, and it happens once per
    /// cycle rather than once per container. Filtering is applied here so that
    /// excluded containers are never read at all.
    pub async fn refresh_containers(&mut self, docker: &DockerApi) -> Result<usize, String> {
        let summaries: Vec<ContainerSummary> = docker.get_json("/containers/json").await?;

        let before = self.names.len();
        self.names.clear();

        for summary in summaries {
            let name = summary.display_name();
            if !self.filter.should_monitor(&name) {
                continue;
            }
            self.names.insert(summary.id, name);
        }

        // Drop rate-tracking state for containers that are gone, so a long
        // running agent does not accumulate dead entries.
        self.previous.retain(|id, _| self.names.contains_key(id));

        let now = self.names.len();
        if now != before {
            debug!(monitored = now, "container set changed");
        }

        Ok(now)
    }

    /// Samples every monitored container. Returns one row per container that
    /// still has a live cgroup.
    pub fn sample(&mut self) -> Vec<ContainerMetricRow> {
        let now = Instant::now();
        let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

        let ids: Vec<String> = self.names.keys().cloned().collect();
        let samples = self.reader.read_many(&ids);

        let mut rows = Vec::with_capacity(samples.len());

        for (id, sample) in samples {
            let name = match self.names.get(&id) {
                Some(name) => name.clone(),
                None => continue,
            };

            let cpu_perc = self
                .previous
                .get(&id)
                .map(|prev| cpu_percent(prev, &sample, now))
                .unwrap_or(0.0);

            let mem_used = sample.memory_usage.saturating_sub(sample.memory_cache);
            let mem_perc = if sample.memory_limit > 0 {
                (mem_used as f64 / sample.memory_limit as f64) * 100.0
            } else {
                0.0
            };

            rows.push(ContainerMetricRow {
                id: None,
                timestamp: timestamp.clone(),
                container_id: id.get(..12).unwrap_or(&id).to_string(),
                name,
                cpu_perc,
                mem_perc,
                mem_used_mb: mem_used as f64 / 1_048_576.0,
                mem_total_mb: sample.memory_limit as f64 / 1_048_576.0,
                // cgroup has no network counters; the streaming path fills these.
                net_in_mb: 0.0,
                net_out_mb: 0.0,
                block_read_mb: sample.io_read_bytes as f64 / 1_048_576.0,
                block_write_mb: sample.io_write_bytes as f64 / 1_048_576.0,
            });

            self.previous.insert(id, Previous { sample, at: now });
        }

        rows
    }

    pub fn monitored_count(&self) -> usize {
        self.names.len()
    }
}

/// CPU as a percentage of one core, from the delta between two cgroup samples.
///
/// cgroup reports cumulative microseconds of CPU time; dividing the delta by
/// wall-clock elapsed gives utilisation. 100% means one core saturated, which
/// matches what `docker stats` reports.
fn cpu_percent(prev: &Previous, current: &CgroupSample, now: Instant) -> f64 {
    let elapsed = now.duration_since(prev.at).as_micros();
    if elapsed == 0 {
        return 0.0;
    }

    let delta = current
        .cpu_usage_usec
        .saturating_sub(prev.sample.cpu_usage_usec);

    (delta as f64 / elapsed as f64) * 100.0
}

/// Logs a warning when the monitored container count is high enough that the
/// operator probably wants a filter.
pub fn warn_if_dense(count: usize, filter_is_unset: bool) {
    const DENSE: usize = 500;

    if count >= DENSE && filter_is_unset {
        warn!(
            containers = count,
            "monitoring every container on a dense host — set INCLUDE_CONTAINERS \
             or EXCLUDE_CONTAINERS to narrow this"
        );
    } else if count >= DENSE {
        info!(containers = count, "monitoring a dense host");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn sample(cpu_usec: u64) -> CgroupSample {
        CgroupSample {
            cpu_usage_usec: cpu_usec,
            ..Default::default()
        }
    }

    #[test]
    fn cpu_percent_is_delta_over_elapsed() {
        let start = Instant::now();
        let prev = Previous {
            sample: sample(1_000_000),
            at: start,
        };
        // 500 ms of CPU over 1 s of wall clock = 50% of one core.
        let now = start + Duration::from_secs(1);
        let percent = cpu_percent(&prev, &sample(1_500_000), now);

        assert!((percent - 50.0).abs() < 0.01, "got {percent}");
    }

    #[test]
    fn cpu_percent_can_exceed_one_hundred_on_multiple_cores() {
        let start = Instant::now();
        let prev = Previous {
            sample: sample(0),
            at: start,
        };
        // 2 s of CPU in 1 s of wall clock = two cores saturated.
        let now = start + Duration::from_secs(1);
        let percent = cpu_percent(&prev, &sample(2_000_000), now);

        assert!((percent - 200.0).abs() < 0.01, "got {percent}");
    }

    #[test]
    fn a_counter_going_backwards_does_not_underflow() {
        // Can happen if a container is replaced under the same id.
        let start = Instant::now();
        let prev = Previous {
            sample: sample(5_000_000),
            at: start,
        };
        let now = start + Duration::from_secs(1);

        assert_eq!(cpu_percent(&prev, &sample(1_000_000), now), 0.0);
    }

    #[test]
    fn zero_elapsed_does_not_divide_by_zero() {
        let start = Instant::now();
        let prev = Previous {
            sample: sample(0),
            at: start,
        };

        assert_eq!(cpu_percent(&prev, &sample(1_000), start), 0.0);
    }
}
