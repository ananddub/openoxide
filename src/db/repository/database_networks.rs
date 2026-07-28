use crate::db::models::database_networks::DatabaseNetwork;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct DatabaseNetworkRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl DatabaseNetworkRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> sqlx::Result<Vec<DatabaseNetwork>> {
        sqlx::query_as!(
            DatabaseNetwork,
            r#"SELECT id, name, docker_network_name, description, external, server_id, created_at, updated_at
               FROM database_networks
               ORDER BY name ASC, id ASC"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn list_by_server(
        &self,
        server_id: Option<i64>,
    ) -> sqlx::Result<Vec<DatabaseNetwork>> {
        sqlx::query_as!(
            DatabaseNetwork,
            r#"SELECT id, name, docker_network_name, description, external, server_id, created_at, updated_at
               FROM database_networks
               WHERE (? IS NULL AND server_id IS NULL) OR server_id = ?
               ORDER BY name ASC, id ASC"#,
            server_id,
            server_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Option<DatabaseNetwork>> {
        sqlx::query_as!(
            DatabaseNetwork,
            r#"SELECT id, name, docker_network_name, description, external, server_id, created_at, updated_at
               FROM database_networks
               WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(
        &self,
        name: &str,
        docker_network_name: &str,
        description: Option<&str>,
        external: i64,
        server_id: Option<i64>,
    ) -> sqlx::Result<i64> {
        let result = sqlx::query!(
            r#"INSERT INTO database_networks (name, docker_network_name, description, external, server_id)
               VALUES (?, ?, ?, ?, ?)"#,
            name,
            docker_network_name,
            description,
            external,
            server_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn update(
        &self,
        id: i64,
        name: Option<&str>,
        docker_network_name: Option<&str>,
        description: Option<&str>,
        external: Option<i64>,
        server_id: Option<i64>,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            r#"UPDATE database_networks
               SET name = COALESCE(?, name),
                   docker_network_name = COALESCE(?, docker_network_name),
                   description = COALESCE(?, description),
                   external = COALESCE(?, external),
                   server_id = COALESCE(?, server_id)
               WHERE id = ?"#,
            name,
            docker_network_name,
            description,
            external,
            server_id,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        sqlx::query!("DELETE FROM database_networks WHERE id = ?", id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn resolve_names(&self, values: &[String]) -> sqlx::Result<Vec<String>> {
        let mut names = Vec::with_capacity(values.len());
        for value in values {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                continue;
            }

            if let Ok(id) = trimmed.parse::<i64>() {
                if let Some(network) = self.get_by_id(id).await? {
                    names.push(network.docker_network_name);
                    continue;
                }
            }

            names.push(trimmed.to_string());
        }
        Ok(names)
    }
}
