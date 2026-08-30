use std::{collections::BTreeMap, sync::Arc};

use auto_di::singleton;
use serde::{Deserialize, Serialize};

use crate::{
    db::queries::FlashQueries,
    flash_gen::{models::ServerManagement, server_infra_extra::SrvInfManagementUpdateParams},
};

pub struct ServerManagementService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl ServerManagementService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn get(&self, server_id: i64) -> sqlx::Result<ServerManagementDto> {
        self.queries.servers_get_by_id(server_id).await?;
        map(self.get_or_create(server_id).await?)
    }

    pub async fn update(
        &self,
        server_id: i64,
        payload: &str,
    ) -> sqlx::Result<ServerManagementDto> {
        let input: UpdateServerManagementDto = serde_json::from_str(payload)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        self.queries.servers_get_by_id(server_id).await?;
        validate_labels(&input.labels)?;
        let labels = serde_json::to_string(&input.labels)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let policy = serde_json::to_string(&input.cleanup_policy)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        self.get_or_create(server_id).await?;
        map(self
            .queries
            .srv_inf_management_update(&SrvInfManagementUpdateParams {
                server_id,
                maintenance_mode: i64::from(input.maintenance_mode),
                maintenance_message: input.maintenance_message.unwrap_or_default(),
                labels,
                cleanup_policy: policy,
                gpu_enabled: i64::from(input.gpu_enabled),
            })
            .await?)
    }

    async fn get_or_create(&self, server_id: i64) -> sqlx::Result<ServerManagement> {
        self.queries.server_management_get_or_create1(server_id).await?;
        self.queries
            .server_management_get_or_create2(server_id)
            .await
    }
}

fn map(value: ServerManagement) -> sqlx::Result<ServerManagementDto> {
    Ok(ServerManagementDto {
        server_id: value.server_id,
        maintenance_mode: value.maintenance_mode != 0,
        // Flash binds nullable columns as non-Option; "" is the NULL encoding.
        maintenance_message: value.maintenance_message.filter(|v| !v.is_empty()),
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

// ── JSON payload contract (moved from the retired api::dto layer) ──────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCleanupPolicyDto {
    pub containers: bool,
    pub images: bool,
    pub networks: bool,
    pub volumes: bool,
    pub packages: bool,
}

impl Default for ServerCleanupPolicyDto {
    fn default() -> Self {
        Self {
            containers: true,
            images: true,
            networks: true,
            volumes: false,
            packages: false,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateServerManagementDto {
    pub maintenance_mode: bool,
    pub maintenance_message: Option<String>,
    pub labels: BTreeMap<String, String>,
    pub cleanup_policy: ServerCleanupPolicyDto,
    pub gpu_enabled: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ServerManagementDto {
    pub server_id: i64,
    pub maintenance_mode: bool,
    pub maintenance_message: Option<String>,
    pub labels: BTreeMap<String, String>,
    pub cleanup_policy: ServerCleanupPolicyDto,
    pub gpu_enabled: bool,
    pub updated_at: i64,
}
