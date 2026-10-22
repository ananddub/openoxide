use std::sync::Arc;

use auto_di::singleton;
use serde::{Deserialize, Serialize};

use crate::{
    db::queries::FlashQueries,
    flash_gen::models::Ports,
    flash_gen::ports::{PortsCreateForApplicationParams, PortsUpdateForApplicationParams},
};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PortProtocol {
    Tcp,
    Udp,
}
impl PortProtocol {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tcp => "TCP",
            Self::Udp => "UDP",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PortPublishMode {
    Host,
    Ingress,
}
impl PortPublishMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Host => "HOST",
            Self::Ingress => "INGRESS",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpsertApplicationPortDto {
    pub published_port: i64,
    pub target_port: i64,
    pub protocol: PortProtocol,
    pub publish_mode: PortPublishMode,
}

impl UpsertApplicationPortDto {
    pub fn normalize(self) -> Result<Self, String> {
        Ok(self)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ApplicationPortResponseDto {
    pub id: i64,
    pub published_port: i64,
    pub target_port: i64,
    pub protocol: String,
    pub publish_mode: String,
    pub application_id: i64,
    pub created_at: i64,
}

impl From<Ports> for ApplicationPortResponseDto {
    fn from(value: Ports) -> Self {
        Self {
            id: value.id,
            published_port: value.published_port,
            target_port: value.target_port,
            protocol: value.protocol,
            publish_mode: value.publish_mode,
            application_id: value.service_id,
            created_at: value.created_at,
        }
    }
}

pub struct PortService {
    queries: Arc<FlashQueries>,
}

#[cfg(test)]
impl PortService {
    pub(crate) fn for_tests(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }
}

#[singleton]
impl PortService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    async fn ensure_application(&self, application_id: i64) -> sqlx::Result<()> {
        self.queries
            .app_sub_get_application_by_i_d(application_id)
            .await
            .map(|_| ())
    }

    pub async fn list(&self, application_id: i64) -> sqlx::Result<Vec<Ports>> {
        self.ensure_application(application_id).await?;
        self.queries.ports_list_by_application(application_id).await
    }

    pub async fn create(
        &self,
        application_id: i64,
        input: UpsertApplicationPortDto,
    ) -> sqlx::Result<Ports> {
        self.ensure_application(application_id).await?;
        let input = input.normalize().map_err(sqlx::Error::Protocol)?;
        let params = PortsCreateForApplicationParams {
            published_port: input.published_port,
            target_port: input.target_port,
            protocol: input.protocol.as_str().to_owned(),
            publish_mode: input.publish_mode.as_str().to_owned(),
            service_id: application_id,
        };
        self.queries.ports_create_for_application(&params).await
    }

    pub async fn update(
        &self,
        application_id: i64,
        id: i64,
        input: UpsertApplicationPortDto,
    ) -> sqlx::Result<Ports> {
        self.ensure_application(application_id).await?;
        let input = input.normalize().map_err(sqlx::Error::Protocol)?;
        let params = PortsUpdateForApplicationParams {
            published_port: input.published_port,
            target_port: input.target_port,
            protocol: input.protocol.as_str().to_owned(),
            publish_mode: input.publish_mode.as_str().to_owned(),
            id,
            service_id: application_id,
        };
        self.queries.ports_update_for_application(&params).await
    }

    pub async fn delete(&self, application_id: i64, id: i64) -> sqlx::Result<bool> {
        self.ensure_application(application_id).await?;
        match self
            .queries
            .ports_delete_for_application(id, application_id)
            .await
        {
            Ok(_) => Ok(true),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(error) => Err(error),
        }
    }
}
