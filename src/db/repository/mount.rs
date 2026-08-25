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

    pub async fn fetch_for_database_kind(
        &self,
        kind: crate::services::database::DatabaseKind,
        database_id: i64,
    ) -> sqlx::Result<Vec<Mount>> {
        let column = database_column(kind);
        let query = format!(
            "SELECT id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path, \
             postgres_id, mysql_id, mariadb_id, mongo_id, redis_id, libsql_id, compose_id, application_id, \
             created_at, updated_at FROM mounts WHERE {column} = ? ORDER BY id"
        );
        sqlx::query_as::<_, Mount>(sqlx::AssertSqlSafe(query))
            .bind(database_id)
            .fetch_all(self.pool.as_ref())
            .await
    }

    pub async fn get_for_database(
        &self,
        kind: crate::services::database::DatabaseKind,
        database_id: i64,
        id: i64,
    ) -> sqlx::Result<Option<Mount>> {
        let column = database_column(kind);
        let query = format!(
            "SELECT id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path, \
             postgres_id, mysql_id, mariadb_id, mongo_id, redis_id, libsql_id, compose_id, application_id, \
             created_at, updated_at FROM mounts WHERE id = ? AND {column} = ?"
        );
        sqlx::query_as::<_, Mount>(sqlx::AssertSqlSafe(query))
            .bind(id)
            .bind(database_id)
            .fetch_optional(self.pool.as_ref())
            .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create_for_database(
        &self,
        kind: crate::services::database::DatabaseKind,
        database_id: i64,
        mount_type: &str,
        host_path: Option<&str>,
        volume_name: Option<&str>,
        file_path: Option<&str>,
        content: Option<&str>,
        mount_path: &str,
    ) -> sqlx::Result<Mount> {
        let column = database_column(kind);
        let query = format!(
            "INSERT INTO mounts (mount_type, service_type, host_path, volume_name, file_path, content, mount_path, {column}) \
             VALUES (?, 'DATABASE', ?, ?, ?, ?, ?, ?)"
        );
        let id = sqlx::query(sqlx::AssertSqlSafe(query))
            .bind(mount_type)
            .bind(host_path)
            .bind(volume_name)
            .bind(file_path)
            .bind(content)
            .bind(mount_path)
            .bind(database_id)
            .execute(self.pool.as_ref())
            .await?
            .last_insert_rowid();
        self.get_for_database(kind, database_id, id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update_for_database(
        &self,
        kind: crate::services::database::DatabaseKind,
        database_id: i64,
        id: i64,
        mount_type: &str,
        host_path: Option<&str>,
        volume_name: Option<&str>,
        file_path: Option<&str>,
        content: Option<&str>,
        mount_path: &str,
    ) -> sqlx::Result<Option<Mount>> {
        let column = database_column(kind);
        let query = format!(
            "UPDATE mounts SET mount_type = ?, service_type = 'DATABASE', host_path = ?, volume_name = ?, \
             file_path = ?, content = ?, mount_path = ?, updated_at = strftime('%s', 'now') \
             WHERE id = ? AND {column} = ?"
        );
        let result = sqlx::query(sqlx::AssertSqlSafe(query))
            .bind(mount_type)
            .bind(host_path)
            .bind(volume_name)
            .bind(file_path)
            .bind(content)
            .bind(mount_path)
            .bind(id)
            .bind(database_id)
            .execute(self.pool.as_ref())
            .await?;
        if result.rows_affected() == 0 {
            return Ok(None);
        }
        self.get_for_database(kind, database_id, id).await
    }

    pub async fn delete_for_database(
        &self,
        kind: crate::services::database::DatabaseKind,
        database_id: i64,
        id: i64,
    ) -> sqlx::Result<bool> {
        let column = database_column(kind);
        let query = format!("DELETE FROM mounts WHERE id = ? AND {column} = ?");
        let result = sqlx::query(sqlx::AssertSqlSafe(query))
            .bind(id)
            .bind(database_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(result.rows_affected() == 1)
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

fn database_column(kind: crate::services::database::DatabaseKind) -> &'static str {
    use crate::services::database::DatabaseKind;
    match kind {
        DatabaseKind::Postgres => "postgres_id",
        DatabaseKind::Mysql => "mysql_id",
        DatabaseKind::Mariadb => "mariadb_id",
        DatabaseKind::Mongo => "mongo_id",
        DatabaseKind::Redis => "redis_id",
        DatabaseKind::Libsql => "libsql_id",
    }
}
