use crate::db::models::ai_settings::AiSetting;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct AiSettingRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl AiSettingRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<AiSetting>, sqlx::Error> {
        sqlx::query_as!(
            AiSetting,
            r#"SELECT id AS "id?: i64", name AS "name: String", api_url AS "api_url: String", api_key AS "api_key: String", model AS "model: String", is_enabled AS "is_enabled: i64", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM ai_settings"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<AiSetting>, sqlx::Error> {
        sqlx::query_as!(
            AiSetting,
            r#"SELECT id AS "id?: i64", name AS "name: String", api_url AS "api_url: String", api_key AS "api_key: String", model AS "model: String", is_enabled AS "is_enabled: i64", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM ai_settings WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn list_by_organization(
        &self,
        organization_id: i64,
    ) -> Result<Vec<AiSetting>, sqlx::Error> {
        sqlx::query_as!(
            AiSetting,
            r#"SELECT id AS "id?: i64", name AS "name: String", api_url AS "api_url: String", api_key AS "api_key: String", model AS "model: String", is_enabled AS "is_enabled: i64", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64"
               FROM ai_settings WHERE organization_id = ? ORDER BY created_at DESC, id DESC"#,
            organization_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_for_organization(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<Option<AiSetting>, sqlx::Error> {
        sqlx::query_as!(
            AiSetting,
            r#"SELECT id AS "id?: i64", name AS "name: String", api_url AS "api_url: String", api_key AS "api_key: String", model AS "model: String", is_enabled AS "is_enabled: i64", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64"
               FROM ai_settings WHERE id = ? AND organization_id = ?"#,
            id,
            organization_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &AiSetting) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO ai_settings (name, api_url, api_key, model, is_enabled, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.name,
            &item.api_url,
            &item.api_key,
            &item.model,
            item.is_enabled,
            item.organization_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &AiSetting) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE ai_settings SET name = ?, api_url = ?, api_key = ?, model = ?, is_enabled = ?, organization_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.name,
            &item.api_url,
            &item.api_key,
            &item.model,
            item.is_enabled,
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
        sqlx::query!(r#"DELETE FROM ai_settings WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn delete_for_organization(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM ai_settings WHERE id = ? AND organization_id = ?",
            id,
            organization_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() > 0)
    }
}
