use std::sync::Arc;

use auto_di::singleton;

use super::{
    BulkDeploymentAction, BulkDeploymentRequest, BulkDeploymentResult, BulkResourceAction,
    BulkResourceItem, BulkResourceKind, BulkResourceRequest, BulkResourceResult, GlobalResourceDto,
    GlobalSearchOptions, ServerDependencyView,
};
use crate::{
    db::queries::FlashQueries,
    services::{
        compute::application::{ApplicationOperation, ApplicationService},
        compute::compose::{ComposeOperation, ComposeService},
        compute::database::{DatabaseOperation, DatabaseService},
        compute::engine::{CancelDeploymentResult, DeploymentService},
        infrastructure::server::{ServerCleanupService, ServerDependencyCounts},
    },
};

pub struct GlobalOperationsService {
    queries: Arc<FlashQueries>,
    deployments: Arc<DeploymentService>,
    applications: Arc<ApplicationService>,
    compose: Arc<ComposeService>,
    databases: Arc<DatabaseService>,
    cleanup: Arc<ServerCleanupService>,
}

#[singleton]
impl GlobalOperationsService {
    fn new(
        queries: Arc<FlashQueries>,
        deployments: Arc<DeploymentService>,
        applications: Arc<ApplicationService>,
        compose: Arc<ComposeService>,
        databases: Arc<DatabaseService>,
        cleanup: Arc<ServerCleanupService>,
    ) -> Self {
        Self {
            queries,
            deployments,
            applications,
            compose,
            databases,
            cleanup,
        }
    }

    pub async fn bulk_resources(
        &self,
        request: BulkResourceRequest,
    ) -> sqlx::Result<Vec<BulkResourceResult>> {
        if request.items.is_empty() || request.items.len() > 100 {
            return Err(sqlx::Error::Protocol(
                "items must contain 1 to 100 resources".into(),
            ));
        }
        let mut results = Vec::with_capacity(request.items.len());
        for item in request.items {
            let outcome = self
                .run_resource_action(request.resource_kind, request.action, &item)
                .await;
            results.push(BulkResourceResult {
                id: item.id,
                success: outcome.is_ok(),
                message: outcome.unwrap_or_else(|error| error.to_string()),
            });
        }
        Ok(results)
    }

    async fn run_resource_action(
        &self,
        kind: BulkResourceKind,
        action: BulkResourceAction,
        item: &BulkResourceItem,
    ) -> sqlx::Result<String> {
        match (kind, action) {
            (BulkResourceKind::Application, BulkResourceAction::Start) => self
                .applications
                .run_operation(item.id, ApplicationOperation::Start, 0)
                .await
                .map(|_| "start queued".into()),
            (BulkResourceKind::Application, BulkResourceAction::Stop) => self
                .applications
                .cancel_operation(item.id)
                .await
                .map(|_| "stopped".into()),
            (BulkResourceKind::Application, BulkResourceAction::Redeploy) => self
                .applications
                .run_operation(item.id, ApplicationOperation::Redeploy, 0)
                .await
                .map(|_| "redeploy queued".into()),
            (BulkResourceKind::Application, BulkResourceAction::Delete) => self
                .applications
                .delete(item.id)
                .await
                .map(|()| "deleted".into()),
            (BulkResourceKind::Compose, BulkResourceAction::Start) => self
                .compose
                .run_operation(item.id, ComposeOperation::Start, 0)
                .await
                .map(|_| "start queued".into()),
            (BulkResourceKind::Compose, BulkResourceAction::Stop) => self
                .compose
                .run_operation(item.id, ComposeOperation::Stop, 0)
                .await
                .map(|_| "stop queued".into()),
            (BulkResourceKind::Compose, BulkResourceAction::Redeploy) => self
                .compose
                .run_operation(item.id, ComposeOperation::Redeploy, 0)
                .await
                .map(|_| "redeploy queued".into()),
            (BulkResourceKind::Compose, BulkResourceAction::Delete) => self
                .compose
                .delete(item.id)
                .await
                .map(|()| "deleted".into()),
            (BulkResourceKind::Database, BulkResourceAction::Start) => self
                .databases
                .run_operation(database_kind(item)?, item.id, DatabaseOperation::Start, 0)
                .await
                .map(|_| "start queued".into()),
            (BulkResourceKind::Database, BulkResourceAction::Stop) => self
                .databases
                .run_operation(database_kind(item)?, item.id, DatabaseOperation::Stop, 0)
                .await
                .map(|_| "stop queued".into()),
            (BulkResourceKind::Database, BulkResourceAction::Redeploy) => self
                .databases
                .run_operation(database_kind(item)?, item.id, DatabaseOperation::Redeploy, 0)
                .await
                .map(|_| "redeploy queued".into()),
            (BulkResourceKind::Database, BulkResourceAction::Delete) => self
                .databases
                .delete(database_kind(item)?, item.id)
                .await
                .map(|()| "deleted".into()),
            (BulkResourceKind::Server, BulkResourceAction::Cleanup) => self
                .cleanup
                .run(item.id)
                .await
                .map(|_| "cleanup completed".into()),
            _ => Err(sqlx::Error::Protocol(
                "action is not supported for this resource kind".into(),
            )),
        }
    }

    pub async fn bulk_deployments(
        &self,
        request: BulkDeploymentRequest,
    ) -> sqlx::Result<Vec<BulkDeploymentResult>> {
        if request.deployment_ids.is_empty() || request.deployment_ids.len() > 100 {
            return Err(sqlx::Error::Protocol(
                "deployment_ids must contain 1 to 100 items".into(),
            ));
        }
        let mut results = Vec::with_capacity(request.deployment_ids.len());
        for id in request.deployment_ids {
            let outcome = match request.action {
                BulkDeploymentAction::Cancel => self.deployments.cancel(id).await,
            };
            let (success, message) = match outcome {
                Ok(CancelDeploymentResult::CancelRequested) => {
                    (true, "cancellation requested".into())
                }
                Ok(other) => (false, format!("{other:?}")),
                Err(sqlx::Error::RowNotFound) => (false, "deployment not found".into()),
                Err(error) => (false, error.to_string()),
            };
            results.push(BulkDeploymentResult {
                deployment_id: id,
                success,
                message,
            });
        }
        Ok(results)
    }

    pub async fn server_dependencies(&self, server_id: i64) -> sqlx::Result<ServerDependencyView> {
        self.queries.servers_get_by_id(server_id).await?;
        let value = self.dependency_counts(server_id).await?;
        Ok(ServerDependencyView {
            server_id,
            applications: value.applications,
            build_assignments: value.build_assignments,
            compose_projects: value.compose_projects,
            databases: value.databases,
            certificates: value.certificates,
            schedules: value.schedules,
            safe_to_delete: value.total() == 0,
        })
    }

    async fn dependency_counts(&self, server_id: i64) -> sqlx::Result<ServerDependencyCounts> {
        Ok(ServerDependencyCounts {
            applications: self.queries.servers_count_applications(server_id).await?,
            build_assignments: self
                .queries
                .servers_count_build_assignments(server_id)
                .await?,
            compose_projects: self.queries.servers_count_compose_projects(server_id).await?,
            databases: self.queries.servers_count_databases(server_id).await?,
            certificates: self.queries.servers_count_certificates(server_id).await?,
            schedules: self.queries.servers_count_schedules(server_id).await?,
        })
    }

    pub async fn search(
        &self,
        options: GlobalSearchOptions,
    ) -> sqlx::Result<Vec<GlobalResourceDto>> {
        let pattern = format!("%{}%", options.query.trim());
        let resource_type = options
            .resource_type
            .map(|value| value.trim().to_ascii_uppercase());
        #[derive(sqlx::FromRow)]
        struct SearchRow {
            resource_type: String,
            id: String,
            name: String,
            status: String,
        }
        let rows = sqlx::query_as::<_, SearchRow>(r#"
            SELECT resource_type, id, name, COALESCE(status, '') status FROM (
                SELECT 'APPLICATION' resource_type, CAST(id AS TEXT) id, name, app_status status FROM services WHERE service_type = 'APPLICATION'
                UNION ALL SELECT 'COMPOSE', CAST(id AS TEXT), name, app_status FROM services WHERE service_type = 'COMPOSE'
                UNION ALL SELECT 'SERVER', CAST(id AS TEXT), name, server_status FROM servers
                UNION ALL SELECT 'PROJECT', CAST(id AS TEXT), name, NULL FROM projects
                UNION ALL SELECT 'ENVIRONMENT', CAST(id AS TEXT), name, NULL FROM environments
                UNION ALL SELECT d.db_type, CAST(s.id AS TEXT), s.name, s.app_status FROM services s JOIN service_database d ON d.service_id = s.id
                UNION ALL SELECT 'DOMAIN', CAST(id AS TEXT), host, domain_type FROM domains
                UNION ALL SELECT 'CERTIFICATE', CAST(id AS TEXT), name, CASE auto_renew WHEN 1 THEN 'AUTO_RENEW' ELSE 'MANUAL' END FROM certificates
                UNION ALL SELECT 'REGISTRY', CAST(id AS TEXT), registry_name, registry_type FROM registries
                UNION ALL SELECT 'TAG', CAST(id AS TEXT), name, color FROM tags
                UNION ALL SELECT 'DATABASE_NETWORK', CAST(id AS TEXT), name, CASE external WHEN 1 THEN 'EXTERNAL' ELSE 'MANAGED' END FROM database_networks
            ) resources WHERE name LIKE ? COLLATE NOCASE AND (? IS NULL OR resource_type = ?) ORDER BY name LIMIT ? OFFSET ?
        "#)
        .bind(pattern)
        .bind(resource_type.as_deref())
        .bind(resource_type.as_deref())
        .bind(options.limit)
        .bind(options.offset)
        .fetch_all(&self.queries.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|row| GlobalResourceDto {
                resource_type: row.resource_type,
                id: row.id,
                name: row.name,
                status: (!row.status.is_empty()).then_some(row.status),
            })
            .collect())
    }

    pub async fn cleanup_deployment_queue(&self) -> sqlx::Result<u64> {
        let result = sqlx::query("UPDATE deployments SET status = 'CANCELLED', state = 'CANCELLED', finished_at = strftime('%s', 'now'), last_state_at = strftime('%s', 'now') WHERE status = 'QUEUED'").execute(&self.queries.pool).await?;
        Ok(result.rows_affected())
    }
}

fn database_kind(item: &BulkResourceItem) -> sqlx::Result<crate::services::compute::database::DatabaseKind> {
    item.database_kind
        .ok_or_else(|| sqlx::Error::Protocol("database_kind is required".into()))
}
