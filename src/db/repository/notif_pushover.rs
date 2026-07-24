use crate::db::models::notif_pushover::NotifPushover;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct NotifPushoverRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl NotifPushoverRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<NotifPushover>, sqlx::Error> {
        sqlx::query_as!(
            NotifPushover,
            r#"SELECT id AS "id?: i64", user_key AS "user_key: String", api_token AS "api_token: String", priority AS "priority: i64", retry AS "retry?: i64", expire AS "expire?: i64" FROM notif_pushover"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<NotifPushover>, sqlx::Error> {
        sqlx::query_as!(
            NotifPushover,
            r#"SELECT id AS "id?: i64", user_key AS "user_key: String", api_token AS "api_token: String", priority AS "priority: i64", retry AS "retry?: i64", expire AS "expire?: i64" FROM notif_pushover WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &NotifPushover) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO notif_pushover (user_key, api_token, priority, retry, expire) VALUES (?, ?, ?, ?, ?)"#,
            &item.user_key,
            &item.api_token,
            item.priority,
            item.retry,
            item.expire
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &NotifPushover) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE notif_pushover SET user_key = ?, api_token = ?, priority = ?, retry = ?, expire = ? WHERE id = ?"#,
            &item.user_key,
            &item.api_token,
            item.priority,
            item.retry,
            item.expire,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM notif_pushover WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
