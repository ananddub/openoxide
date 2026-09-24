use super::super::{
    DnsRecordDto, DnsService, DnsServiceError, DnsTestResultDto, DnsZoneDto, UpsertDnsRecordDto,
    extract_credential_field,
};

const GODADDY_API: &str = "https://api.godaddy.com/v1";

/// GoDaddy Domains API: key/secret pair auth (`sso-key KEY:SECRET`).
/// Zones are the account's domains; record upsert replaces the whole
/// name/type set (GoDaddy's PUT semantics).
/// Credentials JSON: `{"apiKey": "...", "apiSecret": "..."}`.
impl DnsService {
    pub(in crate::services::networking::dns) fn godaddy_sso_key(
        credentials_json: &str,
    ) -> Result<String, DnsServiceError> {
        let key = extract_credential_field(credentials_json, "apiKey");
        let secret = extract_credential_field(credentials_json, "apiSecret");
        if key.is_empty() || secret.is_empty() {
            return Err(DnsServiceError::ProviderError(
                "GoDaddy apiKey and apiSecret are required".into(),
            ));
        }
        Ok(format!("sso-key {key}:{secret}"))
    }

    pub(in crate::services::networking::dns) async fn test_godaddy_credentials(
        &self,
        credentials_json: &str,
    ) -> Result<DnsTestResultDto, DnsServiceError> {
        let sso = Self::godaddy_sso_key(credentials_json)?;
        let res = self
            .client
            .get(format!("{GODADDY_API}/domains"))
            .query(&[("limit", "1")])
            .header("Authorization", sso)
            .send()
            .await?;
        let status = res.status();
        if !status.is_success() {
            return Err(DnsServiceError::ProviderError(format!(
                "GoDaddy authentication failed: HTTP {status}"
            )));
        }
        Ok(DnsTestResultDto {
            success: true,
            message: "GoDaddy credentials verified successfully".into(),
        })
    }

    pub(in crate::services::networking::dns) async fn list_godaddy_zones(
        &self,
        credentials_json: &str,
    ) -> Result<Vec<DnsZoneDto>, DnsServiceError> {
        let sso = Self::godaddy_sso_key(credentials_json)?;
        let res = self
            .client
            .get(format!("{GODADDY_API}/domains"))
            .query(&[("limit", "1000")])
            .header("Authorization", sso)
            .send()
            .await?;
        let status = res.status();
        let body: serde_json::Value = res.json().await.map_err(|e| {
            DnsServiceError::ProviderError(format!("GoDaddy domains decode failed: {e}"))
        })?;
        if !status.is_success() {
            return Err(DnsServiceError::ProviderError(format!(
                "GoDaddy list domains failed: HTTP {status}"
            )));
        }
        Ok(body
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|d| DnsZoneDto {
                id: d["domain"].as_str().unwrap_or_default().to_string(),
                name: d["domain"].as_str().unwrap_or_default().to_string(),
                status: d["status"].as_str().map(|s| s.to_string()),
            })
            .collect())
    }

    pub(in crate::services::networking::dns) async fn list_godaddy_records(
        &self,
        credentials_json: &str,
        zone_id: &str,
    ) -> Result<Vec<DnsRecordDto>, DnsServiceError> {
        let sso = Self::godaddy_sso_key(credentials_json)?;
        let res = self
            .client
            .get(format!("{GODADDY_API}/domains/{zone_id}/records"))
            .header("Authorization", sso)
            .send()
            .await?;
        let status = res.status();
        let body: serde_json::Value = res.json().await.map_err(|e| {
            DnsServiceError::ProviderError(format!("GoDaddy records decode failed: {e}"))
        })?;
        if !status.is_success() {
            return Err(DnsServiceError::ProviderError(format!(
                "GoDaddy list records failed: HTTP {status}"
            )));
        }
        // Response is a map of record type -> array of entries.
        let mut records = Vec::new();
        if let Some(map) = body.as_object() {
            for (record_type, entries) in map {
                for entry in entries.as_array().cloned().unwrap_or_default() {
                    records.push(DnsRecordDto {
                        // GoDaddy identifies entries by name/type/data, not id.
                        id: format!(
                            "{}|{}|{}",
                            entry["name"].as_str().unwrap_or("@"),
                            record_type,
                            entry["data"].as_str().unwrap_or_default()
                        ),
                        zone_id: zone_id.to_string(),
                        record_type: record_type.clone(),
                        name: entry["name"].as_str().unwrap_or("@").to_string(),
                        content: entry["data"].as_str().unwrap_or_default().to_string(),
                        ttl: entry["ttl"].as_u64().map(|v| v as u32),
                        proxied: None,
                    });
                }
            }
        }
        Ok(records)
    }

    pub(in crate::services::networking::dns) async fn upsert_godaddy_record(
        &self,
        credentials_json: &str,
        body: &UpsertDnsRecordDto,
    ) -> Result<DnsRecordDto, DnsServiceError> {
        let sso = Self::godaddy_sso_key(credentials_json)?;
        let name = if body.name.trim().is_empty() || body.name == "@" {
            "@"
        } else {
            body.name.as_str()
        };
        // PUT replaces the entire name/type set — send a single entry.
        let payload = serde_json::json!([{
            "data": body.content,
            "ttl": body.ttl.unwrap_or(600),
        }]);
        let res = self
            .client
            .put(format!(
                "{GODADDY_API}/domains/{}/records/{}/{}",
                body.zone_id, body.record_type, name
            ))
            .header("Authorization", sso)
            .json(&payload)
            .send()
            .await?;
        let status = res.status();
        if !status.is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(DnsServiceError::ProviderError(format!(
                "GoDaddy upsert record failed: HTTP {status} {text}"
            )));
        }
        Ok(DnsRecordDto {
            id: format!("{name}|{}|{}", body.record_type, body.content),
            zone_id: body.zone_id.clone(),
            record_type: body.record_type.clone(),
            name: name.to_string(),
            content: body.content.clone(),
            ttl: body.ttl,
            proxied: None,
        })
    }

    pub(in crate::services::networking::dns) async fn delete_godaddy_record(
        &self,
        credentials_json: &str,
        zone_id: &str,
        record_id: &str,
    ) -> Result<bool, DnsServiceError> {
        // record_id is the composite "name|type|data" built above.
        let Some((name, record_type, data)) = record_id
            .split_once('|')
            .and_then(|(name, rest)| rest.split_once('|').map(|(t, d)| (name, t, d)))
        else {
            return Err(DnsServiceError::ProviderError(
                "GoDaddy record id must be the composite name|type|data".into(),
            ));
        };
        let sso = Self::godaddy_sso_key(credentials_json)?;
        // DELETE removes the whole name/type set; to delete one entry among
        // several, re-PUT the remaining ones first.
        let existing = self.list_godaddy_records(credentials_json, zone_id).await?;
        let remaining: Vec<serde_json::Value> = existing
            .iter()
            .filter(|r| r.record_type == record_type && r.name == name)
            .filter(|r| !(r.content == data))
            .map(|r| serde_json::json!({"data": r.content, "ttl": r.ttl.unwrap_or(600)}))
            .collect();
        let url = format!("{GODADDY_API}/domains/{zone_id}/records/{record_type}/{name}");
        let res = if remaining.is_empty() {
            self.client
                .delete(&url)
                .header("Authorization", sso)
                .send()
                .await?
        } else {
            self.client
                .put(&url)
                .header("Authorization", sso.clone())
                .json(&remaining)
                .send()
                .await?
        };
        let status = res.status();
        if status.is_success() {
            Ok(true)
        } else {
            Err(DnsServiceError::ProviderError(format!(
                "GoDaddy delete record failed: HTTP {status}"
            )))
        }
    }
}
