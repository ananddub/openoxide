use crate::db::models::ports::Port;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct PortRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl PortRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Port>, sqlx::Error> {
        sqlx::query_as!(
            Port,
            r#"SELECT id AS "id?: i64", published_port AS "published_port: i64", target_port AS "target_port: i64", protocol AS "protocol: String", publish_mode AS "publish_mode: String", application_id AS "application_id: i64", created_at AS "created_at: i64" FROM ports"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Port>, sqlx::Error> {
        sqlx::query_as!(
            Port,
            r#"SELECT id AS "id?: i64", published_port AS "published_port: i64", target_port AS "target_port: i64", protocol AS "protocol: String", publish_mode AS "publish_mode: String", application_id AS "application_id: i64", created_at AS "created_at: i64" FROM ports WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn list_by_application(&self, application_id: i64) -> Result<Vec<Port>, sqlx::Error> {
        sqlx::query_as!(
            Port,
            r#"SELECT id AS "id?: i64", published_port AS "published_port: i64",
                      target_port AS "target_port: i64", protocol AS "protocol: String",
                      publish_mode AS "publish_mode: String", application_id AS "application_id: i64",
                      created_at AS "created_at: i64"
               FROM ports WHERE application_id = ? ORDER BY id"#,
            application_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn create_for_application(
        &self,
        application_id: i64,
        published_port: i64,
        target_port: i64,
        protocol: &str,
        publish_mode: &str,
    ) -> Result<Port, sqlx::Error> {
        let id = sqlx::query!(
            r#"INSERT INTO ports (published_port, target_port, protocol, publish_mode, application_id)
               VALUES (?, ?, ?, ?, ?)"#,
            published_port,
            target_port,
            protocol,
            publish_mode,
            application_id
        )
        .execute(self.pool.as_ref())
        .await?
        .last_insert_rowid();
        self.get_by_id(id).await?.ok_or(sqlx::Error::RowNotFound)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update_for_application(
        &self,
        id: i64,
        application_id: i64,
        published_port: i64,
        target_port: i64,
        protocol: &str,
        publish_mode: &str,
    ) -> Result<Option<Port>, sqlx::Error> {
        let result = sqlx::query!(
            r#"UPDATE ports SET published_port = ?, target_port = ?, protocol = ?, publish_mode = ?
               WHERE id = ? AND application_id = ?"#,
            published_port,
            target_port,
            protocol,
            publish_mode,
            id,
            application_id
        )
        .execute(self.pool.as_ref())
        .await?;
        if result.rows_affected() == 0 {
            return Ok(None);
        }
        self.get_by_id(id).await
    }

    pub async fn delete_for_application(
        &self,
        id: i64,
        application_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM ports WHERE id = ? AND application_id = ?",
            id,
            application_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn create(&self, item: &Port) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO ports (published_port, target_port, protocol, publish_mode, application_id, created_at) VALUES (?, ?, ?, ?, ?, ?)"#,
            item.published_port,
            item.target_port,
            &item.protocol,
            &item.publish_mode,
            item.application_id,
            item.created_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Port) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE ports SET published_port = ?, target_port = ?, protocol = ?, publish_mode = ?, application_id = ?, created_at = ? WHERE id = ?"#,
            item.published_port,
            item.target_port,
            &item.protocol,
            &item.publish_mode,
            item.application_id,
            item.created_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM ports WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
