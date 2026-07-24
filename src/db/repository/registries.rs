use crate::db::models::registries::Registry;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct RegistryRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl RegistryRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Registry>, sqlx::Error> {
        sqlx::query_as!(
            Registry,
            r#"SELECT id AS "id?: i64", registry_name AS "registry_name: String", image_prefix AS "image_prefix?: String", username AS "username: String", password AS "password: String", registry_url AS "registry_url: String", registry_type AS "registry_type: String", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM registries"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Registry>, sqlx::Error> {
        sqlx::query_as!(
            Registry,
            r#"SELECT id AS "id?: i64", registry_name AS "registry_name: String", image_prefix AS "image_prefix?: String", username AS "username: String", password AS "password: String", registry_url AS "registry_url: String", registry_type AS "registry_type: String", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM registries WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Registry) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO registries (registry_name, image_prefix, username, password, registry_url, registry_type, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.registry_name,
            &item.image_prefix,
            &item.username,
            &item.password,
            &item.registry_url,
            &item.registry_type,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Registry) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE registries SET registry_name = ?, image_prefix = ?, username = ?, password = ?, registry_url = ?, registry_type = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.registry_name,
            &item.image_prefix,
            &item.username,
            &item.password,
            &item.registry_url,
            &item.registry_type,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM registries WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
