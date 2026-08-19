use crate::db::models::vault_providers::VaultProvider;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct VaultProviderRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl VaultProviderRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<VaultProvider>, sqlx::Error> {
        sqlx::query_as!(
            VaultProvider,
            r#"SELECT id AS "id?: i64", name AS "name: String", provider_type AS "provider_type: String", api_url AS "api_url: String", auth_token AS "auth_token: String", namespace AS "namespace?: String", config_json AS "config_json?: String", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM vault_providers ORDER BY created_at DESC"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<VaultProvider>, sqlx::Error> {
        sqlx::query_as!(
            VaultProvider,
            r#"SELECT id AS "id?: i64", name AS "name: String", provider_type AS "provider_type: String", api_url AS "api_url: String", auth_token AS "auth_token: String", namespace AS "namespace?: String", config_json AS "config_json?: String", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM vault_providers WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn list_by_organization(
        &self,
        organization_id: i64,
    ) -> Result<Vec<VaultProvider>, sqlx::Error> {
        sqlx::query_as!(
            VaultProvider,
            r#"SELECT id AS "id?: i64", name AS "name: String", provider_type AS "provider_type: String", api_url AS "api_url: String", auth_token AS "auth_token: String", namespace AS "namespace?: String", config_json AS "config_json?: String", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM vault_providers WHERE organization_id = ? ORDER BY created_at DESC"#,
            organization_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &VaultProvider) -> Result<i64, sqlx::Error> {
        let res = sqlx::query!(
            r#"INSERT INTO vault_providers (name, provider_type, api_url, auth_token, namespace, config_json, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.name,
            &item.provider_type,
            &item.api_url,
            &item.auth_token,
            item.namespace.as_deref(),
            item.config_json.as_deref(),
            item.organization_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &VaultProvider) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE vault_providers SET name = ?, provider_type = ?, api_url = ?, auth_token = ?, namespace = ?, config_json = ?, updated_at = ? WHERE id = ? AND organization_id = ?"#,
            &item.name,
            &item.provider_type,
            &item.api_url,
            &item.auth_token,
            item.namespace.as_deref(),
            item.config_json.as_deref(),
            item.updated_at,
            id,
            item.organization_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64, organization_id: i64) -> Result<bool, sqlx::Error> {
        let res = sqlx::query!(
            r#"DELETE FROM vault_providers WHERE id = ? AND organization_id = ?"#,
            id,
            organization_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(res.rows_affected() > 0)
    }
}
