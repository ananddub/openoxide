use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::path::Path;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
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

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
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
}

pub struct Db {
    pub pool: SqlitePool,
}

impl Db {
    pub async fn init(db_url: &str) -> Result<Self, sqlx::Error> {
        if let Some(path_str) = db_url.strip_prefix("sqlite://") {
            if path_str != ":memory:" {
                if let Some(parent) = Path::new(path_str).parent() {
                    let _ = tokio::fs::create_dir_all(parent).await;
                }
                if !Path::new(path_str).exists() {
                    let _ = tokio::fs::File::create(path_str).await;
                }
            }
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS server_metrics (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                cpu REAL NOT NULL,
                cpu_model TEXT NOT NULL,
                cpu_cores INTEGER NOT NULL,
                cpu_physical_cores INTEGER NOT NULL,
                cpu_speed REAL NOT NULL,
                os TEXT NOT NULL,
                distro TEXT NOT NULL,
                kernel TEXT NOT NULL,
                arch TEXT NOT NULL,
                mem_used REAL NOT NULL,
                mem_used_gb REAL NOT NULL,
                mem_total REAL NOT NULL,
                uptime INTEGER NOT NULL,
                disk_used REAL NOT NULL,
                total_disk REAL NOT NULL,
                network_in REAL NOT NULL,
                network_out REAL NOT NULL
            );

            CREATE TABLE IF NOT EXISTS container_metrics (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                container_id TEXT NOT NULL,
                name TEXT NOT NULL,
                cpu_perc REAL NOT NULL,
                mem_perc REAL NOT NULL,
                mem_used_mb REAL NOT NULL,
                mem_total_mb REAL NOT NULL,
                net_in_mb REAL NOT NULL,
                net_out_mb REAL NOT NULL,
                block_read_mb REAL NOT NULL,
                block_write_mb REAL NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_server_metrics_ts ON server_metrics(timestamp);
            CREATE INDEX IF NOT EXISTS idx_container_metrics_name ON container_metrics(name);
            CREATE INDEX IF NOT EXISTS idx_container_metrics_ts ON container_metrics(timestamp);
            "#,
        )
        .execute(&pool)
        .await?;

        info!(
            "Independent Dokploy-compatible Monitor SQLite DB initialized: {}",
            db_url
        );
        Ok(Self { pool })
    }

    pub async fn save_server_metric(&self, m: &ServerMetric) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO server_metrics (
                timestamp, cpu, cpu_model, cpu_cores, cpu_physical_cores, cpu_speed,
                os, distro, kernel, arch, mem_used, mem_used_gb, mem_total, uptime,
                disk_used, total_disk, network_in, network_out
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&m.timestamp)
        .bind(m.cpu)
        .bind(&m.cpu_model)
        .bind(m.cpu_cores)
        .bind(m.cpu_physical_cores)
        .bind(m.cpu_speed)
        .bind(&m.os)
        .bind(&m.distro)
        .bind(&m.kernel)
        .bind(&m.arch)
        .bind(m.mem_used)
        .bind(m.mem_used_gb)
        .bind(m.mem_total)
        .bind(m.uptime as i64)
        .bind(m.disk_used)
        .bind(m.total_disk)
        .bind(m.network_in)
        .bind(m.network_out)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_last_n_server_metrics(
        &self,
        limit: i64,
    ) -> Result<Vec<ServerMetric>, sqlx::Error> {
        sqlx::query_as::<_, ServerMetric>(
            r#"SELECT id, timestamp, cpu, cpu_model, cpu_cores, cpu_physical_cores, cpu_speed, os, distro, kernel, arch, mem_used, mem_used_gb, mem_total, uptime, disk_used, total_disk, network_in, network_out FROM server_metrics ORDER BY id DESC LIMIT ?"#
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn save_container_metric(&self, m: &ContainerMetricRow) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO container_metrics (
                timestamp, container_id, name, cpu_perc, mem_perc, mem_used_mb,
                mem_total_mb, net_in_mb, net_out_mb, block_read_mb, block_write_mb
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&m.timestamp)
        .bind(&m.container_id)
        .bind(&m.name)
        .bind(m.cpu_perc)
        .bind(m.mem_perc)
        .bind(m.mem_used_mb)
        .bind(m.mem_total_mb)
        .bind(m.net_in_mb)
        .bind(m.net_out_mb)
        .bind(m.block_read_mb)
        .bind(m.block_write_mb)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_last_n_container_metrics(
        &self,
        app_name: &str,
        limit: i64,
    ) -> Result<Vec<ContainerMetricRow>, sqlx::Error> {
        sqlx::query_as::<_, ContainerMetricRow>(
            r#"SELECT id, timestamp, container_id, name, cpu_perc, mem_perc, mem_used_mb, mem_total_mb, net_in_mb, net_out_mb, block_read_mb, block_write_mb FROM container_metrics WHERE name LIKE ? ORDER BY id DESC LIMIT ?"#
        )
        .bind(format!("%{}%", app_name))
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn cleanup_old_metrics(&self, retention_days: i64) -> Result<u64, sqlx::Error> {
        let cutoff = chrono::Utc::now() - chrono::Duration::days(retention_days);
        let cutoff_str = cutoff.format("%Y-%m-%dT%H:%M:%SZ").to_string();

        let res1 = sqlx::query("DELETE FROM server_metrics WHERE timestamp < ?")
            .bind(&cutoff_str)
            .execute(&self.pool)
            .await?;

        let res2 = sqlx::query("DELETE FROM container_metrics WHERE timestamp < ?")
            .bind(&cutoff_str)
            .execute(&self.pool)
            .await?;

        Ok(res1.rows_affected() + res2.rows_affected())
    }
}
