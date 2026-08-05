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

    pub async fn list_by_application(
        &self,
        application_id: i64,
    ) -> Result<Vec<Patch>, sqlx::Error> {
        sqlx::query_as!(
            Patch,
            r#"SELECT id AS "id?: i64", patch_type AS "patch_type: String",
                      file_path AS "file_path: String", enabled AS "enabled: i64",
                      content AS "content: String", application_id AS "application_id?: i64",
                      compose_id AS "compose_id?: i64", created_at AS "created_at: i64",
                      updated_at AS "updated_at: i64"
               FROM patches WHERE application_id = ? ORDER BY file_path, id"#,
            application_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn list_by_compose(&self, compose_id: i64) -> Result<Vec<Patch>, sqlx::Error> {
        sqlx::query_as!(Patch, r#"SELECT id AS "id?: i64", patch_type AS "patch_type: String", file_path AS "file_path: String", enabled AS "enabled: i64", content AS "content: String", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM patches WHERE compose_id = ? ORDER BY file_path, id"#, compose_id)
            .fetch_all(self.pool.as_ref()).await
    }

    pub async fn get_for_compose(
        &self,
        id: i64,
        compose_id: i64,
    ) -> Result<Option<Patch>, sqlx::Error> {
        sqlx::query_as!(Patch, r#"SELECT id AS "id?: i64", patch_type AS "patch_type: String", file_path AS "file_path: String", enabled AS "enabled: i64", content AS "content: String", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM patches WHERE id = ? AND compose_id = ?"#, id, compose_id)
            .fetch_optional(self.pool.as_ref()).await
    }

    pub async fn create_for_compose(
        &self,
        compose_id: i64,
        patch_type: &str,
        file_path: &str,
        enabled: i64,
        content: &str,
    ) -> Result<Patch, sqlx::Error> {
        let id = sqlx::query!("INSERT INTO patches (patch_type, file_path, enabled, content, compose_id) VALUES (?, ?, ?, ?, ?)", patch_type, file_path, enabled, content, compose_id)
            .execute(self.pool.as_ref()).await?.last_insert_rowid();
        self.get_for_compose(id, compose_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn update_for_compose(
        &self,
        id: i64,
        compose_id: i64,
        patch_type: &str,
        file_path: &str,
        enabled: i64,
        content: &str,
    ) -> Result<Option<Patch>, sqlx::Error> {
        let result = sqlx::query!("UPDATE patches SET patch_type = ?, file_path = ?, enabled = ?, content = ? WHERE id = ? AND compose_id = ?", patch_type, file_path, enabled, content, id, compose_id)
            .execute(self.pool.as_ref()).await?;
        if result.rows_affected() == 0 {
            return Ok(None);
        }
        self.get_for_compose(id, compose_id).await
    }

    pub async fn delete_for_compose(&self, id: i64, compose_id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM patches WHERE id = ? AND compose_id = ?",
            id,
            compose_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn get_for_application(
        &self,
        id: i64,
        application_id: i64,
    ) -> Result<Option<Patch>, sqlx::Error> {
        sqlx::query_as!(Patch, r#"SELECT id AS "id?: i64", patch_type AS "patch_type: String", file_path AS "file_path: String", enabled AS "enabled: i64", content AS "content: String", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM patches WHERE id = ? AND application_id = ?"#, id, application_id)
            .fetch_optional(self.pool.as_ref()).await
    }

    pub async fn create_for_application(
        &self,
        application_id: i64,
        patch_type: &str,
        file_path: &str,
        enabled: i64,
        content: &str,
    ) -> Result<Patch, sqlx::Error> {
        let id = sqlx::query!("INSERT INTO patches (patch_type, file_path, enabled, content, application_id) VALUES (?, ?, ?, ?, ?)", patch_type, file_path, enabled, content, application_id)
            .execute(self.pool.as_ref()).await?.last_insert_rowid();
        self.get_for_application(id, application_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn update_for_application(
        &self,
        id: i64,
        application_id: i64,
        patch_type: &str,
        file_path: &str,
        enabled: i64,
        content: &str,
    ) -> Result<Option<Patch>, sqlx::Error> {
        let result = sqlx::query!("UPDATE patches SET patch_type = ?, file_path = ?, enabled = ?, content = ? WHERE id = ? AND application_id = ?", patch_type, file_path, enabled, content, id, application_id)
            .execute(self.pool.as_ref()).await?;
        if result.rows_affected() == 0 {
            return Ok(None);
        }
        self.get_for_application(id, application_id).await
    }

    pub async fn delete_for_application(
        &self,
        id: i64,
        application_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM patches WHERE id = ? AND application_id = ?",
            id,
            application_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() == 1)
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
