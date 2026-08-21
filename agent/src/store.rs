use std::{
    collections::VecDeque,
    sync::atomic::{AtomicI64, Ordering},
};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::error::MonitorError;

const SESSION_WINDOW_MINUTES: i64 = 5;

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

pub struct Store {
    server_metrics: RwLock<VecDeque<ServerMetric>>,
    container_metrics: RwLock<VecDeque<ContainerMetricRow>>,
    sequence: AtomicI64,
}

impl Store {
    pub async fn init(_database_url: &str) -> Result<Self, MonitorError> {
        tracing::info!(
            session_minutes = SESSION_WINDOW_MINUTES,
            "in-memory metric session initialized"
        );
        Ok(Self::new())
    }

    pub fn new() -> Self {
        Self {
            server_metrics: RwLock::new(VecDeque::new()),
            container_metrics: RwLock::new(VecDeque::new()),
            sequence: AtomicI64::new(1),
        }
    }

    pub async fn save_server_metric(&self, metric: &ServerMetric) -> Result<(), MonitorError> {
        let mut metric = metric.clone();
        metric.id = Some(self.next_id());
        let mut rows = self.server_metrics.write().await;
        rows.push_back(metric);
        prune(&mut rows, |row| &row.timestamp);
        Ok(())
    }

    pub async fn get_last_n_server_metrics(
        &self,
        limit: i64,
    ) -> Result<Vec<ServerMetric>, MonitorError> {
        let mut rows = self.server_metrics.write().await;
        prune(&mut rows, |row| &row.timestamp);
        Ok(rows
            .iter()
            .rev()
            .take(limit.max(0) as usize)
            .cloned()
            .collect())
    }

    #[allow(dead_code)]
    pub async fn save_container_metric(
        &self,
        metric: &ContainerMetricRow,
    ) -> Result<(), MonitorError> {
        self.save_container_metrics_batch(std::slice::from_ref(metric))
            .await
    }

    pub async fn save_container_metrics_batch(
        &self,
        metrics: &[ContainerMetricRow],
    ) -> Result<(), MonitorError> {
        let mut rows = self.container_metrics.write().await;
        for metric in metrics {
            let mut metric = metric.clone();
            metric.id = Some(self.next_id());
            rows.push_back(metric);
        }
        prune(&mut rows, |row| &row.timestamp);
        Ok(())
    }

    pub async fn get_last_n_container_metrics(
        &self,
        app_name: &str,
        limit: i64,
    ) -> Result<Vec<ContainerMetricRow>, MonitorError> {
        let mut rows = self.container_metrics.write().await;
        prune(&mut rows, |row| &row.timestamp);
        Ok(rows
            .iter()
            .rev()
            .filter(|row| row.name.contains(app_name))
            .take(limit.max(0) as usize)
            .cloned()
            .collect())
    }

    pub async fn cleanup_old_metrics(&self, _retention_days: i64) -> Result<u64, MonitorError> {
        let mut server = self.server_metrics.write().await;
        let mut containers = self.container_metrics.write().await;
        let before = server.len() + containers.len();
        prune(&mut server, |row| &row.timestamp);
        prune(&mut containers, |row| &row.timestamp);
        Ok((before - server.len() - containers.len()) as u64)
    }

    fn next_id(&self) -> i64 {
        self.sequence.fetch_add(1, Ordering::Relaxed)
    }
}

fn prune<T>(rows: &mut VecDeque<T>, timestamp: impl Fn(&T) -> &str) {
    let cutoff = Utc::now() - Duration::minutes(SESSION_WINDOW_MINUTES);
    while rows
        .front()
        .is_some_and(|row| is_older(timestamp(row), cutoff))
    {
        rows.pop_front();
    }
}

fn is_older(raw: &str, cutoff: DateTime<Utc>) -> bool {
    DateTime::parse_from_rfc3339(raw)
        .map(|value| value.with_timezone(&Utc) < cutoff)
        .unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn old_rows_are_removed_from_the_session() {
        let store = Store::new();
        let old = ServerMetric {
            id: None,
            timestamp: (Utc::now() - Duration::minutes(6)).to_rfc3339(),
            cpu: 0.0,
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
        };
        store.save_server_metric(&old).await.unwrap();
        assert!(
            store
                .get_last_n_server_metrics(10)
                .await
                .unwrap()
                .is_empty()
        );
    }
}
