use std::collections::HashMap;

use crate::docker::types::ContainerId;
use crate::store::ContainerMetricRow;

pub const PEAK_SUFFIX: &str = " (peak)";

#[derive(Debug, Default, Clone)]
struct Accumulator {
    count: u32,
    cpu_sum: f64,
    cpu_max: f64,
    mem_sum: f64,
    mem_max: f64,
    mem_perc_sum: f64,
    mem_perc_max: f64,
    mem_total_mb: f64,
    net_in_mb: f64,
    net_out_mb: f64,
    block_read_mb: f64,
    block_write_mb: f64,
    name: String,
    container_id: String,
}

impl Accumulator {
    fn add(&mut self, row: &ContainerMetricRow) {
        self.count += 1;
        self.cpu_sum += row.cpu_perc;
        self.cpu_max = self.cpu_max.max(row.cpu_perc);
        self.mem_sum += row.mem_used_mb;
        self.mem_max = self.mem_max.max(row.mem_used_mb);
        self.mem_perc_sum += row.mem_perc;
        self.mem_perc_max = self.mem_perc_max.max(row.mem_perc);

        self.mem_total_mb = row.mem_total_mb;
        self.net_in_mb = row.net_in_mb;
        self.net_out_mb = row.net_out_mb;
        self.block_read_mb = row.block_read_mb;
        self.block_write_mb = row.block_write_mb;
        self.name = row.name.clone();
        self.container_id = row.container_id.clone();
    }

    fn finish(&self, timestamp: &str) -> Vec<ContainerMetricRow> {
        if self.count == 0 {
            return Vec::new();
        }

        let n = f64::from(self.count);

        let avg = ContainerMetricRow {
            id: None,
            timestamp: timestamp.to_string(),
            container_id: self.container_id.clone(),
            name: self.name.clone(),
            cpu_perc: self.cpu_sum / n,
            mem_perc: self.mem_perc_sum / n,
            mem_used_mb: self.mem_sum / n,
            mem_total_mb: self.mem_total_mb,
            net_in_mb: self.net_in_mb,
            net_out_mb: self.net_out_mb,
            block_read_mb: self.block_read_mb,
            block_write_mb: self.block_write_mb,
        };

        let peak = ContainerMetricRow {
            id: None,
            timestamp: timestamp.to_string(),
            name: format!("{} {}", self.name, PEAK_SUFFIX.trim()),
            cpu_perc: self.cpu_max,
            mem_perc: self.mem_perc_max,
            mem_used_mb: self.mem_max,
            ..avg.clone()
        };

        vec![avg, peak]
    }
}

pub struct Rollup {
    windows: HashMap<ContainerId, Accumulator>,
    window_size: u32,
}

impl Rollup {
    pub fn new(window_size: u32) -> Self {
        Self {
            windows: HashMap::new(),
            window_size: window_size.max(1),
        }
    }

    pub fn add(&mut self, rows: &[ContainerMetricRow]) -> Vec<ContainerMetricRow> {
        for row in rows {
            self.windows
                .entry(ContainerId::new(&row.container_id))
                .or_default()
                .add(row);
        }

        let ready = self
            .windows
            .values()
            .any(|acc| acc.count >= self.window_size);

        if !ready {
            return Vec::new();
        }

        self.flush()
    }

    pub fn flush(&mut self) -> Vec<ContainerMetricRow> {
        let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

        let out: Vec<ContainerMetricRow> = self
            .windows
            .values()
            .flat_map(|acc| acc.finish(&timestamp))
            .collect();

        self.windows.clear();
        out
    }

    pub fn is_passthrough(&self) -> bool {
        self.window_size <= 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(name: &str, cpu: f64, mem: f64) -> ContainerMetricRow {
        ContainerMetricRow {
            id: None,
            timestamp: "2026-01-01T00:00:00Z".into(),
            container_id: name.into(),
            name: name.into(),
            cpu_perc: cpu,
            mem_perc: 0.0,
            mem_used_mb: mem,
            mem_total_mb: 1024.0,
            net_in_mb: 0.0,
            net_out_mb: 0.0,
            block_read_mb: 0.0,
            block_write_mb: 0.0,
        }
    }

    #[test]
    fn buffers_until_the_window_fills() {
        let mut rollup = Rollup::new(3);

        assert!(rollup.add(&[row("web", 10.0, 100.0)]).is_empty());
        assert!(rollup.add(&[row("web", 20.0, 200.0)]).is_empty());

        let out = rollup.add(&[row("web", 30.0, 300.0)]);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn emits_average_and_peak() {
        let mut rollup = Rollup::new(3);
        rollup.add(&[row("web", 10.0, 100.0)]);
        rollup.add(&[row("web", 20.0, 200.0)]);
        let out = rollup.add(&[row("web", 90.0, 300.0)]);

        let avg = out.iter().find(|r| !r.name.ends_with(PEAK_SUFFIX)).unwrap();
        let peak = out.iter().find(|r| r.name.ends_with(PEAK_SUFFIX)).unwrap();

        assert!((avg.cpu_perc - 40.0).abs() < 0.01, "got {}", avg.cpu_perc);
        assert_eq!(peak.cpu_perc, 90.0);
        assert_eq!(peak.mem_used_mb, 300.0);
    }

    #[test]
    fn a_spike_survives_averaging() {
        let mut rollup = Rollup::new(10);
        for _ in 0..9 {
            rollup.add(&[row("web", 1.0, 10.0)]);
        }
        let out = rollup.add(&[row("web", 100.0, 10.0)]);

        let avg = out.iter().find(|r| !r.name.ends_with(PEAK_SUFFIX)).unwrap();
        let peak = out.iter().find(|r| r.name.ends_with(PEAK_SUFFIX)).unwrap();

        assert!(avg.cpu_perc < 11.0);
        assert_eq!(peak.cpu_perc, 100.0);
    }

    #[test]
    fn tracks_containers_independently() {
        let mut rollup = Rollup::new(2);
        rollup.add(&[row("web", 10.0, 100.0), row("db", 50.0, 500.0)]);
        let out = rollup.add(&[row("web", 30.0, 100.0), row("db", 70.0, 500.0)]);

        assert_eq!(out.len(), 4);

        let web = out
            .iter()
            .find(|r| r.name == "web")
            .expect("web average row");
        assert!((web.cpu_perc - 20.0).abs() < 0.01);
    }

    #[test]
    fn cumulative_counters_keep_the_latest_value() {
        let mut rollup = Rollup::new(2);

        let mut first = row("web", 1.0, 10.0);
        first.block_read_mb = 100.0;
        let mut second = row("web", 1.0, 10.0);
        second.block_read_mb = 150.0;

        rollup.add(&[first]);
        let out = rollup.add(&[second]);

        let avg = out.iter().find(|r| !r.name.ends_with(PEAK_SUFFIX)).unwrap();
        assert_eq!(avg.block_read_mb, 150.0);
    }

    #[test]
    fn window_of_one_passes_through() {
        let mut rollup = Rollup::new(1);
        assert!(rollup.is_passthrough());

        let out = rollup.add(&[row("web", 42.0, 100.0)]);
        assert_eq!(out.len(), 2);
        assert_eq!(
            out.iter()
                .find(|r| !r.name.ends_with(PEAK_SUFFIX))
                .unwrap()
                .cpu_perc,
            42.0
        );
    }

    #[test]
    fn window_size_zero_is_clamped() {
        let rollup = Rollup::new(0);
        assert!(rollup.is_passthrough());
    }

    #[test]
    fn flush_on_an_empty_rollup_emits_nothing() {
        let mut rollup = Rollup::new(5);
        assert!(rollup.flush().is_empty());
    }

    #[test]
    fn windows_reset_after_emitting() {
        let mut rollup = Rollup::new(2);
        rollup.add(&[row("web", 10.0, 100.0)]);
        rollup.add(&[row("web", 10.0, 100.0)]);

        assert!(rollup.add(&[row("web", 90.0, 100.0)]).is_empty());
        let out = rollup.add(&[row("web", 90.0, 100.0)]);
        let avg = out.iter().find(|r| !r.name.ends_with(PEAK_SUFFIX)).unwrap();
        assert_eq!(avg.cpu_perc, 90.0);
    }
}
