use crate::{
    api::dto::application::security::UpsertApplicationSecurityDto,
    db::models::security::Security,
    repository::{ApplicationRepository, SecurityRepository},
};
use auto_di::singleton;
use std::sync::Arc;

pub struct SecurityService {
    applications: Arc<ApplicationRepository>,
    security: Arc<SecurityRepository>,
}
#[singleton]
impl SecurityService {
    fn new(applications: Arc<ApplicationRepository>, security: Arc<SecurityRepository>) -> Self {
        Self {
            applications,
            security,
        }
    }
    async fn ensure_app(&self, id: i64) -> sqlx::Result<()> {
        self.applications
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
            .map(|_| ())
    }
    pub async fn list(&self, app: i64) -> sqlx::Result<Vec<Security>> {
        self.ensure_app(app).await?;
        self.security.list_by_application(app).await
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
        self.security
            .create_for_application(app, &username, &password)
            .await
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
        self.security
            .update_for_application(id, app, &username, &password)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }
    pub async fn delete(&self, app: i64, id: i64) -> sqlx::Result<bool> {
        self.ensure_app(app).await?;
        self.security.delete_for_application(id, app).await
    }
}

fn validate_username(username: &str) -> Result<String, String> {
    let username = username.trim();
    if username.is_empty() || username.contains(':') || username.contains(',') {
        return Err("username cannot be empty or contain ':' or ','".into());
    }
    Ok(username.to_string())
}
