mod cleanup;
mod lifecycle;
mod private_network;
mod remote_server;

pub use cleanup::ServerCleanupService;
pub use lifecycle::ServerLifecycleService;
pub use private_network::ServerPrivateNetworkService;
pub use remote_server::*;

use std::{collections::BTreeMap, sync::Arc};

use auto_di::singleton;

use crate::{
    api::dto::server::{ServerManagementDto, UpdateServerManagementDto},
    repository::{ServerManagementRepository, ServerRepository},
};

pub struct ServerManagementService {
    servers: Arc<ServerRepository>,
    management: Arc<ServerManagementRepository>,
}

#[singleton]
impl ServerManagementService {
    fn new(servers: Arc<ServerRepository>, management: Arc<ServerManagementRepository>) -> Self {
        Self {
            servers,
            management,
        }
    }

    pub async fn get(&self, server_id: i64) -> sqlx::Result<ServerManagementDto> {
        self.servers
            .get_by_id(server_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        map(self.management.get_or_create(server_id).await?)
    }

    pub async fn update(
        &self,
        server_id: i64,
        input: UpdateServerManagementDto,
    ) -> sqlx::Result<ServerManagementDto> {
        self.servers
            .get_by_id(server_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        validate_labels(&input.labels)?;
        let labels = serde_json::to_string(&input.labels)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let policy = serde_json::to_string(&input.cleanup_policy)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        map(self
            .management
            .update(
                server_id,
                i64::from(input.maintenance_mode),
                input.maintenance_message.as_deref(),
                &labels,
                &policy,
                i64::from(input.gpu_enabled),
            )
            .await?)
    }
}

fn map(
    value: crate::db::models::server_management::ServerManagement,
) -> sqlx::Result<ServerManagementDto> {
    Ok(ServerManagementDto {
        server_id: value.server_id,
        maintenance_mode: value.maintenance_mode != 0,
        maintenance_message: value.maintenance_message,
        labels: serde_json::from_str(&value.labels)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?,
        cleanup_policy: serde_json::from_str(&value.cleanup_policy)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?,
        gpu_enabled: value.gpu_enabled != 0,
        updated_at: value.updated_at,
    })
}

fn validate_labels(labels: &BTreeMap<String, String>) -> sqlx::Result<()> {
    if labels.len() > 64 {
        return Err(sqlx::Error::Protocol(
            "server supports at most 64 labels".into(),
        ));
    }
    for (key, value) in labels {
        if key.is_empty() || key.len() > 128 || value.len() > 512 || key.contains(['\n', '\r', '='])
        {
            return Err(sqlx::Error::Protocol(format!(
                "invalid server label: {key}"
            )));
        }
    }
    Ok(())
}
