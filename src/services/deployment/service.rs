use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use super::types::{CancelDeploymentResult, DeploymentListFilter};
use crate::db::models::deployments::Deployment;
use crate::db::repository::PostgresRepository;
use crate::repository::{ApplicationRepository, ComposeProjectRepository, DeploymentRepository};
use crate::utils::builder::custom_type::IdType;

pub struct DeploymentService {
    db: Arc<SqlitePool>,
    pub(super) repo_deploy: Arc<DeploymentRepository>,
    pub(super) repo_app: Arc<ApplicationRepository>,
    pub(super) repo_compose: Arc<ComposeProjectRepository>,
    pub(super) repo_postgres: Arc<PostgresRepository>,
}

#[singleton]
impl DeploymentService {
    pub(in crate::services::deployment) fn database(&self) -> &SqlitePool {
        self.db.as_ref()
    }

    fn new(
        db: Arc<SqlitePool>,
        repo_deploy: Arc<DeploymentRepository>,
        repo_app: Arc<ApplicationRepository>,
        repo_compose: Arc<ComposeProjectRepository>,
        repo_postgres: Arc<PostgresRepository>,
    ) -> Self {
        Self {
            db,
            repo_deploy,
            repo_app,
            repo_compose,
            repo_postgres,
        }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Deployment> {
        self.repo_deploy
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn list(&self, filter: DeploymentListFilter) -> sqlx::Result<Vec<Deployment>> {
        let status = normalize_filter_text(filter.status);
        let state = normalize_filter_text(filter.state);
        let limit = filter.limit.clamp(1, 200);
        let offset = filter.offset.max(0);

        self.repo_deploy
            .list_filtered(
                status,
                state,
                filter.application_id,
                filter.compose_id,
                filter.database_id,
                filter.server_id,
                limit,
                offset,
            )
            .await
    }

    pub async fn list_running(&self, limit: i64, offset: i64) -> sqlx::Result<Vec<Deployment>> {
        self.list(DeploymentListFilter {
            status: Some("RUNNING".into()),
            limit,
            offset,
            ..Default::default()
        })
        .await
    }

    pub async fn cancel(&self, deployment_id: i64) -> sqlx::Result<CancelDeploymentResult> {
        let (application_id, compose_id, database_id, status) = self
            .repo_deploy
            .get_cancel_info(deployment_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;

        if status == "QUEUED" {
            self.repo_deploy
                .cancel_queued_deployment(deployment_id)
                .await?;
            return Ok(CancelDeploymentResult::CancelRequested);
        }

        if status != "RUNNING" {
            return Ok(CancelDeploymentResult::NotRunning);
        }

        let Some(target_id) = application_id
            .map(IdType::AppId)
            .or_else(|| compose_id.map(IdType::ComposeId))
            .or_else(|| database_id.map(IdType::DatabaseId))
        else {
            return Ok(CancelDeploymentResult::NotCancellable);
        };

        let state = auto_di::resolve::<crate::utils::builder::hash_state::ApplicationState>()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        if !state.cancel_by_id(target_id) {
            return Ok(CancelDeploymentResult::NotActiveInThisProcess);
        }

        self.repo_deploy.set_cancel_requested(deployment_id).await?;

        Ok(CancelDeploymentResult::CancelRequested)
    }

    pub async fn list_active_components(
        &self,
    ) -> sqlx::Result<Vec<crate::utils::builder::custom_type::ActiveDeploySnapshot>> {
        let state = auto_di::resolve::<crate::utils::builder::hash_state::ApplicationState>()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(state.active_deployments())
    }

    pub async fn subscribe_component(
        &self,
        component_id: IdType,
    ) -> sqlx::Result<Option<crate::utils::builder::custom_type::DeploySubscription>> {
        let state = auto_di::resolve::<crate::utils::builder::hash_state::ApplicationState>()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(state.subscribe(component_id))
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<bool> {
        let deployment = match self.repo_deploy.get_by_id(id).await? {
            Some(d) => d,
            None => return Ok(false),
        };

        if deployment.status == "QUEUED" || deployment.status == "RUNNING" {
            let _ = self.cancel(id).await;
        }

        self.repo_deploy.delete(id).await?;

        if !deployment.log_path.is_empty() {
            let path = std::path::Path::new(&deployment.log_path);
            if path.exists() {
                let _ = tokio::fs::remove_file(path).await;
            }
        }

        Ok(true)
    }

    pub async fn clear_all(
        &self,
        application_id: Option<i64>,
        compose_id: Option<i64>,
        database_id: Option<i64>,
        server_id: Option<i64>,
    ) -> sqlx::Result<u64> {
        let log_paths = self
            .repo_deploy
            .delete_filtered_finished(application_id, compose_id, database_id, server_id)
            .await?;

        let count = log_paths.len() as u64;
        for log_path in log_paths {
            if !log_path.is_empty() {
                let path = std::path::Path::new(&log_path);
                if path.exists() {
                    let _ = tokio::fs::remove_file(path).await;
                }
            }
        }

        Ok(count)
    }
}

fn normalize_filter_text(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_ascii_uppercase())
        .filter(|value| !value.is_empty())
}
