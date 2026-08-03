use super::webhook::post_webhook;
use crate::{
    db::models::notif_mattermost::NotifMattermost,
    services::notification::message::NotificationMessage,
};
use reqwest::Client;
use serde_json::json;

pub async fn send_mattermost(
    client: &Client,
    cfg: &NotifMattermost,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let mut payload = json!({ "text": msg.to_plain_text() });

    if let Some(channel) = cfg.channel.as_deref().filter(|c| !c.is_empty()) {
        payload["channel"] = json!(channel);
    }
    if let Some(username) = cfg.username.as_deref().filter(|u| !u.is_empty()) {
        payload["username"] = json!(username);
    }

    post_webhook(client, &cfg.webhook_url, &payload, "mattermost").await
}
