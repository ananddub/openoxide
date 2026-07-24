use crate::db::models::notif_mattermost::NotifMattermost;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct NotifMattermostRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl NotifMattermostRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<NotifMattermost>, sqlx::Error> {
        sqlx::query_as!(
            NotifMattermost,
            r#"SELECT id AS "id?: i64", webhook_url AS "webhook_url: String", channel AS "channel?: String", username AS "username?: String" FROM notif_mattermost"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<NotifMattermost>, sqlx::Error> {
        sqlx::query_as!(
            NotifMattermost,
            r#"SELECT id AS "id?: i64", webhook_url AS "webhook_url: String", channel AS "channel?: String", username AS "username?: String" FROM notif_mattermost WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &NotifMattermost) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO notif_mattermost (webhook_url, channel, username) VALUES (?, ?, ?)"#,
            &item.webhook_url,
            &item.channel,
            &item.username
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &NotifMattermost) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE notif_mattermost SET webhook_url = ?, channel = ?, username = ? WHERE id = ?"#,
            &item.webhook_url,
            &item.channel,
            &item.username,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM notif_mattermost WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
