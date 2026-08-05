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
    ScheduleSuccess,
    ScheduleFailure,
}

impl NotificationTrigger {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AppDeploy => "APP_DEPLOY",
            Self::AppBuildError => "APP_BUILD_ERROR",
            Self::DatabaseBackup => "DATABASE_BACKUP",
            Self::VolumeBackup => "VOLUME_BACKUP",
            Self::PanelRestart => "PANEL_RESTART",
            Self::DockerCleanup => "DOCKER_CLEANUP",
            Self::ServerThreshold => "SERVER_THRESHOLD",
            Self::PanelBackup => "PANEL_BACKUP",
            Self::ScheduleSuccess => "SCHEDULE_SUCCESS",
            Self::ScheduleFailure => "SCHEDULE_FAILURE",
        }
    }

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
            Self::ScheduleSuccess => n.on_schedule_success,
            Self::ScheduleFailure => n.on_schedule_failure,
        };
        flag != 0
    }
}
