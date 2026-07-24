use crate::db::models::mounts::Mount;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct MountRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl MountRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn fetch_for_database(&self, db_id: i64) -> sqlx::Result<Vec<Mount>> {
        sqlx::query_as!(
            Mount,
            r#"SELECT id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path,
               postgres_id, mysql_id, mariadb_id, mongo_id, redis_id, libsql_id, compose_id, application_id,
               created_at, updated_at
               FROM mounts
               WHERE postgres_id = ? OR mysql_id = ? OR mariadb_id = ? OR mongo_id = ? OR redis_id = ? OR libsql_id = ?"#,
            db_id, db_id, db_id, db_id, db_id, db_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn fetch_for_application(&self, application_id: i64) -> sqlx::Result<Vec<Mount>> {
        sqlx::query_as!(
            Mount,
            r#"SELECT id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path,
               postgres_id, mysql_id, mariadb_id, mongo_id, redis_id, libsql_id, compose_id, application_id,
               created_at, updated_at
               FROM mounts
               WHERE application_id = ?
               ORDER BY id"#,
            application_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn fetch_for_compose(&self, compose_id: i64) -> sqlx::Result<Vec<Mount>> {
        sqlx::query_as!(
            Mount,
            r#"SELECT id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path,
               postgres_id, mysql_id, mariadb_id, mongo_id, redis_id, libsql_id, compose_id, application_id,
               created_at, updated_at
               FROM mounts
               WHERE compose_id = ?
               ORDER BY id"#,
            compose_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }
}
