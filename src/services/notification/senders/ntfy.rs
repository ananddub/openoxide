use crate::{
    db::models::notif_ntfy::NotifNtfy,
    services::notification::{guard::validate_webhook_url, message::NotificationMessage},
};
use reqwest::Client;

pub async fn send_ntfy(
    client: &Client,
    cfg: &NotifNtfy,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let base = cfg.server_url.trim_end_matches('/');
    let url = format!("{base}/{}", cfg.topic);
    validate_webhook_url(&url)?;

    let priority = if cfg.priority > 0 {
        cfg.priority
    } else {
        msg.level.ntfy_priority()
    };

    let mut request = client
        .post(&url)
        .header("Title", msg.subject())
        .header("Priority", priority.to_string())
        .body(msg.to_plain_text());

    if let Some(token) = cfg.access_token.as_deref().filter(|t| !t.is_empty()) {
        request = request.bearer_auth(token);
    }

    if let Some(link) = &msg.url {
        request = request.header("Click", link);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("ntfy request failed: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("ntfy returned status {}", response.status()));
    }

    Ok(())
}
