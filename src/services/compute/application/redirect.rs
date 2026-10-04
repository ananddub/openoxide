use std::sync::Arc;

use auto_di::singleton;
use serde::{Deserialize, Serialize};

use crate::db::queries::FlashQueries;

/// Raw SQL is required for redirects because the FlashORM schema parser
/// silently drops the `unique_config_key` column, making it unusable in
/// generated queries and models. These statements preserve the exact SQL
/// semantics of the deleted `RedirectRepository`.
/// Local row shape for the `redirects` table. It lives here instead of
/// `flash_gen::models` because the generator drops `unique_config_key`, so
/// no generated `Redirects` model can represent this table.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Redirect {
    pub id: i64,
    pub regex: String,
    pub replacement: String,
    pub permanent: i64,
    pub unique_config_key: Option<i64>,
    pub service_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpsertApplicationRedirectDto {
    pub regex: String,
    pub replacement: String,
    pub permanent: i64,
    pub unique_config_key: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApplicationRedirectResponseDto {
    pub id: i64,
    pub regex: String,
    pub replacement: String,
    pub permanent: i64,
    pub unique_config_key: Option<i64>,
    pub application_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Redirect> for ApplicationRedirectResponseDto {
    fn from(value: Redirect) -> Self {
        Self {
            id: value.id,
            regex: value.regex,
            replacement: value.replacement,
            permanent: value.permanent,
            unique_config_key: value.unique_config_key,
            application_id: value.service_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub struct RedirectService {
    queries: Arc<FlashQueries>,
}
#[singleton]
impl RedirectService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }
    async fn ensure_app(&self, id: i64) -> sqlx::Result<()> {
        self.queries
            .app_sub_get_application_by_i_d(id)
            .await
            .map(|_| ())
    }
    fn validate(input: UpsertApplicationRedirectDto) -> sqlx::Result<UpsertApplicationRedirectDto> {
        if input.permanent != 0 && input.permanent != 1 {
            return Err(sqlx::Error::Protocol("permanent must be 0 or 1".into()));
        }
        Ok(input)
    }
    async fn get_by_id(&self, id: i64) -> sqlx::Result<Option<Redirect>> {
        sqlx::query_as::<_, Redirect>(
            "SELECT id, regex, replacement, permanent, unique_config_key, service_id, created_at, updated_at \
             FROM redirects WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.queries.pool)
        .await
    }
    pub async fn list(&self, app: i64) -> sqlx::Result<Vec<Redirect>> {
        self.ensure_app(app).await?;
        sqlx::query_as::<_, Redirect>(
            "SELECT id, regex, replacement, permanent, unique_config_key, service_id, created_at, updated_at \
             FROM redirects WHERE service_id = ? ORDER BY id",
        )
        .bind(app)
        .fetch_all(&self.queries.pool)
        .await
    }
    pub async fn create(
        &self,
        app: i64,
        input: UpsertApplicationRedirectDto,
    ) -> sqlx::Result<Redirect> {
        self.ensure_app(app).await?;
        let i = Self::validate(input)?;
        let result = sqlx::query(
            "INSERT INTO redirects (regex, replacement, permanent, unique_config_key, service_id) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&i.regex)
        .bind(&i.replacement)
        .bind(i.permanent)
        .bind(i.unique_config_key)
        .bind(app)
        .execute(&self.queries.pool)
        .await?;
        let id = result.last_insert_rowid();
        self.get_by_id(id).await?.ok_or(sqlx::Error::RowNotFound)
    }
    pub async fn update(
        &self,
        app: i64,
        id: i64,
        input: UpsertApplicationRedirectDto,
    ) -> sqlx::Result<Redirect> {
        self.ensure_app(app).await?;
        let i = Self::validate(input)?;
        let result = sqlx::query(
            "UPDATE redirects SET regex = ?, replacement = ?, permanent = ?, unique_config_key = ? WHERE id = ? AND service_id = ?",
        )
        .bind(&i.regex)
        .bind(&i.replacement)
        .bind(i.permanent)
        .bind(i.unique_config_key)
        .bind(id)
        .bind(app)
        .execute(&self.queries.pool)
        .await?;
        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        self.get_by_id(id).await?.ok_or(sqlx::Error::RowNotFound)
    }
    pub async fn delete(&self, app: i64, id: i64) -> sqlx::Result<bool> {
        self.ensure_app(app).await?;
        let result = sqlx::query("DELETE FROM redirects WHERE id = ? AND service_id = ?")
            .bind(id)
            .bind(app)
            .execute(&self.queries.pool)
            .await?;
        Ok(result.rows_affected() == 1)
    }
}
