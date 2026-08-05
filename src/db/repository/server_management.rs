use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::db::models::server_management::{ServerCleanupExecution, ServerManagement};

pub struct ServerManagementRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl ServerManagementRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_or_create(&self, server_id: i64) -> sqlx::Result<ServerManagement> {
        sqlx::query!(
            "INSERT INTO server_management (server_id) VALUES (?) ON CONFLICT(server_id) DO NOTHING",
            server_id
        )
        .execute(self.pool.as_ref())
        .await?;
        sqlx::query_as!(
            ServerManagement,
            r#"SELECT server_id AS "server_id!: i64", maintenance_mode, maintenance_message,
                      labels, cleanup_policy, gpu_enabled, updated_at
               FROM server_management WHERE server_id = ?"#,
            server_id
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn update(
        &self,
        server_id: i64,
        maintenance_mode: i64,
        maintenance_message: Option<&str>,
        labels: &str,
        cleanup_policy: &str,
        gpu_enabled: i64,
    ) -> sqlx::Result<ServerManagement> {
        self.get_or_create(server_id).await?;
        sqlx::query!(
            r#"UPDATE server_management SET maintenance_mode = ?, maintenance_message = ?,
               labels = ?, cleanup_policy = ?, gpu_enabled = ?, updated_at = strftime('%s', 'now')
               WHERE server_id = ?"#,
            maintenance_mode,
            maintenance_message,
            labels,
            cleanup_policy,
            gpu_enabled,
            server_id
        )
        .execute(self.pool.as_ref())
        .await?;
        self.get_or_create(server_id).await
    }

    pub async fn assert_deployable(&self, server_id: Option<i64>) -> sqlx::Result<()> {
        let Some(server_id) = server_id else {
            return Ok(());
        };
        let row = self.get_or_create(server_id).await?;
        if row.maintenance_mode != 0 {
            return Err(sqlx::Error::Protocol(
                row.maintenance_message
                    .unwrap_or_else(|| "server is in maintenance mode".into()),
            ));
        }
        Ok(())
    }

    pub async fn start_cleanup(&self, server_id: i64, policy: &str) -> sqlx::Result<i64> {
        Ok(sqlx::query!(
            "INSERT INTO server_cleanup_executions (server_id, policy) VALUES (?, ?)",
            server_id,
            policy
        )
        .execute(self.pool.as_ref())
        .await?
        .last_insert_rowid())
    }

    pub async fn finish_cleanup(
        &self,
        id: i64,
        success: bool,
        stdout: &str,
        stderr: &str,
    ) -> sqlx::Result<()> {
        let status = if success { "SUCCEEDED" } else { "FAILED" };
        sqlx::query!(
            r#"UPDATE server_cleanup_executions SET status = ?, stdout = ?, stderr = ?,
               finished_at = strftime('%s', 'now') WHERE id = ?"#,
            status,
            stdout,
            stderr,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn list_cleanup(&self, server_id: i64) -> sqlx::Result<Vec<ServerCleanupExecution>> {
        sqlx::query_as!(
            ServerCleanupExecution,
            r#"SELECT id AS "id!: i64", server_id, status, policy, stdout, stderr,
                      started_at, finished_at
               FROM server_cleanup_executions WHERE server_id = ?
               ORDER BY started_at DESC, id DESC LIMIT 100"#,
            server_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }
}
