use crate::db::models::notif_custom::NotifCustom;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct NotifCustomRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl NotifCustomRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<NotifCustom>, sqlx::Error> {
        sqlx::query_as!(
            NotifCustom,
            r#"SELECT id AS "id?: i64", endpoint AS "endpoint: String", headers AS "headers?: String" FROM notif_custom"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<NotifCustom>, sqlx::Error> {
        sqlx::query_as!(
            NotifCustom,
            r#"SELECT id AS "id?: i64", endpoint AS "endpoint: String", headers AS "headers?: String" FROM notif_custom WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &NotifCustom) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO notif_custom (endpoint, headers) VALUES (?, ?)"#,
            &item.endpoint,
            &item.headers
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &NotifCustom) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE notif_custom SET endpoint = ?, headers = ? WHERE id = ?"#,
            &item.endpoint,
            &item.headers,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM notif_custom WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
