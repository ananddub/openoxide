use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::services::database::DatabaseKind;

#[derive(Debug, Clone)]
pub struct DatabaseCredentials {
    pub app_name: String,
    pub database_name: Option<String>,
    pub username: Option<String>,
    pub password: String,
    pub external_port: Option<i64>,
    pub server_id: Option<i64>,
}

pub struct DatabaseManagementRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl DatabaseManagementRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn credentials(
        &self,
        kind: DatabaseKind,
        id: i64,
    ) -> sqlx::Result<DatabaseCredentials> {
        match kind {
            DatabaseKind::Postgres => {
                let row = sqlx::query!("SELECT app_name, database_name, database_user, database_password, external_port, server_id FROM postgres_dbs WHERE id = ?", id).fetch_one(self.pool.as_ref()).await?;
                Ok(DatabaseCredentials {
                    app_name: row.app_name,
                    database_name: Some(row.database_name),
                    username: Some(row.database_user),
                    password: row.database_password,
                    external_port: row.external_port,
                    server_id: row.server_id,
                })
            }
            DatabaseKind::Mysql => {
                let row = sqlx::query!("SELECT app_name, database_name, database_user, database_password, external_port, server_id FROM mysql_dbs WHERE id = ?", id).fetch_one(self.pool.as_ref()).await?;
                Ok(DatabaseCredentials {
                    app_name: row.app_name,
                    database_name: Some(row.database_name),
                    username: Some(row.database_user),
                    password: row.database_password,
                    external_port: row.external_port,
                    server_id: row.server_id,
                })
            }
            DatabaseKind::Mariadb => {
                let row = sqlx::query!("SELECT app_name, database_name, database_user, database_password, external_port, server_id FROM mariadb_dbs WHERE id = ?", id).fetch_one(self.pool.as_ref()).await?;
                Ok(DatabaseCredentials {
                    app_name: row.app_name,
                    database_name: Some(row.database_name),
                    username: Some(row.database_user),
                    password: row.database_password,
                    external_port: row.external_port,
                    server_id: row.server_id,
                })
            }
            DatabaseKind::Mongo => {
                let row = sqlx::query!("SELECT app_name, database_user, database_password, external_port, server_id FROM mongo_dbs WHERE id = ?", id).fetch_one(self.pool.as_ref()).await?;
                Ok(DatabaseCredentials {
                    app_name: row.app_name,
                    database_name: Some("admin".into()),
                    username: Some(row.database_user),
                    password: row.database_password,
                    external_port: row.external_port,
                    server_id: row.server_id,
                })
            }
            DatabaseKind::Redis => {
                let row = sqlx::query!("SELECT app_name, database_password, external_port, server_id FROM redis_dbs WHERE id = ?", id).fetch_one(self.pool.as_ref()).await?;
                Ok(DatabaseCredentials {
                    app_name: row.app_name,
                    database_name: None,
                    username: None,
                    password: row.database_password,
                    external_port: row.external_port,
                    server_id: row.server_id,
                })
            }
            DatabaseKind::Libsql => {
                let row = sqlx::query!("SELECT app_name, database_user, database_password, external_port, server_id FROM libsql_dbs WHERE id = ?", id).fetch_one(self.pool.as_ref()).await?;
                Ok(DatabaseCredentials {
                    app_name: row.app_name,
                    database_name: None,
                    username: Some(row.database_user),
                    password: row.database_password,
                    external_port: row.external_port,
                    server_id: row.server_id,
                })
            }
        }
    }

    pub async fn rotate_password(
        &self,
        kind: DatabaseKind,
        id: i64,
        password: &str,
    ) -> sqlx::Result<bool> {
        let result = match kind {
            DatabaseKind::Postgres => {
                sqlx::query!(
                    "UPDATE postgres_dbs SET database_password = ? WHERE id = ?",
                    password,
                    id
                )
                .execute(self.pool.as_ref())
                .await?
            }
            DatabaseKind::Mysql => {
                sqlx::query!(
                    "UPDATE mysql_dbs SET database_password = ? WHERE id = ?",
                    password,
                    id
                )
                .execute(self.pool.as_ref())
                .await?
            }
            DatabaseKind::Mariadb => {
                sqlx::query!(
                    "UPDATE mariadb_dbs SET database_password = ? WHERE id = ?",
                    password,
                    id
                )
                .execute(self.pool.as_ref())
                .await?
            }
            DatabaseKind::Mongo => {
                sqlx::query!(
                    "UPDATE mongo_dbs SET database_password = ? WHERE id = ?",
                    password,
                    id
                )
                .execute(self.pool.as_ref())
                .await?
            }
            DatabaseKind::Redis => {
                sqlx::query!(
                    "UPDATE redis_dbs SET database_password = ? WHERE id = ?",
                    password,
                    id
                )
                .execute(self.pool.as_ref())
                .await?
            }
            DatabaseKind::Libsql => {
                sqlx::query!(
                    "UPDATE libsql_dbs SET database_password = ? WHERE id = ?",
                    password,
                    id
                )
                .execute(self.pool.as_ref())
                .await?
            }
        };
        Ok(result.rows_affected() == 1)
    }
}
