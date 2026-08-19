use super::{DnsService, DnsServiceError, extract_credential_field};
use crate::api::dto::dns::{DnsRecordDto, DnsTestResultDto, DnsZoneDto, UpsertDnsRecordDto};

impl DnsService {
    pub(super) async fn test_cloudflare_token(
        &self,
        credentials_json: &str,
    ) -> Result<DnsTestResultDto, DnsServiceError> {
        let token = extract_credential_field(credentials_json, "apiToken");
        if token.is_empty() {
            return Ok(DnsTestResultDto {
                success: false,
                message: "Cloudflare API Token is required".into(),
            });
        }

        let res = self
            .client
            .get("https://api.cloudflare.com/client/v4/zones")
            .query(&[("per_page", "1")])
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        let _: Vec<serde_json::Value> = self.cloudflare_result(res, "/zones?per_page=1").await?;
        Ok(DnsTestResultDto {
            success: true,
            message: "Cloudflare API Token verified successfully!".into(),
        })
    }

    pub(super) async fn list_cloudflare_zones(
        &self,
        credentials_json: &str,
    ) -> Result<Vec<DnsZoneDto>, DnsServiceError> {
        let token = extract_credential_field(credentials_json, "apiToken");
        let mut zones = Vec::new();
        let mut page = 1;
        loop {
            let res = self
                .client
                .get("https://api.cloudflare.com/client/v4/zones")
                .query(&[("per_page", "50"), ("page", &page.to_string())])
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await?;
            let result: Vec<serde_json::Value> = self.cloudflare_result(res, "/zones").await?;
            let count = result.len();
            zones.extend(result.into_iter().map(|z| DnsZoneDto {
                id: z["id"].as_str().unwrap_or_default().to_string(),
                name: z["name"].as_str().unwrap_or_default().to_string(),
                status: z["status"].as_str().map(str::to_string),
            }));
            if count < 50 {
                break;
            }
            page += 1;
        }
        Ok(zones)
    }

    pub(super) async fn list_cloudflare_records(
        &self,
        credentials_json: &str,
        zone_id: &str,
    ) -> Result<Vec<DnsRecordDto>, DnsServiceError> {
        let token = extract_credential_field(credentials_json, "apiToken");
        let url = format!("https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records");
        let mut records = Vec::new();
        let mut page = 1;
        loop {
            let res = self
                .client
                .get(&url)
                .query(&[("per_page", "50"), ("page", &page.to_string())])
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await?;
            let result: Vec<serde_json::Value> =
                self.cloudflare_result(res, "list DNS records").await?;
            let count = result.len();
            records.extend(result.into_iter().map(|r| DnsRecordDto {
                id: r["id"].as_str().unwrap_or_default().to_string(),
                zone_id: zone_id.to_string(),
                record_type: r["type"].as_str().unwrap_or_default().to_string(),
                name: r["name"].as_str().unwrap_or_default().to_string(),
                content: r["content"].as_str().unwrap_or_default().to_string(),
                ttl: r["ttl"].as_u64().map(|v| v as u32),
                proxied: r["proxied"].as_bool(),
            }));
            if count < 50 {
                break;
            }
            page += 1;
        }
        Ok(records)
    }

    pub(super) async fn upsert_cloudflare_record(
        &self,
        credentials_json: &str,
        body: &UpsertDnsRecordDto,
    ) -> Result<DnsRecordDto, DnsServiceError> {
        let token = extract_credential_field(credentials_json, "apiToken");
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
            body.zone_id
        );

        if !matches!(body.record_type.as_str(), "A" | "CNAME") {
            return Err(DnsServiceError::ProviderError(
                "Only A and CNAME records are supported".into(),
            ));
        }

        let payload = serde_json::json!({
            "type": body.record_type,
            "name": body.name,
            "content": body.content,
            "ttl": body.ttl.unwrap_or(1),
        });

        let lookup = self
            .client
            .get(&url)
            .query(&[
                ("type", body.record_type.as_str()),
                ("name", body.name.as_str()),
            ])
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        let existing: Vec<serde_json::Value> =
            self.cloudflare_result(lookup, "find DNS record").await?;
        let (request, operation) =
            if let Some(record_id) = existing.first().and_then(|r| r["id"].as_str()) {
                (
                    self.client.put(format!("{url}/{record_id}")),
                    "update DNS record",
                )
            } else {
                (self.client.post(&url), "create DNS record")
            };
        let res = request
            .header("Authorization", format!("Bearer {}", token))
            .json(&payload)
            .send()
            .await?;
        let created: serde_json::Value = self.cloudflare_result(res, operation).await?;
        Ok(DnsRecordDto {
            id: created["id"].as_str().unwrap_or_default().to_string(),
            zone_id: body.zone_id.clone(),
            record_type: body.record_type.clone(),
            name: body.name.clone(),
            content: body.content.clone(),
            ttl: body.ttl,
            proxied: body.proxied,
        })
    }

    pub(super) async fn delete_cloudflare_record(
        &self,
        credentials_json: &str,
        zone_id: &str,
        record_id: &str,
    ) -> Result<bool, DnsServiceError> {
        let token = extract_credential_field(credentials_json, "apiToken");
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
            zone_id, record_id
        );

        let res = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        let _: serde_json::Value = self.cloudflare_result(res, "delete DNS record").await?;
        Ok(true)
    }
}
