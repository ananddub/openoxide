use crate::db::models::notif_lark::NotifLark;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct NotifLarkRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl NotifLarkRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<NotifLark>, sqlx::Error> {
        sqlx::query_as!(
            NotifLark,
            r#"SELECT id AS "id?: i64", webhook_url AS "webhook_url: String" FROM notif_lark"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<NotifLark>, sqlx::Error> {
        sqlx::query_as!(
            NotifLark,
            r#"SELECT id AS "id?: i64", webhook_url AS "webhook_url: String" FROM notif_lark WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &NotifLark) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO notif_lark (webhook_url) VALUES (?)"#,
            &item.webhook_url
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &NotifLark) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE notif_lark SET webhook_url = ? WHERE id = ?"#,
            &item.webhook_url,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM notif_lark WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
