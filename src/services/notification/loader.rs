use std::sync::Arc;

use crate::db::{
    models::{
        notif_custom::NotifCustom, notif_discord::NotifDiscord, notif_email::NotifEmail,
        notif_gotify::NotifGotify, notif_lark::NotifLark, notif_mattermost::NotifMattermost,
        notif_ntfy::NotifNtfy, notif_pushover::NotifPushover, notif_resend::NotifResend,
        notif_slack::NotifSlack, notif_teams::NotifTeam, notif_telegram::NotifTelegram,
        notifications::Notification,
    },
    repository::{
        NotifCustomRepository, NotifDiscordRepository, NotifEmailRepository, NotifGotifyRepository,
        NotifLarkRepository, NotifMattermostRepository, NotifNtfyRepository,
        NotifPushoverRepository, NotifResendRepository, NotifSlackRepository, NotifTeamRepository,
        NotifTelegramRepository,
    },
};

pub struct NotificationConfigLoader {
    pub slack: Arc<NotifSlackRepository>,
    pub telegram: Arc<NotifTelegramRepository>,
    pub discord: Arc<NotifDiscordRepository>,
    pub email: Arc<NotifEmailRepository>,
    pub resend: Arc<NotifResendRepository>,
    pub gotify: Arc<NotifGotifyRepository>,
    pub ntfy: Arc<NotifNtfyRepository>,
    pub mattermost: Arc<NotifMattermostRepository>,
    pub custom: Arc<NotifCustomRepository>,
    pub lark: Arc<NotifLarkRepository>,
    pub pushover: Arc<NotifPushoverRepository>,
    pub teams: Arc<NotifTeamRepository>,
}

impl NotificationConfigLoader {
    pub async fn load_slack(&self, n: &Notification) -> Result<NotifSlack, String> {
        let id = n.slack_id.ok_or("slack notification has no slack_id")?;
        self.slack
            .get_by_id(id)
            .await
            .map_err(|e| format!("could not load slack config: {e}"))?
            .ok_or_else(|| format!("slack config {id} not found"))
    }

    pub async fn load_telegram(&self, n: &Notification) -> Result<NotifTelegram, String> {
        let id = n.telegram_id.ok_or("telegram notification has no telegram_id")?;
        self.telegram
            .get_by_id(id)
            .await
            .map_err(|e| format!("could not load telegram config: {e}"))?
            .ok_or_else(|| format!("telegram config {id} not found"))
    }

    pub async fn load_discord(&self, n: &Notification) -> Result<NotifDiscord, String> {
        let id = n.discord_id.ok_or("discord notification has no discord_id")?;
        self.discord
            .get_by_id(id)
            .await
            .map_err(|e| format!("could not load discord config: {e}"))?
            .ok_or_else(|| format!("discord config {id} not found"))
    }

    pub async fn load_email(&self, n: &Notification) -> Result<NotifEmail, String> {
        let id = n.email_id.ok_or("email notification has no email_id")?;
        self.email
            .get_by_id(id)
            .await
            .map_err(|e| format!("could not load email config: {e}"))?
            .ok_or_else(|| format!("email config {id} not found"))
    }

    pub async fn load_resend(&self, n: &Notification) -> Result<NotifResend, String> {
        let id = n.resend_id.ok_or("resend notification has no resend_id")?;
        self.resend
            .get_by_id(id)
            .await
            .map_err(|e| format!("could not load resend config: {e}"))?
            .ok_or_else(|| format!("resend config {id} not found"))
    }

    pub async fn load_gotify(&self, n: &Notification) -> Result<NotifGotify, String> {
        let id = n.gotify_id.ok_or("gotify notification has no gotify_id")?;
        self.gotify
            .get_by_id(id)
            .await
            .map_err(|e| format!("could not load gotify config: {e}"))?
            .ok_or_else(|| format!("gotify config {id} not found"))
    }

    pub async fn load_ntfy(&self, n: &Notification) -> Result<NotifNtfy, String> {
        let id = n.ntfy_id.ok_or("ntfy notification has no ntfy_id")?;
        self.ntfy
            .get_by_id(id)
            .await
            .map_err(|e| format!("could not load ntfy config: {e}"))?
            .ok_or_else(|| format!("ntfy config {id} not found"))
    }

    pub async fn load_mattermost(&self, n: &Notification) -> Result<NotifMattermost, String> {
        let id = n.mattermost_id.ok_or("mattermost notification has no mattermost_id")?;
        self.mattermost
            .get_by_id(id)
            .await
            .map_err(|e| format!("could not load mattermost config: {e}"))?
            .ok_or_else(|| format!("mattermost config {id} not found"))
    }

    pub async fn load_teams(&self, n: &Notification) -> Result<NotifTeam, String> {
        let id = n.teams_id.ok_or("teams notification has no teams_id")?;
        self.teams
            .get_by_id(id)
            .await
            .map_err(|e| format!("could not load teams config: {e}"))?
            .ok_or_else(|| format!("teams config {id} not found"))
    }

    pub async fn load_lark(&self, n: &Notification) -> Result<NotifLark, String> {
        let id = n.lark_id.ok_or("lark notification has no lark_id")?;
        self.lark
            .get_by_id(id)
            .await
            .map_err(|e| format!("could not load lark config: {e}"))?
            .ok_or_else(|| format!("lark config {id} not found"))
    }

    pub async fn load_pushover(&self, n: &Notification) -> Result<NotifPushover, String> {
        let id = n.pushover_id.ok_or("pushover notification has no pushover_id")?;
        self.pushover
            .get_by_id(id)
            .await
            .map_err(|e| format!("could not load pushover config: {e}"))?
            .ok_or_else(|| format!("pushover config {id} not found"))
    }

    pub async fn load_custom(&self, n: &Notification) -> Result<NotifCustom, String> {
        let id = n.custom_id.ok_or("custom notification has no custom_id")?;
        self.custom
            .get_by_id(id)
            .await
            .map_err(|e| format!("could not load custom config: {e}"))?
            .ok_or_else(|| format!("custom config {id} not found"))
    }
}
