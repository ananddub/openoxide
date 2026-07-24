use crate::db::models::security::Security;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct SecurityRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl SecurityRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Security>, sqlx::Error> {
        sqlx::query_as!(
            Security,
            r#"SELECT id AS "id?: i64", username AS "username: String", password AS "password: String", application_id AS "application_id: i64", created_at AS "created_at: i64" FROM security"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Security>, sqlx::Error> {
        sqlx::query_as!(
            Security,
            r#"SELECT id AS "id?: i64", username AS "username: String", password AS "password: String", application_id AS "application_id: i64", created_at AS "created_at: i64" FROM security WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Security) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO security (username, password, application_id, created_at) VALUES (?, ?, ?, ?)"#,
            &item.username,
            &item.password,
            item.application_id,
            item.created_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Security) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE security SET username = ?, password = ?, application_id = ?, created_at = ? WHERE id = ?"#,
            &item.username,
            &item.password,
            item.application_id,
            item.created_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM security WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
