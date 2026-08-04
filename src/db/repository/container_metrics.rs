use crate::db::models::container_metrics::ContainerMetric;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct ContainerMetricRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl ContainerMetricRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<ContainerMetric>, sqlx::Error> {
        sqlx::query_as!(
            ContainerMetric,
            r#"SELECT id AS "id?: i64", timestamp AS "timestamp: i64", container_id AS "container_id: String", container_name AS "container_name: String", metrics_json AS "metrics_json: String", server_id AS "server_id: i64", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64" FROM container_metrics"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<ContainerMetric>, sqlx::Error> {
        sqlx::query_as!(
            ContainerMetric,
            r#"SELECT id AS "id?: i64", timestamp AS "timestamp: i64", container_id AS "container_id: String", container_name AS "container_name: String", metrics_json AS "metrics_json: String", server_id AS "server_id: i64", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64" FROM container_metrics WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn history_for_server(
        &self,
        server_id: i64,
        container_id: Option<&str>,
        limit: i64,
    ) -> Result<Vec<ContainerMetric>, sqlx::Error> {
        sqlx::query_as!(
            ContainerMetric,
            r#"SELECT id AS "id?: i64", timestamp AS "timestamp: i64", container_id AS "container_id: String", container_name AS "container_name: String", metrics_json AS "metrics_json: String", server_id AS "server_id: i64", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64"
               FROM container_metrics
               WHERE server_id = ? AND (? IS NULL OR container_id = ?)
               ORDER BY timestamp DESC LIMIT ?"#,
            server_id, container_id, container_id, limit
        ).fetch_all(self.pool.as_ref()).await
    }

    pub async fn create(&self, item: &ContainerMetric) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO container_metrics (timestamp, container_id, container_name, metrics_json, server_id, application_id, compose_id) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
            item.timestamp,
            &item.container_id,
            &item.container_name,
            &item.metrics_json,
            item.server_id,
            item.application_id,
            item.compose_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn delete_older_than(&self, cutoff: i64) -> Result<u64, sqlx::Error> {
        Ok(sqlx::query("DELETE FROM container_metrics WHERE timestamp < ?")
            .bind(cutoff).execute(self.pool.as_ref()).await?.rows_affected())
    }

    pub async fn update(&self, id: i64, item: &ContainerMetric) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE container_metrics SET timestamp = ?, container_id = ?, container_name = ?, metrics_json = ? WHERE id = ?"#,
            item.timestamp,
            &item.container_id,
            &item.container_name,
            &item.metrics_json,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM container_metrics WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
