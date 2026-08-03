use super::webhook::post_webhook;
use crate::{
    db::models::notif_discord::NotifDiscord,
    services::notification::message::NotificationMessage,
};
use reqwest::Client;
use serde_json::json;

pub async fn send_discord(
    client: &Client,
    cfg: &NotifDiscord,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let payload = if cfg.decoration == 0 {
        json!({ "content": msg.to_plain_text() })
    } else {
        let mut embed = json!({
            "title": msg.subject(),
            "description": msg.body,
            "color": msg.level.discord_color(),
            "timestamp": chrono::DateTime::from_timestamp(msg.timestamp, 0)
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339(),
        });

        if !msg.fields.is_empty() {
            embed["fields"] = json!(
                msg.fields
                    .iter()
                    .map(|(k, v)| json!({"name": k, "value": v, "inline": true}))
                    .collect::<Vec<_>>()
            );
        }

        if let Some(url) = &msg.url {
            embed["url"] = json!(url);
        }

        json!({ "embeds": [embed] })
    };

    post_webhook(client, &cfg.webhook_url, &payload, "discord").await
}
