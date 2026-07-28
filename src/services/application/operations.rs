use auto_di::resolve;

use crate::services::application::auto_excuter::app_new_cmd;
use crate::utils::builder::queue::BuilderQueue;
use crate::utils::builder::{custom_type::IdType, hash_state::ApplicationState};
use crate::utils::docker::DockerCli;
use crate::utils::docker::query::filter::ServiceFilter;

use super::{ApplicationOperation, ApplicationOperationResult, ApplicationService};

impl ApplicationService {
    pub async fn run_operation(
        &self,
        id: i64,
        operation: ApplicationOperation,
    ) -> sqlx::Result<ApplicationOperationResult> {
        let running_deployment = self.repo_deploy.has_running_deployment(id).await?;
        if running_deployment {
            return Err(sqlx::Error::Protocol(
                "application deployment already queued or running; cancel it first".into(),
            ));
        }

        resolve::<BuilderQueue>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .ensure_capacity()
            .await?;

        let _ = self.repo_app.update_status(id, "STARTING").await;
        self.cache
            .invalidate(&crate::core::cache::CacheKey::Application(id))
            .await;
        let app = self.get_by_id(id).await?;

        let log_path = format!("pending-app-{}", id);
        let deployment_id = self
            .repo_deploy
            .create_queued_deployment(
                operation.title().to_string(),
                Some(format!("{} requested for {}", operation.as_str(), app.name)),
                log_path,
                operation.as_str().to_string(),
                id,
                app.server_id,
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

        Ok(ApplicationOperationResult {
            application: app,
            deployment_id: Some(deployment_id),
            operation,
        })
    }

    pub async fn cancel_operation(&self, id: i64) -> sqlx::Result<bool> {
        let app_user = self.get_by_id(id).await?;

        let queue = resolve::<BuilderQueue>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        let _ = queue.cancel_queued_application(id).await?;

        if let Ok(state) = resolve::<ApplicationState>().await {
            state.cancel_by_id(IdType::AppId(id));
        }

        let _ = self.repo_deploy.request_cancel_deployment(id).await;

        if let Ok(cmd) = app_new_cmd(self.db.clone(), id).await {
            let docker_cli = DockerCli::from_executor(cmd);
            if let Ok(services) = docker_cli
                .services()
                .list()
                .filter(ServiceFilter::name(format!("{}_", app_user.app_name)))
                .run_json()
                .await
            {
                for s in services.iter() {
                    if &s.replicas != "0/0" {
                        let _ = docker_cli
                            .services()
                            .scale()
                            .service(&s.name, 0)
                            .run()
                            .await;
                    }
                }
            }
        }

        self.repo_app.update_status(id, "STOPPED").await?;
        self.cache
            .invalidate(&crate::core::cache::CacheKey::Application(id))
            .await;
        Ok(true)
    }
}
