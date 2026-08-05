use auto_di::resolve;

use crate::utils::builder::queue::BuilderQueue;

use super::ApplicationService;

#[derive(Debug, Clone, Copy)]
pub struct ApplicationCleanupResult {
    pub affected: u64,
}

#[derive(Debug, Clone)]
pub struct ApplicationForceKillResult {
    pub deployment_id: i64,
    pub result: crate::services::deployment::CancelDeploymentResult,
}

impl ApplicationService {
    pub async fn force_kill_deployment(&self, id: i64) -> sqlx::Result<ApplicationForceKillResult> {
        self.get_by_id(id).await?;
        let deployment = self
            .repo_deploy
            .running_for_application(id)
            .await?
            .ok_or_else(|| sqlx::Error::Protocol("application has no running deployment".into()))?;
        let deployment_id = deployment.id;
        let mut result = self.deployment_service.cancel(deployment_id).await?;

        if matches!(
            result,
            crate::services::deployment::CancelDeploymentResult::NotActiveInThisProcess
        ) && let (Some(server_id), Some(pid_file)) =
            (deployment.server_id, deployment.pid.as_deref())
        {
            let executor =
                crate::services::application::remote::remote_executor(self.db.as_ref(), server_id)
                    .await
                    .map_err(sqlx::Error::Protocol)?;
            executor
                .kill_pid_file(pid_file)
                .await
                .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
            self.repo_deploy
                .finalize_with_resource(
                    deployment_id,
                    "CANCELLED",
                    Some("deployment force-killed"),
                    Some(id),
                    None,
                    None,
                    None,
                    "ERROR",
                )
                .await?;
            result = crate::services::deployment::CancelDeploymentResult::CancelRequested;
        }
        Ok(ApplicationForceKillResult {
            deployment_id,
            result,
        })
    }

    pub async fn move_to_environment(
        &self,
        id: i64,
        target_environment_id: i64,
    ) -> sqlx::Result<crate::services::application::ApplicationRecord> {
        self.get_by_id(id).await?;
        if self.repo_deploy.has_running_deployment(id).await? {
            return Err(sqlx::Error::Protocol(
                "application deployment is already queued or running".into(),
            ));
        }

        let application = self
            .repo_app
            .move_to_environment_in_same_organization(id, target_environment_id)
            .await?
            .ok_or_else(|| {
                sqlx::Error::Protocol(
                    "target environment not found or belongs to another organization".into(),
                )
            })?;
        let record = crate::services::application::ApplicationRecord::from(application);
        self.cache
            .insert(
                crate::core::cache::CacheKey::Application(id),
                crate::core::cache::CacheEnum::Application(record.clone()),
            )
            .await;
        Ok(record)
    }

    pub async fn rotate_webhook_token(&self, id: i64) -> sqlx::Result<String> {
        self.get_by_id(id).await?;
        let mut bytes = [0_u8; 32];
        getrandom::fill(&mut bytes).map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let token: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
        if !self.repo_app.rotate_refresh_token(id, &token).await? {
            return Err(sqlx::Error::RowNotFound);
        }
        self.cache
            .invalidate(&crate::core::cache::CacheKey::Application(id))
            .await;
        Ok(token)
    }

    pub async fn clear_deployment_history(
        &self,
        id: i64,
    ) -> sqlx::Result<ApplicationCleanupResult> {
        self.get_by_id(id).await?;
        let deployment_ids = self
            .repo_deploy
            .list_finished_ids_for_application(id)
            .await?;
        let affected = self.repo_deploy.delete_finished_for_application(id).await?;

        for deployment_id in deployment_ids {
            let path = crate::utils::paths::rustploy_paths().deployment_log_file(deployment_id);
            match tokio::fs::remove_file(&path).await {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => {
                    tracing::warn!(deployment_id, %error, "could not delete deployment log")
                }
            }
        }
        Ok(ApplicationCleanupResult { affected })
    }

    pub async fn cleanup_deployment_queue(
        &self,
        id: i64,
    ) -> sqlx::Result<ApplicationCleanupResult> {
        self.get_by_id(id).await?;
        if let Ok(queue) = resolve::<BuilderQueue>().await {
            let _ = queue.cancel_queued_application(id).await;
        }
        let affected = self
            .repo_deploy
            .cancel_queued_for_application_count(id)
            .await?;
        Ok(ApplicationCleanupResult { affected })
    }
}
