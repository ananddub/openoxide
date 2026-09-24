use super::super::{
    DnsRecordDto, DnsService, DnsServiceError, DnsTestResultDto, DnsZoneDto, UpsertDnsRecordDto,
    extract_credential_field,
};

/// Porkbun DNS: POST-only API with both keys carried in every request body.
/// Zones are the account's domains; MX/SRV priority travels inline in the
/// record content ("10 mail.example.com").
/// Credentials JSON: `{"apiKey": "...", "secretApiKey": "..."}`.
const PORKBUN_API: &str = "https://api.porkbun.com/api/json/v3";

fn porkbun_keys(credentials_json: &str) -> Result<(String, String), DnsServiceError> {
    let api_key = extract_credential_field(credentials_json, "apiKey");
    let secret_key = extract_credential_field(credentials_json, "secretApiKey");
    if api_key.is_empty() || secret_key.is_empty() {
        return Err(DnsServiceError::ProviderError(
            "Porkbun apiKey and secretApiKey are required".into(),
        ));
    }
    Ok((api_key, secret_key))
}

async fn pb_fetch(
    client: &reqwest::Client,
    api_key: &str,
    secret_key: &str,
    path: &str,
    extra: serde_json::Value,
) -> Result<serde_json::Value, DnsServiceError> {
    let mut payload = serde_json::json!({ "apikey": api_key, "secretapikey": secret_key });
    if let (Some(obj), Some(extra_obj)) = (payload.as_object_mut(), extra.as_object()) {
        for (key, value) in extra_obj {
            obj.insert(key.clone(), value.clone());
        }
    }
    let response = client
        .post(format!("{PORKBUN_API}{path}"))
        .json(&payload)
        .send()
        .await?;
    let status = u16::from(response.status());
    let body: serde_json::Value = response.json().await.map_err(|e| {
        DnsServiceError::ProviderError(format!("Porkbun {path} decode failed: {e}"))
    })?;
    if !(200..300).contains(&status) || body["status"].as_str() != Some("SUCCESS") {
        let detail = body["message"].as_str().unwrap_or_default();
        return Err(DnsServiceError::ProviderError(if detail.is_empty() {
            format!("Porkbun: request to {path} failed (status {status})")
        } else {
            format!("Porkbun: request to {path} failed: {detail}")
        }));
    }
    Ok(body)
}

// Porkbun's record "name" only accepts the subdomain portion, without the
// zone (domain) itself.
fn to_subdomain(name: &str, domain: &str) -> String {
    let fqdn = name.trim_end_matches('.');
    if fqdn == domain {
        return String::new();
    }
    match fqdn.strip_suffix(&format!(".{domain}")) {
        Some(sub) => sub.to_string(),
        None => fqdn.to_string(),
    }
}

// MX/SRV content may carry the priority inline ("10 mail.example.com").
fn split_priority(record_type: &str, content: &str) -> (String, Option<String>) {
    let value = content.trim();
    if record_type == "MX" || record_type == "SRV" {
        if let Some((prio, rest)) = value.split_once(' ') {
            let rest = rest.trim();
            if !prio.is_empty() && prio.bytes().all(|b| b.is_ascii_digit()) && !rest.is_empty() {
                return (rest.to_string(), Some(prio.to_string()));
            }
        }
    }
    (value.to_string(), None)
}

fn inline_priority(record_type: &str, content: &str, prio: Option<&str>) -> String {
    match prio {
        Some(prio) if (record_type == "MX" || record_type == "SRV") && !prio.is_empty() => {
            format!("{prio} {content}")
        }
        _ => content.to_string(),
    }
}

fn record_body(record: &UpsertDnsRecordDto, zone_id: &str) -> serde_json::Value {
    let subdomain = to_subdomain(&record.name, zone_id);
    let (content, prio) = split_priority(&record.record_type, &record.content);
    let mut body = serde_json::json!({
        "name": subdomain,
        "type": record.record_type,
        "content": content,
        "ttl": record.ttl.unwrap_or(600),
    });
    // Only MX/SRV carry a priority; leave it off otherwise.
    if let Some(prio) = prio {
        body["prio"] = serde_json::Value::String(prio);
    }
    body
}

fn ttl_of(value: &serde_json::Value) -> Option<u32> {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|s| s.parse().ok()))
        .map(|v| v as u32)
}

impl DnsService {
    pub(in crate::services::networking::dns) async fn test_porkbun_credentials(
        &self,
        credentials_json: &str,
    ) -> Result<DnsTestResultDto, DnsServiceError> {
        let (api_key, secret_key) = porkbun_keys(credentials_json)?;
        pb_fetch(
            &self.client,
            &api_key,
            &secret_key,
            "/ping",
            serde_json::json!({}),
        )
        .await?;
        Ok(DnsTestResultDto {
            success: true,
            message: "Porkbun credentials verified successfully".into(),
        })
    }

    pub(in crate::services::networking::dns) async fn list_porkbun_zones(
        &self,
        credentials_json: &str,
    ) -> Result<Vec<DnsZoneDto>, DnsServiceError> {
        let (api_key, secret_key) = porkbun_keys(credentials_json)?;
        let body = pb_fetch(
            &self.client,
            &api_key,
            &secret_key,
            "/domain/listAll",
            serde_json::json!({}),
        )
        .await?;
        Ok(body["domains"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|d| DnsZoneDto {
                id: d["domain"].as_str().unwrap_or_default().to_string(),
                name: d["domain"].as_str().unwrap_or_default().to_string(),
                status: None,
            })
            .collect())
    }

    pub(in crate::services::networking::dns) async fn list_porkbun_records(
        &self,
        credentials_json: &str,
        zone_id: &str,
    ) -> Result<Vec<DnsRecordDto>, DnsServiceError> {
        let (api_key, secret_key) = porkbun_keys(credentials_json)?;
        let body = pb_fetch(
            &self.client,
            &api_key,
            &secret_key,
            &format!("/dns/retrieve/{zone_id}"),
            serde_json::json!({}),
        )
        .await?;
        Ok(body["records"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|r| DnsRecordDto {
                id: r["id"].as_str().unwrap_or_default().to_string(),
                zone_id: zone_id.to_string(),
                record_type: r["type"].as_str().unwrap_or_default().to_string(),
                name: r["name"].as_str().unwrap_or_default().to_string(),
                content: inline_priority(
                    r["type"].as_str().unwrap_or_default(),
                    r["content"].as_str().unwrap_or_default(),
                    Some(r["prio"].as_str().unwrap_or_default()),
                ),
                ttl: ttl_of(&r["ttl"]),
                proxied: None,
            })
            .collect())
    }

    pub(in crate::services::networking::dns) async fn upsert_porkbun_record(
        &self,
        credentials_json: &str,
        body: &UpsertDnsRecordDto,
    ) -> Result<DnsRecordDto, DnsServiceError> {
        let (api_key, secret_key) = porkbun_keys(credentials_json)?;
        let subdomain = to_subdomain(&body.name, &body.zone_id);
        let payload = record_body(body, &body.zone_id);
        let expected_content = payload["content"]
            .as_str()
            .map(|c| inline_priority(&body.record_type, c, payload["prio"].as_str()))
            .unwrap_or_default();

        let existing = pb_fetch(
            &self.client,
            &api_key,
            &secret_key,
            &format!(
                "/dns/retrieveByNameType/{}/{}/{}",
                body.zone_id, body.record_type, subdomain
            ),
            serde_json::json!({}),
        )
        .await?;

        let match_id = existing["records"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .find(|r| {
                inline_priority(
                    r["type"].as_str().unwrap_or_default(),
                    r["content"].as_str().unwrap_or_default(),
                    Some(r["prio"].as_str().unwrap_or_default()),
                ) == expected_content
            })
            .and_then(|r| r["id"].as_str())
            .map(str::to_string);

        let record_id = match match_id {
            Some(id) => {
                pb_fetch(
                    &self.client,
                    &api_key,
                    &secret_key,
                    &format!("/dns/edit/{}/{}", body.zone_id, id),
                    payload,
                )
                .await?;
                id
            }
            None => {
                let created = pb_fetch(
                    &self.client,
                    &api_key,
                    &secret_key,
                    &format!("/dns/create/{}", body.zone_id),
                    payload,
                )
                .await?;
                created["id"].as_str().unwrap_or_default().to_string()
            }
        };

        Ok(DnsRecordDto {
            id: record_id,
            zone_id: body.zone_id.clone(),
            record_type: body.record_type.clone(),
            name: body.name.clone(),
            content: body.content.clone(),
            ttl: body.ttl,
            proxied: None,
        })
    }

    pub(in crate::services::networking::dns) async fn delete_porkbun_record(
        &self,
        credentials_json: &str,
        zone_id: &str,
        record_id: &str,
    ) -> Result<bool, DnsServiceError> {
        let (api_key, secret_key) = porkbun_keys(credentials_json)?;
        pb_fetch(
            &self.client,
            &api_key,
            &secret_key,
            &format!("/dns/delete/{zone_id}/{record_id}"),
            serde_json::json!({}),
        )
        .await?;
        Ok(true)
    }
}
