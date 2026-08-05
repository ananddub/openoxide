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

    pub async fn get_for_application(
        &self,
        id: i64,
        application_id: i64,
    ) -> sqlx::Result<Option<Mount>> {
        sqlx::query_as!(
            Mount,
            r#"SELECT id, mount_type, service_type, host_path, volume_name, file_path, content,
                      mount_path, postgres_id, mysql_id, mariadb_id, mongo_id, redis_id,
                      libsql_id, compose_id, application_id, created_at, updated_at
               FROM mounts WHERE id = ? AND application_id = ?"#,
            id,
            application_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create_for_application(
        &self,
        application_id: i64,
        mount_type: &str,
        host_path: Option<&str>,
        volume_name: Option<&str>,
        file_path: Option<&str>,
        content: Option<&str>,
        mount_path: &str,
    ) -> sqlx::Result<Mount> {
        let id = sqlx::query!(
            r#"INSERT INTO mounts
               (mount_type, service_type, host_path, volume_name, file_path, content,
                mount_path, application_id)
               VALUES (?, 'APPLICATION', ?, ?, ?, ?, ?, ?)"#,
            mount_type,
            host_path,
            volume_name,
            file_path,
            content,
            mount_path,
            application_id
        )
        .execute(self.pool.as_ref())
        .await?
        .last_insert_rowid();
        self.get_for_application(id, application_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update_for_application(
        &self,
        id: i64,
        application_id: i64,
        mount_type: &str,
        host_path: Option<&str>,
        volume_name: Option<&str>,
        file_path: Option<&str>,
        content: Option<&str>,
        mount_path: &str,
    ) -> sqlx::Result<Option<Mount>> {
        let result = sqlx::query!(
            r#"UPDATE mounts
               SET mount_type = ?, service_type = 'APPLICATION', host_path = ?,
                   volume_name = ?, file_path = ?, content = ?, mount_path = ?
               WHERE id = ? AND application_id = ?"#,
            mount_type,
            host_path,
            volume_name,
            file_path,
            content,
            mount_path,
            id,
            application_id
        )
        .execute(self.pool.as_ref())
        .await?;
        if result.rows_affected() == 0 {
            return Ok(None);
        }
        self.get_for_application(id, application_id).await
    }

    pub async fn delete_for_application(&self, id: i64, application_id: i64) -> sqlx::Result<bool> {
        let result = sqlx::query!(
            "DELETE FROM mounts WHERE id = ? AND application_id = ?",
            id,
            application_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() == 1)
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

    pub async fn create_compose_file(
        &self,
        compose_id: i64,
        file_path: &str,
        content: &str,
    ) -> sqlx::Result<i64> {
        let result = sqlx::query!(
            r#"INSERT INTO mounts
               (mount_type, service_type, file_path, content, mount_path, compose_id)
               VALUES ('FILE', 'COMPOSE', ?, ?, '', ?)"#,
            file_path,
            content,
            compose_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get_for_compose(&self, id: i64, compose_id: i64) -> sqlx::Result<Option<Mount>> {
        sqlx::query_as!(Mount, r#"SELECT id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path, postgres_id, mysql_id, mariadb_id, mongo_id, redis_id, libsql_id, compose_id, application_id, created_at, updated_at FROM mounts WHERE id = ? AND compose_id = ?"#, id, compose_id)
            .fetch_optional(self.pool.as_ref()).await
    }

    pub async fn create_for_compose(
        &self,
        compose_id: i64,
        mount_type: &str,
        host_path: Option<&str>,
        volume_name: Option<&str>,
        file_path: Option<&str>,
        content: Option<&str>,
        mount_path: &str,
    ) -> sqlx::Result<Mount> {
        let id = sqlx::query!(r#"INSERT INTO mounts (mount_type, service_type, host_path, volume_name, file_path, content, mount_path, compose_id) VALUES (?, 'COMPOSE', ?, ?, ?, ?, ?, ?)"#, mount_type, host_path, volume_name, file_path, content, mount_path, compose_id)
            .execute(self.pool.as_ref()).await?.last_insert_rowid();
        self.get_for_compose(id, compose_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn update_for_compose(
        &self,
        id: i64,
        compose_id: i64,
        mount_type: &str,
        host_path: Option<&str>,
        volume_name: Option<&str>,
        file_path: Option<&str>,
        content: Option<&str>,
        mount_path: &str,
    ) -> sqlx::Result<Option<Mount>> {
        let result = sqlx::query!(r#"UPDATE mounts SET mount_type = ?, service_type = 'COMPOSE', host_path = ?, volume_name = ?, file_path = ?, content = ?, mount_path = ?, updated_at = strftime('%s', 'now') WHERE id = ? AND compose_id = ?"#, mount_type, host_path, volume_name, file_path, content, mount_path, id, compose_id)
            .execute(self.pool.as_ref()).await?;
        if result.rows_affected() == 0 {
            return Ok(None);
        }
        self.get_for_compose(id, compose_id).await
    }

    pub async fn delete_for_compose(&self, id: i64, compose_id: i64) -> sqlx::Result<bool> {
        let result = sqlx::query!(
            "DELETE FROM mounts WHERE id = ? AND compose_id = ?",
            id,
            compose_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() == 1)
    }
}
