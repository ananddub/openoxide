use crate::{
    api::dto::application::redirect::UpsertApplicationRedirectDto,
    db::models::redirects::Redirect,
    repository::{ApplicationRepository, RedirectRepository},
};
use auto_di::singleton;
use std::sync::Arc;

pub struct RedirectService {
    applications: Arc<ApplicationRepository>,
    redirects: Arc<RedirectRepository>,
}
#[singleton]
impl RedirectService {
    fn new(applications: Arc<ApplicationRepository>, redirects: Arc<RedirectRepository>) -> Self {
        Self {
            applications,
            redirects,
        }
    }
    async fn ensure_app(&self, id: i64) -> sqlx::Result<()> {
        self.applications
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
            .map(|_| ())
    }
    fn validate(input: UpsertApplicationRedirectDto) -> sqlx::Result<UpsertApplicationRedirectDto> {
        if input.permanent != 0 && input.permanent != 1 {
            return Err(sqlx::Error::Protocol("permanent must be 0 or 1".into()));
        }
        Ok(input)
    }
    pub async fn list(&self, app: i64) -> sqlx::Result<Vec<Redirect>> {
        self.ensure_app(app).await?;
        self.redirects.list_by_application(app).await
    }
    pub async fn create(
        &self,
        app: i64,
        input: UpsertApplicationRedirectDto,
    ) -> sqlx::Result<Redirect> {
        self.ensure_app(app).await?;
        let i = Self::validate(input)?;
        self.redirects
            .create_for_application(
                app,
                &i.regex,
                &i.replacement,
                i.permanent,
                i.unique_config_key,
            )
            .await
    }
    pub async fn update(
        &self,
        app: i64,
        id: i64,
        input: UpsertApplicationRedirectDto,
    ) -> sqlx::Result<Redirect> {
        self.ensure_app(app).await?;
        let i = Self::validate(input)?;
        self.redirects
            .update_for_application(
                id,
                app,
                &i.regex,
                &i.replacement,
                i.permanent,
                i.unique_config_key,
            )
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }
    pub async fn delete(&self, app: i64, id: i64) -> sqlx::Result<bool> {
        self.ensure_app(app).await?;
        self.redirects.delete_for_application(id, app).await
    }
}
