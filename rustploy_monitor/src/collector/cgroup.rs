use std::collections::HashMap;
use std::time::Instant;

use tracing::{debug, info, warn};

use super::bytes_to_mb;
use crate::docker::api::DockerApi;
use crate::docker::cgroup::{CgroupReader, CgroupSample};
use crate::docker::types::{ContainerId, ContainerName, ContainerSummary};
use crate::error::MonitorError;
use crate::filter::ContainerFilter;
use crate::store::ContainerMetricRow;

struct Previous {
    sample: CgroupSample,
    at: Instant,
}

pub struct CgroupCollector {
    reader: CgroupReader,
    filter: ContainerFilter,
    previous: HashMap<ContainerId, Previous>,
    names: HashMap<ContainerId, ContainerName>,
}

impl CgroupCollector {
    pub fn new(filter: ContainerFilter) -> Option<Self> {
        let reader = CgroupReader::discover()?;
        Some(Self {
            reader,
            filter,
            previous: HashMap::new(),
            names: HashMap::new(),
        })
    }

    pub async fn refresh_containers(&mut self, docker: &DockerApi) -> Result<usize, MonitorError> {
        let summaries: Vec<ContainerSummary> = docker.get_json("/containers/json").await?;

        let before = self.names.len();
        self.names.clear();

        for summary in summaries {
            let name = summary.display_name();
            if !self.filter.should_monitor(name.as_str()) {
                continue;
            }
            self.names.insert(summary.id, name);
        }

        self.previous.retain(|id, _| self.names.contains_key(id));

        let now = self.names.len();
        if now != before {
            debug!(monitored = now, "container set changed");
        }

        Ok(now)
    }

    pub fn sample(&mut self) -> Vec<ContainerMetricRow> {
        let now = Instant::now();
        let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

        let ids: Vec<String> = self
            .names
            .keys()
            .map(|id| id.as_str().to_string())
            .collect();
        let samples = self.reader.read_many(&ids);

        let mut rows = Vec::with_capacity(samples.len());

        for (id_str, sample) in samples {
            let id = ContainerId::new(&id_str);
            let name = match self.names.get(&id) {
                Some(name) => name.as_str().to_string(),
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
                container_id: id.short().to_string(),
                name,
                cpu_perc,
                mem_perc,
                mem_used_mb: bytes_to_mb(mem_used as f64),
                mem_total_mb: bytes_to_mb(sample.memory_limit as f64),
                net_in_mb: 0.0,
                net_out_mb: 0.0,
                block_read_mb: bytes_to_mb(sample.io_read_bytes as f64),
                block_write_mb: bytes_to_mb(sample.io_write_bytes as f64),
                application_id: None,
                compose_id: None,
            });

            self.previous.insert(id, Previous { sample, at: now });
        }

        rows
    }

    pub fn monitored_count(&self) -> usize {
        self.names.len()
    }
}

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
        let now = start + Duration::from_secs(1);
        let percent = cpu_percent(&prev, &sample(2_000_000), now);

        assert!((percent - 200.0).abs() < 0.01, "got {percent}");
    }

    #[test]
    fn a_counter_going_backwards_does_not_underflow() {
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
