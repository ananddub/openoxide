use crate::utils::builder::queue::deployment_log::DeploymentLog;
use crate::utils::builder::spec::ApplicationSpec;
use crate::utils::docker::DockerCli;

use super::ApplicationService;
use super::auto_excuter::app_new_cmd;

#[derive(Debug, Clone)]
pub struct ApplicationRollbackTriggerResult {
    pub deployment_id: i64,
    pub message: String,
}

impl ApplicationService {
    /// Trigger a rollback to a specific rollback snapshot.
    /// This updates the Docker Swarm service to use the versioned rollback image
    /// and the saved configuration from that snapshot.
    pub async fn trigger_rollback(
        &self,
        application_id: i64,
        rollback_id: i64,
    ) -> sqlx::Result<ApplicationRollbackTriggerResult> {
        let application = self.get_by_id(application_id).await?;
        if self
            .repo_deploy
            .has_running_deployment(application_id)
            .await?
        {
            return Err(sqlx::Error::Protocol(
                "application deployment is already queued or running".into(),
            ));
        }

        let rollback = self
            .repo_rollback
            .get_for_application(rollback_id, application_id)
            .await?
            .ok_or_else(|| sqlx::Error::Protocol(format!("rollback {} not found", rollback_id)))?;

        let rollback_image = rollback
            .image
            .as_deref()
            .ok_or_else(|| sqlx::Error::Protocol("rollback has no image".into()))?;

        // 2. Deserialize the saved ApplicationSpec from full_context
        let spec: ApplicationSpec = match rollback.full_context.as_deref() {
            Some(json) => serde_json::from_str(json).map_err(|e| {
                sqlx::Error::Protocol(format!("could not parse rollback context: {e}"))
            })?,
            None => {
                return Err(sqlx::Error::Protocol(
                    "rollback has no saved context".into(),
                ));
            }
        };

        let deployment_id = self
            .repo_deploy
            .create_running_deployment(
                "Application rollback",
                Some(&format!(
                    "Rollback {} to version {}",
                    application.name, rollback.version
                )),
                "ROLLBACK",
                application_id,
                application.server_id,
            )
            .await?;
        let log_path = crate::utils::paths::openoxide_paths().deployment_log_file(deployment_id);
        self.repo_deploy
            .update_log_path(deployment_id, &log_path)
            .await?;
        let mut log = DeploymentLog::open(deployment_id).await.ok();
        if let Some(log) = log.as_mut() {
            let _ = log
                .write_line(&format!(
                    "[RUNNING] Rollback {} to version {} using image {}",
                    application.name, rollback.version, rollback_image
                ))
                .await;
        }

        let service_name = spec.service_name();
        let result = async {
            let executor = app_new_cmd(self.db.clone(), application_id).await?;
            let docker = DockerCli::from_executor(executor);

            // 4. Update the Docker Swarm service with the rollback image
            let mut update = docker
                .services()
                .update(&service_name)
                .image(rollback_image)
                .force();

            // Apply environment variables from the snapshot
            for (k, v) in &spec.environment {
                update = update.env_add(k, v);
            }

            update
                .run()
                .await
                .map_err(|e| sqlx::Error::Protocol(format!("rollback service update failed: {e}")))
        }
        .await;

        match result {
            Ok(_) => {
                let message = format!(
                    "Rolled back {} to version {} (image: {})",
                    service_name, rollback.version, rollback_image
                );
                if let Some(log) = log.as_mut() {
                    let _ = log.write_line(&format!("[DONE] {message}")).await;
                }
                self.repo_deploy
                    .update_final_status(deployment_id, "DONE", None)
                    .await?;
                tracing::info!(
                    deployment_id,
                    application_id,
                    rollback_id,
                    rollback_image,
                    service_name,
                    version = rollback.version,
                    "rollback: service updated successfully"
                );
                Ok(ApplicationRollbackTriggerResult {
                    deployment_id,
                    message,
                })
            }
            Err(e) => {
                let error = e.to_string();
                if let Some(log) = log.as_mut() {
                    let _ = log.write_line(&format!("[ERROR] {error}")).await;
                }
                self.repo_deploy
                    .update_final_status(deployment_id, "ERROR", Some(&error))
                    .await?;
                tracing::error!(
                    deployment_id,
                    application_id,
                    rollback_id,
                    rollback_image,
                    error = %e,
                    "rollback: failed to update service"
                );
                Err(e)
            }
        }
    }

    /// List all rollback snapshots available for an application, newest first.
    pub async fn list_rollbacks(
        &self,
        application_id: i64,
    ) -> sqlx::Result<Vec<crate::db::models::rollbacks::Rollback>> {
        self.get_by_id(application_id).await?;
        self.repo_rollback.list_by_application(application_id).await
    }

    pub async fn delete_rollback(
        &self,
        application_id: i64,
        rollback_id: i64,
    ) -> sqlx::Result<bool> {
        self.get_by_id(application_id).await?;
        self.repo_rollback
            .delete_for_application(rollback_id, application_id)
            .await
    }
}
