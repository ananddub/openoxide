use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct MonitoringPolicy {
    pub organization_id: i64,
    pub desired_agent_version: Option<String>,
    pub retention_days: i64,
    pub updated_at: i64,
}

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct MaintenanceWindow {
    pub id: i64,
    pub organization_id: i64,
    pub server_id: Option<i64>,
    pub starts_at: i64,
    pub ends_at: i64,
    pub reason: String,
    pub created_at: i64,
}

pub struct MonitoringLifecycleRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl MonitoringLifecycleRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn policy(&self, organization_id: i64) -> sqlx::Result<MonitoringPolicy> {
        sqlx::query!(
            "INSERT INTO monitoring_policy (organization_id) VALUES (?) ON CONFLICT(organization_id) DO NOTHING",
            organization_id
        )
        .execute(self.pool.as_ref())
        .await?;
        sqlx::query_as!(
            MonitoringPolicy,
            r#"SELECT organization_id AS "organization_id!: i64", desired_agent_version, retention_days AS "retention_days!: i64", updated_at AS "updated_at!: i64" FROM monitoring_policy WHERE organization_id = ?"#,
            organization_id
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn update_policy(
        &self,
        organization_id: i64,
        desired_agent_version: Option<&str>,
        retention_days: i64,
    ) -> sqlx::Result<MonitoringPolicy> {
        sqlx::query!(
            "INSERT INTO monitoring_policy (organization_id, desired_agent_version, retention_days, updated_at) VALUES (?, ?, ?, unixepoch()) ON CONFLICT(organization_id) DO UPDATE SET desired_agent_version=excluded.desired_agent_version, retention_days=excluded.retention_days, updated_at=excluded.updated_at",
            organization_id,
            desired_agent_version,
            retention_days
        )
        .execute(self.pool.as_ref())
        .await?;
        self.policy(organization_id).await
    }

    pub async fn acknowledge_event(
        &self,
        event_id: i64,
        organization_id: i64,
        user_id: i64,
    ) -> sqlx::Result<bool> {
        let result = sqlx::query!(
            "UPDATE alert_events SET acknowledged_at=unixepoch(), acknowledged_by=? WHERE id=? AND organization_id=?",
            user_id,
            event_id,
            organization_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() != 0)
    }

    pub async fn silence_event(
        &self,
        event_id: i64,
        organization_id: i64,
        until: i64,
    ) -> sqlx::Result<bool> {
        let result = sqlx::query!(
            "UPDATE alert_events SET silenced_until=? WHERE id=? AND organization_id=?",
            until,
            event_id,
            organization_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() != 0)
    }

    pub async fn create_window(
        &self,
        organization_id: i64,
        server_id: Option<i64>,
        starts_at: i64,
        ends_at: i64,
        reason: &str,
    ) -> sqlx::Result<i64> {
        let result = sqlx::query!(
            "INSERT INTO monitoring_maintenance_windows (organization_id, server_id, starts_at, ends_at, reason) VALUES (?, ?, ?, ?, ?)",
            organization_id,
            server_id,
            starts_at,
            ends_at,
            reason
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn list_windows(&self, organization_id: i64) -> sqlx::Result<Vec<MaintenanceWindow>> {
        sqlx::query_as!(
            MaintenanceWindow,
            r#"SELECT id AS "id!: i64", organization_id AS "organization_id!: i64", server_id, starts_at AS "starts_at!: i64", ends_at AS "ends_at!: i64", reason AS "reason!: String", created_at AS "created_at!: i64" FROM monitoring_maintenance_windows WHERE organization_id=? ORDER BY starts_at DESC"#,
            organization_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn delete_window(&self, id: i64, organization_id: i64) -> sqlx::Result<bool> {
        let result = sqlx::query!(
            "DELETE FROM monitoring_maintenance_windows WHERE id=? AND organization_id=?",
            id,
            organization_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() != 0)
    }

    pub async fn is_in_maintenance(
        &self,
        organization_id: i64,
        server_id: i64,
    ) -> sqlx::Result<bool> {
        let active = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM monitoring_maintenance_windows WHERE organization_id=? AND (server_id IS NULL OR server_id=?) AND starts_at <= unixepoch() AND ends_at > unixepoch())",
            organization_id,
            server_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(active != 0)
    }
}
