use crate::db::models::backups::Backup;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct BackupRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl BackupRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Backup>, sqlx::Error> {
        sqlx::query_as!(
            Backup,
            r#"SELECT id AS "id?: i64", app_name AS "app_name: String", schedule AS "schedule: String", enabled AS "enabled: i64", database_name AS "database_name: String", prefix AS "prefix: String", service_name AS "service_name?: String", keep_latest_count AS "keep_latest_count?: i64", backup_type AS "backup_type: String", database_type AS "database_type: String", metadata AS "metadata?: String", compose_id AS "compose_id?: i64", postgres_id AS "postgres_id?: i64", mysql_id AS "mysql_id?: i64", mariadb_id AS "mariadb_id?: i64", mongo_id AS "mongo_id?: i64", redis_id AS "redis_id?: i64", libsql_id AS "libsql_id?: i64", destination_id AS "destination_id: i64", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM backups"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Backup>, sqlx::Error> {
        sqlx::query_as!(
            Backup,
            r#"SELECT id AS "id?: i64", app_name AS "app_name: String", schedule AS "schedule: String", enabled AS "enabled: i64", database_name AS "database_name: String", prefix AS "prefix: String", service_name AS "service_name?: String", keep_latest_count AS "keep_latest_count?: i64", backup_type AS "backup_type: String", database_type AS "database_type: String", metadata AS "metadata?: String", compose_id AS "compose_id?: i64", postgres_id AS "postgres_id?: i64", mysql_id AS "mysql_id?: i64", mariadb_id AS "mariadb_id?: i64", mongo_id AS "mongo_id?: i64", redis_id AS "redis_id?: i64", libsql_id AS "libsql_id?: i64", destination_id AS "destination_id: i64", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM backups WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Backup) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO backups (app_name, schedule, enabled, database_name, prefix, service_name, keep_latest_count, backup_type, database_type, metadata, compose_id, postgres_id, mysql_id, mariadb_id, mongo_id, redis_id, libsql_id, destination_id, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.app_name,
            &item.schedule,
            item.enabled,
            &item.database_name,
            &item.prefix,
            &item.service_name,
            item.keep_latest_count,
            &item.backup_type,
            &item.database_type,
            &item.metadata,
            item.compose_id,
            item.postgres_id,
            item.mysql_id,
            item.mariadb_id,
            item.mongo_id,
            item.redis_id,
            item.libsql_id,
            item.destination_id,
            item.organization_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Backup) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE backups SET app_name = ?, schedule = ?, enabled = ?, database_name = ?, prefix = ?, service_name = ?, keep_latest_count = ?, backup_type = ?, database_type = ?, metadata = ?, compose_id = ?, postgres_id = ?, mysql_id = ?, mariadb_id = ?, mongo_id = ?, redis_id = ?, libsql_id = ?, destination_id = ?, organization_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.app_name,
            &item.schedule,
            item.enabled,
            &item.database_name,
            &item.prefix,
            &item.service_name,
            item.keep_latest_count,
            &item.backup_type,
            &item.database_type,
            &item.metadata,
            item.compose_id,
            item.postgres_id,
            item.mysql_id,
            item.mariadb_id,
            item.mongo_id,
            item.redis_id,
            item.libsql_id,
            item.destination_id,
            item.organization_id,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM backups WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
