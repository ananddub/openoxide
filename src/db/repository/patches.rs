use crate::db::models::patches::Patch;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct PatchRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl PatchRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Patch>, sqlx::Error> {
        sqlx::query_as!(
            Patch,
            r#"SELECT id AS "id?: i64", patch_type AS "patch_type: String", file_path AS "file_path: String", enabled AS "enabled: i64", content AS "content: String", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM patches"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Patch>, sqlx::Error> {
        sqlx::query_as!(
            Patch,
            r#"SELECT id AS "id?: i64", patch_type AS "patch_type: String", file_path AS "file_path: String", enabled AS "enabled: i64", content AS "content: String", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM patches WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Patch) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO patches (patch_type, file_path, enabled, content, application_id, compose_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.patch_type,
            &item.file_path,
            item.enabled,
            &item.content,
            item.application_id,
            item.compose_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Patch) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE patches SET patch_type = ?, file_path = ?, enabled = ?, content = ?, application_id = ?, compose_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.patch_type,
            &item.file_path,
            item.enabled,
            &item.content,
            item.application_id,
            item.compose_id,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM patches WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
