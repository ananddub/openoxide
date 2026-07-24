use crate::db::models::gitea_providers::GiteaProvider;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct GiteaProviderRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl GiteaProviderRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<GiteaProvider>, sqlx::Error> {
        sqlx::query_as!(
            GiteaProvider,
            r#"SELECT id AS "id?: i64", gitea_url AS "gitea_url: String", gitea_internal_url AS "gitea_internal_url?: String", redirect_uri AS "redirect_uri?: String", client_id AS "client_id?: String", client_secret AS "client_secret?: String", access_token AS "access_token?: String", refresh_token AS "refresh_token?: String", expires_at AS "expires_at?: i64", scopes AS "scopes?: String", last_authenticated_at AS "last_authenticated_at?: i64", git_provider_id AS "git_provider_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM gitea_providers"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<GiteaProvider>, sqlx::Error> {
        sqlx::query_as!(
            GiteaProvider,
            r#"SELECT id AS "id?: i64", gitea_url AS "gitea_url: String", gitea_internal_url AS "gitea_internal_url?: String", redirect_uri AS "redirect_uri?: String", client_id AS "client_id?: String", client_secret AS "client_secret?: String", access_token AS "access_token?: String", refresh_token AS "refresh_token?: String", expires_at AS "expires_at?: i64", scopes AS "scopes?: String", last_authenticated_at AS "last_authenticated_at?: i64", git_provider_id AS "git_provider_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM gitea_providers WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &GiteaProvider) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO gitea_providers (gitea_url, gitea_internal_url, redirect_uri, client_id, client_secret, access_token, refresh_token, expires_at, scopes, last_authenticated_at, git_provider_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.gitea_url,
            &item.gitea_internal_url,
            &item.redirect_uri,
            &item.client_id,
            &item.client_secret,
            &item.access_token,
            &item.refresh_token,
            item.expires_at,
            &item.scopes,
            item.last_authenticated_at,
            item.git_provider_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &GiteaProvider) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE gitea_providers SET gitea_url = ?, gitea_internal_url = ?, redirect_uri = ?, client_id = ?, client_secret = ?, access_token = ?, refresh_token = ?, expires_at = ?, scopes = ?, last_authenticated_at = ?, git_provider_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.gitea_url,
            &item.gitea_internal_url,
            &item.redirect_uri,
            &item.client_id,
            &item.client_secret,
            &item.access_token,
            &item.refresh_token,
            item.expires_at,
            &item.scopes,
            item.last_authenticated_at,
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
        sqlx::query!(r#"DELETE FROM gitea_providers WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
