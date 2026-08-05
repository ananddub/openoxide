use std::sync::Arc;

use auto_di::singleton;

use crate::{
    api::dto::application::port::UpsertApplicationPortDto,
    db::models::ports::Port,
    repository::{ApplicationRepository, PortRepository},
};

pub struct PortService {
    applications: Arc<ApplicationRepository>,
    ports: Arc<PortRepository>,
}

#[singleton]
impl PortService {
    fn new(applications: Arc<ApplicationRepository>, ports: Arc<PortRepository>) -> Self {
        Self {
            applications,
            ports,
        }
    }

    async fn ensure_application(&self, application_id: i64) -> sqlx::Result<()> {
        self.applications
            .get_by_id(application_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
            .map(|_| ())
    }

    pub async fn list(&self, application_id: i64) -> sqlx::Result<Vec<Port>> {
        self.ensure_application(application_id).await?;
        self.ports.list_by_application(application_id).await
    }

    pub async fn create(
        &self,
        application_id: i64,
        input: UpsertApplicationPortDto,
    ) -> sqlx::Result<Port> {
        self.ensure_application(application_id).await?;
        let input = input.normalize().map_err(sqlx::Error::Protocol)?;
        self.ports
            .create_for_application(
                application_id,
                input.published_port,
                input.target_port,
                input.protocol.as_str(),
                input.publish_mode.as_str(),
            )
            .await
    }

    pub async fn update(
        &self,
        application_id: i64,
        id: i64,
        input: UpsertApplicationPortDto,
    ) -> sqlx::Result<Port> {
        self.ensure_application(application_id).await?;
        let input = input.normalize().map_err(sqlx::Error::Protocol)?;
        self.ports
            .update_for_application(
                id,
                application_id,
                input.published_port,
                input.target_port,
                input.protocol.as_str(),
                input.publish_mode.as_str(),
            )
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn delete(&self, application_id: i64, id: i64) -> sqlx::Result<bool> {
        self.ensure_application(application_id).await?;
        self.ports.delete_for_application(id, application_id).await
    }
}
