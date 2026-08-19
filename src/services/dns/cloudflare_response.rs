use super::{DnsService, DnsServiceError};
use serde::de::DeserializeOwned;

impl DnsService {
    pub(super) async fn cloudflare_result<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
        operation: &str,
    ) -> Result<T, DnsServiceError> {
        let status = response.status();
        let body: serde_json::Value = response.json().await?;
        if !status.is_success() || body["success"].as_bool() != Some(true) {
            let detail = body["errors"]
                .as_array()
                .map(|errors| {
                    errors
                        .iter()
                        .filter_map(|error| error["message"].as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_default();
            return Err(DnsServiceError::ProviderError(format!(
                "Cloudflare: {operation} failed{}",
                if detail.is_empty() {
                    format!(" (status {status})")
                } else {
                    format!(": {detail}")
                }
            )));
        }
        serde_json::from_value(body["result"].clone()).map_err(|error| {
            DnsServiceError::ProviderError(format!(
                "Cloudflare returned an invalid response: {error}"
            ))
        })
    }
}
