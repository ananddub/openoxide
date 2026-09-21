use std::{
    collections::HashMap,
    sync::atomic::{AtomicI64, Ordering},
};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::core::config::Config;
use crate::error::MonitorError;
use auto_di::singleton;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMetric {
    pub id: Option<i64>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerMetricRow {
    pub id: Option<i64>,
    pub timestamp: String,
    pub container_id: String,
    pub name: String,
    pub cpu_perc: f64,
    pub mem_perc: f64,
    pub mem_used_mb: f64,
    pub mem_total_mb: f64,
    pub net_in_mb: f64,
    pub net_out_mb: f64,
    pub block_read_mb: f64,
    pub block_write_mb: f64,
    #[serde(default)]
    pub application_id: Option<i64>,
    #[serde(default)]
    pub compose_id: Option<i64>,
}

/// Latest-only metric store. The Agent keeps exactly one current reading per
/// server and per container — no time-series history, so memory stays flat
/// regardless of uptime. The panel shows current values only; alerting keeps
/// its own short sample windows server-side.
pub struct Store {
    server_metric: RwLock<Option<ServerMetric>>,
    container_metrics: RwLock<HashMap<String, ContainerMetricRow>>,
    sequence: AtomicI64,
}

impl Store {
    pub async fn init(_database_url: &str) -> Result<Self, MonitorError> {
        tracing::info!("latest-only metric store initialized (no history buffer)");
        Ok(Self::empty())
    }

    pub fn empty() -> Self {
        Self {
            server_metric: RwLock::new(None),
            container_metrics: RwLock::new(HashMap::new()),
            sequence: AtomicI64::new(1),
        }
    }

    pub async fn save_server_metric(&self, metric: &ServerMetric) -> Result<(), MonitorError> {
        let mut metric = metric.clone();
        metric.id = Some(self.next_id());
        *self.server_metric.write().await = Some(metric);
        Ok(())
    }

    /// Newest-first, like the old history API — but at most one row exists.
    pub async fn get_last_n_server_metrics(
        &self,
        limit: i64,
    ) -> Result<Vec<ServerMetric>, MonitorError> {
        let rows = self.server_metric.read().await;
        Ok(match (&*rows, limit) {
            (Some(metric), limit) if limit > 0 => vec![metric.clone()],
            _ => Vec::new(),
        })
    }

    pub async fn save_container_metrics_batch(
        &self,
        metrics: &[ContainerMetricRow],
    ) -> Result<(), MonitorError> {
        let mut rows = self.container_metrics.write().await;
        for metric in metrics {
            let mut metric = metric.clone();
            metric.id = Some(self.next_id());
            rows.insert(metric.name.clone(), metric);
        }
        Ok(())
    }

    /// Latest reading per container, newest first, filtered by name.
    pub async fn get_last_n_container_metrics(
        &self,
        app_name: &str,
        limit: i64,
    ) -> Result<Vec<ContainerMetricRow>, MonitorError> {
        let rows = self.container_metrics.read().await;
        let mut matched: Vec<ContainerMetricRow> = rows
            .values()
            .filter(|row| row.name.contains(app_name))
            .cloned()
            .collect();
        matched.sort_by_key(|row| std::cmp::Reverse(row.id.unwrap_or(0)));
        matched.truncate(limit.max(0) as usize);
        Ok(matched)
    }

    /// No-op: the store holds only current values, so nothing ages out.
    /// Kept so the maintenance task and `RETENTION_DAYS` config stay valid.
    pub async fn cleanup_old_metrics(&self, _retention_days: i64) -> Result<u64, MonitorError> {
        Ok(0)
    }

    fn next_id(&self) -> i64 {
        self.sequence.fetch_add(1, Ordering::Relaxed)
    }
}

#[singleton]
impl Store {
    pub async fn new(config: std::sync::Arc<Config>) -> Result<Self, MonitorError> {
        Self::init(&config.database_url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn metric(timestamp: &str, cpu: f64) -> ServerMetric {
        ServerMetric {
            id: None,
            timestamp: timestamp.to_string(),
            cpu,
            cpu_model: String::new(),
            cpu_cores: 0,
            cpu_physical_cores: 0,
            cpu_speed: 0.0,
            os: String::new(),
            distro: String::new(),
            kernel: String::new(),
            arch: String::new(),
            mem_used: 0.0,
            mem_used_gb: 0.0,
            mem_total: 0.0,
            uptime: 0,
            disk_used: 0.0,
            total_disk: 0.0,
            network_in: 0.0,
            network_out: 0.0,
        }
    }

    #[tokio::test]
    async fn only_the_newest_server_metric_survives() {
        let store = Store::init("memory://metrics").await.unwrap();
        store.save_server_metric(&metric("old", 1.0)).await.unwrap();
        store
            .save_server_metric(&metric("new", 2.0))
            .await
            .unwrap();
        let rows = store.get_last_n_server_metrics(50).await.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].timestamp, "new");
    }

    #[tokio::test]
    async fn container_metrics_keep_one_row_per_container() {
        let store = Store::init("memory://metrics").await.unwrap();
        let row = |name: &str, cpu: f64| ContainerMetricRow {
            id: None,
            timestamp: "t".to_string(),
            container_id: name.to_string(),
            name: name.to_string(),
            cpu_perc: cpu,
            mem_perc: 0.0,
            mem_used_mb: 0.0,
            mem_total_mb: 0.0,
            net_in_mb: 0.0,
            net_out_mb: 0.0,
            block_read_mb: 0.0,
            block_write_mb: 0.0,
            application_id: None,
            compose_id: None,
        };
        store
            .save_container_metrics_batch(&[row("web-1", 1.0)])
            .await
            .unwrap();
        store
            .save_container_metrics_batch(&[row("web-1", 5.0), row("db-1", 2.0)])
            .await
            .unwrap();
        let rows = store.get_last_n_container_metrics("", 50).await.unwrap();
        assert_eq!(rows.len(), 2);
        let web = rows.iter().find(|r| r.name == "web-1").unwrap();
        assert_eq!(web.cpu_perc, 5.0);
    }
}
