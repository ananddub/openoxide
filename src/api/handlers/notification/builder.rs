use connectrpc::{ConnectError, ErrorCode};
use serde_json::json;

use crate::flash_gen::models::Notifications as Notification;

use super::helper::{CreateNotificationDto, NotificationConfigDto};

type ApiError = connectrpc::ConnectError;

/// Binds the provider-specific request config onto a notification row.
///
/// Channel settings live in `notifications.config_json`; the JSON key layout
/// per `notification_type` is documented in `.flash/queries/notifications.sql`.
pub struct NotificationProviderBuilder;

impl NotificationProviderBuilder {
    pub async fn bind_provider(
        &self,
        notif: &mut Notification,
        dto: &CreateNotificationDto,
    ) -> Result<(), ApiError> {
        let config = match &dto.provider {
            NotificationConfigDto::Slack(config) => json!({
                "webhook_url": config.webhook_url,
                "channel": config.channel,
            }),
            NotificationConfigDto::Telegram(config) => json!({
                "bot_token": config.bot_token,
                "chat_id": config.chat_id,
                "message_thread_id": config.message_thread_id,
            }),
            NotificationConfigDto::Discord(config) => json!({
                "webhook_url": config.webhook_url,
                "decoration": config.decoration,
            }),
            NotificationConfigDto::Email(config) => json!({
                "smtp_server": config.smtp_server,
                "smtp_port": config.smtp_port,
                "username": config.username,
                "password": config.password,
                "from_address": config.from_address,
                "to_addresses": config.to_addresses,
            }),
            NotificationConfigDto::Resend(config) => json!({
                "api_key": config.api_key,
                "from_address": config.from_address,
                "to_addresses": config.to_addresses,
            }),
            NotificationConfigDto::Gotify(config) => json!({
                "server_url": config.server_url,
                "app_token": config.app_token,
                "priority": config.priority,
                "decoration": config.decoration,
            }),
            NotificationConfigDto::Ntfy(config) => json!({
                "server_url": config.server_url,
                "topic": config.topic,
                "access_token": config.access_token,
                "priority": config.priority,
            }),
            NotificationConfigDto::Mattermost(config) => json!({
                "webhook_url": config.webhook_url,
                "channel": config.channel,
                "username": config.username,
            }),
            NotificationConfigDto::Custom(config) => json!({
                "url": config.endpoint,
                "headers": config.headers,
            }),
            NotificationConfigDto::Lark(config) => json!({
                "webhook_url": config.webhook_url,
            }),
            NotificationConfigDto::Pushover(config) => json!({
                "api_token": config.api_token,
                "user_key": config.user_key,
                "priority": config.priority,
                "retry": config.retry,
                "expire": config.expire,
            }),
            NotificationConfigDto::Teams(config) => json!({
                "webhook_url": config.webhook_url,
            }),
        };

        notif.config_json = serde_json::to_string(&config).map_err(|e| {
            ConnectError::new(
                ErrorCode::Internal,
                format!("could not serialize notification config: {e}"),
            )
        })?;
        Ok(())
    }
}
