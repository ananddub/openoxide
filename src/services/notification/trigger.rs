use crate::db::models::notifications::Notification;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationTrigger {
    AppDeploy,
    AppBuildError,
    DatabaseBackup,
    VolumeBackup,
    PanelRestart,
    DockerCleanup,
    ServerThreshold,
    PanelBackup,
}

impl NotificationTrigger {
    pub fn is_enabled_for(&self, n: &Notification) -> bool {
        let flag = match self {
            Self::AppDeploy => n.on_app_deploy,
            Self::AppBuildError => n.on_app_build_error,
            Self::DatabaseBackup => n.on_database_backup,
            Self::VolumeBackup => n.on_volume_backup,
            Self::PanelRestart => n.on_panel_restart,
            Self::DockerCleanup => n.on_docker_cleanup,
            Self::ServerThreshold => n.on_server_threshold,
            Self::PanelBackup => n.on_panel_backup,
        };
        flag != 0
    }
}
