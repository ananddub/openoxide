use crate::db::models::gitlab_providers::GitlabProvider;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct GitlabProviderRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl GitlabProviderRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<GitlabProvider>, sqlx::Error> {
        sqlx::query_as!(
            GitlabProvider,
            r#"SELECT id AS "id?: i64", gitlab_url AS "gitlab_url: String", gitlab_internal_url AS "gitlab_internal_url?: String", application_id AS "application_id?: String", redirect_uri AS "redirect_uri?: String", secret AS "secret?: String", access_token AS "access_token?: String", refresh_token AS "refresh_token?: String", group_name AS "group_name?: String", expires_at AS "expires_at?: i64", git_provider_id AS "git_provider_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM gitlab_providers"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<GitlabProvider>, sqlx::Error> {
        sqlx::query_as!(
            GitlabProvider,
            r#"SELECT id AS "id?: i64", gitlab_url AS "gitlab_url: String", gitlab_internal_url AS "gitlab_internal_url?: String", application_id AS "application_id?: String", redirect_uri AS "redirect_uri?: String", secret AS "secret?: String", access_token AS "access_token?: String", refresh_token AS "refresh_token?: String", group_name AS "group_name?: String", expires_at AS "expires_at?: i64", git_provider_id AS "git_provider_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM gitlab_providers WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn get_by_git_provider_id(
        &self,
        id: i64,
    ) -> Result<Option<GitlabProvider>, sqlx::Error> {
        sqlx::query_as!(GitlabProvider, r#"SELECT id AS "id?: i64", gitlab_url AS "gitlab_url: String", gitlab_internal_url AS "gitlab_internal_url?: String", application_id AS "application_id?: String", redirect_uri AS "redirect_uri?: String", secret AS "secret?: String", access_token AS "access_token?: String", refresh_token AS "refresh_token?: String", group_name AS "group_name?: String", expires_at AS "expires_at?: i64", git_provider_id AS "git_provider_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM gitlab_providers WHERE git_provider_id = ?"#, id).fetch_optional(self.pool.as_ref()).await
    }

    pub async fn create(&self, item: &GitlabProvider) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO gitlab_providers (gitlab_url, gitlab_internal_url, application_id, redirect_uri, secret, access_token, refresh_token, group_name, expires_at, git_provider_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.gitlab_url,
            &item.gitlab_internal_url,
            &item.application_id,
            &item.redirect_uri,
            &item.secret,
            &item.access_token,
            &item.refresh_token,
            &item.group_name,
            item.expires_at,
            item.git_provider_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &GitlabProvider) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE gitlab_providers SET gitlab_url = ?, gitlab_internal_url = ?, application_id = ?, redirect_uri = ?, secret = ?, access_token = ?, refresh_token = ?, group_name = ?, expires_at = ?, git_provider_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.gitlab_url,
            &item.gitlab_internal_url,
            &item.application_id,
            &item.redirect_uri,
            &item.secret,
            &item.access_token,
            &item.refresh_token,
            &item.group_name,
            item.expires_at,
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
        sqlx::query!(r#"DELETE FROM gitlab_providers WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn set_oauth_tokens(
        &self,
        git_provider_id: i64,
        access_token: &str,
        refresh_token: Option<&str>,
        expires_at: Option<i64>,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE gitlab_providers SET access_token = ?, refresh_token = ?, expires_at = ?, updated_at = strftime('%s', 'now') WHERE git_provider_id = ?",
            access_token,
            refresh_token,
            expires_at,
            git_provider_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn disconnect(&self, git_provider_id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE gitlab_providers SET access_token = NULL, refresh_token = NULL, expires_at = NULL, updated_at = strftime('%s', 'now') WHERE git_provider_id = ?",
            git_provider_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() == 1)
    }
}
