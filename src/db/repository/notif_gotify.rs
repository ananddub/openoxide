use crate::db::models::notif_gotify::NotifGotify;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct NotifGotifyRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl NotifGotifyRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<NotifGotify>, sqlx::Error> {
        sqlx::query_as!(
            NotifGotify,
            r#"SELECT id AS "id?: i64", server_url AS "server_url: String", app_token AS "app_token: String", priority AS "priority: i64", decoration AS "decoration: i64" FROM notif_gotify"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<NotifGotify>, sqlx::Error> {
        sqlx::query_as!(
            NotifGotify,
            r#"SELECT id AS "id?: i64", server_url AS "server_url: String", app_token AS "app_token: String", priority AS "priority: i64", decoration AS "decoration: i64" FROM notif_gotify WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &NotifGotify) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO notif_gotify (server_url, app_token, priority, decoration) VALUES (?, ?, ?, ?)"#,
            &item.server_url,
            &item.app_token,
            item.priority,
            item.decoration
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &NotifGotify) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE notif_gotify SET server_url = ?, app_token = ?, priority = ?, decoration = ? WHERE id = ?"#,
            &item.server_url,
            &item.app_token,
            item.priority,
            item.decoration,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM notif_gotify WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
