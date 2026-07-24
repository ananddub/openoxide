use crate::db::models::volume_backups::VolumeBackup;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct VolumeBackupRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl VolumeBackupRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<VolumeBackup>, sqlx::Error> {
        sqlx::query_as!(
            VolumeBackup,
            r#"SELECT id AS "id?: i64", name AS "name: String", volume_name AS "volume_name: String", prefix AS "prefix: String", service_type AS "service_type: String", app_name AS "app_name: String", service_name AS "service_name?: String", turn_off AS "turn_off: i64", cron_expression AS "cron_expression: String", keep_latest_count AS "keep_latest_count?: i64", enabled AS "enabled: i64", destination_id AS "destination_id: i64", organization_id AS "organization_id: i64", application_id AS "application_id?: i64", postgres_id AS "postgres_id?: i64", mysql_id AS "mysql_id?: i64", mariadb_id AS "mariadb_id?: i64", mongo_id AS "mongo_id?: i64", redis_id AS "redis_id?: i64", libsql_id AS "libsql_id?: i64", compose_id AS "compose_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM volume_backups"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<VolumeBackup>, sqlx::Error> {
        sqlx::query_as!(
            VolumeBackup,
            r#"SELECT id AS "id?: i64", name AS "name: String", volume_name AS "volume_name: String", prefix AS "prefix: String", service_type AS "service_type: String", app_name AS "app_name: String", service_name AS "service_name?: String", turn_off AS "turn_off: i64", cron_expression AS "cron_expression: String", keep_latest_count AS "keep_latest_count?: i64", enabled AS "enabled: i64", destination_id AS "destination_id: i64", organization_id AS "organization_id: i64", application_id AS "application_id?: i64", postgres_id AS "postgres_id?: i64", mysql_id AS "mysql_id?: i64", mariadb_id AS "mariadb_id?: i64", mongo_id AS "mongo_id?: i64", redis_id AS "redis_id?: i64", libsql_id AS "libsql_id?: i64", compose_id AS "compose_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM volume_backups WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &VolumeBackup) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO volume_backups (name, volume_name, prefix, service_type, app_name, service_name, turn_off, cron_expression, keep_latest_count, enabled, destination_id, organization_id, application_id, postgres_id, mysql_id, mariadb_id, mongo_id, redis_id, libsql_id, compose_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.name,
            &item.volume_name,
            &item.prefix,
            &item.service_type,
            &item.app_name,
            &item.service_name,
            item.turn_off,
            &item.cron_expression,
            item.keep_latest_count,
            item.enabled,
            item.destination_id,
            item.organization_id,
            item.application_id,
            item.postgres_id,
            item.mysql_id,
            item.mariadb_id,
            item.mongo_id,
            item.redis_id,
            item.libsql_id,
            item.compose_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &VolumeBackup) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE volume_backups SET name = ?, volume_name = ?, prefix = ?, service_type = ?, app_name = ?, service_name = ?, turn_off = ?, cron_expression = ?, keep_latest_count = ?, enabled = ?, destination_id = ?, organization_id = ?, application_id = ?, postgres_id = ?, mysql_id = ?, mariadb_id = ?, mongo_id = ?, redis_id = ?, libsql_id = ?, compose_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.name,
            &item.volume_name,
            &item.prefix,
            &item.service_type,
            &item.app_name,
            &item.service_name,
            item.turn_off,
            &item.cron_expression,
            item.keep_latest_count,
            item.enabled,
            item.destination_id,
            item.organization_id,
            item.application_id,
            item.postgres_id,
            item.mysql_id,
            item.mariadb_id,
            item.mongo_id,
            item.redis_id,
            item.libsql_id,
            item.compose_id,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM volume_backups WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
