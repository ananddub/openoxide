use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use super::types::{CancelDeploymentResult, DeploymentListFilter};
use crate::db::queries::FlashQueries;
use crate::flash_gen::models::Deployments;
use crate::utils::builder::custom_type::{ActiveDeploySnapshot, IdType};

pub struct DeploymentService {
    db: Arc<SqlitePool>,
    pub(super) queries: Arc<FlashQueries>,
}

#[cfg(test)]
impl DeploymentService {
    pub(crate) fn for_tests(db: Arc<SqlitePool>, queries: Arc<FlashQueries>) -> Self {
        Self { db, queries }
    }
}

#[singleton]
impl DeploymentService {
    pub(in crate::services::compute::engine) fn database(&self) -> &SqlitePool {
        self.db.as_ref()
    }

    fn new(db: Arc<SqlitePool>, queries: Arc<FlashQueries>) -> Self {
        Self { db, queries }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Deployments> {
        self.queries.get_deployment_by_i_d(id).await
    }

    /// Email of the user who triggered a deployment (0 / unknown → `None`,
    /// which the UI renders as "System").
    pub async fn actor_email(&self, user_id: i64) -> Option<String> {
        if user_id == 0 {
            return None;
        }
        self.queries
            .get_user_by_i_d(user_id)
            .await
            .ok()
            .and_then(|user| user.email)
    }

    pub async fn list(&self, filter: DeploymentListFilter) -> sqlx::Result<Vec<Deployments>> {
        let limit = filter.limit.clamp(1, 200);
        let offset = filter.offset.max(0);
        let service_id = filter
            .application_id
            .or(filter.compose_id)
            .or(filter.database_id);

        let rows: Vec<Deployments> = if let Some(service_id) = service_id {
            self.queries
                .list_deployments_by_application(service_id)
                .await?
        } else if let Some(server_id) = filter.server_id {
            self.queries.list_deployments_by_server(server_id).await?
        } else {
            self.queries.list_recent_deployments(limit + offset).await?
        };

        Ok(apply_filter(rows, &filter, service_id))
    }

    /// Loads a deployment and rejects it unless it belongs to the caller's
    /// organization. Deployments inherit tenancy from their service's
    /// environment → project chain, or — for standalone server jobs — from
    /// the server row.
    pub async fn ensure_organization_access(
        &self,
        id: i64,
        organization_id: i64,
    ) -> sqlx::Result<()> {
        let deployment = self.get_by_id(id).await?;
        if self
            .deployment_belongs_to_organization(&deployment, organization_id)
            .await?
        {
            Ok(())
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }

    /// Rejects service ids (application/compose/database rows share the
    /// `services` table) outside the caller's organization.
    pub async fn ensure_service_organization_access(
        &self,
        service_id: i64,
        organization_id: i64,
    ) -> sqlx::Result<()> {
        if self
            .queries
            .services_belongs_to_organization(service_id, organization_id)
            .await?
            > 0
        {
            Ok(())
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }

    async fn deployment_belongs_to_organization(
        &self,
        deployment: &Deployments,
        organization_id: i64,
    ) -> sqlx::Result<bool> {
        if let Some(service_id) = deployment.service_id {
            return Ok(self
                .queries
                .services_belongs_to_organization(service_id, organization_id)
                .await?
                > 0);
        }
        if let Some(server_id) = deployment.server_id {
            let server = self.queries.get_server_by_i_d(server_id).await?;
            return Ok(server.organization_id == organization_id);
        }
        Ok(false)
    }

    /// Lists deployments visible to an organization. Rows owned by another
    /// tenant are never returned; explicit service/server filters pointing
    /// outside the organization report not-found.
    pub async fn list_for_organization(
        &self,
        organization_id: i64,
        filter: DeploymentListFilter,
    ) -> sqlx::Result<Vec<Deployments>> {
        if let Some(service_id) = filter
            .application_id
            .or(filter.compose_id)
            .or(filter.database_id)
        {
            let owned = self
                .queries
                .services_belongs_to_organization(service_id, organization_id)
                .await?;
            if owned == 0 {
                return Err(sqlx::Error::RowNotFound);
            }
        }
        if let Some(server_id) = filter.server_id {
            let server = self.queries.get_server_by_i_d(server_id).await?;
            if server.organization_id != organization_id {
                return Err(sqlx::Error::RowNotFound);
            }
        }
        let service_id = filter
            .application_id
            .or(filter.compose_id)
            .or(filter.database_id);
        let rows = self.list_organization_deployments(organization_id).await?;
        Ok(apply_filter(rows, &filter, service_id))
    }

    pub async fn list_running(&self, limit: i64, offset: i64) -> sqlx::Result<Vec<Deployments>> {
        self.list(DeploymentListFilter {
            status: Some("RUNNING".into()),
            limit,
            offset,
            ..Default::default()
        })
        .await
    }

    pub async fn list_running_for_organization(
        &self,
        organization_id: i64,
        limit: i64,
        offset: i64,
    ) -> sqlx::Result<Vec<Deployments>> {
        self.list_for_organization(
            organization_id,
            DeploymentListFilter {
                status: Some("RUNNING".into()),
                limit,
                offset,
                ..Default::default()
            },
        )
        .await
    }

    async fn list_organization_deployments(
        &self,
        organization_id: i64,
    ) -> sqlx::Result<Vec<Deployments>> {
        sqlx::query_as::<_, Deployments>(
            // Service-owned rows follow the service's tenant even when they
            // run on a shared server (e.g. the panel host); only standalone
            // server jobs fall back to the server's organization.
            "SELECT d.* FROM deployments d \
             WHERE d.service_id IN (SELECT s.id FROM services s \
                                    JOIN environments e ON e.id = s.environment_id \
                                    JOIN projects p ON p.id = e.project_id \
                                    WHERE p.organization_id = ?) \
                OR (d.service_id IS NULL AND d.server_id IN \
                    (SELECT id FROM servers WHERE organization_id = ?)) \
             ORDER BY d.created_at DESC, d.id DESC",
        )
        .bind(organization_id)
        .bind(organization_id)
        .fetch_all(self.database())
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

        let target_id = match self
            .queries
            .deployments_claim_service_type(service_id)
            .await
        {
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

    pub async fn list_active_components_for_organization(
        &self,
        organization_id: i64,
    ) -> sqlx::Result<Vec<ActiveDeploySnapshot>> {
        let state = auto_di::resolve::<crate::utils::builder::hash_state::ApplicationState>()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let mut visible = Vec::new();
        for component in state.active_deployments() {
            let service_id = match component.id {
                IdType::AppId(id) | IdType::ComposeId(id) | IdType::DatabaseId(id) => id,
            };
            if self
                .queries
                .services_belongs_to_organization(service_id, organization_id)
                .await?
                > 0
            {
                visible.push(component);
            }
        }
        Ok(visible)
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
        let deployment = match self.queries.get_deployment_by_i_d(id).await {
            Ok(row) => row,
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
                let targets: Vec<Deployments> = rows
                    .into_iter()
                    .filter(is_finished)
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
                let targets: Vec<Deployments> = rows.into_iter().filter(is_finished).collect();
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
                let paths = self.queries.deployments_delete_filtered_finished1().await?;
                self.queries.deployments_delete_filtered_finished2().await?;
                paths
            }
        };

        let count = log_paths.len() as u64;
        for log_path in log_paths {
            remove_log_file(&log_path).await;
        }

        Ok(count)
    }

    /// Deletes finished deployments (and their log files) that belong to an
    /// organization. Running and queued deployments are never touched.
    pub async fn clear_all_for_organization(&self, organization_id: i64) -> sqlx::Result<u64> {
        let rows = self.list_organization_deployments(organization_id).await?;
        let targets: Vec<Deployments> = rows.into_iter().filter(is_finished).collect();
        let count = targets.len() as u64;
        for row in targets {
            self.queries.deployments_delete(row.id).await?;
            remove_log_file(&row.log_path).await;
        }
        Ok(count)
    }
}

fn apply_filter(
    rows: Vec<Deployments>,
    filter: &DeploymentListFilter,
    service_id: Option<i64>,
) -> Vec<Deployments> {
    let status = normalize_filter_text(filter.status.clone());
    let state = normalize_filter_text(filter.state.clone());
    let limit = filter.limit.clamp(1, 200);
    let offset = filter.offset.max(0);
    rows.into_iter()
        .filter(|row| status.as_deref().is_none_or(|value| row.status == value))
        .filter(|row| state.as_deref().is_none_or(|value| row.state == value))
        .filter(|row| service_id.is_none_or(|value| row.service_id == Some(value)))
        .filter(|row| {
            filter
                .server_id
                .is_none_or(|value| row.server_id == Some(value))
        })
        .skip(offset as usize)
        .take(limit as usize)
        .collect()
}

async fn remove_log_file(log_path: &str) {
    if log_path.is_empty() {
        return;
    }
    let path = std::path::Path::new(log_path);
    if path.exists() {
        let _ = tokio::fs::remove_file(path).await;
    }
}

fn is_finished(deployment: &Deployments) -> bool {
    deployment.status != "QUEUED" && deployment.status != "RUNNING"
}

fn normalize_filter_text(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_ascii_uppercase())
        .filter(|value| !value.is_empty())
}
