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
        if self
            .previews
            .find_by_preview_application(application_id)
            .await
            .map_err(|error| error.to_string())?
            .is_some()
        {
            return Err(
                "Preview applications must be deleted through the preview deployment API".into(),
            );
        }
        Ok(())
    }

    pub async fn remove_for_base_application(
        &self,
        base_application_id: i64,
    ) -> Result<(), String> {
        let _guard = self.lifecycle_lock.lock().await;
        let rows = self
            .previews
            .list_open_for_base_application(base_application_id)
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
        let row = self
            .previews
            .get_by_id(id)
            .await
            .map_err(|error| error.to_string())?
            .ok_or("Preview deployment not found")?;

        if let Some(application_id) = row.preview_application_id {
            let application = self
                .applications
                .get_by_id(application_id)
                .await
                .map_err(|error| error.to_string())?;
            if let Some(application) = application {
                if let Ok(service) = resolve::<ApplicationService>().await {
                    let _ = service.cancel_operation(application_id).await;
                }
                if let Ok(executor) = app_new_cmd(self.db.clone(), application_id).await {
                    if let Err(error) = DockerCli::from_executor(executor)
                        .stacks()
                        .remove(&application.app_name)
                        .run()
                        .await
                    {
                        tracing::warn!(preview_id = id, error = %error, "preview stack cleanup failed");
                    }
                }
                self.applications
                    .delete(application_id)
                    .await
                    .map_err(|error| error.to_string())?;
            }
        }

        if hard_delete {
            self.previews
                .delete(id)
                .await
                .map_err(|error| error.to_string())?;
        } else {
            self.previews
                .close(id)
                .await
                .map_err(|error| error.to_string())?;
        }
        Ok(())
    }
}
