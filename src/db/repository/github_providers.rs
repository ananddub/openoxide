use crate::db::models::github_providers::GithubProvider;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct GithubProviderRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl GithubProviderRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<GithubProvider>, sqlx::Error> {
        sqlx::query_as!(
            GithubProvider,
            r#"SELECT id AS "id?: i64", github_app_name AS "github_app_name?: String", github_app_id AS "github_app_id?: i64", github_client_id AS "github_client_id?: String", github_client_secret AS "github_client_secret?: String", github_installation_id AS "github_installation_id?: String", github_private_key AS "github_private_key?: String", github_webhook_secret AS "github_webhook_secret?: String", git_provider_id AS "git_provider_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM github_providers"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<GithubProvider>, sqlx::Error> {
        sqlx::query_as!(
            GithubProvider,
            r#"SELECT id AS "id?: i64", github_app_name AS "github_app_name?: String", github_app_id AS "github_app_id?: i64", github_client_id AS "github_client_id?: String", github_client_secret AS "github_client_secret?: String", github_installation_id AS "github_installation_id?: String", github_private_key AS "github_private_key?: String", github_webhook_secret AS "github_webhook_secret?: String", git_provider_id AS "git_provider_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM github_providers WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &GithubProvider) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO github_providers (github_app_name, github_app_id, github_client_id, github_client_secret, github_installation_id, github_private_key, github_webhook_secret, git_provider_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.github_app_name,
            item.github_app_id,
            &item.github_client_id,
            &item.github_client_secret,
            &item.github_installation_id,
            &item.github_private_key,
            &item.github_webhook_secret,
            item.git_provider_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &GithubProvider) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE github_providers SET github_app_name = ?, github_app_id = ?, github_client_id = ?, github_client_secret = ?, github_installation_id = ?, github_private_key = ?, github_webhook_secret = ?, git_provider_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.github_app_name,
            item.github_app_id,
            &item.github_client_id,
            &item.github_client_secret,
            &item.github_installation_id,
            &item.github_private_key,
            &item.github_webhook_secret,
            item.git_provider_id,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM github_providers WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
