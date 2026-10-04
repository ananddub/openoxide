use auto_di::resolve;
use sqlx::SqlitePool;

use super::{
    ComposeOperation, ComposeOperationResult, ComposeRecord, ComposeService, ComposeStatus,
    ComposeType,
};
use crate::db::queries::FlashQueries;
use crate::flash_gen::compose_extra::{
    ComposeQueueDeploymentNoServerParams, ComposeQueueDeploymentWithServerParams,
};
use crate::utils::builder::queue::BuilderQueue;
use crate::utils::builder::{custom_type::IdType, hash_state::ApplicationState};
use crate::utils::docker::DockerCli;
use crate::utils::docker::query::ServiceFilter;
use crate::utils::exec::CommandExecutor;

impl ComposeService {
    pub async fn run_operation(
        &self,
        id: i64,
        operation: ComposeOperation,
        actor_id: i64,
    ) -> sqlx::Result<ComposeOperationResult> {
        let running_deployment = self
            .queries
            .deployments_has_running_compose_deployment(&id.to_string())
            .await?;
        if running_deployment {
            if matches!(operation, ComposeOperation::Stop) {
                self.cancel_operation(id).await?;
                return Ok(ComposeOperationResult {
                    compose: self.get_by_id(id).await?,
                    deployment_id: None,
                    operation: ComposeOperation::Stop,
                });
            }
            return Err(sqlx::Error::Protocol(
                "compose deployment already queued or running".into(),
            ));
        }

        resolve::<BuilderQueue>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .ensure_capacity()
            .await?;

        let compose = self.get_by_id(id).await?;
        if !matches!(operation, ComposeOperation::Stop) {
            assert_deployable(&self.queries, compose.server_id).await?;
        }

        let log_path = format!("pending-compose-{}", id);
        let description = format!("{} requested for {}", operation.as_str(), compose.name);
        // The old DeploymentRepository::create_queued_compose_deployment
        // returned the new row id; the With/NoServer split keeps a NULL
        // server_id insertable (flash binds are non-Option).
        let deployment_id = match compose.server_id {
            Some(server_id) => {
                self.queries
                    .compose_queue_deployment_with_server(&ComposeQueueDeploymentWithServerParams {
                        title: operation.title().to_string(),
                        description,
                        log_path,
                        operation: operation.as_str().to_string(),
                        service_id: id,
                        server_id,
                        user_id: actor_id,
                    })
                    .await?
            }
            None => {
                self.queries
                    .compose_queue_deployment_no_server(&ComposeQueueDeploymentNoServerParams {
                        title: operation.title().to_string(),
                        description,
                        log_path,
                        operation: operation.as_str().to_string(),
                        service_id: id,
                        user_id: actor_id,
                    })
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
                    "[QUEUED] deployment queued for {}",
                    operation.as_str()
                ))
                .await;
        }

        resolve::<BuilderQueue>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .notify();

        let status = match operation {
            ComposeOperation::Stop => ComposeStatus::Stopping,
            _ => ComposeStatus::Starting,
        };

        self.queries
            .update_service_status(status.as_str(), id)
            .await?;
        let record: ComposeRecord = self.load_record(id).await?;

        self.cache
            .insert(
                crate::core::cache::CacheKey::Compose(id),
                crate::core::cache::CacheEnum::Compose(Box::new(record.clone())),
            )
            .await;

        Ok(ComposeOperationResult {
            compose: record,
            deployment_id: Some(deployment_id),
            operation,
        })
    }

    pub async fn stop_operation(&self, id: i64) -> sqlx::Result<bool> {
        self.stop_or_cancel_smart(id).await
    }

    pub async fn cancel_operation(&self, id: i64) -> sqlx::Result<bool> {
        self.stop_or_cancel_smart(id).await
    }

    pub async fn stop_or_cancel_smart(&self, id: i64) -> sqlx::Result<bool> {
        let compose = self.get_by_id(id).await?;
        let current_status =
            ComposeStatus::from_str_opt(&compose.compose_status).unwrap_or(ComposeStatus::Idle);

        let (intermediate, final_st) = if current_status.is_building() {
            (
                ComposeStatus::Cancelling.as_str(),
                ComposeStatus::Cancelled.as_str(),
            )
        } else {
            (
                ComposeStatus::Stopping.as_str(),
                ComposeStatus::Stopped.as_str(),
            )
        };

        let _ = self.queries.update_service_status(intermediate, id).await;
        self.cache
            .invalidate(&crate::core::cache::CacheKey::Compose(id))
            .await;

        if let Ok(queue) = resolve::<BuilderQueue>().await {
            let _ = queue.cancel_queued_compose(id).await;
        }

        if let Ok(state) = resolve::<ApplicationState>().await {
            state.cancel_by_id(IdType::ComposeId(id));
        }

        let _ = self
            .queries
            .deployments_request_cancel_compose_deployment(id)
            .await;

        let db_ref = self.queries.pool.clone();
        let queries = self.queries.clone();
        let cache = self.cache.clone();
        let app_name = compose.app_name.clone();
        let compose_type = compose.compose_type;

        tokio::spawn(async move {
            if let Err(e) = scale_down_compose(db_ref, id, &app_name, compose_type).await {
                tracing::warn!(compose_id = id, error = %e, "could not scale down compose on stop/cancel");
            }

            let _ = queries.update_service_status(final_st, id).await;
            cache
                .invalidate(&crate::core::cache::CacheKey::Compose(id))
                .await;
        });

        Ok(true)
    }
}

/// Replaces the retired `ServerManagementRepository::assert_deployable`:
/// a missing management row is created on first use, and maintenance mode
/// blocks the deployment with the stored (or default) message.
async fn assert_deployable(queries: &FlashQueries, server_id: Option<i64>) -> sqlx::Result<()> {
    let Some(server_id) = server_id else {
        return Ok(());
    };
    queries.server_management_get_or_create1(server_id).await?;
    let row = queries.server_management_get_or_create2(server_id).await?;
    if row.maintenance_mode != 0 {
        // Flash binds the nullable message column as non-Option; "" is the
        // NULL encoding.
        let message = row
            .maintenance_message
            .filter(|message| !message.is_empty())
            .unwrap_or_else(|| "server is in maintenance mode".into());
        return Err(sqlx::Error::Protocol(message));
    }
    Ok(())
}

pub(super) async fn scale_down_compose(
    db: SqlitePool,
    compose_id: i64,
    app_name: &str,
    compose_type: ComposeType,
) -> Result<(), String> {
    let cmd = compose_new_db(db, compose_id)
        .await
        .map_err(|e| format!("could not build executor: {e}"))?;
    let docker = DockerCli::from_executor(cmd);

    match compose_type {
        ComposeType::DockerCompose => {
            let _ = tokio::time::timeout(
                std::time::Duration::from_secs(6),
                docker
                    .compose()
                    .down()
                    .project(app_name)
                    .remove_orphans()
                    .run(),
            )
            .await;
            Ok(())
        }
        ComposeType::Stack => {
            let _ = tokio::time::timeout(
                std::time::Duration::from_secs(4),
                docker.stacks().remove(app_name).run(),
            )
            .await;

            if let Ok(services) = tokio::time::timeout(
                std::time::Duration::from_secs(4),
                docker
                    .services()
                    .list()
                    .filter(ServiceFilter::Name(app_name.to_string()))
                    .run_json(),
            )
            .await
            .unwrap_or(Err(
                crate::utils::docker::error::DockerError::CommandFailed {
                    code: None,
                    stderr: "timeout".into(),
                },
            )) {
                for service in services {
                    let _ = tokio::time::timeout(
                        std::time::Duration::from_secs(3),
                        docker.services().remove(&service.name).run(),
                    )
                    .await;
                }
            }
            Ok(())
        }
    }
}

/// `db` is retained for signature compatibility with the caller in
/// `operations.rs`; the FlashQueries client is resolved from DI.
pub async fn compose_new_db(
    _db: sqlx::SqlitePool,
    compose_id: i64,
) -> Result<CommandExecutor, sqlx::Error> {
    let queries = resolve::<FlashQueries>()
        .await
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
    let service = queries.get_service_by_i_d(compose_id).await?;
    crate::services::compute::compose::remote::agent_executor_for(service.server_id)
        .await
        .map_err(sqlx::Error::Protocol)
}
