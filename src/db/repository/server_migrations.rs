use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ServerMigration {
    pub id: String,
    pub source_server_id: i64,
    pub target_server_id: i64,
    pub status: String,
    pub application_ids: String,
    pub build_application_ids: String,
    pub compose_ids: String,
    pub certificate_ids: String,
    pub schedule_ids: String,
    pub queued_applications: i64,
    pub queued_compose_projects: i64,
    pub error: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct ServerMigrationRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl ServerMigrationRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn begin(
        &self,
        id: &str,
        source: i64,
        target: i64,
        application_ids: &str,
        build_application_ids: &str,
        compose_ids: &str,
        certificate_ids: &str,
        schedule_ids: &str,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO server_migrations (id, source_server_id, target_server_id, status, application_ids, build_application_ids, compose_ids, certificate_ids, schedule_ids) VALUES (?, ?, ?, 'RUNNING', ?, ?, ?, ?, ?)",
            id, source, target, application_ids, build_application_ids, compose_ids, certificate_ids, schedule_ids
        ).execute(self.pool.as_ref()).await?;
        Ok(())
    }

    pub async fn finish(
        &self,
        id: &str,
        succeeded: bool,
        queued_applications: i64,
        queued_compose_projects: i64,
        error: Option<&str>,
    ) -> sqlx::Result<()> {
        let status = if succeeded { "SUCCEEDED" } else { "FAILED" };
        sqlx::query!(
            "UPDATE server_migrations SET status = ?, queued_applications = ?, queued_compose_projects = ?, error = ?, updated_at = unixepoch() WHERE id = ?",
            status, queued_applications, queued_compose_projects, error, id
        ).execute(self.pool.as_ref()).await?;
        Ok(())
    }

    pub async fn get(&self, id: &str) -> sqlx::Result<Option<ServerMigration>> {
        sqlx::query_as!(ServerMigration, r#"SELECT id, source_server_id, target_server_id, status, application_ids, build_application_ids, compose_ids, certificate_ids, schedule_ids, queued_applications, queued_compose_projects, error, created_at, updated_at FROM server_migrations WHERE id = ?"#, id)
            .fetch_optional(self.pool.as_ref()).await
    }

    pub async fn mark_rolled_back(&self, id: &str) -> sqlx::Result<()> {
        sqlx::query!("UPDATE server_migrations SET status = 'ROLLED_BACK', updated_at = unixepoch() WHERE id = ?", id)
            .execute(self.pool.as_ref()).await?;
        Ok(())
    }
}
