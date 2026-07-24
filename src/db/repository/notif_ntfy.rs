use crate::db::models::notif_ntfy::NotifNtfy;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct NotifNtfyRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl NotifNtfyRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<NotifNtfy>, sqlx::Error> {
        sqlx::query_as!(
            NotifNtfy,
            r#"SELECT id AS "id?: i64", server_url AS "server_url: String", topic AS "topic: String", access_token AS "access_token?: String", priority AS "priority: i64" FROM notif_ntfy"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<NotifNtfy>, sqlx::Error> {
        sqlx::query_as!(
            NotifNtfy,
            r#"SELECT id AS "id?: i64", server_url AS "server_url: String", topic AS "topic: String", access_token AS "access_token?: String", priority AS "priority: i64" FROM notif_ntfy WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &NotifNtfy) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO notif_ntfy (server_url, topic, access_token, priority) VALUES (?, ?, ?, ?)"#,
            &item.server_url,
            &item.topic,
            &item.access_token,
            item.priority
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &NotifNtfy) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE notif_ntfy SET server_url = ?, topic = ?, access_token = ?, priority = ? WHERE id = ?"#,
            &item.server_url,
            &item.topic,
            &item.access_token,
            item.priority,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM notif_ntfy WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
