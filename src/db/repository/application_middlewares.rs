use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::db::models::application_middlewares::ApplicationMiddleware;

pub struct ApplicationMiddlewareRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl ApplicationMiddlewareRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn list_by_application(
        &self,
        application_id: i64,
    ) -> sqlx::Result<Vec<ApplicationMiddleware>> {
        sqlx::query_as!(ApplicationMiddleware, r#"SELECT id AS "id!: i64", application_id AS "application_id!: i64", name AS "name!: String", middleware_type AS "middleware_type!: String", enabled AS "enabled!: i64", config AS "config!: String", created_at AS "created_at!: i64", updated_at AS "updated_at!: i64" FROM application_middlewares WHERE application_id = ? ORDER BY name, id"#, application_id)
            .fetch_all(self.pool.as_ref()).await
    }

    pub async fn get_for_application(
        &self,
        id: i64,
        application_id: i64,
    ) -> sqlx::Result<Option<ApplicationMiddleware>> {
        sqlx::query_as!(ApplicationMiddleware, r#"SELECT id AS "id!: i64", application_id AS "application_id!: i64", name AS "name!: String", middleware_type AS "middleware_type!: String", enabled AS "enabled!: i64", config AS "config!: String", created_at AS "created_at!: i64", updated_at AS "updated_at!: i64" FROM application_middlewares WHERE id = ? AND application_id = ?"#, id, application_id)
            .fetch_optional(self.pool.as_ref()).await
    }

    pub async fn create(
        &self,
        application_id: i64,
        name: &str,
        middleware_type: &str,
        enabled: i64,
        config: &str,
    ) -> sqlx::Result<ApplicationMiddleware> {
        let id = sqlx::query!("INSERT INTO application_middlewares (application_id, name, middleware_type, enabled, config) VALUES (?, ?, ?, ?, ?)", application_id, name, middleware_type, enabled, config).execute(self.pool.as_ref()).await?.last_insert_rowid();
        self.get_for_application(id, application_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn update(
        &self,
        id: i64,
        application_id: i64,
        name: &str,
        middleware_type: &str,
        enabled: i64,
        config: &str,
    ) -> sqlx::Result<Option<ApplicationMiddleware>> {
        let result = sqlx::query!("UPDATE application_middlewares SET name = ?, middleware_type = ?, enabled = ?, config = ? WHERE id = ? AND application_id = ?", name, middleware_type, enabled, config, id, application_id).execute(self.pool.as_ref()).await?;
        if result.rows_affected() == 0 {
            return Ok(None);
        }
        self.get_for_application(id, application_id).await
    }

    pub async fn delete(&self, id: i64, application_id: i64) -> sqlx::Result<bool> {
        let result = sqlx::query!(
            "DELETE FROM application_middlewares WHERE id = ? AND application_id = ?",
            id,
            application_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() == 1)
    }
}
