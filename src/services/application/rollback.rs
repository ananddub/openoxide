use auto_di::resolve;

use crate::repository::RollbackRepository;
use crate::utils::builder::spec::ApplicationSpec;
use crate::utils::docker::DockerCli;

use super::ApplicationService;
use super::auto_excuter::app_new_cmd;

impl ApplicationService {
    /// Trigger a rollback to a specific rollback snapshot.
    /// This updates the Docker Swarm service to use the versioned rollback image
    /// and the saved configuration from that snapshot.
    pub async fn trigger_rollback(
        &self,
        application_id: i64,
        rollback_id: i64,
    ) -> sqlx::Result<String> {
        // 1. Load the rollback record
        let rollback_repo = resolve::<RollbackRepository>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        let rollback = rollback_repo
            .get_by_id(rollback_id)
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

        // 3. Get the executor for this application's server
        let executor = app_new_cmd(self.db.clone(), application_id).await?;
        let docker = DockerCli::from_executor(executor);

        // 4. Update the Docker Swarm service with the rollback image
        let service_name = spec.service_name();

        let mut update = docker
            .services()
            .update(&service_name)
            .image(rollback_image)
            .force();

        // Apply environment variables from the snapshot
        for (k, v) in &spec.environment {
            update = update.env_add(k, v);
        }

        match update.run().await {
            Ok(_) => {
                tracing::info!(
                    application_id,
                    rollback_id,
                    rollback_image,
                    service_name,
                    version = rollback.version,
                    "rollback: service updated successfully"
                );
                Ok(format!(
                    "Rolled back {} to version {} (image: {})",
                    service_name, rollback.version, rollback_image
                ))
            }
            Err(e) => {
                tracing::error!(
                    application_id,
                    rollback_id,
                    rollback_image,
                    error = %e,
                    "rollback: failed to update service"
                );
                Err(sqlx::Error::Protocol(format!(
                    "rollback service update failed: {e}"
                )))
            }
        }
    }

    /// List all rollback snapshots available for an application, newest first.
    pub async fn list_rollbacks(
        &self,
        application_id: i64,
    ) -> sqlx::Result<Vec<crate::db::models::rollbacks::Rollback>> {
        let rollback_repo = resolve::<RollbackRepository>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        rollback_repo.list_by_application(application_id).await
    }
}
