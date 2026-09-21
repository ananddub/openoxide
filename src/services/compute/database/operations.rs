use auto_di::resolve;

use super::{DatabaseKind, DatabaseOperation, DatabaseOperationResult, DatabaseService};
use crate::flash_gen::database_extra::{
    DbCreateQueuedDatabaseDeploymentForServerParams, DbCreateQueuedDatabaseDeploymentGlobalParams,
};
use crate::flash_gen::services::UpdateServiceStatusWithErrorParams;
use crate::utils::builder::queue::BuilderQueue;

impl DatabaseService {
    pub async fn run_operation(
        &self,
        kind: DatabaseKind,
        id: i64,
        operation: DatabaseOperation,
        actor_id: i64,
    ) -> sqlx::Result<DatabaseOperationResult> {
        if matches!(operation, DatabaseOperation::Stop) {
            self.cancel_operation(kind, id).await?;
            return Ok(DatabaseOperationResult {
                database: self.get_by_id(kind, id).await?,
                operation: DatabaseOperation::Stop,
            });
        }

        let running_deployment = self
            .queries
            .deployments_has_running_database_deployment(&id.to_string())
            .await?;
        if running_deployment {
            return Err(sqlx::Error::Protocol(
                "database deployment already queued or running; cancel it first".into(),
            ));
        }

        resolve::<BuilderQueue>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .ensure_capacity()
            .await?;

        let kind_str = kind.as_str();

        let server_row = self.queries.services_get_server_id_and_name(id).await?;
        let server_id = server_row.server_id;
        let name = server_row.app_name;
        // Same contract as the deleted ServerManagementRepository::assert_deployable:
        // servers in maintenance mode refuse new deployments.
        if let Some(server_id) = server_id {
            self.queries.server_management_get_or_create1(server_id).await?;
            let row = self
                .queries
                .server_management_get_or_create2(server_id)
                .await?;
            if row.maintenance_mode != 0 {
                return Err(sqlx::Error::Protocol(
                    row.maintenance_message
                        .unwrap_or_else(|| "server is in maintenance mode".into()),
                ));
            }
        }

        if matches!(operation, DatabaseOperation::Stop) {
            let _ = self.cancel_operation(kind, id).await;
            return Ok(DatabaseOperationResult {
                database: self.get_by_id(kind, id).await?,
                operation,
            });
        }

        let target_status = match operation {
            DatabaseOperation::Deploy | DatabaseOperation::Redeploy => "STARTING",
            _ => "STARTING",
        };

        self.queries
            .update_service_status(target_status, id)
            .await?;
        self.cache
            .invalidate(&crate::core::cache::CacheKey::Database(id))
            .await;

        let log_path = format!("pending-db-{}", id);
        let deployment_id = match server_id {
            None => {
                self.queries
                    .db_create_queued_database_deployment_global(
                        &DbCreateQueuedDatabaseDeploymentGlobalParams {
                            title: operation.title().to_string(),
                            description: format!(
                                "{} requested for database {}",
                                operation.as_str(),
                                name
                            ),
                            log_path,
                            operation: operation.as_str().to_string(),
                            service_id: id,
                            database_kind: kind_str.to_string(),
                            user_id: actor_id,
                        },
                    )
                    .await?
            }
            Some(server_id) => {
                self.queries
                    .db_create_queued_database_deployment_for_server(
                        &DbCreateQueuedDatabaseDeploymentForServerParams {
                            title: operation.title().to_string(),
                            description: format!(
                                "{} requested for database {}",
                                operation.as_str(),
                                name
                            ),
                            log_path,
                            operation: operation.as_str().to_string(),
                            service_id: id,
                            database_kind: kind_str.to_string(),
                            server_id,
                            user_id: actor_id,
                        },
                    )
                    .await?
            }
        };

        let log_path = crate::utils::paths::openoxide_paths().deployment_log_file(deployment_id);
        self.queries
            .deployments_update_log_path(&log_path, deployment_id)
            .await?;

        if let Ok(mut log) =
            crate::utils::builder::queue::deployment_log::DeploymentLog::open(deployment_id).await
        {
            let _ = log
                .write_line(&format!(
                    "[QUEUED] database deployment queued for {}",
                    operation.as_str()
                ))
                .await;
        }

        resolve::<BuilderQueue>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .notify();

        Ok(DatabaseOperationResult {
            database: self.get_by_id(kind, id).await?,
            operation,
        })
    }

    pub async fn stop_operation(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<bool> {
        self.cancel_operation(kind, id).await
    }

    pub async fn cancel_operation(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<bool> {
        let db_record = self.get_by_id(kind, id).await.ok();
        let current_st_str = db_record
            .as_ref()
            .map(|d| d.app_status.as_str())
            .unwrap_or("IDLE");
        let current_status =
            crate::services::compute::database::types::DatabaseRuntimeStatus::from_str(current_st_str)
                .unwrap_or(crate::services::compute::database::types::DatabaseRuntimeStatus::Idle);

        let (intermediate, final_st) = if current_status.is_building() {
            (
                crate::services::compute::database::types::DatabaseRuntimeStatus::Cancelling.as_str(),
                crate::services::compute::database::types::DatabaseRuntimeStatus::Cancelled.as_str(),
            )
        } else {
            (
                crate::services::compute::database::types::DatabaseRuntimeStatus::Stopping.as_str(),
                crate::services::compute::database::types::DatabaseRuntimeStatus::Stopped.as_str(),
            )
        };

        let _ = self
            .queries
            .update_service_status(intermediate, id)
            .await;
        self.cache
            .invalidate(&crate::core::cache::CacheKey::Database(id))
            .await;

        if let Ok(queue) = resolve::<BuilderQueue>().await {
            let _ = queue.cancel_queued_database(id).await;
        }

        if let Ok(state) = resolve::<crate::utils::builder::hash_state::ApplicationState>().await {
            state.cancel_by_id(crate::utils::builder::custom_type::IdType::DatabaseId(id));
        }

        let _ = self.queries.deployments_cancel_queued_for_database(id).await;
        let _ = self
            .queries
            .deployments_request_cancel_database_deployment(id)
            .await;

        let app_name = db_record.as_ref().map(|d| d.app_name.clone());
        let server_id = db_record.as_ref().and_then(|d| d.server_id);
        let queries = self.queries.clone();
        let cache = self.cache.clone();

        // Spawn Docker container cleanup asynchronously so HTTP handler returns INSTANTLY (<5ms)
        tokio::spawn(async move {
            if let Some(ref app_name) = app_name {
                let docker_cli =
                    match crate::services::compute::compose::remote::agent_executor_for(server_id).await {
                        Ok(executor) => crate::utils::docker::DockerCli::from_executor(executor),
                        Err(error) => {
                            let reason = format!("could not stop database: {error}");
                            let _ = queries
                                .update_service_status_with_error(
                                    &UpdateServiceStatusWithErrorParams {
                                        app_status: "ERROR".into(),
                                        last_error: reason,
                                        id,
                                    },
                                )
                                .await;
                            cache
                                .invalidate(&crate::core::cache::CacheKey::Database(id))
                                .await;
                            return;
                        }
                    };

                let candidates = [
                    app_name.clone(),
                    format!("{}_db", app_name),
                    format!("{}-db", app_name),
                ];

                if let Ok(services) = tokio::time::timeout(
                    std::time::Duration::from_secs(4),
                    docker_cli
                        .services()
                        .list()
                        .filter(crate::utils::docker::query::ServiceFilter::name(format!(
                            "{}_",
                            app_name
                        )))
                        .run_json(),
                )
                .await
                .unwrap_or(Err(
                    crate::utils::docker::error::DockerError::CommandFailed {
                        code: None,
                        stderr: "timeout".into(),
                    },
                )) {
                    for s in services {
                        let _ = tokio::time::timeout(
                            std::time::Duration::from_secs(3),
                            docker_cli.services().remove(&s.name).run(),
                        )
                        .await;
                    }
                }

                for candidate in &candidates {
                    let _ = tokio::time::timeout(
                        std::time::Duration::from_secs(3),
                        docker_cli.services().remove(candidate).run(),
                    )
                    .await;
                    let _ = tokio::time::timeout(
                        std::time::Duration::from_secs(3),
                        docker_cli.container(candidate).stop().run(),
                    )
                    .await;
                }
            }

            let _ = queries.update_service_status(final_st, id).await;
            cache
                .invalidate(&crate::core::cache::CacheKey::Database(id))
                .await;
        });

        Ok(true)
    }

    #[allow(dead_code)]
    async fn database_server_id_and_name(
        &self,
        kind: DatabaseKind,
        id: i64,
    ) -> sqlx::Result<(Option<i64>, String)> {
        let _ = kind;
        let row = self.queries.services_get_server_id_and_name(id).await?;
        Ok((row.server_id, row.app_name))
    }

    #[allow(dead_code)]
    async fn database_docker(
        &self,
        server_id: Option<i64>,
    ) -> sqlx::Result<crate::utils::docker::DockerCli> {
        match server_id {
            Some(server_id) => {
                let executor = crate::services::compute::compose::remote::agent_executor(server_id)
                    .await
                    .map_err(sqlx::Error::Protocol)?;
                Ok(crate::utils::docker::DockerCli::from_executor(executor))
            }
            None => crate::services::compute::compose::remote::agent_executor_for(None)
                .await
                .map(crate::utils::docker::DockerCli::from_executor)
                .map_err(sqlx::Error::Protocol),
        }
    }
}
