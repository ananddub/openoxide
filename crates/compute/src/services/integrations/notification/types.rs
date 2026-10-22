use crate::flash_gen::models::Notifications as Notification;
use os::string_enum;
use serde::{Deserialize, Serialize};

string_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum NotificationProvider {
        default = Slack;

        Slack => "SLACK",
        Telegram => "TELEGRAM",
        Discord => "DISCORD",
        Email => "EMAIL",
        Resend => "RESEND",
        Gotify => "GOTIFY",
        Ntfy => "NTFY",
        Mattermost => "MATTERMOST",
        Pushover => "PUSHOVER",
        Custom => "CUSTOM",
        Lark => "LARK",
        Teams => "TEAMS",
    }
}

/// Which organizations' notification channels a dispatch may reach.
///
/// Notifications belong to an organization, so anything raised on behalf of a
/// tenant must name that tenant. `AllOrganizations` exists only for panel-wide
/// events that genuinely concern every operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotificationScope {
    Organization(i64),
    AllOrganizations,
}

impl NotificationScope {
    pub fn organization_id(&self) -> Option<i64> {
        match self {
            Self::Organization(id) => Some(*id),
            Self::AllOrganizations => None,
        }
    }

    pub fn is_tenant_scoped(&self) -> bool {
        matches!(self, Self::Organization(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tenant_scope_carries_its_id() {
        let scope = NotificationScope::Organization(7);
        assert_eq!(scope.organization_id(), Some(7));
        assert!(scope.is_tenant_scoped());
    }

    #[test]
    fn the_panel_wide_scope_names_no_organization() {
        let scope = NotificationScope::AllOrganizations;
        assert_eq!(scope.organization_id(), None);
        assert!(!scope.is_tenant_scoped());
    }
}

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
