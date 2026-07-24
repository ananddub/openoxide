use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::db::models::resource_access::ResourceAccess;

pub struct ResourceAccessRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl ResourceAccessRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn check_access(
        &self,
        user_id: i64,
        org_id: i64,
        resource_type: &str,
        resource_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM resource_access 
            WHERE user_id = ? AND org_id = ? AND resource_type = ? AND resource_id = ?
            "#,
        )
        .bind(user_id)
        .bind(org_id)
        .bind(resource_type)
        .bind(resource_id)
        .fetch_one(&*self.pool)
        .await?;

        Ok(count > 0)
    }

    pub async fn grant_access(
        &self,
        user_id: i64,
        org_id: i64,
        resource_type: &str,
        resource_id: i64,
    ) -> Result<ResourceAccess, sqlx::Error> {
        sqlx::query_as::<_, ResourceAccess>(
            r#"
            INSERT INTO resource_access (user_id, org_id, resource_type, resource_id)
            VALUES (?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(org_id)
        .bind(resource_type)
        .bind(resource_id)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn revoke_access(
        &self,
        user_id: i64,
        org_id: i64,
        resource_type: &str,
        resource_id: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM resource_access WHERE user_id = ? AND org_id = ? AND resource_type = ? AND resource_id = ?"
        )
        .bind(user_id)
        .bind(org_id)
        .bind(resource_type)
        .bind(resource_id)
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}
