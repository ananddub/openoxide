use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use super::{
    BulkDeploymentAction, BulkDeploymentRequest, BulkDeploymentResult, GlobalResourceDto,
    GlobalSearchOptions, ServerDependencyView,
};
use crate::{
    repository::ServerRepository,
    services::deployment::{CancelDeploymentResult, DeploymentService},
};

pub struct GlobalOperationsService {
    db: Arc<SqlitePool>,
    deployments: Arc<DeploymentService>,
    servers: Arc<ServerRepository>,
}

#[singleton]
impl GlobalOperationsService {
    fn new(
        db: Arc<SqlitePool>,
        deployments: Arc<DeploymentService>,
        servers: Arc<ServerRepository>,
    ) -> Self {
        Self {
            db,
            deployments,
            servers,
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
        self.servers
            .get_by_id(server_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let value = self.servers.dependency_counts(server_id).await?;
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

    pub async fn search(
        &self,
        options: GlobalSearchOptions,
    ) -> sqlx::Result<Vec<GlobalResourceDto>> {
        let pattern = format!("%{}%", options.query.trim());
        let resource_type = options
            .resource_type
            .map(|value| value.trim().to_ascii_uppercase());
        let rows = sqlx::query!(r#"
            SELECT resource_type, id, name, COALESCE(status, '') status FROM (
                SELECT 'APPLICATION' resource_type, CAST(id AS TEXT) id, name, app_status status FROM applications
                UNION ALL SELECT 'COMPOSE', CAST(id AS TEXT), name, compose_status FROM compose_projects
                UNION ALL SELECT 'SERVER', CAST(id AS TEXT), name, server_status FROM servers
                UNION ALL SELECT 'PROJECT', CAST(id AS TEXT), name, NULL FROM projects
                UNION ALL SELECT 'ENVIRONMENT', CAST(id AS TEXT), name, NULL FROM environments
                UNION ALL SELECT 'POSTGRES', CAST(id AS TEXT), name, app_status FROM postgres_dbs
                UNION ALL SELECT 'MYSQL', CAST(id AS TEXT), name, app_status FROM mysql_dbs
                UNION ALL SELECT 'MARIADB', CAST(id AS TEXT), name, app_status FROM mariadb_dbs
                UNION ALL SELECT 'MONGO', CAST(id AS TEXT), name, app_status FROM mongo_dbs
                UNION ALL SELECT 'REDIS', CAST(id AS TEXT), name, app_status FROM redis_dbs
                UNION ALL SELECT 'LIBSQL', CAST(id AS TEXT), name, app_status FROM libsql_dbs
                UNION ALL SELECT 'DOMAIN', CAST(id AS TEXT), host, domain_type FROM domains
                UNION ALL SELECT 'CERTIFICATE', CAST(id AS TEXT), name, CASE auto_renew WHEN 1 THEN 'AUTO_RENEW' ELSE 'MANUAL' END FROM certificates
                UNION ALL SELECT 'REGISTRY', CAST(id AS TEXT), registry_name, registry_type FROM registries
                UNION ALL SELECT 'TAG', CAST(id AS TEXT), name, color FROM tags
                UNION ALL SELECT 'DATABASE_NETWORK', CAST(id AS TEXT), name, CASE external WHEN 1 THEN 'EXTERNAL' ELSE 'MANAGED' END FROM database_networks
            ) resources WHERE name LIKE ? COLLATE NOCASE AND (? IS NULL OR resource_type = ?) ORDER BY name LIMIT ? OFFSET ?
        "#, pattern, resource_type, resource_type, options.limit, options.offset).fetch_all(self.db.as_ref()).await?;
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
        let result = sqlx::query!("UPDATE deployments SET status = 'CANCELLED', state = 'CANCELLED', finished_at = strftime('%s', 'now'), last_state_at = strftime('%s', 'now') WHERE status = 'QUEUED'").execute(self.db.as_ref()).await?;
        Ok(result.rows_affected())
    }
}
