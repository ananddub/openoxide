use super::webhook::post_webhook;
use crate::{
    db::models::notif_slack::NotifSlack,
    services::notification::message::NotificationMessage,
};
use reqwest::Client;
use serde_json::json;

pub async fn send_slack(
    client: &Client,
    cfg: &NotifSlack,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let mut blocks = vec![
        json!({
            "type": "header",
            "text": {"type": "plain_text", "text": msg.subject(), "emoji": true}
        }),
        json!({
            "type": "section",
            "text": {"type": "mrkdwn", "text": msg.body}
        }),
    ];

    if !msg.fields.is_empty() {
        blocks.push(json!({
            "type": "section",
            "fields": msg.fields.iter().map(|(k, v)| json!({
                "type": "mrkdwn",
                "text": format!("*{k}*\n{v}")
            })).collect::<Vec<_>>()
        }));
    }

    if let Some(url) = &msg.url {
        blocks.push(json!({
            "type": "section",
            "text": {"type": "mrkdwn", "text": format!("<{url}|Open in Rustploy>")}
        }));
    }

    let mut payload = json!({ "text": msg.subject(), "blocks": blocks });
    if let Some(channel) = cfg.channel.as_deref().filter(|c| !c.is_empty()) {
        payload["channel"] = json!(channel);
    }

    post_webhook(client, &cfg.webhook_url, &payload, "slack").await
}
