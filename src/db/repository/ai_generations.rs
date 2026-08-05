use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::db::models::ai_generations::AiGeneration;

pub struct AiGenerationRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl AiGenerationRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        ai_setting_id: i64,
        organization_id: i64,
        created_by: i64,
        prompt: &str,
        output_json: &str,
    ) -> sqlx::Result<AiGeneration> {
        sqlx::query_as!(
            AiGeneration,
            r#"INSERT INTO ai_generations
               (ai_setting_id, organization_id, created_by, prompt, output_json)
               VALUES (?, ?, ?, ?, ?)
               RETURNING id AS "id!", ai_setting_id, organization_id, created_by,
                         prompt, output_json, status, compose_id, created_at, updated_at"#,
            ai_setting_id,
            organization_id,
            created_by,
            prompt,
            output_json
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn get_for_organization(
        &self,
        id: i64,
        organization_id: i64,
    ) -> sqlx::Result<Option<AiGeneration>> {
        sqlx::query_as!(
            AiGeneration,
            r#"SELECT id AS "id!", ai_setting_id, organization_id, created_by,
                      prompt, output_json, status, compose_id, created_at, updated_at
               FROM ai_generations
               WHERE id = ? AND organization_id = ?"#,
            id,
            organization_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn list_for_organization(
        &self,
        organization_id: i64,
        limit: i64,
    ) -> sqlx::Result<Vec<AiGeneration>> {
        sqlx::query_as!(
            AiGeneration,
            r#"SELECT id AS "id!", ai_setting_id, organization_id, created_by,
                      prompt, output_json, status, compose_id, created_at, updated_at
               FROM ai_generations
               WHERE organization_id = ?
               ORDER BY created_at DESC, id DESC
               LIMIT ?"#,
            organization_id,
            limit
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn review(
        &self,
        id: i64,
        organization_id: i64,
        output_json: &str,
    ) -> sqlx::Result<Option<AiGeneration>> {
        sqlx::query_as!(
            AiGeneration,
            r#"UPDATE ai_generations
               SET output_json = ?, status = 'REVIEWED'
               WHERE id = ? AND organization_id = ? AND status != 'DEPLOYED'
               RETURNING id AS "id!", ai_setting_id, organization_id, created_by,
                         prompt, output_json, status, compose_id, created_at, updated_at"#,
            output_json,
            id,
            organization_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn mark_deployed(
        &self,
        id: i64,
        organization_id: i64,
        compose_id: i64,
    ) -> sqlx::Result<Option<AiGeneration>> {
        sqlx::query_as!(
            AiGeneration,
            r#"UPDATE ai_generations
               SET status = 'DEPLOYED', compose_id = ?
               WHERE id = ? AND organization_id = ? AND status = 'DEPLOYING'
               RETURNING id AS "id!", ai_setting_id, organization_id, created_by,
                         prompt, output_json, status, compose_id, created_at, updated_at"#,
            compose_id,
            id,
            organization_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn claim_for_deploy(&self, id: i64, organization_id: i64) -> sqlx::Result<bool> {
        let result = sqlx::query!(
            r#"UPDATE ai_generations SET status = 'DEPLOYING'
               WHERE id = ? AND organization_id = ? AND status = 'REVIEWED'"#,
            id,
            organization_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn release_deploy(&self, id: i64, organization_id: i64) -> sqlx::Result<()> {
        sqlx::query!(
            r#"UPDATE ai_generations SET status = 'REVIEWED'
               WHERE id = ? AND organization_id = ? AND status = 'DEPLOYING'"#,
            id,
            organization_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}
