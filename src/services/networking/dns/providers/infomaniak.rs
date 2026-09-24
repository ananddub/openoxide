use super::super::{
    DnsRecordDto, DnsService, DnsServiceError, DnsTestResultDto, DnsZoneDto, UpsertDnsRecordDto,
    extract_credential_field,
};

/// Infomaniak DNS: Bearer-token auth. Zones are the account's domain products
/// keyed by name; the v2 record endpoints take the zone (domain) name, not the
/// numeric product id. TXT targets are stored quoted.
/// Credentials JSON: `{"apiToken": "..."}`.
const INFOMANIAK_API: &str = "https://api.infomaniak.com";

// Infomaniak requires a TTL on every record, within a 60..86400 range.
const DEFAULT_TTL: u32 = 300;
const PRODUCTS_PER_PAGE: u32 = 100;

fn infomaniak_token(credentials_json: &str) -> Result<String, DnsServiceError> {
    let token = extract_credential_field(credentials_json, "apiToken");
    if token.is_empty() {
        return Err(DnsServiceError::ProviderError(
            "Infomaniak apiToken is required".into(),
        ));
    }
    Ok(token)
}

// The envelope is {result, data?, error?, page?, pages?}; a non-2xx status or a
// result other than "success" is an error carrying an optional description.
async fn ik_request(
    token: &str,
    request: reqwest::RequestBuilder,
    context: &str,
) -> Result<serde_json::Value, DnsServiceError> {
    let response = request
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await?;
    let status = u16::from(response.status());
    let body: serde_json::Value = response.json().await.map_err(|e| {
        DnsServiceError::ProviderError(format!("Infomaniak {context} decode failed: {e}"))
    })?;
    if !(200..300).contains(&status) || body["result"].as_str() != Some("success") {
        let detail = body["error"]["description"]
            .as_str()
            .or_else(|| body["error"]["code"].as_str())
            .unwrap_or_default();
        return Err(DnsServiceError::ProviderError(if detail.is_empty() {
            format!("Infomaniak: {context} failed (status {status})")
        } else {
            format!("Infomaniak: {context} failed: {detail}")
        }));
    }
    Ok(body)
}

// "source" holds the subdomain only, relative to the zone; the apex is a bare
// root dot ("" and "@" are accepted on read so hand-written records round-trip).
const APEX_SOURCES: [&str; 3] = ["", ".", "@"];

fn to_source(name: &str, zone: &str) -> String {
    let fqdn = name.trim_end_matches('.');
    if fqdn == zone {
        return ".".to_string();
    }
    match fqdn.strip_suffix(&format!(".{zone}")) {
        Some(sub) => sub.to_string(),
        None => fqdn.to_string(),
    }
}

fn to_fqdn(source: &str, zone: &str) -> String {
    if APEX_SOURCES.contains(&source) {
        zone.to_string()
    } else {
        format!("{source}.{zone}")
    }
}

fn normalize_source(source: &str) -> &str {
    if APEX_SOURCES.contains(&source) {
        "."
    } else {
        source
    }
}

// TXT targets are stored quoted; surface them unquoted so editing does not
// stack a new pair of quotes on every save.
fn unquote_target(target: &str) -> String {
    if target.len() >= 2 && target.starts_with('"') && target.ends_with('"') {
        return serde_json::from_str::<String>(target).unwrap_or_else(|_| target.to_string());
    }
    target.to_string()
}

fn quote_target(record_type: &str, content: &str) -> String {
    let value = content.trim();
    if record_type != "TXT" {
        return value.to_string();
    }
    if value.starts_with('"') && value.ends_with('"') {
        value.to_string()
    } else {
        serde_json::to_string(value).unwrap_or_else(|_| format!("\"{value}\""))
    }
}

fn record_payload(record: &UpsertDnsRecordDto, zone: &str) -> serde_json::Value {
    serde_json::json!({
        "type": record.record_type,
        "source": to_source(&record.name, zone),
        "target": quote_target(&record.record_type, &record.content),
        "ttl": record.ttl.unwrap_or(DEFAULT_TTL),
    })
}

impl DnsService {
    pub(in crate::services::networking::dns) async fn test_infomaniak_token(
        &self,
        credentials_json: &str,
    ) -> Result<DnsTestResultDto, DnsServiceError> {
        let token = infomaniak_token(credentials_json)?;
        ik_request(
            &token,
            self.client
                .get(format!("{INFOMANIAK_API}/1/products"))
                .query(&[("service_name", "domain"), ("per_page", "1")]),
            "auth check",
        )
        .await?;
        Ok(DnsTestResultDto {
            success: true,
            message: "Infomaniak credentials verified successfully".into(),
        })
    }

    pub(in crate::services::networking::dns) async fn list_infomaniak_zones(
        &self,
        credentials_json: &str,
    ) -> Result<Vec<DnsZoneDto>, DnsServiceError> {
        let token = infomaniak_token(credentials_json)?;
        let mut zones = Vec::new();
        let mut page = 1u32;
        loop {
            let body = ik_request(
                &token,
                self.client
                    .get(format!("{INFOMANIAK_API}/1/products"))
                    .query(&[
                        ("service_name", "domain"),
                        ("page", &page.to_string()),
                        ("per_page", &PRODUCTS_PER_PAGE.to_string()),
                    ]),
                "list products",
            )
            .await?;
            for domain in body["data"].as_array().cloned().unwrap_or_default() {
                let name = domain["customer_name"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string();
                zones.push(DnsZoneDto {
                    id: name.clone(),
                    name,
                    status: None,
                });
            }
            // The products endpoint paginates, so keep going until the last page.
            let pages = body["pages"].as_u64().unwrap_or(1) as u32;
            if page >= pages {
                break;
            }
            page += 1;
        }
        Ok(zones)
    }

    pub(in crate::services::networking::dns) async fn list_infomaniak_records(
        &self,
        credentials_json: &str,
        zone_id: &str,
    ) -> Result<Vec<DnsRecordDto>, DnsServiceError> {
        let token = infomaniak_token(credentials_json)?;
        let body = ik_request(
            &token,
            self.client
                .get(format!("{INFOMANIAK_API}/2/zones/{zone_id}/records"))
                .query(&[("with", "records_description")]),
            "list records",
        )
        .await?;
        Ok(body["data"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|r| DnsRecordDto {
                id: match &r["id"] {
                    serde_json::Value::String(s) => s.clone(),
                    other => other.to_string(),
                },
                zone_id: zone_id.to_string(),
                record_type: r["type"].as_str().unwrap_or_default().to_string(),
                name: to_fqdn(r["source"].as_str().unwrap_or("."), zone_id),
                content: unquote_target(r["target"].as_str().unwrap_or_default()),
                ttl: r["ttl"].as_u64().map(|v| v as u32),
                proxied: None,
            })
            .collect())
    }

    pub(in crate::services::networking::dns) async fn upsert_infomaniak_record(
        &self,
        credentials_json: &str,
        body: &UpsertDnsRecordDto,
    ) -> Result<DnsRecordDto, DnsServiceError> {
        let token = infomaniak_token(credentials_json)?;
        let source = to_source(&body.name, &body.zone_id);
        let expected_content = unquote_target(&quote_target(&body.record_type, &body.content));

        // The API filters server-side, but re-check the match here: nothing
        // guarantees filter[source] compares the apex exactly the way to_source
        // writes it, and an over-broad filter would turn an update into a dup.
        let candidates = ik_request(
            &token,
            self.client
                .get(format!("{INFOMANIAK_API}/2/zones/{}/records", body.zone_id))
                .query(&[
                    ("filter[source]", source.as_str()),
                    ("filter[types][]", body.record_type.as_str()),
                ]),
            "find record",
        )
        .await?;

        let match_id = candidates["data"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .find(|c| {
                c["type"].as_str() == Some(body.record_type.as_str())
                    && normalize_source(c["source"].as_str().unwrap_or(".")) == source
                    && unquote_target(c["target"].as_str().unwrap_or_default()) == expected_content
            })
            .map(|c| match &c["id"] {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            });

        let payload = record_payload(body, &body.zone_id);
        let record_id = match match_id {
            Some(id) => {
                ik_request(
                    &token,
                    self.client.put(format!(
                        "{INFOMANIAK_API}/2/zones/{}/records/{id}",
                        body.zone_id
                    )),
                    "update record",
                )
                .await?;
                id
            }
            None => {
                let created = ik_request(
                    &token,
                    self.client
                        .post(format!("{INFOMANIAK_API}/2/zones/{}/records", body.zone_id))
                        .json(&payload),
                    "create record",
                )
                .await?;
                match &created["data"] {
                    serde_json::Value::Object(_) => match &created["data"]["id"] {
                        serde_json::Value::String(s) => s.clone(),
                        other => other.to_string(),
                    },
                    serde_json::Value::String(s) => s.clone(),
                    other => other.to_string(),
                }
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

    pub(in crate::services::networking::dns) async fn delete_infomaniak_record(
        &self,
        credentials_json: &str,
        zone_id: &str,
        record_id: &str,
    ) -> Result<bool, DnsServiceError> {
        let token = infomaniak_token(credentials_json)?;
        ik_request(
            &token,
            self.client.delete(format!(
                "{INFOMANIAK_API}/2/zones/{zone_id}/records/{record_id}"
            )),
            "delete record",
        )
        .await?;
        Ok(true)
    }
}
