use crate::api::dto::database::{CreateDatabaseDto, PatchDatabaseDto, serialize_json_string_vec};
use crate::services::database::{DatabaseKind, DatabaseRecord};
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct LibsqlRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl LibsqlRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<DatabaseRecord> {
        sqlx::query_as!(
            DatabaseRecord,
            r#"SELECT 'libsql' AS "kind: DatabaseKind", id AS "id!: i64", name, app_name, description, docker_image,
               CAST(NULL AS TEXT) AS "database_name?", database_user AS "database_user?", external_port,
               env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
               network_ids, detach_rustploy_network, app_status, environment_id, server_id, created_at, updated_at
               FROM libsql_dbs WHERE id = ?"#,
            id
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn create(
        &self,
        input: &CreateDatabaseDto,
        app_name: &str,
        image: &str,
        db_user: &str,
        db_password: &str,
    ) -> sqlx::Result<i64> {
        let args = serialize_json_string_vec(input.args.as_ref())?;
        let network_ids = serialize_json_string_vec(input.network_ids.as_ref())?;
        let result = sqlx::query!(
            r#"INSERT INTO libsql_dbs
               (name, app_name, description, docker_image, database_user, database_password,
                sqld_node, sqld_primary_url, enable_namespaces,
                external_port, external_grpc_port, external_admin_port, command, args, env_var, network_ids, detach_rustploy_network, environment_id, server_id)
               VALUES (?, ?, ?, ?, ?, ?, COALESCE(?, 'PRIMARY'), COALESCE(?, ''), COALESCE(?, 0), ?, ?, ?, ?, ?, ?, COALESCE(?, '[]'), COALESCE(?, 0), ?, ?)"#,
            input.name,
            app_name,
            input.description,
            image,
            db_user,
            db_password,
            input.sqld_node,
            input.sqld_primary_url,
            input.enable_namespaces,
            input.external_port,
            input.external_grpc_port,
            input.external_admin_port,
            input.command,
            args,
            input.env_var,
            network_ids,
            input.detach_rustploy_network,
            input.environment_id,
            input.server_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, input: &PatchDatabaseDto) -> sqlx::Result<()> {
        let args = serialize_json_string_vec(input.args.as_ref())?;
        let network_ids = serialize_json_string_vec(input.network_ids.as_ref())?;
        sqlx::query!(
            "UPDATE libsql_dbs SET name = COALESCE(?, name), description = COALESCE(?, description), docker_image = COALESCE(?, docker_image), external_port = COALESCE(?, external_port), external_grpc_port = COALESCE(?, external_grpc_port), external_admin_port = COALESCE(?, external_admin_port), command = COALESCE(?, command), args = COALESCE(?, args), env_var = COALESCE(?, env_var), memory_reservation = COALESCE(?, memory_reservation), memory_limit = COALESCE(?, memory_limit), cpu_reservation = COALESCE(?, cpu_reservation), cpu_limit = COALESCE(?, cpu_limit), replicas = COALESCE(?, replicas), server_id = COALESCE(?, server_id), network_ids = COALESCE(?, network_ids), detach_rustploy_network = COALESCE(?, detach_rustploy_network) WHERE id = ?",
            input.name,
            input.description,
            input.docker_image,
            input.external_port,
            input.external_grpc_port,
            input.external_admin_port,
            input.command,
            args,
            input.env_var,
            input.memory_reservation,
            input.memory_limit,
            input.cpu_reservation,
            input.cpu_limit,
            input.replicas,
            input.server_id,
            network_ids,
            input.detach_rustploy_network,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        sqlx::query!("DELETE FROM libsql_dbs WHERE id = ?", id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn get_server_id_and_name(&self, id: i64) -> sqlx::Result<(Option<i64>, String)> {
        let r = sqlx::query!("SELECT server_id, name FROM libsql_dbs WHERE id = ?", id)
            .fetch_one(self.pool.as_ref())
            .await?;
        Ok((r.server_id, r.name))
    }

    pub async fn update_status(&self, id: i64, status: &str) -> sqlx::Result<()> {
        let upper = status.to_uppercase();
        sqlx::query!(
            "UPDATE libsql_dbs SET app_status = ? WHERE id = ?",
            upper,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn get_details(&self, id: i64) -> sqlx::Result<LibsqlDbDetails> {
        sqlx::query_as!(
            LibsqlDbDetails,
            r#"SELECT name, app_name, docker_image, database_user, database_password, sqld_node, sqld_primary_url, enable_namespaces,
               external_port, external_grpc_port, external_admin_port, command, args, env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit,
               replicas, network_ids, detach_rustploy_network, environment_id
               FROM libsql_dbs WHERE id = ?"#,
            id
        )
        .fetch_one(self.pool.as_ref())
        .await
    }
}

pub struct LibsqlDbDetails {
    pub name: String,
    pub app_name: String,
    pub docker_image: String,
    pub database_user: String,
    pub database_password: String,
    pub sqld_node: String,
    pub sqld_primary_url: Option<String>,
    pub enable_namespaces: i64,
    pub external_port: Option<i64>,
    pub external_grpc_port: Option<i64>,
    pub external_admin_port: Option<i64>,
    pub command: Option<String>,
    pub args: Option<String>,
    pub env_var: Option<String>,
    pub memory_reservation: Option<String>,
    pub memory_limit: Option<String>,
    pub cpu_reservation: Option<String>,
    pub cpu_limit: Option<String>,
    pub replicas: i64,
    pub network_ids: String,
    pub detach_rustploy_network: i64,
    pub environment_id: i64,
}
