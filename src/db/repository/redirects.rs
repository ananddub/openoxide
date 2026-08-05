use crate::db::models::redirects::Redirect;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct RedirectRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl RedirectRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Redirect>, sqlx::Error> {
        sqlx::query_as!(
            Redirect,
            r#"SELECT id AS "id?: i64", regex AS "regex: String", replacement AS "replacement: String", permanent AS "permanent: i64", unique_config_key AS "unique_config_key?: i64", application_id AS "application_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM redirects"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Redirect>, sqlx::Error> {
        sqlx::query_as!(
            Redirect,
            r#"SELECT id AS "id?: i64", regex AS "regex: String", replacement AS "replacement: String", permanent AS "permanent: i64", unique_config_key AS "unique_config_key?: i64", application_id AS "application_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM redirects WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn list_by_application(
        &self,
        application_id: i64,
    ) -> Result<Vec<Redirect>, sqlx::Error> {
        sqlx::query_as!(
            Redirect,
            r#"SELECT id AS "id?: i64", regex AS "regex: String", replacement AS "replacement: String",
                      permanent AS "permanent: i64", unique_config_key AS "unique_config_key?: i64",
                      application_id AS "application_id: i64", created_at AS "created_at: i64",
                      updated_at AS "updated_at: i64"
               FROM redirects WHERE application_id = ? ORDER BY id"#,
            application_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn create_for_application(
        &self,
        application_id: i64,
        regex: &str,
        replacement: &str,
        permanent: i64,
        unique_config_key: Option<i64>,
    ) -> Result<Redirect, sqlx::Error> {
        let id = sqlx::query!(
            "INSERT INTO redirects (regex, replacement, permanent, unique_config_key, application_id) VALUES (?, ?, ?, ?, ?)",
            regex, replacement, permanent, unique_config_key, application_id
        ).execute(self.pool.as_ref()).await?.last_insert_rowid();
        self.get_by_id(id).await?.ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn update_for_application(
        &self,
        id: i64,
        application_id: i64,
        regex: &str,
        replacement: &str,
        permanent: i64,
        unique_config_key: Option<i64>,
    ) -> Result<Option<Redirect>, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE redirects SET regex = ?, replacement = ?, permanent = ?, unique_config_key = ? WHERE id = ? AND application_id = ?",
            regex, replacement, permanent, unique_config_key, id, application_id
        ).execute(self.pool.as_ref()).await?;
        if result.rows_affected() == 0 {
            return Ok(None);
        }
        self.get_by_id(id).await
    }

    pub async fn delete_for_application(
        &self,
        id: i64,
        application_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM redirects WHERE id = ? AND application_id = ?",
            id,
            application_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn create(&self, item: &Redirect) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO redirects (regex, replacement, permanent, unique_config_key, application_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
            &item.regex,
            &item.replacement,
            item.permanent,
            item.unique_config_key,
            item.application_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Redirect) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE redirects SET regex = ?, replacement = ?, permanent = ?, unique_config_key = ?, application_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.regex,
            &item.replacement,
            item.permanent,
            item.unique_config_key,
            item.application_id,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM redirects WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
