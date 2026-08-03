use super::{
    email::send_email, loader::NotificationConfigLoader, message::NotificationMessage,
    provider::NotificationProvider, senders, trigger::NotificationTrigger,
};
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

pub struct NotificationService {
    repo: Arc<NotificationRepository>,
    client: Client,
    send_limit: Arc<Semaphore>,
    loader: NotificationConfigLoader,
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

        let loader = NotificationConfigLoader {
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
        };

        Self {
            repo,
            client,
            send_limit: Arc::new(Semaphore::new(5)),
            loader,
        }
    }

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
                Err(_) => return,
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

    pub async fn send_test(&self, id: i64, msg: &NotificationMessage) -> Result<(), String> {
        let notification = self
            .repo
            .get_by_id(id)
            .await
            .map_err(|e| format!("could not load notification {id}: {e}"))?
            .ok_or_else(|| format!("notification {id} not found"))?;

        self.dispatch_one(&notification, msg).await
    }

    async fn dispatch_one(
        &self,
        notification: &Notification,
        msg: &NotificationMessage,
    ) -> Result<(), String> {
        let provider: NotificationProvider = notification.notification_type.parse()?;

        match provider {
            NotificationProvider::Slack => {
                let cfg = self.loader.load_slack(notification).await?;
                senders::send_slack(&self.client, &cfg, msg).await
            }
            NotificationProvider::Telegram => {
                let cfg = self.loader.load_telegram(notification).await?;
                senders::send_telegram(&self.client, &cfg, msg).await
            }
            NotificationProvider::Discord => {
                let cfg = self.loader.load_discord(notification).await?;
                senders::send_discord(&self.client, &cfg, msg).await
            }
            NotificationProvider::Email => {
                let cfg = self.loader.load_email(notification).await?;
                send_email(&cfg, msg).await
            }
            NotificationProvider::Resend => {
                let cfg = self.loader.load_resend(notification).await?;
                senders::send_resend(&self.client, &cfg, msg).await
            }
            NotificationProvider::Gotify => {
                let cfg = self.loader.load_gotify(notification).await?;
                senders::send_gotify(&self.client, &cfg, msg).await
            }
            NotificationProvider::Ntfy => {
                let cfg = self.loader.load_ntfy(notification).await?;
                senders::send_ntfy(&self.client, &cfg, msg).await
            }
            NotificationProvider::Mattermost => {
                let cfg = self.loader.load_mattermost(notification).await?;
                senders::send_mattermost(&self.client, &cfg, msg).await
            }
            NotificationProvider::Teams => {
                let cfg = self.loader.load_teams(notification).await?;
                senders::send_teams(&self.client, &cfg, msg).await
            }
            NotificationProvider::Lark => {
                let cfg = self.loader.load_lark(notification).await?;
                senders::send_lark(&self.client, &cfg, msg).await
            }
            NotificationProvider::Pushover => {
                let cfg = self.loader.load_pushover(notification).await?;
                senders::send_pushover(&self.client, &cfg, msg).await
            }
            NotificationProvider::Custom => {
                let cfg = self.loader.load_custom(notification).await?;
                senders::send_custom(&self.client, &cfg, msg).await
            }
        }
    }
}
