use super::{PreviewDeploymentService, PreviewDeploymentView};

impl PreviewDeploymentService {
    pub async fn list(&self, active_only: bool) -> sqlx::Result<Vec<PreviewDeploymentView>> {
        self.previews
            .list(active_only)
            .await
            .map(|rows| rows.into_iter().map(Into::into).collect())
    }

    pub async fn get(&self, id: i64) -> sqlx::Result<PreviewDeploymentView> {
        self.previews
            .get_by_id(id)
            .await?
            .map(Into::into)
            .ok_or(sqlx::Error::RowNotFound)
    }
}
