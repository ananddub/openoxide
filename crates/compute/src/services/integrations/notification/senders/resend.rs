use crate::services::integrations::notification::{
    loader::ResendConfig, message::NotificationMessage,
};
use reqwest::Client;
use serde_json::json;

pub async fn send_resend(
    client: &Client,
    cfg: &ResendConfig,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let to: Vec<&str> = cfg
        .to_addresses
        .iter()
        .map(|a| a.trim())
        .filter(|a| !a.is_empty())
        .collect();
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
    cfg: &ResendConfig,
    recipient: &str,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let mut targeted = cfg.clone();
    targeted.to_addresses = vec![recipient.to_owned()];
    send_resend(client, &targeted, msg).await
}
