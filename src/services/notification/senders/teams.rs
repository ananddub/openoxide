use super::webhook::post_webhook;
use crate::{
    db::models::notif_teams::NotifTeam,
    services::notification::message::NotificationMessage,
};
use reqwest::Client;
use serde_json::json;

pub async fn send_teams(
    client: &Client,
    cfg: &NotifTeam,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let payload = json!({
        "@type": "MessageCard",
        "@context": "https://schema.org/extensions",
        "summary": msg.title,
        "themeColor": msg.level.teams_color(),
        "title": msg.subject(),
        "text": msg.body,
        "sections": [{
            "facts": msg.fields.iter()
                .map(|(k, v)| json!({"name": k, "value": v}))
                .collect::<Vec<_>>()
        }],
    });

    post_webhook(client, &cfg.webhook_url, &payload, "teams").await
}
