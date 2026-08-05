use crate::db::models::servers::Server;
use auto_di::singleton;
use sqlx::Row;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct ServerRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl ServerRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Server>, sqlx::Error> {
        sqlx::query_as!(
            Server,
            r#"SELECT id AS "id?: i64", name AS "name: String", description AS "description?: String", ip_address AS "ip_address: String", port AS "port: i64", username AS "username: String", app_name AS "app_name: String", server_status AS "server_status: String", server_type AS "server_type: String", enable_docker_cleanup AS "enable_docker_cleanup: i64", log_cleanup_cron AS "log_cleanup_cron?: String", command AS "command: String", metrics_config AS "metrics_config: String", ssh_key_id AS "ssh_key_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64", build_memory_limit AS "build_memory_limit?: String", build_cpu_limit AS "build_cpu_limit?: String" FROM servers"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Server>, sqlx::Error> {
        sqlx::query_as!(
            Server,
            r#"SELECT id AS "id?: i64", name AS "name: String", description AS "description?: String", ip_address AS "ip_address: String", port AS "port: i64", username AS "username: String", app_name AS "app_name: String", server_status AS "server_status: String", server_type AS "server_type: String", enable_docker_cleanup AS "enable_docker_cleanup: i64", log_cleanup_cron AS "log_cleanup_cron?: String", command AS "command: String", metrics_config AS "metrics_config: String", ssh_key_id AS "ssh_key_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64", build_memory_limit AS "build_memory_limit?: String", build_cpu_limit AS "build_cpu_limit?: String" FROM servers WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Server) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO servers (name, description, ip_address, port, username, app_name, server_status, server_type, enable_docker_cleanup, log_cleanup_cron, command, metrics_config, ssh_key_id, created_at, updated_at, build_memory_limit, build_cpu_limit) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.name,
            &item.description,
            &item.ip_address,
            item.port,
            &item.username,
            &item.app_name,
            &item.server_status,
            &item.server_type,
            item.enable_docker_cleanup,
            &item.log_cleanup_cron,
            &item.command,
            &item.metrics_config,
            item.ssh_key_id,
            item.created_at,
            item.updated_at,
            &item.build_memory_limit,
            &item.build_cpu_limit
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Server) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE servers SET name = ?, description = ?, ip_address = ?, port = ?, username = ?, app_name = ?, server_status = ?, server_type = ?, enable_docker_cleanup = ?, log_cleanup_cron = ?, command = ?, metrics_config = ?, ssh_key_id = ?, created_at = ?, updated_at = ?, build_memory_limit = ?, build_cpu_limit = ? WHERE id = ?"#,
            &item.name,
            &item.description,
            &item.ip_address,
            item.port,
            &item.username,
            &item.app_name,
            &item.server_status,
            &item.server_type,
            item.enable_docker_cleanup,
            &item.log_cleanup_cron,
            &item.command,
            &item.metrics_config,
            item.ssh_key_id,
            item.created_at,
            item.updated_at,
            &item.build_memory_limit,
            &item.build_cpu_limit,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM servers WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn get_ssh_credentials(
        &self,
        server_id: i64,
    ) -> Result<Option<(String, i64, String, String, String)>, sqlx::Error> {
        let res = sqlx::query(
            r#"SELECT COALESCE(CASE WHEN n.status = 'ACTIVE' AND n.connection_mode != 'DIRECT_SSH' THEN n.private_host END, s.ip_address) AS ip_address, s.port, s.username, k.private_key, k.public_key
               FROM servers s JOIN ssh_keys k ON k.id = s.ssh_key_id LEFT JOIN server_private_networks n ON n.server_id = s.id WHERE s.id = ?"#,
        )
        .bind(server_id)
        .fetch_optional(self.pool.as_ref())
        .await?;

        res.map(|row| {
            Ok((
                row.try_get("ip_address")?,
                row.try_get("port")?,
                row.try_get("username")?,
                row.try_get("private_key")?,
                row.try_get("public_key")?,
            ))
        })
        .transpose()
    }

    pub async fn get_direct_ssh_credentials(
        &self,
        server_id: i64,
    ) -> Result<Option<(String, i64, String, String, String)>, sqlx::Error> {
        let row = sqlx::query(
            "SELECT s.ip_address, s.port, s.username, k.private_key, k.public_key FROM servers s JOIN ssh_keys k ON k.id = s.ssh_key_id WHERE s.id = ?",
        )
        .bind(server_id)
        .fetch_optional(self.pool.as_ref())
        .await?;
        row.map(|row| {
            Ok((
                row.try_get("ip_address")?,
                row.try_get("port")?,
                row.try_get("username")?,
                row.try_get("private_key")?,
                row.try_get("public_key")?,
            ))
        })
        .transpose()
    }

    pub async fn list_ordered(&self) -> Result<Vec<Server>, sqlx::Error> {
        sqlx::query_as!(
            Server,
            r#"SELECT id AS "id?: i64", name AS "name: String", description AS "description?: String", ip_address AS "ip_address: String", port AS "port: i64", username AS "username: String", app_name AS "app_name: String", server_status AS "server_status: String", server_type AS "server_type: String", enable_docker_cleanup AS "enable_docker_cleanup: i64", log_cleanup_cron AS "log_cleanup_cron?: String", command AS "command: String", metrics_config AS "metrics_config: String", ssh_key_id AS "ssh_key_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64", build_memory_limit AS "build_memory_limit?: String", build_cpu_limit AS "build_cpu_limit?: String"
               FROM servers ORDER BY created_at DESC, id DESC"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn create_and_return(
        &self,
        name: String,
        description: Option<String>,
        ip_address: String,
        port: i64,
        username: String,
        app_name: String,
        server_type: String,
        ssh_key_id: Option<i64>,
        build_memory_limit: Option<String>,
        build_cpu_limit: Option<String>,
    ) -> Result<Server, sqlx::Error> {
        sqlx::query_as!(
            Server,
            r#"INSERT INTO servers
               (name, description, ip_address, port, username, app_name, server_type, ssh_key_id, build_memory_limit, build_cpu_limit)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING id AS "id?: i64", name AS "name: String", description AS "description?: String", ip_address AS "ip_address: String", port AS "port: i64", username AS "username: String", app_name AS "app_name: String", server_status AS "server_status: String", server_type AS "server_type: String", enable_docker_cleanup AS "enable_docker_cleanup: i64", log_cleanup_cron AS "log_cleanup_cron?: String", command AS "command: String", metrics_config AS "metrics_config: String", ssh_key_id AS "ssh_key_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64", build_memory_limit AS "build_memory_limit?: String", build_cpu_limit AS "build_cpu_limit?: String""#,
            name,
            description,
            ip_address,
            port,
            username,
            app_name,
            server_type,
            ssh_key_id,
            build_memory_limit,
            build_cpu_limit
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn update_and_return(
        &self,
        id: i64,
        name: String,
        description: Option<String>,
        ip_address: String,
        port: i64,
        username: String,
        server_status: String,
        server_type: String,
        enable_docker_cleanup: i64,
        log_cleanup_cron: Option<String>,
        command: String,
        metrics_config: String,
        ssh_key_id: Option<i64>,
        build_memory_limit: Option<String>,
        build_cpu_limit: Option<String>,
    ) -> Result<Server, sqlx::Error> {
        sqlx::query_as!(
            Server,
            r#"UPDATE servers SET
               name = ?, description = ?, ip_address = ?, port = ?, username = ?,
               server_status = ?, server_type = ?, enable_docker_cleanup = ?,
               log_cleanup_cron = ?, command = ?, metrics_config = ?, ssh_key_id = ?,
               build_memory_limit = ?, build_cpu_limit = ?
               WHERE id = ?
               RETURNING id AS "id?: i64", name AS "name: String", description AS "description?: String", ip_address AS "ip_address: String", port AS "port: i64", username AS "username: String", app_name AS "app_name: String", server_status AS "server_status: String", server_type AS "server_type: String", enable_docker_cleanup AS "enable_docker_cleanup: i64", log_cleanup_cron AS "log_cleanup_cron?: String", command AS "command: String", metrics_config AS "metrics_config: String", ssh_key_id AS "ssh_key_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64", build_memory_limit AS "build_memory_limit?: String", build_cpu_limit AS "build_cpu_limit?: String""#,
            name,
            description,
            ip_address,
            port,
            username,
            server_status,
            server_type,
            enable_docker_cleanup,
            log_cleanup_cron,
            command,
            metrics_config,
            ssh_key_id,
            build_memory_limit,
            build_cpu_limit,
            id
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn set_status(&self, id: i64, status: &str) -> Result<Server, sqlx::Error> {
        sqlx::query_as!(
            Server,
            r#"UPDATE servers SET server_status = ? WHERE id = ?
               RETURNING id AS "id?: i64", name AS "name: String", description AS "description?: String", ip_address AS "ip_address: String", port AS "port: i64", username AS "username: String", app_name AS "app_name: String", server_status AS "server_status: String", server_type AS "server_type: String", enable_docker_cleanup AS "enable_docker_cleanup: i64", log_cleanup_cron AS "log_cleanup_cron?: String", command AS "command: String", metrics_config AS "metrics_config: String", ssh_key_id AS "ssh_key_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64", build_memory_limit AS "build_memory_limit?: String", build_cpu_limit AS "build_cpu_limit?: String""#,
            status,
            id
        )
        .fetch_one(self.pool.as_ref())
        .await
    }
}
