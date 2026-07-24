use crate::db::models::bitbucket_providers::BitbucketProvider;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct BitbucketProviderRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl BitbucketProviderRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<BitbucketProvider>, sqlx::Error> {
        sqlx::query_as!(
            BitbucketProvider,
            r#"SELECT id AS "id?: i64", bitbucket_username AS "bitbucket_username?: String", bitbucket_email AS "bitbucket_email?: String", app_password AS "app_password?: String", api_token AS "api_token?: String", bitbucket_workspace_name AS "bitbucket_workspace_name?: String", git_provider_id AS "git_provider_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM bitbucket_providers"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<BitbucketProvider>, sqlx::Error> {
        sqlx::query_as!(
            BitbucketProvider,
            r#"SELECT id AS "id?: i64", bitbucket_username AS "bitbucket_username?: String", bitbucket_email AS "bitbucket_email?: String", app_password AS "app_password?: String", api_token AS "api_token?: String", bitbucket_workspace_name AS "bitbucket_workspace_name?: String", git_provider_id AS "git_provider_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM bitbucket_providers WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &BitbucketProvider) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO bitbucket_providers (bitbucket_username, bitbucket_email, app_password, api_token, bitbucket_workspace_name, git_provider_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.bitbucket_username,
            &item.bitbucket_email,
            &item.app_password,
            &item.api_token,
            &item.bitbucket_workspace_name,
            item.git_provider_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &BitbucketProvider) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE bitbucket_providers SET bitbucket_username = ?, bitbucket_email = ?, app_password = ?, api_token = ?, bitbucket_workspace_name = ?, git_provider_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.bitbucket_username,
            &item.bitbucket_email,
            &item.app_password,
            &item.api_token,
            &item.bitbucket_workspace_name,
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
        sqlx::query!(r#"DELETE FROM bitbucket_providers WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
