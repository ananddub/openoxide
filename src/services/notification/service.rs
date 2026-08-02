use super::{email::send_email, message::NotificationMessage, senders};
use crate::db::{
    models::notifications::Notification,
    repository::{
        NotifCustomRepository, NotifDiscordRepository, NotifEmailRepository, NotifGotifyRepository,
        NotifLarkRepository, NotifMattermostRepository, NotifNtfyRepository,
        NotifPushoverRepository, NotifResendRepository, NotifSlackRepository, NotifTeamRepository,
        NotifTelegramRepository, NotificationRepository,
    },
};
use auto_di::singleton;
use reqwest::Client;
use std::{sync::Arc, time::Duration};
use tokio::sync::Semaphore;

/// The event kinds a notification row can subscribe to. Each maps to one of the
/// `on_*` columns on `notifications`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotificationTrigger {
    AppDeploy,
    AppBuildError,
    DatabaseBackup,
    VolumeBackup,
    PanelRestart,
    DockerCleanup,
    ServerThreshold,
}

impl NotificationTrigger {
    fn is_enabled_for(&self, n: &Notification) -> bool {
        let flag = match self {
            Self::AppDeploy => n.on_app_deploy,
            Self::AppBuildError => n.on_app_build_error,
            Self::DatabaseBackup => n.on_database_backup,
            Self::VolumeBackup => n.on_volume_backup,
            Self::PanelRestart => n.on_panel_restart,
            Self::DockerCleanup => n.on_docker_cleanup,
            Self::ServerThreshold => n.on_server_threshold,
        };
        flag != 0
    }
}

/// Dispatches notifications to the configured providers.
///
/// A `notifications` row names one provider (`notification_type`) plus the set
/// of events it wants (`on_*` columns). Sending means: find the rows subscribed
/// to this trigger, resolve each one's provider config, and fan out.
pub struct NotificationService {
    repo: Arc<NotificationRepository>,
    client: Client,
    /// Caps concurrent outbound sends so a burst of alerts can't open hundreds
    /// of sockets at once.
    send_limit: Arc<Semaphore>,

    slack: Arc<NotifSlackRepository>,
    telegram: Arc<NotifTelegramRepository>,
    discord: Arc<NotifDiscordRepository>,
    email: Arc<NotifEmailRepository>,
    resend: Arc<NotifResendRepository>,
    gotify: Arc<NotifGotifyRepository>,
    ntfy: Arc<NotifNtfyRepository>,
    mattermost: Arc<NotifMattermostRepository>,
    custom: Arc<NotifCustomRepository>,
    lark: Arc<NotifLarkRepository>,
    pushover: Arc<NotifPushoverRepository>,
    teams: Arc<NotifTeamRepository>,
}

#[singleton]
impl NotificationService {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        repo: Arc<NotificationRepository>,
        slack: Arc<NotifSlackRepository>,
        telegram: Arc<NotifTelegramRepository>,
        discord: Arc<NotifDiscordRepository>,
        email: Arc<NotifEmailRepository>,
        resend: Arc<NotifResendRepository>,
        gotify: Arc<NotifGotifyRepository>,
        ntfy: Arc<NotifNtfyRepository>,
        mattermost: Arc<NotifMattermostRepository>,
        custom: Arc<NotifCustomRepository>,
        lark: Arc<NotifLarkRepository>,
        pushover: Arc<NotifPushoverRepository>,
        teams: Arc<NotifTeamRepository>,
    ) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(5))
            .user_agent(concat!("Rustploy/", env!("CARGO_PKG_VERSION")))
            .build()
            .unwrap_or_default();

        Self {
            repo,
            client,
            send_limit: Arc::new(Semaphore::new(5)),
            slack,
            telegram,
            discord,
            email,
            resend,
            gotify,
            ntfy,
            mattermost,
            custom,
            lark,
            pushover,
            teams,
        }
    }

    /// Sends a message to every notification subscribed to `trigger`.
    ///
    /// Failures are logged per-provider and never propagated: one dead webhook
    /// must not stop the others, and callers are usually deploy/backup paths
    /// where a notification failure shouldn't fail the operation.
    pub async fn notify(&self, trigger: NotificationTrigger, msg: &NotificationMessage) {
        let notifications = match self.repo.get_all().await {
            Ok(items) => items,
            Err(error) => {
                tracing::error!(error = %error, "could not load notifications for dispatch");
                return;
            }
        };

        let targets: Vec<_> = notifications
            .into_iter()
            .filter(|n| trigger.is_enabled_for(n))
            .collect();

        if targets.is_empty() {
            return;
        }

        tracing::debug!(
            trigger = ?trigger,
            count = targets.len(),
            "dispatching notification"
        );

        for notification in targets {
            let permit = match self.send_limit.clone().acquire_owned().await {
                Ok(permit) => permit,
                Err(_) => return, // semaphore closed — shutting down
            };

            let name = notification.name.clone();
            let kind = notification.notification_type.clone();

            if let Err(error) = self.dispatch_one(&notification, msg).await {
                tracing::warn!(
                    notification = %name,
                    provider = %kind,
                    error = %error,
                    "notification dispatch failed"
                );
            }

            drop(permit);
        }
    }

    /// Sends a message through one specific notification row, surfacing the
    /// error. Used by the "test notification" endpoint where the operator needs
    /// to see exactly what went wrong.
    pub async fn send_test(&self, id: i64, msg: &NotificationMessage) -> Result<(), String> {
        let notification = self
            .repo
            .get_by_id(id)
            .await
            .map_err(|e| format!("could not load notification {id}: {e}"))?
            .ok_or_else(|| format!("notification {id} not found"))?;

        self.dispatch_one(&notification, msg).await
    }

    /// Resolves the provider config for one notification row and sends.
    async fn dispatch_one(
        &self,
        notification: &Notification,
        msg: &NotificationMessage,
    ) -> Result<(), String> {
        let kind = notification.notification_type.to_ascii_uppercase();

        match kind.as_str() {
            "SLACK" => {
                let cfg = self.load_slack(notification).await?;
                senders::send_slack(&self.client, &cfg, msg).await
            }
            "TELEGRAM" => {
                let cfg = self.load_telegram(notification).await?;
                senders::send_telegram(&self.client, &cfg, msg).await
            }
            "DISCORD" => {
                let cfg = self.load_discord(notification).await?;
                senders::send_discord(&self.client, &cfg, msg).await
            }
            "EMAIL" => {
                let cfg = self.load_email(notification).await?;
                send_email(&cfg, msg).await
            }
            "RESEND" => {
                let cfg = self.load_resend(notification).await?;
                senders::send_resend(&self.client, &cfg, msg).await
            }
            "GOTIFY" => {
                let cfg = self.load_gotify(notification).await?;
                senders::send_gotify(&self.client, &cfg, msg).await
            }
            "NTFY" => {
                let cfg = self.load_ntfy(notification).await?;
                senders::send_ntfy(&self.client, &cfg, msg).await
            }
            "MATTERMOST" => {
                let cfg = self.load_mattermost(notification).await?;
                senders::send_mattermost(&self.client, &cfg, msg).await
            }
            "TEAMS" => {
                let cfg = self.load_teams(notification).await?;
                senders::send_teams(&self.client, &cfg, msg).await
            }
            "LARK" => {
                let cfg = self.load_lark(notification).await?;
                senders::send_lark(&self.client, &cfg, msg).await
            }
            "PUSHOVER" => {
                let cfg = self.load_pushover(notification).await?;
                senders::send_pushover(&self.client, &cfg, msg).await
            }
            "CUSTOM" => {
                let cfg = self.load_custom(notification).await?;
                senders::send_custom(&self.client, &cfg, msg).await
            }
            other => Err(format!("unknown notification type {other}")),
        }
    }
}

/// Generates the `load_*` helpers, which all follow the same shape: read the
/// provider FK off the notification row, load that provider config, and turn
/// both "no FK set" and "row missing" into a clear error.
macro_rules! provider_loader {
    ($method:ident, $field:ident, $repo:ident, $model:ty, $label:literal) => {
        impl NotificationService {
            async fn $method(&self, notification: &Notification) -> Result<$model, String> {
                let id = notification
                    .$field
                    .ok_or_else(|| format!(concat!($label, " notification has no ", stringify!($field))))?;

                self.$repo
                    .get_by_id(id)
                    .await
                    .map_err(|e| format!(concat!("could not load ", $label, " config: {}"), e))?
                    .ok_or_else(|| format!(concat!($label, " config {} not found"), id))
            }
        }
    };
}

use crate::db::models::{
    notif_custom::NotifCustom, notif_discord::NotifDiscord, notif_email::NotifEmail,
    notif_gotify::NotifGotify, notif_lark::NotifLark, notif_mattermost::NotifMattermost,
    notif_ntfy::NotifNtfy, notif_pushover::NotifPushover, notif_resend::NotifResend,
    notif_slack::NotifSlack, notif_teams::NotifTeam, notif_telegram::NotifTelegram,
};

provider_loader!(load_slack, slack_id, slack, NotifSlack, "slack");
provider_loader!(load_telegram, telegram_id, telegram, NotifTelegram, "telegram");
provider_loader!(load_discord, discord_id, discord, NotifDiscord, "discord");
provider_loader!(load_email, email_id, email, NotifEmail, "email");
provider_loader!(load_resend, resend_id, resend, NotifResend, "resend");
provider_loader!(load_gotify, gotify_id, gotify, NotifGotify, "gotify");
provider_loader!(load_ntfy, ntfy_id, ntfy, NotifNtfy, "ntfy");
provider_loader!(
    load_mattermost,
    mattermost_id,
    mattermost,
    NotifMattermost,
    "mattermost"
);
provider_loader!(load_teams, teams_id, teams, NotifTeam, "teams");
provider_loader!(load_lark, lark_id, lark, NotifLark, "lark");
provider_loader!(load_pushover, pushover_id, pushover, NotifPushover, "pushover");
provider_loader!(load_custom, custom_id, custom, NotifCustom, "custom");
