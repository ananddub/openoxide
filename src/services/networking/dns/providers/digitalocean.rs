use super::super::{
    DnsRecordDto, DnsService, DnsServiceError, DnsTestResultDto, DnsZoneDto, UpsertDnsRecordDto,
    extract_credential_field,
};

const DO_API: &str = "https://api.digitalocean.com/v2";

/// DigitalOcean Domains API: PAT auth; zones are identified by domain name.
/// Credentials JSON: `{"apiToken": "..."}`.
impl DnsService {
    pub(in crate::services::networking::dns) async fn test_digitalocean_token(
        &self,
        credentials_json: &str,
    ) -> Result<DnsTestResultDto, DnsServiceError> {
        let token = extract_credential_field(credentials_json, "apiToken");
        if token.is_empty() {
            return Err(DnsServiceError::ProviderError(
                "DigitalOcean apiToken is required".into(),
            ));
        }
        let res = self
            .client
            .get(format!("{DO_API}/domains"))
            .query(&[("per_page", "1")])
            .bearer_auth(token)
            .send()
            .await?;
        let status = res.status();
        if !status.is_success() {
            return Err(DnsServiceError::ProviderError(format!(
                "DigitalOcean authentication failed: HTTP {status}"
            )));
        }
        Ok(DnsTestResultDto {
            success: true,
            message: "DigitalOcean credentials verified successfully".into(),
        })
    }

    pub(in crate::services::networking::dns) async fn list_digitalocean_zones(
        &self,
        credentials_json: &str,
    ) -> Result<Vec<DnsZoneDto>, DnsServiceError> {
        let token = extract_credential_field(credentials_json, "apiToken");
        let res = self
            .client
            .get(format!("{DO_API}/domains"))
            .query(&[("per_page", "200")])
            .bearer_auth(token)
            .send()
            .await?;
        let status = res.status();
        let body: serde_json::Value = res.json().await.map_err(|e| {
            DnsServiceError::ProviderError(format!("DigitalOcean zones decode failed: {e}"))
        })?;
        if !status.is_success() {
            return Err(DnsServiceError::ProviderError(format!(
                "DigitalOcean list domains failed: HTTP {status}"
            )));
        }
        Ok(body["domains"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|d| DnsZoneDto {
                // The domain name doubles as the zone id in this API.
                id: d["name"].as_str().unwrap_or_default().to_string(),
                name: d["name"].as_str().unwrap_or_default().to_string(),
                status: d["ttl"].as_u64().map(|_| "ACTIVE".to_string()),
            })
            .collect())
    }

    pub(in crate::services::networking::dns) async fn list_digitalocean_records(
        &self,
        credentials_json: &str,
        zone_id: &str,
    ) -> Result<Vec<DnsRecordDto>, DnsServiceError> {
        let token = extract_credential_field(credentials_json, "apiToken");
        let res = self
            .client
            .get(format!("{DO_API}/domains/{zone_id}/records"))
            .query(&[("per_page", "200")])
            .bearer_auth(token)
            .send()
            .await?;
        let status = res.status();
        let body: serde_json::Value = res.json().await.map_err(|e| {
            DnsServiceError::ProviderError(format!("DigitalOcean records decode failed: {e}"))
        })?;
        if !status.is_success() {
            return Err(DnsServiceError::ProviderError(format!(
                "DigitalOcean list records failed: HTTP {status}"
            )));
        }
        Ok(body["domain_records"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|r| DnsRecordDto {
                id: r["id"].as_i64().map(|v| v.to_string()).unwrap_or_default(),
                zone_id: zone_id.to_string(),
                record_type: r["type"].as_str().unwrap_or_default().to_string(),
                name: r["name"].as_str().unwrap_or_default().to_string(),
                content: r["data"].as_str().unwrap_or_default().to_string(),
                ttl: r["ttl"].as_u64().map(|v| v as u32),
                proxied: None,
            })
            .collect())
    }

    pub(in crate::services::networking::dns) async fn upsert_digitalocean_record(
        &self,
        credentials_json: &str,
        body: &UpsertDnsRecordDto,
    ) -> Result<DnsRecordDto, DnsServiceError> {
        let token = extract_credential_field(credentials_json, "apiToken");
        let name = if body.name.trim().is_empty() {
            "@".to_string()
        } else {
            body.name.clone()
        };
        let payload = serde_json::json!({
            "type": body.record_type,
            "name": name,
            "data": body.content,
            "ttl": body.ttl.unwrap_or(1800),
        });
        let res = self
            .client
            .post(format!("{DO_API}/domains/{}/records", body.zone_id))
            .bearer_auth(token)
            .json(&payload)
            .send()
            .await?;
        let status = res.status();
        let created: serde_json::Value = res.json().await.map_err(|e| {
            DnsServiceError::ProviderError(format!("DigitalOcean create decode failed: {e}"))
        })?;
        if !status.is_success() {
            return Err(DnsServiceError::ProviderError(format!(
                "DigitalOcean create record failed: HTTP {status} {}",
                created
                    .get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
            )));
        }
        Ok(DnsRecordDto {
            id: created["id"]
                .as_i64()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            zone_id: body.zone_id.clone(),
            record_type: body.record_type.clone(),
            name,
            content: body.content.clone(),
            ttl: created["ttl"].as_u64().map(|v| v as u32).or(body.ttl),
            proxied: None,
        })
    }

    pub(in crate::services::networking::dns) async fn delete_digitalocean_record(
        &self,
        credentials_json: &str,
        zone_id: &str,
        record_id: &str,
    ) -> Result<bool, DnsServiceError> {
        let token = extract_credential_field(credentials_json, "apiToken");
        let res = self
            .client
            .delete(format!("{DO_API}/domains/{zone_id}/records/{record_id}"))
            .bearer_auth(token)
            .send()
            .await?;
        let status = res.status();
        if status.is_success() {
            Ok(true)
        } else {
            Err(DnsServiceError::ProviderError(format!(
                "DigitalOcean delete record failed: HTTP {status}"
            )))
        }
    }
}
