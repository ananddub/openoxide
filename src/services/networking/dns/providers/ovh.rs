use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use futures::stream::{self, StreamExt};
use sha1::{Digest, Sha1};

use super::super::{
    DnsRecordDto, DnsService, DnsServiceError, DnsTestResultDto, DnsZoneDto, UpsertDnsRecordDto,
    extract_credential_field,
};

/// OVH DNS: signed REST calls (SHA1 over appSecret+consumerKey+method+url+
/// body+timestamp) against a per-brand endpoint; requests are signed with the
/// API's own clock and every mutation must be followed by a zone refresh.
/// Credentials JSON:
/// `{"endpoint": "ovh-eu", "applicationKey": "...", "applicationSecret": "...", "consumerKey": "..."}`.
const CLOCK_SKEW_TTL: Duration = Duration::from_secs(60 * 60);

// Fetching every record of a zone takes one call per record, so cap how many
// of them are in flight at once.
const RECORD_CONCURRENCY: usize = 8;

// baseUrl -> (delta seconds, measured at). Re-measured periodically in case
// the host clock is corrected under us.
static CLOCK_SKEWS: OnceLock<std::sync::Mutex<HashMap<String, (i64, Instant)>>> = OnceLock::new();

fn ovh_endpoint(endpoint: &str) -> Result<&'static str, DnsServiceError> {
    match endpoint {
        "ovh-eu" => Ok("https://eu.api.ovh.com/1.0"),
        "ovh-ca" => Ok("https://ca.api.ovh.com/1.0"),
        "ovh-us" => Ok("https://api.us.ovhcloud.com/1.0"),
        "kimsufi-eu" => Ok("https://eu.api.kimsufi.com/1.0"),
        "kimsufi-ca" => Ok("https://ca.api.kimsufi.com/1.0"),
        "soyoustart-eu" => Ok("https://eu.api.soyoustart.com/1.0"),
        "soyoustart-ca" => Ok("https://ca.api.soyoustart.com/1.0"),
        other => Err(DnsServiceError::ProviderError(format!(
            "OVH: unknown endpoint '{other}'"
        ))),
    }
}

struct OvhCreds {
    application_key: String,
    application_secret: String,
    consumer_key: String,
    base_url: String,
}

fn ovh_credentials(credentials_json: &str) -> Result<OvhCreds, DnsServiceError> {
    let application_key = extract_credential_field(credentials_json, "applicationKey");
    let application_secret = extract_credential_field(credentials_json, "applicationSecret");
    let consumer_key = extract_credential_field(credentials_json, "consumerKey");
    let endpoint = extract_credential_field(credentials_json, "endpoint");
    let endpoint = if endpoint.is_empty() {
        "ovh-eu"
    } else {
        &endpoint
    };
    if application_key.is_empty() || application_secret.is_empty() || consumer_key.is_empty() {
        return Err(DnsServiceError::ProviderError(
            "OVH applicationKey, applicationSecret and consumerKey are required".into(),
        ));
    }
    Ok(OvhCreds {
        base_url: ovh_endpoint(endpoint)?.to_string(),
        application_key,
        application_secret,
        consumer_key,
    })
}

// A local clock more than a few seconds off gets every signed call rejected, so
// sign with the API's own time; the drift is measured once per hour per host.
async fn signed_timestamp(
    client: &reqwest::Client,
    base_url: &str,
) -> Result<i64, DnsServiceError> {
    let skews = CLOCK_SKEWS.get_or_init(Default::default);
    let now = chrono::Utc::now().timestamp();
    if let Some((delta, measured)) = skews.lock().unwrap().get(base_url) {
        if measured.elapsed() < CLOCK_SKEW_TTL {
            return Ok(now + delta);
        }
    }
    let response = client.get(format!("{base_url}/auth/time")).send().await?;
    let text = response.text().await?;
    let server_time: i64 = text.trim().parse().map_err(|_| {
        DnsServiceError::ProviderError("OVH: could not read the API server time".to_string())
    })?;
    skews
        .lock()
        .unwrap()
        .insert(base_url.to_string(), (server_time - now, Instant::now()));
    Ok(server_time)
}

fn sign(creds: &OvhCreds, method: &str, url: &str, body: &str, timestamp: i64) -> String {
    let joined = [
        creds.application_secret.as_str(),
        creds.consumer_key.as_str(),
        method,
        url,
        body,
        &timestamp.to_string(),
    ]
    .join("+");
    let digest = Sha1::digest(joined.as_bytes());
    format!(
        "$1${}",
        digest
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>()
    )
}

async fn ovh_request(
    client: &reqwest::Client,
    creds: &OvhCreds,
    method: reqwest::Method,
    path: &str,
    body: Option<&serde_json::Value>,
) -> Result<serde_json::Value, DnsServiceError> {
    let url = format!("{}{path}", creds.base_url);
    let body_str = body.map(|v| v.to_string()).unwrap_or_default();
    let timestamp = signed_timestamp(client, &creds.base_url).await?;
    let method_str = method.as_str().to_string();

    let mut request = client
        .request(method.clone(), &url)
        .header("Content-Type", "application/json")
        .header("X-Ovh-Application", &creds.application_key)
        .header("X-Ovh-Consumer", &creds.consumer_key)
        .header("X-Ovh-Timestamp", timestamp.to_string())
        .header(
            "X-Ovh-Signature",
            sign(creds, &method_str, &url, &body_str, timestamp),
        );
    if !body_str.is_empty() {
        request = request.body(body_str);
    }

    let response = request.send().await?;
    let status = u16::from(response.status());
    let text = response.text().await?;
    let payload: serde_json::Value = serde_json::from_str(&text).unwrap_or(serde_json::Value::Null);
    if !(200..300).contains(&status) {
        let detail = payload["message"].as_str().unwrap_or_default();
        return Err(DnsServiceError::ProviderError(if detail.is_empty() {
            format!("OVH: request to {method_str} {path} failed (status {status})")
        } else {
            format!("OVH: request to {method_str} {path} failed: {detail}")
        }));
    }
    Ok(payload)
}

// OVH addresses records by their subdomain, relative to the zone and empty for
// the apex, while the panel works with fully-qualified names.
fn to_sub_domain(name: &str, zone: &str) -> String {
    let fqdn = name.trim_end_matches('.');
    if fqdn == zone {
        return String::new();
    }
    match fqdn.strip_suffix(&format!(".{zone}")) {
        Some(sub) => sub.to_string(),
        None => fqdn.to_string(),
    }
}

fn to_fqdn(sub_domain: Option<&str>, zone: &str) -> String {
    match sub_domain {
        Some(sub) if !sub.is_empty() => format!("{sub}.{zone}"),
        _ => zone.to_string(),
    }
}

fn record_body(record: &UpsertDnsRecordDto, zone: &str) -> serde_json::Value {
    let mut body = serde_json::json!({
        "subDomain": to_sub_domain(&record.name, zone),
        "target": record.content,
    });
    // Leaving the ttl out lets OVH apply the zone's default.
    if let Some(ttl) = record.ttl {
        body["ttl"] = serde_json::json!(ttl);
    }
    body
}

// OVH only applies zone changes once the zone is explicitly refreshed. This
// runs after the record write succeeded, so a failure here means the change
// exists at the provider but is not served yet — rolling the write back would
// destroy correct state, so say what actually happened.
async fn refresh_zone(
    client: &reqwest::Client,
    creds: &OvhCreds,
    zone: &str,
) -> Result<(), DnsServiceError> {
    ovh_request(
        client,
        creds,
        reqwest::Method::POST,
        &format!("/domain/zone/{zone}/refresh"),
        None,
    )
    .await
    .map_err(|error| {
        DnsServiceError::ProviderError(format!(
            "OVH: the record change was applied, but refreshing zone \"{zone}\" failed, so it \
             is not served yet. The next successful change to this zone will publish it, or you \
             can refresh the zone from the OVH manager. Cause: {error}"
        ))
    })
    .map(|_| ())
}

// OVH grants access per exact path: a `/domain/zone/*` rule covers the subtree
// but not the bare `/domain/zone` listing. That is an easy one to leave out of
// a token, so say so plainly rather than surfacing a bare "not been granted".
async fn list_zone_names(
    client: &reqwest::Client,
    creds: &OvhCreds,
) -> Result<Vec<String>, DnsServiceError> {
    match ovh_request(client, creds, reqwest::Method::GET, "/domain/zone", None).await {
        Ok(zones) => Ok(zones
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .filter_map(|z| z.as_str().map(str::to_string))
            .collect()),
        Err(error) => {
            if error.to_string().contains("has not been granted") {
                return Err(DnsServiceError::ProviderError(
                    "OVH: the credentials are missing the `GET /domain/zone` right, which lists \
                     your zones. A `GET /domain/zone/*` rule does not cover it — add the rule \
                     without the wildcard as well."
                        .into(),
                ));
            }
            Err(error)
        }
    }
}

impl DnsService {
    pub(in crate::services::networking::dns) async fn test_ovh_credentials(
        &self,
        credentials_json: &str,
    ) -> Result<DnsTestResultDto, DnsServiceError> {
        let creds = ovh_credentials(credentials_json)?;
        list_zone_names(&self.client, &creds).await?;
        Ok(DnsTestResultDto {
            success: true,
            message: "OVH credentials verified successfully".into(),
        })
    }

    pub(in crate::services::networking::dns) async fn list_ovh_zones(
        &self,
        credentials_json: &str,
    ) -> Result<Vec<DnsZoneDto>, DnsServiceError> {
        let creds = ovh_credentials(credentials_json)?;
        Ok(list_zone_names(&self.client, &creds)
            .await?
            .into_iter()
            .map(|zone| DnsZoneDto {
                id: zone.clone(),
                name: zone,
                status: None,
            })
            .collect())
    }

    pub(in crate::services::networking::dns) async fn list_ovh_records(
        &self,
        credentials_json: &str,
        zone_id: &str,
    ) -> Result<Vec<DnsRecordDto>, DnsServiceError> {
        let creds = ovh_credentials(credentials_json)?;
        // The listing endpoint only returns ids, so each record is fetched on
        // its own — bounded rather than one-per-record in series.
        let ids = ovh_request(
            &self.client,
            &creds,
            reqwest::Method::GET,
            &format!("/domain/zone/{zone_id}/record"),
            None,
        )
        .await?
        .as_array()
        .cloned()
        .unwrap_or_default();
        let records = stream::iter(ids)
            .map(|id| self.ovh_fetch_record(&creds, zone_id, id))
            .buffered(RECORD_CONCURRENCY)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;
        Ok(records
            .into_iter()
            .map(|record| DnsRecordDto {
                id: record["id"].to_string(),
                zone_id: zone_id.to_string(),
                record_type: record["fieldType"].as_str().unwrap_or_default().to_string(),
                name: to_fqdn(record["subDomain"].as_str(), zone_id),
                content: record["target"].as_str().unwrap_or_default().to_string(),
                ttl: record["ttl"].as_u64().map(|v| v as u32),
                proxied: None,
            })
            .collect())
    }

    async fn ovh_fetch_record(
        &self,
        creds: &OvhCreds,
        zone_id: &str,
        id: serde_json::Value,
    ) -> Result<serde_json::Value, DnsServiceError> {
        ovh_request(
            &self.client,
            creds,
            reqwest::Method::GET,
            &format!("/domain/zone/{zone_id}/record/{id}"),
            None,
        )
        .await
    }

    pub(in crate::services::networking::dns) async fn upsert_ovh_record(
        &self,
        credentials_json: &str,
        body: &UpsertDnsRecordDto,
    ) -> Result<DnsRecordDto, DnsServiceError> {
        let creds = ovh_credentials(credentials_json)?;
        let sub_domain = to_sub_domain(&body.name, &body.zone_id);
        let existing_ids = ovh_request(
            &self.client,
            &creds,
            reqwest::Method::GET,
            &format!(
                "/domain/zone/{}/record?fieldType={}&subDomain={}",
                body.zone_id, body.record_type, sub_domain
            ),
            None,
        )
        .await?
        .as_array()
        .cloned()
        .unwrap_or_default();

        // OVH keys records by name+type but a name can hold several values;
        // reuse the one carrying this exact target instead of duplicating it.
        let mut existing_id: Option<serde_json::Value> = None;
        for id in &existing_ids {
            let candidate = self
                .ovh_fetch_record(&creds, &body.zone_id, id.clone())
                .await?;
            if candidate["target"].as_str() == Some(body.content.as_str()) {
                existing_id = Some(id.clone());
                break;
            }
        }

        let record_id = match existing_id {
            Some(id) => {
                ovh_request(
                    &self.client,
                    &creds,
                    reqwest::Method::PUT,
                    &format!("/domain/zone/{}/record/{id}", body.zone_id),
                    Some(&record_body(body, &body.zone_id)),
                )
                .await?;
                id.to_string()
            }
            None => {
                let mut payload = record_body(body, &body.zone_id);
                payload["fieldType"] = serde_json::json!(body.record_type);
                let created = ovh_request(
                    &self.client,
                    &creds,
                    reqwest::Method::POST,
                    &format!("/domain/zone/{}/record", body.zone_id),
                    Some(&payload),
                )
                .await?;
                created["id"].to_string()
            }
        };

        refresh_zone(&self.client, &creds, &body.zone_id).await?;
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

    pub(in crate::services::networking::dns) async fn delete_ovh_record(
        &self,
        credentials_json: &str,
        zone_id: &str,
        record_id: &str,
    ) -> Result<bool, DnsServiceError> {
        let creds = ovh_credentials(credentials_json)?;
        ovh_request(
            &self.client,
            &creds,
            reqwest::Method::DELETE,
            &format!("/domain/zone/{zone_id}/record/{record_id}"),
            None,
        )
        .await?;
        refresh_zone(&self.client, &creds, zone_id).await?;
        Ok(true)
    }
}
