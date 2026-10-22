use connectrpc::ConnectError;
use serde_json::json;

use crate::api::connect::generated::openoxide::api::v1::create_notification_request::ProviderView as CreateProviderView;
use crate::api::connect::generated::openoxide::api::v1::replace_notification_request::ProviderView as ReplaceProviderView;
use crate::flash_gen::models::Notifications as Notification;
use crate::services::integrations::notification::NotificationProvider;

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
        provider: NotificationProvider,
        config: serde_json::Value,
    ) -> Result<(), ApiError> {
        notif.notification_type = provider.as_str().to_owned();
        notif.config_json = serde_json::to_string(&config).map_err(|e| {
            ConnectError::new(
                connectrpc::ErrorCode::Internal,
                format!("could not serialize notification config: {e}"),
            )
        })?;
        Ok(())
    }
}

/// Extracts (provider kind, stored config document) from the create request's
/// `oneof provider`. `Custom`'s `endpoint` is stored under the key `url`.
/// Takes the wrapper by reference because the generated oneof view enum is
/// not `Copy` and the request cannot be moved out of.
pub fn create_provider_parts(
    provider: &CreateProviderView<'_>,
) -> (NotificationProvider, serde_json::Value) {
    match provider {
        CreateProviderView::Slack(config) => (
            NotificationProvider::Slack,
            json!({
                "webhook_url": config.webhook_url,
                "channel": config.channel,
            }),
        ),
        CreateProviderView::Telegram(config) => (
            NotificationProvider::Telegram,
            json!({
                "bot_token": config.bot_token,
                "chat_id": config.chat_id,
                "message_thread_id": config.message_thread_id,
            }),
        ),
        CreateProviderView::Discord(config) => (
            NotificationProvider::Discord,
            json!({
                "webhook_url": config.webhook_url,
                "decoration": config.decoration,
            }),
        ),
        CreateProviderView::Email(config) => (
            NotificationProvider::Email,
            json!({
                "smtp_server": config.smtp_server,
                "smtp_port": config.smtp_port,
                "username": config.username,
                "password": config.password,
                "from_address": config.from_address,
                "to_addresses": config.to_addresses.iter().collect::<Vec<_>>(),
            }),
        ),
        CreateProviderView::Resend(config) => (
            NotificationProvider::Resend,
            json!({
                "api_key": config.api_key,
                "from_address": config.from_address,
                "to_addresses": config.to_addresses.iter().collect::<Vec<_>>(),
            }),
        ),
        CreateProviderView::Gotify(config) => (
            NotificationProvider::Gotify,
            json!({
                "server_url": config.server_url,
                "app_token": config.app_token,
                "priority": config.priority,
                "decoration": config.decoration,
            }),
        ),
        CreateProviderView::Ntfy(config) => (
            NotificationProvider::Ntfy,
            json!({
                "server_url": config.server_url,
                "topic": config.topic,
                "access_token": config.access_token,
                "priority": config.priority,
            }),
        ),
        CreateProviderView::Mattermost(config) => (
            NotificationProvider::Mattermost,
            json!({
                "webhook_url": config.webhook_url,
                "channel": config.channel,
                "username": config.username,
            }),
        ),
        CreateProviderView::Custom(config) => (
            NotificationProvider::Custom,
            json!({
                "url": config.endpoint,
                "headers": config.headers,
            }),
        ),
        CreateProviderView::Lark(config) => (
            NotificationProvider::Lark,
            json!({
                "webhook_url": config.webhook_url,
            }),
        ),
        CreateProviderView::Pushover(config) => (
            NotificationProvider::Pushover,
            json!({
                "api_token": config.api_token,
                "user_key": config.user_key,
                "priority": config.priority,
                "retry": config.retry,
                "expire": config.expire,
            }),
        ),
        CreateProviderView::Teams(config) => (
            NotificationProvider::Teams,
            json!({
                "webhook_url": config.webhook_url,
            }),
        ),
    }
}

/// Same extraction for the replace request's `oneof provider`. The generated
/// wrapper enums for the two oneofs are distinct types, so the arms mirror
/// `create_provider_parts` exactly.
pub fn replace_provider_parts(
    provider: &ReplaceProviderView<'_>,
) -> (NotificationProvider, serde_json::Value) {
    match provider {
        ReplaceProviderView::Slack(config) => (
            NotificationProvider::Slack,
            json!({
                "webhook_url": config.webhook_url,
                "channel": config.channel,
            }),
        ),
        ReplaceProviderView::Telegram(config) => (
            NotificationProvider::Telegram,
            json!({
                "bot_token": config.bot_token,
                "chat_id": config.chat_id,
                "message_thread_id": config.message_thread_id,
            }),
        ),
        ReplaceProviderView::Discord(config) => (
            NotificationProvider::Discord,
            json!({
                "webhook_url": config.webhook_url,
                "decoration": config.decoration,
            }),
        ),
        ReplaceProviderView::Email(config) => (
            NotificationProvider::Email,
            json!({
                "smtp_server": config.smtp_server,
                "smtp_port": config.smtp_port,
                "username": config.username,
                "password": config.password,
                "from_address": config.from_address,
                "to_addresses": config.to_addresses.iter().collect::<Vec<_>>(),
            }),
        ),
        ReplaceProviderView::Resend(config) => (
            NotificationProvider::Resend,
            json!({
                "api_key": config.api_key,
                "from_address": config.from_address,
                "to_addresses": config.to_addresses.iter().collect::<Vec<_>>(),
            }),
        ),
        ReplaceProviderView::Gotify(config) => (
            NotificationProvider::Gotify,
            json!({
                "server_url": config.server_url,
                "app_token": config.app_token,
                "priority": config.priority,
                "decoration": config.decoration,
            }),
        ),
        ReplaceProviderView::Ntfy(config) => (
            NotificationProvider::Ntfy,
            json!({
                "server_url": config.server_url,
                "topic": config.topic,
                "access_token": config.access_token,
                "priority": config.priority,
            }),
        ),
        ReplaceProviderView::Mattermost(config) => (
            NotificationProvider::Mattermost,
            json!({
                "webhook_url": config.webhook_url,
                "channel": config.channel,
                "username": config.username,
            }),
        ),
        ReplaceProviderView::Custom(config) => (
            NotificationProvider::Custom,
            json!({
                "url": config.endpoint,
                "headers": config.headers,
            }),
        ),
        ReplaceProviderView::Lark(config) => (
            NotificationProvider::Lark,
            json!({
                "webhook_url": config.webhook_url,
            }),
        ),
        ReplaceProviderView::Pushover(config) => (
            NotificationProvider::Pushover,
            json!({
                "api_token": config.api_token,
                "user_key": config.user_key,
                "priority": config.priority,
                "retry": config.retry,
                "expire": config.expire,
            }),
        ),
        ReplaceProviderView::Teams(config) => (
            NotificationProvider::Teams,
            json!({
                "webhook_url": config.webhook_url,
            }),
        ),
    }
}
