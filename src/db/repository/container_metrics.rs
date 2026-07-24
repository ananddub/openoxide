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
            r#"SELECT id AS "id?: i64", timestamp AS "timestamp: i64", container_id AS "container_id: String", container_name AS "container_name: String", metrics_json AS "metrics_json: String" FROM container_metrics"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<ContainerMetric>, sqlx::Error> {
        sqlx::query_as!(
            ContainerMetric,
            r#"SELECT id AS "id?: i64", timestamp AS "timestamp: i64", container_id AS "container_id: String", container_name AS "container_name: String", metrics_json AS "metrics_json: String" FROM container_metrics WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &ContainerMetric) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO container_metrics (timestamp, container_id, container_name, metrics_json) VALUES (?, ?, ?, ?)"#,
            item.timestamp,
            &item.container_id,
            &item.container_name,
            &item.metrics_json
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
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
