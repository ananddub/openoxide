use crate::db::models::dns_providers::DnsProvider;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct DnsProviderRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl DnsProviderRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<DnsProvider>, sqlx::Error> {
        sqlx::query_as!(
            DnsProvider,
            r#"SELECT id AS "id?: i64", name AS "name: String", provider_type AS "provider_type: String", credentials_json AS "credentials_json: String", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM dns_providers ORDER BY created_at DESC"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<DnsProvider>, sqlx::Error> {
        sqlx::query_as!(
            DnsProvider,
            r#"SELECT id AS "id?: i64", name AS "name: String", provider_type AS "provider_type: String", credentials_json AS "credentials_json: String", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM dns_providers WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn list_by_organization(&self, organization_id: i64) -> Result<Vec<DnsProvider>, sqlx::Error> {
        sqlx::query_as!(
            DnsProvider,
            r#"SELECT id AS "id?: i64", name AS "name: String", provider_type AS "provider_type: String", credentials_json AS "credentials_json: String", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM dns_providers WHERE organization_id = ? ORDER BY created_at DESC"#,
            organization_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &DnsProvider) -> Result<i64, sqlx::Error> {
        let res = sqlx::query!(
            r#"INSERT INTO dns_providers (name, provider_type, credentials_json, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"#,
            &item.name,
            &item.provider_type,
            &item.credentials_json,
            item.organization_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &DnsProvider) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE dns_providers SET name = ?, provider_type = ?, credentials_json = ?, updated_at = ? WHERE id = ?"#,
            &item.name,
            &item.provider_type,
            &item.credentials_json,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64, organization_id: i64) -> Result<bool, sqlx::Error> {
        let res = sqlx::query!(
            r#"DELETE FROM dns_providers WHERE id = ? AND organization_id = ?"#,
            id,
            organization_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(res.rows_affected() > 0)
    }
}
