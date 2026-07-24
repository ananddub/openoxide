use auto_di::resolve;
use sqlx::SqlitePool;
use std::sync::Arc;

use super::{
    ComposeOperation, ComposeOperationResult, ComposeRecord, ComposeService, ComposeType,
    auto_excuter::compose_new_db,
};
use crate::utils::builder::queue::BuilderQueue;
use crate::utils::builder::{custom_type::IdType, hash_state::ApplicationState};
use crate::utils::docker::DockerCli;
use crate::utils::docker::query::ServiceFilter;

impl ComposeService {
    pub async fn run_operation(
        &self,
        id: i64,
        operation: ComposeOperation,
    ) -> sqlx::Result<ComposeOperationResult> {
        let running_deployment = self.repo_deploy.has_running_compose_deployment(id).await?;
        if running_deployment {
            return Err(sqlx::Error::Protocol(
                "compose deployment already queued or running".into(),
            ));
        }

        resolve::<BuilderQueue>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .ensure_capacity()
            .await?;

        let compose_model = self
            .repo_compose
            .update_status(id, operation.target_status())
            .await?;
        let compose = ComposeRecord::from(compose_model);

        let log_path = format!("pending-compose-{}", id);
        let deployment_id = self
            .repo_deploy
            .create_queued_compose_deployment(
                operation.title().to_string(),
                Some(format!(
                    "{} requested for {}",
                    operation.as_str(),
                    compose.name
                )),
                log_path,
                operation.as_str().to_string(),
                id,
                compose.server_id,
            )
            .await?;

        let log_path = crate::utils::paths::rustploy_paths().deployment_log_file(deployment_id);
        self.repo_deploy
            .update_log_path(deployment_id, &log_path)
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

        Ok(ComposeOperationResult {
            compose,
            deployment_id: Some(deployment_id),
            operation,
        })
    }

    pub async fn cancel_operation(&self, id: i64) -> sqlx::Result<bool> {
        let compose = self.get_by_id(id).await?;

        let queue = resolve::<BuilderQueue>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        if queue.cancel_queued_compose(id).await? {
            self.repo_compose.update_status(id, "IDLE").await?;
            return Ok(true);
        }

        let has_running_deployment = self
            .repo_deploy
            .has_running_status_compose_deployment(id)
            .await?;

        if !has_running_deployment {
            let db = self.db.clone();
            let app_name = compose.app_name.clone();
            let compose_type = compose.compose_type;
            let repo_compose = self.repo_compose.clone();
            tokio::spawn(async move {
                if let Err(e) = scale_down_compose(db.clone(), id, &app_name, compose_type).await {
                    tracing::warn!(compose_id = id, error = %e, "could not scale down compose on cancel");
                }
                let _ = repo_compose.update_status(id, "IDLE").await
                    .map_err(|e| tracing::error!(compose_id = id, error = %e, "could not update compose status to IDLE on cancel"));
            });
            return Ok(true);
        }

        let state = resolve::<ApplicationState>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        state.cancel_by_id(IdType::ComposeId(id));

        self.repo_compose.update_status(id, "IDLE").await?;

        self.repo_deploy
            .request_cancel_compose_deployment(id)
            .await?;

        let db = self.db.clone();
        let app_name = compose.app_name.clone();
        let compose_type = compose.compose_type;
        tokio::spawn(async move {
            if let Err(e) = scale_down_compose(db, id, &app_name, compose_type).await {
                tracing::warn!(compose_id = id, error = %e, "could not scale down compose on cancel");
            }
        });

        Ok(true)
    }
}

async fn scale_down_compose(
    db: Arc<SqlitePool>,
    compose_id: i64,
    app_name: &str,
    compose_type: ComposeType,
) -> Result<(), String> {
    let cmd = compose_new_db(db, compose_id)
        .await
        .map_err(|e| format!("could not build executor: {e}"))?;
    let docker = DockerCli::from_executor(cmd);

    match compose_type {
        ComposeType::DockerCompose => docker
            .compose()
            .down()
            .project(app_name)
            .run()
            .await
            .map(|_| ())
            .map_err(|e| format!("compose down failed: {e}")),
        ComposeType::Stack => {
            let services = docker
                .services()
                .list()
                .filter(ServiceFilter::Name(app_name.to_string()))
                .run_json()
                .await
                .map_err(|e| format!("could not get stack service: {e}"))?;

            if services.is_empty() {
                return Ok(());
            }
            for service in services {
                if service.replicas != "0/0" {
                    docker
                        .services()
                        .scale()
                        .service(&service.name, 0)
                        .run()
                        .await
                        .map_err(|e| format!("service scale 0 failed: {e}"))?;
                }
            }
            Ok(())
        }
    }
}
