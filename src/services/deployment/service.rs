use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use super::types::{CancelDeploymentResult, DeploymentListFilter};
use crate::db::models::deployments::Deployment;
use crate::db::queries::FlashQueries;
use crate::flash_gen::models::Deployments as GenDeployment;
use crate::utils::builder::custom_type::IdType;

impl From<GenDeployment> for Deployment {
    fn from(value: GenDeployment) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
            status: value.status,
            state: value.state,
            log_path: value.log_path,
            pid: value.pid,
            error_message: value.error_message,
            operation: value.operation,
            is_preview_deployment: value.is_preview_deployment,
            started_at: value.started_at,
            last_state_at: value.last_state_at,
            finished_at: value.finished_at,
            service_id: value.service_id,
            server_id: value.server_id,
            created_at: value.created_at,
            database_kind: value.database_kind,
        }
    }
}

pub struct DeploymentService {
    db: Arc<SqlitePool>,
    pub(super) queries: Arc<FlashQueries>,
}

#[singleton]
impl DeploymentService {
    pub(in crate::services::deployment) fn database(&self) -> &SqlitePool {
        self.db.as_ref()
    }

    fn new(db: Arc<SqlitePool>, queries: Arc<FlashQueries>) -> Self {
        Self { db, queries }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Deployment> {
        let row = self.queries.get_deployment_by_i_d(id).await?;
        Ok(row.into())
    }

    pub async fn list(&self, filter: DeploymentListFilter) -> sqlx::Result<Vec<Deployment>> {
        let status = normalize_filter_text(filter.status);
        let state = normalize_filter_text(filter.state);
        let limit = filter.limit.clamp(1, 200);
        let offset = filter.offset.max(0);
        let service_id = filter
            .application_id
            .or(filter.compose_id)
            .or(filter.database_id);

        let rows: Vec<GenDeployment> = if let Some(service_id) = service_id {
            self.queries.list_deployments_by_application(service_id).await?
        } else if let Some(server_id) = filter.server_id {
            self.queries.list_deployments_by_server(server_id).await?
        } else {
            self.queries.list_recent_deployments(limit + offset).await?
        };

        let mut rows: Vec<GenDeployment> = rows
            .into_iter()
            .filter(|row| status.as_deref().is_none_or(|value| row.status == value))
            .filter(|row| state.as_deref().is_none_or(|value| row.state == value))
            .filter(|row| filter.server_id.is_none_or(|value| row.server_id == Some(value)))
            .collect();
        rows = rows
            .into_iter()
            .skip(offset as usize)
            .take(limit as usize)
            .collect();
        Ok(rows.into_iter().map(Into::into).collect())
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
        let row = self
            .queries
            .deployments_get_cancel_info(deployment_id)
            .await?;

        if row.status == "QUEUED" {
            self.queries
                .deployments_cancel_queued_deployment(deployment_id)
                .await?;
            return Ok(CancelDeploymentResult::CancelRequested);
        }

        if row.status != "RUNNING" {
            return Ok(CancelDeploymentResult::NotRunning);
        }

        let Some(service_id) = row.service_id else {
            return Ok(CancelDeploymentResult::NotCancellable);
        };

        let target_id = match self.queries.deployments_claim_service_type(service_id).await {
            Ok(kind) if kind == "COMPOSE" => IdType::ComposeId(service_id),
            Ok(kind) if kind == "DATABASE" => IdType::DatabaseId(service_id),
            Ok(_) => IdType::AppId(service_id),
            Err(_) => return Ok(CancelDeploymentResult::NotCancellable),
        };

        let state = auto_di::resolve::<crate::utils::builder::hash_state::ApplicationState>()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        if !state.cancel_by_id(target_id) {
            return Ok(CancelDeploymentResult::NotActiveInThisProcess);
        }

        self.queries
            .deployments_set_cancel_requested(deployment_id)
            .await?;

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
        let deployment: Deployment = match self.queries.get_deployment_by_i_d(id).await {
            Ok(row) => row.into(),
            Err(sqlx::Error::RowNotFound) => return Ok(false),
            Err(error) => return Err(error),
        };

        if deployment.status == "QUEUED" || deployment.status == "RUNNING" {
            let _ = self.cancel(id).await;
        }

        self.queries.deployments_delete(id).await?;

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
        let service_id = application_id.or(compose_id).or(database_id);
        let log_paths = match (service_id, server_id) {
            (Some(service_id), server_filter) => {
                let rows = self
                    .queries
                    .list_deployments_by_application(service_id)
                    .await?;
                let targets: Vec<GenDeployment> = rows
                    .into_iter()
                    .filter(|row| is_finished(row))
                    .filter(|row| server_filter.is_none_or(|value| row.server_id == Some(value)))
                    .collect();
                let paths = targets
                    .iter()
                    .map(|row| row.log_path.clone())
                    .collect::<Vec<String>>();
                for row in &targets {
                    self.queries.deployments_delete(row.id).await?;
                }
                paths
            }
            (None, Some(server_id)) => {
                let rows = self.queries.list_deployments_by_server(server_id).await?;
                let targets: Vec<GenDeployment> =
                    rows.into_iter().filter(|row| is_finished(row)).collect();
                let paths = targets
                    .iter()
                    .map(|row| row.log_path.clone())
                    .collect::<Vec<String>>();
                for row in &targets {
                    self.queries.deployments_delete(row.id).await?;
                }
                paths
            }
            (None, None) => {
                let paths = self
                    .queries
                    .deployments_delete_filtered_finished1()
                    .await?;
                self.queries.deployments_delete_filtered_finished2().await?;
                paths
            }
        };

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

fn is_finished(deployment: &GenDeployment) -> bool {
    deployment.status != "QUEUED" && deployment.status != "RUNNING"
}

fn normalize_filter_text(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_ascii_uppercase())
        .filter(|value| !value.is_empty())
}
