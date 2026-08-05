use super::parse::parse_addresses;
use crate::{
    db::models::notif_resend::NotifResend, services::notification::message::NotificationMessage,
};
use reqwest::Client;
use serde_json::json;

pub async fn send_resend(
    client: &Client,
    cfg: &NotifResend,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let to = parse_addresses(&cfg.to_addresses);
    if to.is_empty() {
        return Err("resend notification has no recipients".to_string());
    }

    let payload = json!({
        "from": cfg.from_address,
        "to": to,
        "subject": msg.subject(),
        "text": msg.to_plain_text(),
    });

    let response = client
        .post("https://api.resend.com/emails")
        .bearer_auth(&cfg.api_key)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("resend request failed: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("resend returned status {}", response.status()));
    }

    Ok(())
}

pub async fn send_resend_to(
    client: &Client,
    cfg: &NotifResend,
    recipient: &str,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let mut targeted = cfg.clone();
    targeted.to_addresses = serde_json::to_string(&[recipient]).map_err(|e| e.to_string())?;
    send_resend(client, &targeted, msg).await
}
