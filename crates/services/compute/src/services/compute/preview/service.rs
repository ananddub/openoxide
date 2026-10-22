use std::sync::Arc;

use auto_di::singleton;

use crate::db::queries::FlashQueries;

use super::types::PreviewDeploymentView;

pub struct PreviewDeploymentService {
    pub(super) queries: Arc<FlashQueries>,
    pub(super) lifecycle_lock: tokio::sync::Mutex<()>,
}

#[singleton]
impl PreviewDeploymentService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self {
            queries,
            lifecycle_lock: tokio::sync::Mutex::new(()),
        }
    }
}

#[cfg(test)]
impl PreviewDeploymentService {
    pub(crate) fn for_tests(queries: Arc<FlashQueries>) -> Self {
        Self {
            queries,
            lifecycle_lock: tokio::sync::Mutex::new(()),
        }
    }
}

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
