use crate::db::models::notif_telegram::NotifTelegram;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct NotifTelegramRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl NotifTelegramRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<NotifTelegram>, sqlx::Error> {
        sqlx::query_as!(
            NotifTelegram,
            r#"SELECT id AS "id?: i64", bot_token AS "bot_token: String", chat_id AS "chat_id: String", message_thread_id AS "message_thread_id?: String" FROM notif_telegram"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<NotifTelegram>, sqlx::Error> {
        sqlx::query_as!(
            NotifTelegram,
            r#"SELECT id AS "id?: i64", bot_token AS "bot_token: String", chat_id AS "chat_id: String", message_thread_id AS "message_thread_id?: String" FROM notif_telegram WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &NotifTelegram) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO notif_telegram (bot_token, chat_id, message_thread_id) VALUES (?, ?, ?)"#,
            &item.bot_token,
            &item.chat_id,
            &item.message_thread_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &NotifTelegram) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE notif_telegram SET bot_token = ?, chat_id = ?, message_thread_id = ? WHERE id = ?"#,
            &item.bot_token,
            &item.chat_id,
            &item.message_thread_id,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM notif_telegram WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
