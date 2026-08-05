use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

#[derive(Clone, Copy)]
enum DeliveryStatus {
    Delivered,
    Failed,
}
impl DeliveryStatus {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Delivered => "DELIVERED",
            Self::Failed => "FAILED",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct NotificationDeliveryAttempt {
    pub id: i64,
    pub notification_id: i64,
    pub organization_id: i64,
    pub trigger_name: String,
    pub correlation_id: String,
    pub status: String,
    pub attempt: i64,
    pub title: String,
    pub body: String,
    pub error: Option<String>,
    pub created_at: i64,
    pub finished_at: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct NotificationResourceBinding {
    pub id: i64,
    pub notification_id: i64,
    pub organization_id: i64,
    pub resource_type: String,
    pub resource_id: i64,
    pub created_at: i64,
}

pub struct NotificationDeliveryRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl NotificationDeliveryRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn begin(
        &self,
        notification_id: i64,
        organization_id: i64,
        trigger_name: &str,
        correlation_id: &str,
        attempt: i64,
        title: &str,
        body: &str,
    ) -> sqlx::Result<i64> {
        let result = sqlx::query!(
            "INSERT INTO notification_delivery_attempts (notification_id, organization_id, trigger_name, correlation_id, status, attempt, title, body) VALUES (?, ?, ?, ?, 'PENDING', ?, ?, ?)",
            notification_id, organization_id, trigger_name, correlation_id, attempt, title, body
        )
        .execute(self.pool.as_ref()).await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn finish(&self, id: i64, error: Option<&str>) -> sqlx::Result<()> {
        let status = if error.is_some() {
            DeliveryStatus::Failed
        } else {
            DeliveryStatus::Delivered
        };
        sqlx::query!(
            "UPDATE notification_delivery_attempts SET status=?, error=?, finished_at=unixepoch() WHERE id=?",
            status.as_str(), error, id
        ).execute(self.pool.as_ref()).await?;
        Ok(())
    }

    pub async fn list(
        &self,
        organization_id: i64,
        limit: i64,
    ) -> sqlx::Result<Vec<NotificationDeliveryAttempt>> {
        sqlx::query_as!(
            NotificationDeliveryAttempt,
            r#"SELECT id AS "id!: i64", notification_id AS "notification_id!: i64", organization_id AS "organization_id!: i64", trigger_name AS "trigger_name!: String", correlation_id AS "correlation_id!: String", status AS "status!: String", attempt AS "attempt!: i64", title AS "title!: String", body AS "body!: String", error, created_at AS "created_at!: i64", finished_at FROM notification_delivery_attempts WHERE organization_id=? ORDER BY created_at DESC LIMIT ?"#,
            organization_id, limit.clamp(1, 500)
        ).fetch_all(self.pool.as_ref()).await
    }

    pub async fn create_binding(
        &self,
        notification_id: i64,
        organization_id: i64,
        resource_type: &str,
        resource_id: i64,
    ) -> sqlx::Result<i64> {
        let result = sqlx::query!(
            "INSERT INTO notification_resource_bindings (notification_id, organization_id, resource_type, resource_id) VALUES (?, ?, ?, ?)",
            notification_id, organization_id, resource_type, resource_id
        ).execute(self.pool.as_ref()).await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn bindings(
        &self,
        organization_id: i64,
    ) -> sqlx::Result<Vec<NotificationResourceBinding>> {
        sqlx::query_as!(
            NotificationResourceBinding,
            r#"SELECT id AS "id!: i64", notification_id AS "notification_id!: i64", organization_id AS "organization_id!: i64", resource_type AS "resource_type!: String", resource_id AS "resource_id!: i64", created_at AS "created_at!: i64" FROM notification_resource_bindings WHERE organization_id=? ORDER BY created_at DESC"#,
            organization_id
        ).fetch_all(self.pool.as_ref()).await
    }

    pub async fn delete_binding(&self, id: i64, organization_id: i64) -> sqlx::Result<bool> {
        let result = sqlx::query!(
            "DELETE FROM notification_resource_bindings WHERE id=? AND organization_id=?",
            id,
            organization_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() != 0)
    }
}
