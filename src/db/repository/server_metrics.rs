use crate::db::models::server_metrics::ServerMetric;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct ServerMetricRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl ServerMetricRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<ServerMetric>, sqlx::Error> {
        sqlx::query_as!(
            ServerMetric,
            r#"SELECT timestamp AS "timestamp?: i64", cpu AS "cpu: f64", cpu_model AS "cpu_model: String", cpu_cores AS "cpu_cores: i64", cpu_physical_cores AS "cpu_physical_cores: i64", cpu_speed AS "cpu_speed: f64", os AS "os: String", distro AS "distro: String", kernel AS "kernel: String", arch AS "arch: String", mem_used AS "mem_used: f64", mem_used_gb AS "mem_used_gb: f64", mem_total AS "mem_total: f64", uptime AS "uptime: i64", disk_used AS "disk_used: f64", total_disk AS "total_disk: f64", network_in AS "network_in: f64", network_out AS "network_out: f64" FROM server_metrics"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_latest(&self, limit: i64) -> Result<Vec<ServerMetric>, sqlx::Error> {
        sqlx::query_as!(
            ServerMetric,
            r#"SELECT timestamp AS "timestamp?: i64", cpu AS "cpu: f64", cpu_model AS "cpu_model: String", cpu_cores AS "cpu_cores: i64", cpu_physical_cores AS "cpu_physical_cores: i64", cpu_speed AS "cpu_speed: f64", os AS "os: String", distro AS "distro: String", kernel AS "kernel: String", arch AS "arch: String", mem_used AS "mem_used: f64", mem_used_gb AS "mem_used_gb: f64", mem_total AS "mem_total: f64", uptime AS "uptime: i64", disk_used AS "disk_used: f64", total_disk AS "total_disk: f64", network_in AS "network_in: f64", network_out AS "network_out: f64" FROM server_metrics ORDER BY timestamp DESC LIMIT ?"#,
            limit
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, timestamp: i64) -> Result<Option<ServerMetric>, sqlx::Error> {
        sqlx::query_as!(
            ServerMetric,
            r#"SELECT timestamp AS "timestamp?: i64", cpu AS "cpu: f64", cpu_model AS "cpu_model: String", cpu_cores AS "cpu_cores: i64", cpu_physical_cores AS "cpu_physical_cores: i64", cpu_speed AS "cpu_speed: f64", os AS "os: String", distro AS "distro: String", kernel AS "kernel: String", arch AS "arch: String", mem_used AS "mem_used: f64", mem_used_gb AS "mem_used_gb: f64", mem_total AS "mem_total: f64", uptime AS "uptime: i64", disk_used AS "disk_used: f64", total_disk AS "total_disk: f64", network_in AS "network_in: f64", network_out AS "network_out: f64" FROM server_metrics WHERE timestamp = ?"#,
            timestamp
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &ServerMetric) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO server_metrics (cpu, cpu_model, cpu_cores, cpu_physical_cores, cpu_speed, os, distro, kernel, arch, mem_used, mem_used_gb, mem_total, uptime, disk_used, total_disk, network_in, network_out) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            item.cpu,
            &item.cpu_model,
            item.cpu_cores,
            item.cpu_physical_cores,
            item.cpu_speed,
            &item.os,
            &item.distro,
            &item.kernel,
            &item.arch,
            item.mem_used,
            item.mem_used_gb,
            item.mem_total,
            item.uptime,
            item.disk_used,
            item.total_disk,
            item.network_in,
            item.network_out
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, timestamp: i64, item: &ServerMetric) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE server_metrics SET cpu = ?, cpu_model = ?, cpu_cores = ?, cpu_physical_cores = ?, cpu_speed = ?, os = ?, distro = ?, kernel = ?, arch = ?, mem_used = ?, mem_used_gb = ?, mem_total = ?, uptime = ?, disk_used = ?, total_disk = ?, network_in = ?, network_out = ? WHERE timestamp = ?"#,
            item.cpu,
            &item.cpu_model,
            item.cpu_cores,
            item.cpu_physical_cores,
            item.cpu_speed,
            &item.os,
            &item.distro,
            &item.kernel,
            &item.arch,
            item.mem_used,
            item.mem_used_gb,
            item.mem_total,
            item.uptime,
            item.disk_used,
            item.total_disk,
            item.network_in,
            item.network_out,
            timestamp
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, timestamp: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"DELETE FROM server_metrics WHERE timestamp = ?"#,
            timestamp
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}
