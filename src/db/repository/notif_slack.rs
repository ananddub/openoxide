use crate::db::models::notif_slack::NotifSlack;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct NotifSlackRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl NotifSlackRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<NotifSlack>, sqlx::Error> {
        sqlx::query_as!(
            NotifSlack,
            r#"SELECT id AS "id?: i64", webhook_url AS "webhook_url: String", channel AS "channel?: String" FROM notif_slack"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<NotifSlack>, sqlx::Error> {
        sqlx::query_as!(
            NotifSlack,
            r#"SELECT id AS "id?: i64", webhook_url AS "webhook_url: String", channel AS "channel?: String" FROM notif_slack WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &NotifSlack) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO notif_slack (webhook_url, channel) VALUES (?, ?)"#,
            &item.webhook_url,
            &item.channel
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &NotifSlack) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE notif_slack SET webhook_url = ?, channel = ? WHERE id = ?"#,
            &item.webhook_url,
            &item.channel,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM notif_slack WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
