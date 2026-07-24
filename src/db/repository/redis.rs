use crate::api::dto::database::{CreateDatabaseDto, PatchDatabaseDto};
use crate::services::database::{DatabaseKind, DatabaseRecord};
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct RedisRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl RedisRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<DatabaseRecord> {
        sqlx::query_as!(
            DatabaseRecord,
            r#"SELECT 'redis' AS "kind: DatabaseKind", id AS "id!: i64", name, app_name, description, docker_image,
               CAST(NULL AS TEXT) AS "database_name?", CAST(NULL AS TEXT) AS "database_user?", external_port,
               env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
               app_status, environment_id, server_id, created_at, updated_at
               FROM redis_dbs WHERE id = ?"#,
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
        db_password: &str,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT INTO redis_dbs
               (name, app_name, description, docker_image, database_password, external_port, environment_id, server_id)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            input.name,
            app_name,
            input.description,
            image,
            db_password,
            input.external_port,
            input.environment_id,
            input.server_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn update(&self, id: i64, input: &PatchDatabaseDto) -> sqlx::Result<()> {
        sqlx::query!(
            "UPDATE redis_dbs SET name = COALESCE(?, name), description = COALESCE(?, description), docker_image = COALESCE(?, docker_image), external_port = COALESCE(?, external_port), command = COALESCE(?, command), args = COALESCE(?, args), env_var = COALESCE(?, env_var), memory_reservation = COALESCE(?, memory_reservation), memory_limit = COALESCE(?, memory_limit), cpu_reservation = COALESCE(?, cpu_reservation), cpu_limit = COALESCE(?, cpu_limit), replicas = COALESCE(?, replicas), server_id = COALESCE(?, server_id) WHERE id = ?",
            input.name,
            input.description,
            input.docker_image,
            input.external_port,
            input.command,
            input.args,
            input.env_var,
            input.memory_reservation,
            input.memory_limit,
            input.cpu_reservation,
            input.cpu_limit,
            input.replicas,
            input.server_id,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        sqlx::query!("DELETE FROM redis_dbs WHERE id = ?", id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn get_server_id_and_name(&self, id: i64) -> sqlx::Result<(Option<i64>, String)> {
        let r = sqlx::query!("SELECT server_id, name FROM redis_dbs WHERE id = ?", id)
            .fetch_one(self.pool.as_ref())
            .await?;
        Ok((r.server_id, r.name))
    }

    pub async fn update_status(&self, id: i64, status: &str) -> sqlx::Result<()> {
        sqlx::query!(
            "UPDATE redis_dbs SET app_status = ? WHERE id = ?",
            status,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn get_details(&self, id: i64) -> sqlx::Result<RedisDbDetails> {
        sqlx::query_as!(
            RedisDbDetails,
            r#"SELECT name, app_name, docker_image, database_password,
               external_port, command, args, env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit,
               replicas, environment_id
               FROM redis_dbs WHERE id = ?"#,
            id
        )
        .fetch_one(self.pool.as_ref())
        .await
    }
}

pub struct RedisDbDetails {
    pub name: String,
    pub app_name: String,
    pub docker_image: String,
    pub database_password: String,
    pub external_port: Option<i64>,
    pub command: Option<String>,
    pub args: Option<String>,
    pub env_var: Option<String>,
    pub memory_reservation: Option<String>,
    pub memory_limit: Option<String>,
    pub cpu_reservation: Option<String>,
    pub cpu_limit: Option<String>,
    pub replicas: i64,
    pub environment_id: i64,
}
