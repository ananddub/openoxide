use axum::http::StatusCode;
use std::sync::Arc;

use crate::{
    api::dto::notification::CreateNotificationDto,
    api::dto::notification::create::NotificationConfigDto,
    db::{
        models::notifications::Notification,
        repository::{
            NotifCustomRepository, NotifDiscordRepository, NotifEmailRepository,
            NotifGotifyRepository, NotifLarkRepository, NotifMattermostRepository,
            NotifNtfyRepository, NotifPushoverRepository, NotifResendRepository,
            NotifSlackRepository, NotifTeamRepository, NotifTelegramRepository,
        },
    },
};

type ApiError = (StatusCode, String);

pub struct NotificationProviderBuilder {
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

impl NotificationProviderBuilder {
    pub async fn bind_provider(
        &self,
        notif: &mut Notification,
        dto: &CreateNotificationDto,
    ) -> Result<(), ApiError> {
        match &dto.provider {
            NotificationConfigDto::Slack(config) => {
                let id = self
                    .slack
                    .create(&crate::db::models::notif_slack::NotifSlack {
                        id: None,
                        channel: config.channel.clone(),
                        webhook_url: config.webhook_url.clone(),
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.slack_id = Some(id);
            }
            NotificationConfigDto::Telegram(config) => {
                let id = self
                    .telegram
                    .create(&crate::db::models::notif_telegram::NotifTelegram {
                        id: None,
                        bot_token: config.bot_token.clone(),
                        chat_id: config.chat_id.clone(),
                        message_thread_id: config.message_thread_id.clone(),
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.telegram_id = Some(id);
            }
            NotificationConfigDto::Discord(config) => {
                let id = self
                    .discord
                    .create(&crate::db::models::notif_discord::NotifDiscord {
                        id: None,
                        webhook_url: config.webhook_url.clone(),
                        decoration: config.decoration as i64,
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.discord_id = Some(id);
            }
            NotificationConfigDto::Email(config) => {
                let to = serde_json::to_string(&config.to_addresses).unwrap_or_default();
                let id = self
                    .email
                    .create(&crate::db::models::notif_email::NotifEmail {
                        id: None,
                        smtp_server: config.smtp_server.clone(),
                        smtp_port: config.smtp_port,
                        username: config.username.clone(),
                        password: config.password.clone(),
                        from_address: config.from_address.clone(),
                        to_addresses: to,
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.email_id = Some(id);
            }
            NotificationConfigDto::Resend(config) => {
                let to = serde_json::to_string(&config.to_addresses).unwrap_or_default();
                let id = self
                    .resend
                    .create(&crate::db::models::notif_resend::NotifResend {
                        id: None,
                        api_key: config.api_key.clone(),
                        from_address: config.from_address.clone(),
                        to_addresses: to,
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.resend_id = Some(id);
            }
            NotificationConfigDto::Gotify(config) => {
                let id = self
                    .gotify
                    .create(&crate::db::models::notif_gotify::NotifGotify {
                        id: None,
                        server_url: config.server_url.clone(),
                        app_token: config.app_token.clone(),
                        priority: config.priority,
                        decoration: config.decoration as i64,
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.gotify_id = Some(id);
            }
            NotificationConfigDto::Ntfy(config) => {
                let id = self
                    .ntfy
                    .create(&crate::db::models::notif_ntfy::NotifNtfy {
                        id: None,
                        server_url: config.server_url.clone(),
                        topic: config.topic.clone(),
                        access_token: config.access_token.clone(),
                        priority: config.priority,
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.ntfy_id = Some(id);
            }
            NotificationConfigDto::Mattermost(config) => {
                let id = self
                    .mattermost
                    .create(&crate::db::models::notif_mattermost::NotifMattermost {
                        id: None,
                        webhook_url: config.webhook_url.clone(),
                        channel: config.channel.clone(),
                        username: config.username.clone(),
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.mattermost_id = Some(id);
            }
            NotificationConfigDto::Custom(config) => {
                let id = self
                    .custom
                    .create(&crate::db::models::notif_custom::NotifCustom {
                        id: None,
                        endpoint: config.endpoint.clone(),
                        headers: config.headers.clone(),
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.custom_id = Some(id);
            }
            NotificationConfigDto::Lark(config) => {
                let id = self
                    .lark
                    .create(&crate::db::models::notif_lark::NotifLark {
                        id: None,
                        webhook_url: config.webhook_url.clone(),
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.lark_id = Some(id);
            }
            NotificationConfigDto::Pushover(config) => {
                let id = self
                    .pushover
                    .create(&crate::db::models::notif_pushover::NotifPushover {
                        id: None,
                        user_key: config.user_key.clone(),
                        api_token: config.api_token.clone(),
                        priority: config.priority,
                        retry: config.retry,
                        expire: config.expire,
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.pushover_id = Some(id);
            }
            NotificationConfigDto::Teams(config) => {
                let id = self
                    .teams
                    .create(&crate::db::models::notif_teams::NotifTeam {
                        id: None,
                        webhook_url: config.webhook_url.clone(),
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.teams_id = Some(id);
            }
        }
        Ok(())
    }
}
