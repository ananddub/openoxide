use std::sync::Arc;

use auto_di::singleton;
use connectrpc::ConnectError;

use crate::core::cache::AppStateCache;
use crate::db::queries::FlashQueries;
use crate::services::{
    automation::backup::{BackupFileService, ComposeConfigBackupService, PanelBackupService},
    automation::schedule::ScheduleService,
};

use super::helper::map_error;

pub struct BackupController {
    pub(super) service: Arc<ScheduleService>,
    pub(super) queries: Arc<FlashQueries>,
    pub(super) panel_backup: Arc<PanelBackupService>,
    pub(super) backup_files: Arc<BackupFileService>,
    pub(super) compose_config_backup: Arc<ComposeConfigBackupService>,
    pub(super) cache: Arc<AppStateCache>,
}

#[singleton]
impl BackupController {
    fn new(
        service: Arc<ScheduleService>,
        queries: Arc<FlashQueries>,
        panel_backup: Arc<PanelBackupService>,
        backup_files: Arc<BackupFileService>,
        compose_config_backup: Arc<ComposeConfigBackupService>,
        cache: Arc<AppStateCache>,
    ) -> Self {
        Self {
            service,
            queries,
            panel_backup,
            backup_files,
            compose_config_backup,
            cache,
        }
    }
}

// Controller extensions — non-gRPC helper methods live here, not in controller.rs.
impl BackupController {
    pub(super) async fn resolve_organization_id(&self, requested_id: Option<i64>) -> i64 {
        if let Some(id) = requested_id.filter(|id| *id > 0)
            && self.queries.organization_get_by_id(id).await.is_ok()
        {
            return id;
        }
        if let Ok(orgs) = self.queries.organization_get_all().await
            && let Some(first) = orgs.first()
        {
            return first.id;
        }
        1
    }

    pub(super) async fn require_destination_id(
        &self,
        requested_id: Option<i64>,
    ) -> Result<i64, ConnectError> {
        let id = requested_id
            .filter(|id| *id > 0)
            .ok_or_else(|| ConnectError::invalid_argument("S3 destination is required"))?;
        match self.queries.get_destination_by_i_d(id).await {
            Ok(_) => Ok(id),
            Err(sqlx::Error::RowNotFound) => Err(ConnectError::invalid_argument(
                "selected S3 destination does not exist",
            )),
            Err(other) => Err(map_error(other)),
        }
    }

    pub(super) async fn verify_service_exists(&self, id: Option<i64>) -> Option<i64> {
        let target_id = id.filter(|id| *id > 0)?;
        self.queries
            .get_service_by_i_d(target_id)
            .await
            .ok()
            .map(|_| target_id)
    }
}
