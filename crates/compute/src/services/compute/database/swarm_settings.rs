use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::DatabaseKind;
use crate::db::queries::FlashQueries;
use crate::flash_gen::services::UpdateServiceSwarmConfigParams;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DatabaseSwarmSettingsDto {
    pub health_check_swarm: Option<String>,
    pub restart_policy_swarm: Option<String>,
    pub placement_swarm: Option<String>,
    pub update_config_swarm: Option<String>,
    pub rollback_config_swarm: Option<String>,
    pub mode_swarm: Option<String>,
    pub labels_swarm: Option<String>,
    pub network_swarm: Option<String>,
    pub endpoint_spec_swarm: Option<String>,
    pub ulimits_swarm: Option<String>,
    pub stop_grace_period_swarm: Option<i64>,
}

pub struct DatabaseSwarmSettingsService {
    queries: Arc<FlashQueries>,
}

#[auto_di::singleton]
impl DatabaseSwarmSettingsService {
    pub fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn get(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<DatabaseSwarmSettingsDto> {
        let _ = kind;
        let row = self.queries.database_get_by_id(id).await?;
        Ok(DatabaseSwarmSettingsDto {
            health_check_swarm: row.health_check_swarm,
            restart_policy_swarm: row.restart_policy_swarm,
            placement_swarm: row.placement_swarm,
            update_config_swarm: row.update_config_swarm,
            rollback_config_swarm: row.rollback_config_swarm,
            mode_swarm: row.mode_swarm,
            labels_swarm: row.labels_swarm,
            network_swarm: row.network_swarm,
            endpoint_spec_swarm: row.endpoint_spec_swarm,
            ulimits_swarm: row.ulimits_swarm,
            stop_grace_period_swarm: row.stop_grace_period_swarm,
        })
    }

    pub async fn update(
        &self,
        kind: DatabaseKind,
        id: i64,
        value: DatabaseSwarmSettingsDto,
    ) -> sqlx::Result<()> {
        let _ = kind;
        let params = UpdateServiceSwarmConfigParams {
            health_check_swarm: value.health_check_swarm.unwrap_or_default(),
            restart_policy_swarm: value.restart_policy_swarm.unwrap_or_default(),
            placement_swarm: value.placement_swarm.unwrap_or_default(),
            update_config_swarm: value.update_config_swarm.unwrap_or_default(),
            rollback_config_swarm: value.rollback_config_swarm.unwrap_or_default(),
            mode_swarm: value.mode_swarm.unwrap_or_default(),
            labels_swarm: value.labels_swarm.unwrap_or_default(),
            network_swarm: value.network_swarm.unwrap_or_default(),
            endpoint_spec_swarm: value.endpoint_spec_swarm.unwrap_or_default(),
            ulimits_swarm: value.ulimits_swarm.unwrap_or_default(),
            stop_grace_period_swarm: value.stop_grace_period_swarm.unwrap_or_default(),
            id,
        };
        self.queries.update_service_swarm_config(&params).await
    }
}
