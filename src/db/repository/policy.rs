use crate::db::models::policy::Policy;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct PolicyRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl PolicyRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Policy>, sqlx::Error> {
        sqlx::query_as!(
            Policy,
            r#"SELECT id AS "id?: i64", action AS "action: String", created_at AS "created_at: i64" FROM policy"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Policy>, sqlx::Error> {
        sqlx::query_as!(
            Policy,
            r#"SELECT id AS "id?: i64", action AS "action: String", created_at AS "created_at: i64" FROM policy WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Policy) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO policy (action, created_at) VALUES (?, ?)"#,
            &item.action,
            item.created_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Policy) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE policy SET action = ?, created_at = ? WHERE id = ?"#,
            &item.action,
            item.created_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM policy WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
