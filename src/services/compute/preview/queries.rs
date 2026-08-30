use super::{PreviewDeploymentService, PreviewDeploymentView};

impl PreviewDeploymentService {
    pub async fn list(&self, active_only: bool) -> sqlx::Result<Vec<PreviewDeploymentView>> {
        let views = if active_only {
            self.queries
                .preview_deployments_list1()
                .await?
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
        } else {
            self.queries
                .preview_deployments_list2()
                .await?
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
        };
        Ok(views)
    }

    pub async fn get(&self, id: i64) -> sqlx::Result<PreviewDeploymentView> {
        self.queries
            .preview_deployments_get_by_id(id)
            .await
            .map(Into::into)
    }
}
