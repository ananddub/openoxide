use super::super::{
    DnsRecordDto, DnsService, DnsServiceError, DnsTestResultDto, DnsZoneDto,
    UpsertDnsRecordDto, extract_credential_field,
};

/// Hetzner DNS (unified Hetzner Cloud Console API since May 2026 — the old
/// `dns.hetzner.com/api/v1` console was shut down). Bearer auth; records are
/// grouped into rrsets under a zone.
/// Credentials JSON: `{"apiToken": "<cloud api token>"}`.
const HETZNER_API: &str = "https://api.hetzner.cloud/v1";

fn hetzner_error(status: u16, body: &serde_json::Value) -> DnsServiceError {
    DnsServiceError::ProviderError(format!(
        "Hetzner API failed: HTTP {status} {}",
        body["error"]["message"]
            .as_str()
            .or_else(|| body["message"].as_str())
            .unwrap_or_default()
    ))
}

async fn hetzner_send(
    client: &reqwest::Client,
    request: reqwest::RequestBuilder,
    context: &str,
) -> Result<serde_json::Value, DnsServiceError> {
    let response = request.send().await?;
    let status = u16::from(response.status());
    let body: serde_json::Value = response.json().await.map_err(|e| {
        DnsServiceError::ProviderError(format!("Hetzner {context} decode failed: {e}"))
    })?;
    if !(200..300).contains(&status) {
        return Err(hetzner_error(status, &body));
    }
    Ok(body)
}

impl DnsService {
    pub(in crate::services::networking::dns) fn hetzner_token(
        credentials_json: &str,
    ) -> Result<String, DnsServiceError> {
        let token = extract_credential_field(credentials_json, "apiToken");
        if token.is_empty() {
            return Err(DnsServiceError::ProviderError(
                "Hetzner apiToken is required".into(),
            ));
        }
        Ok(token)
    }

    pub(in crate::services::networking::dns) async fn test_hetzner_token(
        &self,
        credentials_json: &str,
    ) -> Result<DnsTestResultDto, DnsServiceError> {
        let token = Self::hetzner_token(credentials_json)?;
        hetzner_send(
            &self.client,
            self.client
                .get(format!("{HETZNER_API}/zones"))
                .query(&[("per_page", "1")])
                .bearer_auth(token),
            "auth check",
        )
        .await?;
        Ok(DnsTestResultDto {
            success: true,
            message: "Hetzner credentials verified successfully".into(),
        })
    }

    pub(in crate::services::networking::dns) async fn list_hetzner_zones(
        &self,
        credentials_json: &str,
    ) -> Result<Vec<DnsZoneDto>, DnsServiceError> {
        let token = Self::hetzner_token(credentials_json)?;
        let body = hetzner_send(
            &self.client,
            self.client
                .get(format!("{HETZNER_API}/zones"))
                .bearer_auth(token),
            "list zones",
        )
        .await?;
        Ok(body["zones"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|z| DnsZoneDto {
                id: z["id"].as_str().unwrap_or_default().to_string(),
                name: z["name"].as_str().unwrap_or_default().to_string(),
                status: z["status"].as_str().map(str::to_string),
            })
            .collect())
    }

    pub(in crate::services::networking::dns) async fn list_hetzner_records(
        &self,
        credentials_json: &str,
        zone_id: &str,
    ) -> Result<Vec<DnsRecordDto>, DnsServiceError> {
        let token = Self::hetzner_token(credentials_json)?;
        let body = hetzner_send(
            &self.client,
            self.client
                .get(format!("{HETZNER_API}/zones/{zone_id}/rrsets"))
                .bearer_auth(token),
            "list rrsets",
        )
        .await?;
        let mut records = Vec::new();
        for rrset in body["rrsets"].as_array().cloned().unwrap_or_default() {
            let record_type = rrset["type"].as_str().unwrap_or_default().to_string();
            let name = rrset["name"].as_str().unwrap_or("@").to_string();
            let ttl = rrset["ttl"].as_u64().map(|v| v as u32);
            for entry in rrset["records"].as_array().cloned().unwrap_or_default() {
                records.push(DnsRecordDto {
                    // The rrset is the unit of identity; the value selects
                    // the entry within it.
                    id: format!("{}|{}", rrset["id"].as_str().unwrap_or_default(), entry["value"].as_str().unwrap_or_default()),
                    zone_id: zone_id.to_string(),
                    record_type: record_type.clone(),
                    name: name.clone(),
                    content: entry["value"].as_str().unwrap_or_default().to_string(),
                    ttl,
                    proxied: None,
                });
            }
        }
        Ok(records)
    }

    pub(in crate::services::networking::dns) async fn upsert_hetzner_record(
        &self,
        credentials_json: &str,
        body: &UpsertDnsRecordDto,
    ) -> Result<DnsRecordDto, DnsServiceError> {
        let token = Self::hetzner_token(credentials_json)?;
        let name = if body.name.trim().is_empty() {
            "@".to_string()
        } else {
            body.name.clone()
        };
        let payload = serde_json::json!({
            "name": name,
            "type": body.record_type,
            "ttl": body.ttl.unwrap_or(600),
            "records": [{"value": body.content}],
        });
        hetzner_send(
            &self.client,
            self.client
                .post(format!("{HETZNER_API}/zones/{}/rrsets", body.zone_id))
                .bearer_auth(token)
                .json(&payload),
            "create rrset",
        )
        .await?;
        Ok(DnsRecordDto {
            id: format!("new|{name}"),
            zone_id: body.zone_id.clone(),
            record_type: body.record_type.clone(),
            name,
            content: body.content.clone(),
            ttl: body.ttl,
            proxied: None,
        })
    }

    pub(in crate::services::networking::dns) async fn delete_hetzner_record(
        &self,
        credentials_json: &str,
        zone_id: &str,
        record_id: &str,
    ) -> Result<bool, DnsServiceError> {
        // Composite id from list: "<rrset id>|<value>". Deleting the rrset
        // removes every value it holds (Hetzner groups by name+type).
        let Some((rrset_id, _)) = record_id.split_once('|') else {
            return Err(DnsServiceError::ProviderError(
                "Hetzner record id must be the composite rrset|value".into(),
            ));
        };
        let token = Self::hetzner_token(credentials_json)?;
        hetzner_send(
            &self.client,
            self.client
                .delete(format!("{HETZNER_API}/zones/{zone_id}/rrsets/{rrset_id}"))
                .bearer_auth(token),
            "delete rrset",
        )
        .await?;
        Ok(true)
    }
}
