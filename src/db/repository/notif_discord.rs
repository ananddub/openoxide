use crate::db::models::notif_discord::NotifDiscord;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct NotifDiscordRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl NotifDiscordRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<NotifDiscord>, sqlx::Error> {
        sqlx::query_as!(
            NotifDiscord,
            r#"SELECT id AS "id?: i64", webhook_url AS "webhook_url: String", decoration AS "decoration: i64" FROM notif_discord"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<NotifDiscord>, sqlx::Error> {
        sqlx::query_as!(
            NotifDiscord,
            r#"SELECT id AS "id?: i64", webhook_url AS "webhook_url: String", decoration AS "decoration: i64" FROM notif_discord WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &NotifDiscord) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO notif_discord (webhook_url, decoration) VALUES (?, ?)"#,
            &item.webhook_url,
            item.decoration
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &NotifDiscord) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE notif_discord SET webhook_url = ?, decoration = ? WHERE id = ?"#,
            &item.webhook_url,
            item.decoration,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM notif_discord WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
