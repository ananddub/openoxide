use crate::db::models::audit_logs::AuditLog;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct AuditLogRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl AuditLogRepository {
    pub async fn list_filtered(
        &self,
        organization_id: i64,
        action: Option<&str>,
        resource_type: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AuditLog>, sqlx::Error> {
        sqlx::query_as::<_, AuditLog>(
            "SELECT id, user_email, user_role, action, resource_type, resource_id, resource_name, metadata, organization_id, user_id, created_at FROM audit_logs WHERE organization_id = ? AND (? IS NULL OR action = ?) AND (? IS NULL OR resource_type = ?) ORDER BY created_at DESC, id DESC LIMIT ? OFFSET ?",
        )
        .bind(organization_id).bind(action).bind(action).bind(resource_type).bind(resource_type).bind(limit.clamp(1, 500)).bind(offset.max(0)).fetch_all(self.pool.as_ref()).await
    }

    pub async fn count_filtered(
        &self,
        organization_id: i64,
        action: Option<&str>,
        resource_type: Option<&str>,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar("SELECT COUNT(*) FROM audit_logs WHERE organization_id = ? AND (? IS NULL OR action = ?) AND (? IS NULL OR resource_type = ?)")
            .bind(organization_id).bind(action).bind(action).bind(resource_type).bind(resource_type).fetch_one(self.pool.as_ref()).await
    }
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<AuditLog>, sqlx::Error> {
        sqlx::query_as!(
            AuditLog,
            r#"SELECT id AS "id?: i64", user_email AS "user_email: String", user_role AS "user_role: String", action AS "action: String", resource_type AS "resource_type: String", resource_id AS "resource_id?: String", resource_name AS "resource_name?: String", metadata AS "metadata?: String", organization_id AS "organization_id?: i64", user_id AS "user_id?: i64", created_at AS "created_at: i64" FROM audit_logs"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<AuditLog>, sqlx::Error> {
        sqlx::query_as!(
            AuditLog,
            r#"SELECT id AS "id?: i64", user_email AS "user_email: String", user_role AS "user_role: String", action AS "action: String", resource_type AS "resource_type: String", resource_id AS "resource_id?: String", resource_name AS "resource_name?: String", metadata AS "metadata?: String", organization_id AS "organization_id?: i64", user_id AS "user_id?: i64", created_at AS "created_at: i64" FROM audit_logs WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &AuditLog) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO audit_logs (user_email, user_role, action, resource_type, resource_id, resource_name, metadata, organization_id, user_id, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.user_email,
            &item.user_role,
            &item.action,
            &item.resource_type,
            &item.resource_id,
            &item.resource_name,
            &item.metadata,
            item.organization_id,
            item.user_id,
            item.created_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &AuditLog) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE audit_logs SET user_email = ?, user_role = ?, action = ?, resource_type = ?, resource_id = ?, resource_name = ?, metadata = ?, organization_id = ?, user_id = ?, created_at = ? WHERE id = ?"#,
            &item.user_email,
            &item.user_role,
            &item.action,
            &item.resource_type,
            &item.resource_id,
            &item.resource_name,
            &item.metadata,
            item.organization_id,
            item.user_id,
            item.created_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM audit_logs WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
