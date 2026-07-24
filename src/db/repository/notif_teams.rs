use crate::db::models::notif_teams::NotifTeam;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct NotifTeamRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl NotifTeamRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<NotifTeam>, sqlx::Error> {
        sqlx::query_as!(
            NotifTeam,
            r#"SELECT id AS "id?: i64", webhook_url AS "webhook_url: String" FROM notif_teams"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<NotifTeam>, sqlx::Error> {
        sqlx::query_as!(
            NotifTeam,
            r#"SELECT id AS "id?: i64", webhook_url AS "webhook_url: String" FROM notif_teams WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &NotifTeam) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO notif_teams (webhook_url) VALUES (?)"#,
            &item.webhook_url
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &NotifTeam) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE notif_teams SET webhook_url = ? WHERE id = ?"#,
            &item.webhook_url,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM notif_teams WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
