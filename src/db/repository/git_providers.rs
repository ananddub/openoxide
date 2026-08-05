use crate::db::models::git_providers::GitProvider;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct GitProviderRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl GitProviderRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<GitProvider>, sqlx::Error> {
        sqlx::query_as!(
            GitProvider,
            r#"SELECT id AS "id?: String", name AS "name: String", provider_type AS "provider_type: String", shared AS "shared: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM git_providers"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<GitProvider>, sqlx::Error> {
        sqlx::query_as!(
            GitProvider,
            r#"SELECT id AS "id?: String", name AS "name: String", provider_type AS "provider_type: String", shared AS "shared: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM git_providers WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &GitProvider) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO git_providers (name, provider_type, shared, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"#,
            &item.name,
            &item.provider_type,
            item.shared,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &GitProvider) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE git_providers SET name = ?, provider_type = ?, shared = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.name,
            &item.provider_type,
            item.shared,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM git_providers WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn rotate_webhook_secret(&self, id: i64, secret: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE git_providers SET webhook_secret = ?, updated_at = strftime('%s', 'now') WHERE id = ?",
            secret,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn has_webhook_secret(&self, id: i64) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query_scalar!(
            "SELECT COUNT(*) AS \"count!: i64\" FROM git_providers WHERE id = ? AND webhook_secret IS NOT NULL AND webhook_secret != ''",
            id
        )
        .fetch_one(self.pool.as_ref())
        .await? > 0)
    }

    pub async fn webhook_secret(&self, id: i64) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar!("SELECT webhook_secret FROM git_providers WHERE id = ?", id)
            .fetch_optional(self.pool.as_ref())
            .await
            .map(Option::flatten)
    }

    pub async fn resource_usage_count(&self, id: i64) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar!(
            r#"SELECT
                (SELECT COUNT(*) FROM applications a
                 WHERE a.github_provider_id IN (SELECT id FROM github_providers WHERE git_provider_id = ?)
                    OR a.gitlab_provider_id IN (SELECT id FROM gitlab_providers WHERE git_provider_id = ?)
                    OR a.gitea_provider_id IN (SELECT id FROM gitea_providers WHERE git_provider_id = ?)
                    OR a.bitbucket_provider_id IN (SELECT id FROM bitbucket_providers WHERE git_provider_id = ?))
              + (SELECT COUNT(*) FROM compose_projects c
                 WHERE c.github_provider_id IN (SELECT id FROM github_providers WHERE git_provider_id = ?)
                    OR c.gitlab_provider_id IN (SELECT id FROM gitlab_providers WHERE git_provider_id = ?)
                    OR c.gitea_provider_id IN (SELECT id FROM gitea_providers WHERE git_provider_id = ?)
                    OR c.bitbucket_provider_id IN (SELECT id FROM bitbucket_providers WHERE git_provider_id = ?)) AS "count!: i64""#,
            id, id, id, id,
            id, id, id, id,
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn delete_if_unused(&self, id: i64) -> Result<bool, sqlx::Error> {
        if self.resource_usage_count(id).await? > 0 {
            return Ok(false);
        }
        let result = sqlx::query!("DELETE FROM git_providers WHERE id = ?", id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(result.rows_affected() == 1)
    }
}
