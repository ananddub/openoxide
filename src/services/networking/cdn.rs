use auto_di::singleton;

use crate::api::dto::networking::{CdnPurgeDto, CdnPurgeResponseDto};

pub struct CdnPurgeService {
    client: reqwest::Client,
}

#[singleton]
impl CdnPurgeService {
    fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn purge(&self, input: CdnPurgeDto) -> Result<CdnPurgeResponseDto, String> {
        validate_urls(&input.urls)?;
        match input.provider {
            crate::api::dto::networking::CdnProvider::Cloudflare => self.cloudflare(input).await,
            crate::api::dto::networking::CdnProvider::Fastly => self.fastly(input).await,
            crate::api::dto::networking::CdnProvider::Bunny => self.bunny(input).await,
        }
    }

    async fn cloudflare(&self, input: CdnPurgeDto) -> Result<CdnPurgeResponseDto, String> {
        let zone = required(input.zone_id, "zone_id")?;
        if !input.purge_all && input.urls.is_empty() {
            return Err("Cloudflare purge requires purge_all or at least one URL".into());
        }
        let body = if input.purge_all {
            serde_json::json!({ "purge_everything": true })
        } else {
            serde_json::json!({ "files": input.urls })
        };
        let response = self
            .client
            .post(format!(
                "https://api.cloudflare.com/client/v4/zones/{zone}/purge_cache"
            ))
            .bearer_auth(input.api_token)
            .json(&body)
            .send()
            .await
            .map_err(|error| error.to_string())?;
        response_result("cloudflare", response).await
    }

    async fn fastly(&self, input: CdnPurgeDto) -> Result<CdnPurgeResponseDto, String> {
        let service = required(input.service_id, "service_id")?;
        if !input.purge_all {
            return Err("Fastly URL purging requires surrogate keys; use purge_all".into());
        }
        let response = self
            .client
            .post(format!(
                "https://api.fastly.com/service/{service}/purge_all"
            ))
            .header("Fastly-Key", input.api_token)
            .send()
            .await
            .map_err(|error| error.to_string())?;
        response_result("fastly", response).await
    }

    async fn bunny(&self, input: CdnPurgeDto) -> Result<CdnPurgeResponseDto, String> {
        let zone = input
            .pull_zone_id
            .ok_or_else(|| "pull_zone_id is required for Bunny CDN".to_string())?;
        let response = if input.purge_all {
            self.client
                .post(format!("https://api.bunny.net/pullzone/{zone}/purgeCache"))
                .header("AccessKey", input.api_token)
                .send()
                .await
        } else {
            let url = input
                .urls
                .first()
                .ok_or_else(|| "Bunny purge requires purge_all or a URL".to_string())?;
            self.client
                .post("https://api.bunny.net/purge")
                .header("AccessKey", input.api_token)
                .query(&[("url", url), ("async", &"false".to_string())])
                .send()
                .await
        }
        .map_err(|error| error.to_string())?;
        response_result("bunny", response).await
    }
}

async fn response_result(
    provider: &str,
    response: reqwest::Response,
) -> Result<CdnPurgeResponseDto, String> {
    let status = response.status();
    let request_id = response
        .headers()
        .get("cf-ray")
        .or_else(|| response.headers().get("fastly-request-id"))
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    let body = response.text().await.unwrap_or_default();
    if !status.is_success() {
        return Err(format!(
            "{provider} purge failed with HTTP {}: {}",
            status.as_u16(),
            truncate(&body)
        ));
    }
    Ok(CdnPurgeResponseDto {
        provider: provider.into(),
        success: true,
        status: status.as_u16(),
        request_id,
        message: truncate(&body),
    })
}

fn required(value: Option<String>, field: &str) -> Result<String, String> {
    value
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| format!("{field} is required"))
}

fn validate_urls(urls: &[String]) -> Result<(), String> {
    for url in urls {
        if !(url.starts_with("https://") || url.starts_with("http://"))
            || url.contains(['\r', '\n'])
        {
            return Err(format!("invalid purge URL: {url}"));
        }
    }
    Ok(())
}

fn truncate(value: &str) -> String {
    value.chars().take(2_000).collect()
}
