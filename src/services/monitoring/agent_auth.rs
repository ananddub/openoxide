use auto_di::singleton;
use sha2::{Digest, Sha256};
use std::sync::Arc;

use crate::repository::{MonitoringAgentRepository, MonitoringAgentStatus};

pub struct MonitoringAgentAuth {
    repo: Arc<MonitoringAgentRepository>,
}

#[singleton]
impl MonitoringAgentAuth {
    pub fn new(repo: Arc<MonitoringAgentRepository>) -> Self {
        Self { repo }
    }

    pub async fn rotate(
        &self,
        server_id: i64,
        organization_id: i64,
    ) -> Result<String, sqlx::Error> {
        let token = format!("rma_{}", uuid::Uuid::new_v4().simple());
        let hash = hash_token(&token);
        self.repo
            .rotate(server_id, organization_id, &token, &hash)
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
        self.repo
            .rotate(server_id, organization_id, token, &hash)
            .await
    }

    pub async fn server_belongs_to_organization(
        &self,
        server_id: i64,
        organization_id: i64,
    ) -> Result<bool, sqlx::Error> {
        self.repo
            .server_belongs_to_organization(server_id, organization_id)
            .await
    }

    pub async fn authenticate(&self, server_id: i64, token: &str) -> Result<bool, sqlx::Error> {
        let stored = self.repo.get_token_hash(server_id).await?;
        let Some(stored) = stored else {
            return Ok(false);
        };
        if stored != hash_token(token) {
            return Ok(false);
        }
        self.repo.touch_seen(server_id).await?;
        Ok(true)
    }

    pub async fn organization_id(&self, server_id: i64) -> Result<Option<i64>, sqlx::Error> {
        self.repo.get_organization_id(server_id).await
    }

    pub async fn touch_seen(&self, server_id: i64) -> Result<(), sqlx::Error> {
        self.repo.touch_seen(server_id).await
    }

    pub async fn query_token(&self, server_id: i64) -> Result<Option<String>, sqlx::Error> {
        self.repo.query_token(server_id).await
    }

    pub async fn status(
        &self,
        server_id: i64,
    ) -> Result<Option<MonitoringAgentStatus>, sqlx::Error> {
        self.repo.status(server_id).await
    }
}

fn hash_token(token: &str) -> String {
    format!("{:x}", Sha256::digest(token.as_bytes()))
}
