use std::sync::Arc;
use axum::http::StatusCode;

use crate::{
    api::dto::notification::CreateNotificationDto,
    db::{
        models::notifications::Notification,
        repository::{
            NotifCustomRepository, NotifDiscordRepository, NotifEmailRepository, NotifGotifyRepository,
            NotifLarkRepository, NotifMattermostRepository, NotifNtfyRepository,
            NotifPushoverRepository, NotifResendRepository, NotifSlackRepository, NotifTeamRepository,
            NotifTelegramRepository,
        },
    },
    services::notification::NotificationProvider,
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
        match dto.notification_type {
            NotificationProvider::Slack => {
                let url = dto
                    .webhook_url
                    .clone()
                    .ok_or((StatusCode::BAD_REQUEST, "webhook_url is required for Slack".into()))?;
                let id = self
                    .slack
                    .create(&crate::db::models::notif_slack::NotifSlack {
                        id: None,
                        channel: dto.channel.clone(),
                        webhook_url: url,
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.slack_id = Some(id);
            }
            NotificationProvider::Telegram => {
                let bot = dto
                    .bot_token
                    .clone()
                    .ok_or((StatusCode::BAD_REQUEST, "bot_token is required for Telegram".into()))?;
                let chat = dto
                    .chat_id
                    .clone()
                    .ok_or((StatusCode::BAD_REQUEST, "chat_id is required for Telegram".into()))?;
                let id = self
                    .telegram
                    .create(&crate::db::models::notif_telegram::NotifTelegram {
                        id: None,
                        bot_token: bot,
                        chat_id: chat,
                        message_thread_id: dto.message_thread_id.clone(),
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.telegram_id = Some(id);
            }
            NotificationProvider::Discord => {
                let url = dto
                    .webhook_url
                    .clone()
                    .ok_or((StatusCode::BAD_REQUEST, "webhook_url is required for Discord".into()))?;
                let id = self
                    .discord
                    .create(&crate::db::models::notif_discord::NotifDiscord {
                        id: None,
                        webhook_url: url,
                        decoration: dto.decoration.unwrap_or(1),
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.discord_id = Some(id);
            }
            NotificationProvider::Email => {
                let server = dto.smtp_server.clone().ok_or((
                    StatusCode::BAD_REQUEST,
                    "smtp_server is required for Email".into(),
                ))?;
                let port = dto
                    .smtp_port
                    .ok_or((StatusCode::BAD_REQUEST, "smtp_port is required for Email".into()))?;
                let user = dto
                    .username
                    .clone()
                    .ok_or((StatusCode::BAD_REQUEST, "username is required for Email".into()))?;
                let pass = dto
                    .password
                    .clone()
                    .ok_or((StatusCode::BAD_REQUEST, "password is required for Email".into()))?;
                let from = dto.from_address.clone().ok_or((
                    StatusCode::BAD_REQUEST,
                    "from_address is required for Email".into(),
                ))?;
                let to = dto
                    .to_addresses
                    .as_ref()
                    .map(|addrs| serde_json::to_string(addrs).unwrap_or_default())
                    .unwrap_or_default();
                let id = self
                    .email
                    .create(&crate::db::models::notif_email::NotifEmail {
                        id: None,
                        smtp_server: server,
                        smtp_port: port,
                        username: user,
                        password: pass,
                        from_address: from,
                        to_addresses: to,
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.email_id = Some(id);
            }
            NotificationProvider::Resend => {
                let key = dto
                    .api_key
                    .clone()
                    .ok_or((StatusCode::BAD_REQUEST, "api_key is required for Resend".into()))?;
                let from = dto.from_address.clone().ok_or((
                    StatusCode::BAD_REQUEST,
                    "from_address is required for Resend".into(),
                ))?;
                let to = dto
                    .to_addresses
                    .as_ref()
                    .map(|addrs| serde_json::to_string(addrs).unwrap_or_default())
                    .unwrap_or_default();
                let id = self
                    .resend
                    .create(&crate::db::models::notif_resend::NotifResend {
                        id: None,
                        api_key: key,
                        from_address: from,
                        to_addresses: to,
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.resend_id = Some(id);
            }
            NotificationProvider::Gotify => {
                let url = dto
                    .server_url
                    .clone()
                    .ok_or((StatusCode::BAD_REQUEST, "server_url is required for Gotify".into()))?;
                let token = dto
                    .app_token
                    .clone()
                    .ok_or((StatusCode::BAD_REQUEST, "app_token is required for Gotify".into()))?;
                let id = self
                    .gotify
                    .create(&crate::db::models::notif_gotify::NotifGotify {
                        id: None,
                        server_url: url,
                        app_token: token,
                        priority: dto.priority.unwrap_or(5),
                        decoration: dto.decoration.unwrap_or(0),
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.gotify_id = Some(id);
            }
            NotificationProvider::Ntfy => {
                let url = dto
                    .server_url
                    .clone()
                    .ok_or((StatusCode::BAD_REQUEST, "server_url is required for Ntfy".into()))?;
                let topic = dto
                    .topic
                    .clone()
                    .ok_or((StatusCode::BAD_REQUEST, "topic is required for Ntfy".into()))?;
                let id = self
                    .ntfy
                    .create(&crate::db::models::notif_ntfy::NotifNtfy {
                        id: None,
                        server_url: url,
                        topic,
                        access_token: dto.access_token.clone(),
                        priority: dto.priority.unwrap_or(3),
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.ntfy_id = Some(id);
            }
            NotificationProvider::Mattermost => {
                let url = dto.webhook_url.clone().ok_or((
                    StatusCode::BAD_REQUEST,
                    "webhook_url is required for Mattermost".into(),
                ))?;
                let id = self
                    .mattermost
                    .create(&crate::db::models::notif_mattermost::NotifMattermost {
                        id: None,
                        webhook_url: url,
                        channel: dto.channel.clone(),
                        username: dto.username.clone(),
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.mattermost_id = Some(id);
            }
            NotificationProvider::Custom => {
                let ep = dto.endpoint.clone().ok_or((
                    StatusCode::BAD_REQUEST,
                    "endpoint is required for Custom webhook".into(),
                ))?;
                let id = self
                    .custom
                    .create(&crate::db::models::notif_custom::NotifCustom {
                        id: None,
                        endpoint: ep,
                        headers: dto.headers.clone(),
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.custom_id = Some(id);
            }
            NotificationProvider::Lark => {
                let url = dto
                    .webhook_url
                    .clone()
                    .ok_or((StatusCode::BAD_REQUEST, "webhook_url is required for Lark".into()))?;
                let id = self
                    .lark
                    .create(&crate::db::models::notif_lark::NotifLark {
                        id: None,
                        webhook_url: url,
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.lark_id = Some(id);
            }
            NotificationProvider::Pushover => {
                let key = dto.user_key.clone().ok_or((
                    StatusCode::BAD_REQUEST,
                    "user_key is required for Pushover".into(),
                ))?;
                let token = dto.api_token.clone().ok_or((
                    StatusCode::BAD_REQUEST,
                    "api_token is required for Pushover".into(),
                ))?;
                let id = self
                    .pushover
                    .create(&crate::db::models::notif_pushover::NotifPushover {
                        id: None,
                        user_key: key,
                        api_token: token,
                        priority: dto.priority.unwrap_or(0),
                        retry: dto.retry,
                        expire: dto.expire,
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.pushover_id = Some(id);
            }
            NotificationProvider::Teams => {
                let url = dto.webhook_url.clone().ok_or((
                    StatusCode::BAD_REQUEST,
                    "webhook_url is required for Teams".into(),
                ))?;
                let id = self
                    .teams
                    .create(&crate::db::models::notif_teams::NotifTeam {
                        id: None,
                        webhook_url: url,
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                notif.teams_id = Some(id);
            }
        }
        Ok(())
    }
}
