use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::db::models::backup_executions::BackupExecution;
use crate::services::backup::types::{BackupKind, BackupOperation};

pub struct BackupExecutionRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl BackupExecutionRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn start(
        &self,
        backup_kind: BackupKind,
        operation: BackupOperation,
        backup_id: Option<i64>,
        object_key: Option<&str>,
    ) -> sqlx::Result<i64> {
        Ok(sqlx::query!(
            r#"INSERT INTO backup_executions
               (backup_kind, operation, backup_id, object_key, attempt)
               SELECT ?, ?, ?, ?, COALESCE(MAX(attempt), 0) + 1
               FROM backup_executions
               WHERE backup_kind = ? AND operation = ?
                 AND ((backup_id IS NULL AND ? IS NULL) OR backup_id = ?)"#,
            backup_kind.as_str(),
            operation.as_str(),
            backup_id,
            object_key,
            backup_kind.as_str(),
            operation.as_str(),
            backup_id,
            backup_id
        )
        .execute(self.pool.as_ref())
        .await?
        .last_insert_rowid())
    }

    pub async fn succeed(
        &self,
        id: i64,
        checksum_sha256: Option<&str>,
        size_bytes: Option<i64>,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            r#"UPDATE backup_executions
               SET status = 'SUCCEEDED', checksum_sha256 = ?, size_bytes = ?,
                   finished_at = strftime('%s', 'now')
               WHERE id = ?"#,
            checksum_sha256,
            size_bytes,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn fail(&self, id: i64, error: &str) -> sqlx::Result<()> {
        sqlx::query!(
            r#"UPDATE backup_executions
               SET status = 'FAILED', error = ?, finished_at = strftime('%s', 'now')
               WHERE id = ?"#,
            error,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn list(
        &self,
        backup_kind: Option<&str>,
        backup_id: Option<i64>,
        limit: i64,
    ) -> sqlx::Result<Vec<BackupExecution>> {
        sqlx::query_as!(
            BackupExecution,
            r#"SELECT id AS "id!: i64", backup_kind, operation, backup_id, status,
                      object_key, checksum_sha256, size_bytes, attempt, error,
                      started_at, finished_at
               FROM backup_executions
               WHERE (? IS NULL OR backup_kind = ?)
                 AND (? IS NULL OR backup_id = ?)
               ORDER BY started_at DESC, id DESC LIMIT ?"#,
            backup_kind,
            backup_kind,
            backup_id,
            backup_id,
            limit
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get(&self, id: i64) -> sqlx::Result<Option<BackupExecution>> {
        sqlx::query_as!(
            BackupExecution,
            r#"SELECT id AS "id!: i64", backup_kind, operation, backup_id, status,
                      object_key, checksum_sha256, size_bytes, attempt, error,
                      started_at, finished_at
               FROM backup_executions WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }
}
