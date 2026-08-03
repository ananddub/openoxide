use crate::{
    db::models::notif_gotify::NotifGotify,
    services::notification::{guard::validate_webhook_url, message::NotificationMessage},
};
use reqwest::Client;
use serde_json::json;

pub async fn send_gotify(
    client: &Client,
    cfg: &NotifGotify,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let base = cfg.server_url.trim_end_matches('/');
    let url = format!("{base}/message");
    validate_webhook_url(&url)?;

    let priority = if cfg.priority > 0 {
        cfg.priority
    } else {
        msg.level.gotify_priority()
    };

    let payload = json!({
        "title": msg.subject(),
        "message": msg.to_plain_text(),
        "priority": priority,
    });

    let response = client
        .post(&url)
        .header("X-Gotify-Key", &cfg.app_token)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("gotify request failed: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("gotify returned status {}", response.status()));
    }

    Ok(())
}
