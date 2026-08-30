use std::sync::Arc;

use auto_di::singleton;
use serde::{Deserialize, Serialize};

use crate::{
    db::models::security::Security, db::queries::FlashQueries, flash_gen::models,
    flash_gen::security::{SecurityCreateForApplicationParams, SecurityUpdateForApplicationParams},
};

#[derive(Debug, Deserialize)]
pub struct UpsertApplicationSecurityDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApplicationSecurityResponseDto {
    pub id: i64,
    pub username: String,
    pub has_password: bool,
    pub application_id: i64,
    pub created_at: i64,
}
impl From<Security> for ApplicationSecurityResponseDto {
    fn from(value: Security) -> Self {
        Self {
            id: value.id,
            username: value.username,
            has_password: !value.password.is_empty(),
            application_id: value.service_id,
            created_at: value.created_at,
        }
    }
}

fn to_model(row: models::Security) -> Security {
    Security {
        id: row.id,
        username: row.username,
        password: row.password,
        service_id: row.service_id,
        created_at: row.created_at,
    }
}

pub struct SecurityService {
    queries: Arc<FlashQueries>,
}
#[singleton]
impl SecurityService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }
    async fn ensure_app(&self, id: i64) -> sqlx::Result<()> {
        self.queries
            .app_sub_get_application_by_i_d(id)
            .await
            .map(|_| ())
    }
    pub async fn list(&self, app: i64) -> sqlx::Result<Vec<Security>> {
        self.ensure_app(app).await?;
        Ok(self
            .queries
            .security_list_by_application(app)
            .await?
            .into_iter()
            .map(to_model)
            .collect())
    }
    pub async fn create(
        &self,
        app: i64,
        input: UpsertApplicationSecurityDto,
    ) -> sqlx::Result<Security> {
        self.ensure_app(app).await?;
        let username = validate_username(&input.username).map_err(sqlx::Error::Protocol)?;
        let password = bcrypt::hash(input.password, bcrypt::DEFAULT_COST)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let params = SecurityCreateForApplicationParams {
            username,
            password,
            service_id: app,
        };
        Ok(to_model(
            self.queries.security_create_for_application(&params).await?,
        ))
    }
    pub async fn update(
        &self,
        app: i64,
        id: i64,
        input: UpsertApplicationSecurityDto,
    ) -> sqlx::Result<Security> {
        self.ensure_app(app).await?;
        let username = validate_username(&input.username).map_err(sqlx::Error::Protocol)?;
        let password = bcrypt::hash(input.password, bcrypt::DEFAULT_COST)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let params = SecurityUpdateForApplicationParams {
            username,
            password,
            id,
            service_id: app,
        };
        Ok(to_model(
            self.queries.security_update_for_application(&params).await?,
        ))
    }
    pub async fn delete(&self, app: i64, id: i64) -> sqlx::Result<bool> {
        self.ensure_app(app).await?;
        match self
            .queries
            .security_delete_for_application(id, app)
            .await
        {
            Ok(_) => Ok(true),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(error) => Err(error),
        }
    }
}

fn validate_username(username: &str) -> Result<String, String> {
    let username = username.trim();
    if username.is_empty() || username.contains(':') || username.contains(',') {
        return Err("username cannot be empty or contain ':' or ','".into());
    }
    Ok(username.to_string())
}
