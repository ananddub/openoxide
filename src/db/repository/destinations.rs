use crate::db::models::destinations::Destination;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct DestinationRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl DestinationRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Destination>, sqlx::Error> {
        sqlx::query_as!(
            Destination,
            r#"SELECT id AS "id?: String", name AS "name: String", provider AS "provider: String", access_key AS "access_key: String", secret_access_key AS "secret_access_key: String", bucket AS "bucket: String", region AS "region: String", endpoint AS "endpoint: String", additional_flags AS "additional_flags?: String", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM destinations"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Destination>, sqlx::Error> {
        sqlx::query_as!(
            Destination,
            r#"SELECT id AS "id?: String", name AS "name: String", provider AS "provider: String", access_key AS "access_key: String", secret_access_key AS "secret_access_key: String", bucket AS "bucket: String", region AS "region: String", endpoint AS "endpoint: String", additional_flags AS "additional_flags?: String", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM destinations WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Destination) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO destinations (name, provider, access_key, secret_access_key, bucket, region, endpoint, additional_flags, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.name,
            &item.provider,
            &item.access_key,
            &item.secret_access_key,
            &item.bucket,
            &item.region,
            &item.endpoint,
            &item.additional_flags,
            item.organization_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Destination) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE destinations SET name = ?, provider = ?, access_key = ?, secret_access_key = ?, bucket = ?, region = ?, endpoint = ?, additional_flags = ?, organization_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.name,
            &item.provider,
            &item.access_key,
            &item.secret_access_key,
            &item.bucket,
            &item.region,
            &item.endpoint,
            &item.additional_flags,
            item.organization_id,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM destinations WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
