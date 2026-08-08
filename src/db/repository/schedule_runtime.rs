use std::sync::Arc;

use crate::services::schedule::types::{ScheduleExecutionStatus, ScheduleTriggerKind};
use auto_di::singleton;
use sqlx::SqlitePool;

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct ScheduleRuntimePolicy {
    pub schedule_id: i64,
    pub retry_count: i64,
    pub retry_delay_seconds: i64,
    pub missed_run_policy: String,
    pub concurrency_policy: String,
    pub lease_seconds: i64,
    pub notify_on_success: i64,
    pub notify_on_failure: i64,
    pub last_scheduled_at: Option<i64>,
    pub updated_at: i64,
}

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct ScheduleExecution {
    pub id: i64,
    pub schedule_id: i64,
    pub organization_id: Option<i64>,
    pub trigger_kind: String,
    pub status: String,
    pub attempt: i64,
    pub scheduled_at: i64,
    pub started_at: i64,
    pub finished_at: Option<i64>,
    pub message: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

pub struct ScheduleRuntimeRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl ScheduleRuntimeRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn policy(&self, schedule_id: i64) -> sqlx::Result<ScheduleRuntimePolicy> {
        sqlx::query!("INSERT INTO schedule_runtime_policies (schedule_id) VALUES (?) ON CONFLICT(schedule_id) DO NOTHING", schedule_id)
            .execute(self.pool.as_ref()).await?;
        sqlx::query_as!(ScheduleRuntimePolicy,
            r#"SELECT schedule_id AS "schedule_id!: i64", retry_count AS "retry_count!: i64", retry_delay_seconds AS "retry_delay_seconds!: i64", missed_run_policy AS "missed_run_policy!: String", concurrency_policy AS "concurrency_policy!: String", lease_seconds AS "lease_seconds!: i64", notify_on_success AS "notify_on_success!: i64", notify_on_failure AS "notify_on_failure!: i64", last_scheduled_at, updated_at AS "updated_at!: i64" FROM schedule_runtime_policies WHERE schedule_id=?"#,
            schedule_id).fetch_one(self.pool.as_ref()).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update_policy(
        &self,
        schedule_id: i64,
        retry_count: i64,
        retry_delay_seconds: i64,
        missed_run_policy: &str,
        concurrency_policy: &str,
        lease_seconds: i64,
        notify_on_success: bool,
        notify_on_failure: bool,
    ) -> sqlx::Result<ScheduleRuntimePolicy> {
        sqlx::query!("INSERT INTO schedule_runtime_policies (schedule_id, retry_count, retry_delay_seconds, missed_run_policy, concurrency_policy, lease_seconds, notify_on_success, notify_on_failure, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, unixepoch()) ON CONFLICT(schedule_id) DO UPDATE SET retry_count=excluded.retry_count, retry_delay_seconds=excluded.retry_delay_seconds, missed_run_policy=excluded.missed_run_policy, concurrency_policy=excluded.concurrency_policy, lease_seconds=excluded.lease_seconds, notify_on_success=excluded.notify_on_success, notify_on_failure=excluded.notify_on_failure, updated_at=excluded.updated_at",
            schedule_id, retry_count, retry_delay_seconds, missed_run_policy, concurrency_policy, lease_seconds, notify_on_success, notify_on_failure)
            .execute(self.pool.as_ref()).await?;
        self.policy(schedule_id).await
    }

    pub async fn acquire(
        &self,
        schedule_id: i64,
        owner_id: &str,
        lease_seconds: i64,
    ) -> sqlx::Result<bool> {
        let now = chrono::Utc::now().timestamp();
        let expires = now + lease_seconds;
        let result = sqlx::query!("INSERT INTO schedule_execution_locks (schedule_id, owner_id, acquired_at, expires_at) VALUES (?, ?, ?, ?) ON CONFLICT(schedule_id) DO UPDATE SET owner_id=excluded.owner_id, acquired_at=excluded.acquired_at, expires_at=excluded.expires_at WHERE schedule_execution_locks.expires_at <= ?",
            schedule_id, owner_id, now, expires, now).execute(self.pool.as_ref()).await?;
        Ok(result.rows_affected() != 0)
    }

    pub async fn release(&self, schedule_id: i64, owner_id: &str) -> sqlx::Result<()> {
        sqlx::query!(
            "DELETE FROM schedule_execution_locks WHERE schedule_id=? AND owner_id=?",
            schedule_id,
            owner_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn mark_scheduled(&self, schedule_id: i64, timestamp: i64) -> sqlx::Result<()> {
        sqlx::query!("UPDATE schedule_runtime_policies SET last_scheduled_at=?, updated_at=unixepoch() WHERE schedule_id=?", timestamp, schedule_id)
            .execute(self.pool.as_ref()).await?;
        Ok(())
    }

    pub async fn begin(
        &self,
        schedule_id: i64,
        organization_id: Option<i64>,
        trigger_kind: ScheduleTriggerKind,
        attempt: i64,
        scheduled_at: i64,
    ) -> sqlx::Result<i64> {
        let result = sqlx::query!("INSERT INTO schedule_executions (schedule_id, organization_id, trigger_kind, status, attempt, scheduled_at) VALUES (?, ?, ?, 'RUNNING', ?, ?)", schedule_id, organization_id, trigger_kind.as_str(), attempt, scheduled_at)
            .execute(self.pool.as_ref()).await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn finish(
        &self,
        id: i64,
        status: ScheduleExecutionStatus,
        message: Option<&str>,
        stdout: Option<&str>,
        stderr: Option<&str>,
    ) -> sqlx::Result<()> {
        let schedule_id: i64 =
            sqlx::query_scalar("SELECT schedule_id FROM schedule_executions WHERE id=?")
                .bind(id)
                .fetch_one(self.pool.as_ref())
                .await?;
        sqlx::query!("UPDATE schedule_executions SET status=?, message=?, stdout=?, stderr=?, finished_at=unixepoch() WHERE id=?", status.as_str(), message, stdout, stderr, id)
            .execute(self.pool.as_ref()).await?;
        if let Err(error) = crate::services::schedule::file_log::append(
            &format!("schedule-{schedule_id}"),
            Some(id),
            status.as_str(),
            message,
            stdout,
            stderr,
        )
        .await
        {
            tracing::warn!(execution_id = id, %error, "could not write schedule execution log file");
        }
        Ok(())
    }

    pub async fn list(&self, schedule_id: i64, limit: i64) -> sqlx::Result<Vec<ScheduleExecution>> {
        sqlx::query_as!(ScheduleExecution,
            r#"SELECT id AS "id!: i64", schedule_id AS "schedule_id!: i64", organization_id, trigger_kind AS "trigger_kind!: String", status AS "status!: String", attempt AS "attempt!: i64", scheduled_at AS "scheduled_at!: i64", started_at AS "started_at!: i64", finished_at, message, stdout, stderr FROM schedule_executions WHERE schedule_id=? ORDER BY started_at DESC LIMIT ?"#,
            schedule_id, limit.clamp(1, 500)).fetch_all(self.pool.as_ref()).await
    }
}
