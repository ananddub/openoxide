use crate::db::models::servers::Server;
use auto_di::singleton;
use sqlx::Row;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct ServerRepository {
    pool: Arc<SqlitePool>,
}

#[derive(Debug, Clone, Default)]
pub struct ServerDependencyCounts {
    pub applications: i64,
    pub build_assignments: i64,
    pub compose_projects: i64,
    pub databases: i64,
    pub certificates: i64,
    pub schedules: i64,
}

impl ServerDependencyCounts {
    pub fn total(&self) -> i64 {
        self.applications
            + self.build_assignments
            + self.compose_projects
            + self.databases
            + self.certificates
            + self.schedules
    }
}

#[cfg(test)]
mod dependency_tests {
    use super::ServerDependencyCounts;
    #[test]
    fn totals_every_blocking_dependency() {
        let counts = ServerDependencyCounts {
            applications: 1,
            build_assignments: 2,
            compose_projects: 3,
            databases: 4,
            certificates: 5,
            schedules: 6,
        };
        assert_eq!(counts.total(), 21);
    }
}

#[singleton]
impl ServerRepository {
    pub async fn migrate_dependencies(
        &self,
        source: i64,
        target: i64,
    ) -> Result<ServerDependencyCounts, sqlx::Error> {
        let counts = self.dependency_counts(source).await?;
        let mut transaction = self.pool.begin().await?;
        sqlx::query!(
            "UPDATE applications SET server_id = ? WHERE server_id = ?",
            target,
            source
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query!(
            "UPDATE applications SET build_server_id = ? WHERE build_server_id = ?",
            target,
            source
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query!(
            "UPDATE compose_projects SET server_id = ? WHERE server_id = ?",
            target,
            source
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query!(
            "UPDATE certificates SET server_id = ? WHERE server_id = ?",
            target,
            source
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query!(
            "UPDATE schedules SET server_id = ? WHERE server_id = ?",
            target,
            source
        )
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        Ok(counts)
    }

    pub async fn migratable_resource_ids(
        &self,
        server_id: i64,
    ) -> sqlx::Result<(Vec<i64>, Vec<i64>, Vec<i64>, Vec<i64>, Vec<i64>)> {
        let applications = sqlx::query_scalar!(
            r#"SELECT id AS "id!: i64" FROM applications WHERE server_id = ? ORDER BY id"#,
            server_id
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        let compose = sqlx::query_scalar!(
            r#"SELECT id AS "id!: i64" FROM compose_projects WHERE server_id = ? ORDER BY id"#,
            server_id
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        let build_applications = sqlx::query_scalar!(
            r#"SELECT id AS "id!: i64" FROM applications WHERE build_server_id = ? ORDER BY id"#,
            server_id
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        let certificates = sqlx::query_scalar!(
            r#"SELECT id AS "id!: i64" FROM certificates WHERE server_id = ? ORDER BY id"#,
            server_id
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        let schedules = sqlx::query_scalar!(
            r#"SELECT id AS "id!: i64" FROM schedules WHERE server_id = ? ORDER BY id"#,
            server_id
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok((
            applications,
            build_applications,
            compose,
            certificates,
            schedules,
        ))
    }

    pub async fn rollback_migrated_resources(
        &self,
        source: i64,
        target: i64,
        applications: &[i64],
        build_applications: &[i64],
        compose: &[i64],
        certificates: &[i64],
        schedules: &[i64],
    ) -> sqlx::Result<()> {
        let mut transaction = self.pool.begin().await?;
        for id in applications {
            sqlx::query!(
                "UPDATE applications SET server_id = ? WHERE id = ? AND server_id = ?",
                source,
                id,
                target
            )
            .execute(&mut *transaction)
            .await?;
        }
        for id in compose {
            sqlx::query!(
                "UPDATE compose_projects SET server_id = ? WHERE id = ? AND server_id = ?",
                source,
                id,
                target
            )
            .execute(&mut *transaction)
            .await?;
        }
        for id in build_applications {
            sqlx::query!(
                "UPDATE applications SET build_server_id = ? WHERE id = ? AND build_server_id = ?",
                source,
                id,
                target
            )
            .execute(&mut *transaction)
            .await?;
        }
        for id in certificates {
            sqlx::query!(
                "UPDATE certificates SET server_id = ? WHERE id = ? AND server_id = ?",
                source,
                id,
                target
            )
            .execute(&mut *transaction)
            .await?;
        }
        for id in schedules {
            sqlx::query!(
                "UPDATE schedules SET server_id = ? WHERE id = ? AND server_id = ?",
                source,
                id,
                target
            )
            .execute(&mut *transaction)
            .await?;
        }
        transaction.commit().await
    }
    pub async fn dependency_counts(
        &self,
        server_id: i64,
    ) -> Result<ServerDependencyCounts, sqlx::Error> {
        let applications = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM applications WHERE server_id = ?",
            server_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        let build_assignments = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM applications WHERE build_server_id = ?",
            server_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        let compose_projects = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM compose_projects WHERE server_id = ?",
            server_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        let postgres = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM postgres_dbs WHERE server_id = ?",
            server_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        let mysql = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM mysql_dbs WHERE server_id = ?",
            server_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        let mariadb = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM mariadb_dbs WHERE server_id = ?",
            server_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        let mongo = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM mongo_dbs WHERE server_id = ?",
            server_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        let redis = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM redis_dbs WHERE server_id = ?",
            server_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        let libsql = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM libsql_dbs WHERE server_id = ?",
            server_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        let certificates = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM certificates WHERE server_id = ?",
            server_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        let schedules = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM schedules WHERE server_id = ?",
            server_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(ServerDependencyCounts {
            applications,
            build_assignments,
            compose_projects,
            databases: postgres + mysql + mariadb + mongo + redis + libsql,
            certificates,
            schedules,
        })
    }
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
            r#"SELECT 
                  s.ip_address AS public_ip,
                  n.private_host AS private_ip,
                  n.status AS private_status,
                  n.connection_mode AS connection_mode,
                  s.port,
                  s.username,
                  k.private_key,
                  k.public_key
               FROM servers s 
               JOIN ssh_keys k ON k.id = s.ssh_key_id 
               LEFT JOIN server_private_networks n ON n.server_id = s.id 
               WHERE s.id = ?"#,
        )
        .bind(server_id)
        .fetch_optional(self.pool.as_ref())
        .await?;

        if let Some(row) = res {
            let public_ip: String = row.try_get("public_ip")?;
            let private_ip: Option<String> = row.try_get("private_ip").ok().flatten();
            let private_status: Option<String> = row.try_get("private_status").ok().flatten();
            let connection_mode: Option<String> = row.try_get("connection_mode").ok().flatten();
            let port_i64: i64 = row.try_get("port")?;
            let username: String = row.try_get("username")?;
            let private_key: String = row.try_get("private_key")?;
            let public_key: String = row.try_get("public_key")?;

            let mut selected_ip = public_ip.clone();

            if let Some(priv_ip) = private_ip {
                let mode = connection_mode.unwrap_or_default();
                let status = private_status.unwrap_or_default();

                if !priv_ip.trim().is_empty() && mode != "DIRECT_SSH" && status != "INACTIVE" {
                    selected_ip = priv_ip.trim().to_string();
                }
            }

            return Ok(Some((selected_ip, port_i64, username, private_key, public_key)));
        }

        Ok(None)
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
