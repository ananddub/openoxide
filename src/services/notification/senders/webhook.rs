use crate::services::notification::guard::validate_webhook_url;
use reqwest::Client;
use serde_json::Value;

pub async fn post_webhook(
    client: &Client,
    url: &str,
    payload: &Value,
    provider: &str,
) -> Result<(), String> {
    validate_webhook_url(url)?;

    let response = client
        .post(url)
        .json(payload)
        .send()
        .await
        .map_err(|e| format!("{provider} request failed: {e}"))?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        tracing::debug!(provider, status = status.as_u16(), body = %body, "notification provider returned an error");
        return Err(format!("{provider} returned status {status}"));
    }

    Ok(())
}
