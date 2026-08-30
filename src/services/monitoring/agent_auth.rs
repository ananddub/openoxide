use auto_di::singleton;
use sha2::{Digest, Sha256};
use std::sync::Arc;

use crate::db::queries::FlashQueries;
use crate::flash_gen::monitoring_agents::{MonitoringAgentsRotateParams, MonitoringAgentsStatusRow};

#[derive(Clone, Debug, serde::Serialize)]
pub struct MonitoringAgentStatus {
    pub server_id: i64,
    pub organization_id: i64,
    pub last_seen_at: Option<i64>,
    pub agent_version: Option<String>,
}

impl From<MonitoringAgentsStatusRow> for MonitoringAgentStatus {
    fn from(row: MonitoringAgentsStatusRow) -> Self {
        Self {
            server_id: row.server_id,
            organization_id: row.organization_id,
            last_seen_at: row.last_seen_at,
            agent_version: row.agent_version,
        }
    }
}

pub struct MonitoringAgentAuth {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl MonitoringAgentAuth {
    pub fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn rotate(
        &self,
        server_id: i64,
        organization_id: i64,
    ) -> Result<String, sqlx::Error> {
        let token = format!("rma_{}", uuid::Uuid::new_v4().simple());
        let hash = hash_token(&token);
        self.queries
            .monitoring_agents_rotate(&MonitoringAgentsRotateParams {
                server_id,
                organization_id,
                token_hash: hash,
                query_token: token.clone(),
            })
            .await?;
        Ok(token)
    }

    pub async fn register_token(
        &self,
        server_id: i64,
        organization_id: i64,
        token: &str,
    ) -> Result<(), sqlx::Error> {
        let hash = hash_token(token);
        self.queries
            .monitoring_agents_rotate(&MonitoringAgentsRotateParams {
                server_id,
                organization_id,
                token_hash: hash,
                query_token: token.to_string(),
            })
            .await
    }

    pub async fn server_belongs_to_organization(
        &self,
        server_id: i64,
        organization_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let linked = self
            .queries
            .monitoring_agents_server_linked_service_count(server_id, organization_id)
            .await?;
        if linked != 0 {
            return Ok(true);
        }
        let existing = self
            .queries
            .monitoring_agents_get_organization_id(server_id)
            .await;
        match existing {
            Ok(id) => Ok(id == organization_id),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub async fn authenticate(&self, server_id: i64, token: &str) -> Result<bool, sqlx::Error> {
        let stored = match self
            .queries
            .monitoring_agents_get_token_hash(server_id)
            .await
        {
            Ok(stored) => stored,
            Err(sqlx::Error::RowNotFound) => return Ok(false),
            Err(e) => return Err(e),
        };
        if stored != hash_token(token) {
            return Ok(false);
        }
        self.queries.monitoring_agents_touch_seen(server_id).await?;
        Ok(true)
    }

    pub async fn organization_id(&self, server_id: i64) -> Result<Option<i64>, sqlx::Error> {
        match self
            .queries
            .monitoring_agents_get_organization_id(server_id)
            .await
        {
            Ok(id) => Ok(Some(id)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn touch_seen(&self, server_id: i64) -> Result<(), sqlx::Error> {
        self.queries
            .monitoring_agents_touch_seen(server_id)
            .await
    }

    pub async fn query_token(&self, server_id: i64) -> Result<Option<String>, sqlx::Error> {
        match self.queries.monitoring_agents_query_token(server_id).await {
            Ok(token) => Ok(token),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn status(
        &self,
        server_id: i64,
    ) -> Result<Option<MonitoringAgentStatus>, sqlx::Error> {
        match self.queries.monitoring_agents_status(server_id).await {
            Ok(row) => Ok(Some(MonitoringAgentStatus::from(row))),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

fn hash_token(token: &str) -> String {
    format!("{:x}", Sha256::digest(token.as_bytes()))
}
