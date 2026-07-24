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
}
