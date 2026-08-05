use crate::db::models::rollbacks::Rollback;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct RollbackRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl RollbackRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Rollback>, sqlx::Error> {
        sqlx::query_as!(
            Rollback,
            r#"SELECT id AS "id?: i64", deployment_id AS "deployment_id: i64", version AS "version: i64", image AS "image?: String", full_context AS "full_context?: String", created_at AS "created_at: i64" FROM rollbacks"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Rollback>, sqlx::Error> {
        sqlx::query_as!(
            Rollback,
            r#"SELECT id AS "id?: i64", deployment_id AS "deployment_id: i64", version AS "version: i64", image AS "image?: String", full_context AS "full_context?: String", created_at AS "created_at: i64" FROM rollbacks WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Rollback) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO rollbacks (deployment_id, version, image, full_context, created_at) VALUES (?, ?, ?, ?, ?)"#,
            item.deployment_id,
            item.version,
            &item.image,
            &item.full_context,
            item.created_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Rollback) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE rollbacks SET deployment_id = ?, version = ?, image = ?, full_context = ?, created_at = ? WHERE id = ?"#,
            item.deployment_id,
            item.version,
            &item.image,
            &item.full_context,
            item.created_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM rollbacks WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn get_by_deployment_id(
        &self,
        deployment_id: i64,
    ) -> Result<Option<Rollback>, sqlx::Error> {
        sqlx::query_as!(
            Rollback,
            r#"SELECT id AS "id?: i64", deployment_id AS "deployment_id: i64", version AS "version: i64", image AS "image?: String", full_context AS "full_context?: String", created_at AS "created_at: i64" FROM rollbacks WHERE deployment_id = ?"#,
            deployment_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    /// List all rollbacks for an application (via its deployments), newest first
    pub async fn list_by_application(
        &self,
        application_id: i64,
    ) -> Result<Vec<Rollback>, sqlx::Error> {
        sqlx::query_as!(
            Rollback,
            r#"SELECT r.id AS "id?: i64", r.deployment_id AS "deployment_id: i64", r.version AS "version: i64", r.image AS "image?: String", r.full_context AS "full_context?: String", r.created_at AS "created_at: i64"
               FROM rollbacks r
               INNER JOIN deployments d ON d.id = r.deployment_id
               WHERE d.application_id = ?
               ORDER BY r.version DESC"#,
            application_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    /// Get the next version number for a given application's rollbacks
    pub async fn get_next_version_for_application(
        &self,
        application_id: i64,
    ) -> Result<i64, sqlx::Error> {
        let row = sqlx::query_scalar!(
            r#"SELECT COALESCE(MAX(r.version), 0) AS "max_version: i64"
               FROM rollbacks r
               INNER JOIN deployments d ON d.id = r.deployment_id
               WHERE d.application_id = ?"#,
            application_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row + 1)
    }

    pub async fn get_for_application(
        &self,
        id: i64,
        application_id: i64,
    ) -> Result<Option<Rollback>, sqlx::Error> {
        sqlx::query_as!(
            Rollback,
            r#"SELECT r.id AS "id?: i64", r.deployment_id AS "deployment_id: i64",
                      r.version AS "version: i64", r.image AS "image?: String",
                      r.full_context AS "full_context?: String", r.created_at AS "created_at: i64"
               FROM rollbacks r
               INNER JOIN deployments d ON d.id = r.deployment_id
               WHERE r.id = ? AND d.application_id = ?"#,
            id,
            application_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn delete_for_application(
        &self,
        id: i64,
        application_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"DELETE FROM rollbacks
               WHERE id = ? AND deployment_id IN (
                   SELECT id FROM deployments WHERE application_id = ?
               )"#,
            id,
            application_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() == 1)
    }
}
