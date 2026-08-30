use std::sync::Arc;

use auto_di::resolve;

use crate::{
    services::application::{ApplicationService, auto_excuter::app_new_cmd},
    utils::docker::DockerCli,
};

use super::PreviewDeploymentService;

impl PreviewDeploymentService {
    pub async fn ensure_application_can_be_deleted(
        &self,
        application_id: i64,
    ) -> Result<(), String> {
        match self
            .queries
            .preview_deployments_find_by_preview_application(application_id)
            .await
        {
            Ok(_) => Err(
                "Preview applications must be deleted through the preview deployment API".into(),
            ),
            Err(sqlx::Error::RowNotFound) => Ok(()),
            Err(error) => Err(error.to_string()),
        }
    }

    pub async fn remove_for_base_application(
        &self,
        base_application_id: i64,
    ) -> Result<(), String> {
        let _guard = self.lifecycle_lock.lock().await;
        let rows = self
            .queries
            .preview_deployments_list_open_for_base_application(base_application_id)
            .await
            .map_err(|error| error.to_string())?;
        for row in rows {
            self.remove_unlocked(row.id, true).await?;
        }
        Ok(())
    }

    pub async fn remove(&self, id: i64, hard_delete: bool) -> Result<(), String> {
        let _guard = self.lifecycle_lock.lock().await;
        self.remove_unlocked(id, hard_delete).await
    }

    pub(super) async fn remove_unlocked(&self, id: i64, hard_delete: bool) -> Result<(), String> {
        let row = match self.queries.preview_deployments_get_by_id(id).await {
            Ok(row) => row,
            Err(sqlx::Error::RowNotFound) => return Err("Preview deployment not found".into()),
            Err(error) => return Err(error.to_string()),
        };

        if let Some(application_id) = row.preview_service_id {
            let application = match self.queries.get_service_by_i_d(application_id).await {
                Ok(service) => Some(service),
                Err(sqlx::Error::RowNotFound) => None,
                Err(error) => return Err(error.to_string()),
            };
            if let Some(application) = application {
                if let Ok(service) = resolve::<ApplicationService>().await {
                    let _ = service.cancel_operation(application_id).await;
                }
                if let Ok(executor) = app_new_cmd(
                    Arc::new(self.queries.pool.clone()),
                    application_id,
                )
                .await
                {
                    if let Err(error) = DockerCli::from_executor(executor)
                        .stacks()
                        .remove(&application.app_name)
                        .run()
                        .await
                    {
                        tracing::warn!(preview_id = id, error = %error, "preview stack cleanup failed");
                    }
                }
                // service_git / service_application / domains cascade via FK.
                self.queries
                    .delete_service(application_id)
                    .await
                    .map_err(|error| error.to_string())?;
            }
        }

        if hard_delete {
            self.queries
                .preview_deployments_delete(id)
                .await
                .map_err(|error| error.to_string())?;
        } else {
            self.queries
                .preview_deployments_close(id)
                .await
                .map_err(|error| error.to_string())?;
        }
        Ok(())
    }
}
