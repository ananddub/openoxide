use crate::db::models::notif_resend::NotifResend;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct NotifResendRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl NotifResendRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<NotifResend>, sqlx::Error> {
        sqlx::query_as!(
            NotifResend,
            r#"SELECT id AS "id?: i64", api_key AS "api_key: String", from_address AS "from_address: String", to_addresses AS "to_addresses: String" FROM notif_resend"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<NotifResend>, sqlx::Error> {
        sqlx::query_as!(
            NotifResend,
            r#"SELECT id AS "id?: i64", api_key AS "api_key: String", from_address AS "from_address: String", to_addresses AS "to_addresses: String" FROM notif_resend WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &NotifResend) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO notif_resend (api_key, from_address, to_addresses) VALUES (?, ?, ?)"#,
            &item.api_key,
            &item.from_address,
            &item.to_addresses
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &NotifResend) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE notif_resend SET api_key = ?, from_address = ?, to_addresses = ? WHERE id = ?"#,
            &item.api_key,
            &item.from_address,
            &item.to_addresses,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM notif_resend WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn find_for_user(&self, user_id: i64) -> Result<Option<NotifResend>, sqlx::Error> {
        sqlx::query_as!(
            NotifResend,
            r#"SELECT e.id AS "id?: i64", e.api_key AS "api_key: String", e.from_address AS "from_address: String", e.to_addresses AS "to_addresses: String"
               FROM notif_resend e
               JOIN notifications n ON n.resend_id = e.id
               JOIN organization o ON o.id = n.organization_id
               LEFT JOIN organization_members m ON m.organization_id = o.id AND m.user_id = ?
               WHERE o.owner_id = ? OR m.user_id IS NOT NULL
               ORDER BY n.id LIMIT 1"#,
            user_id,
            user_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }
}
