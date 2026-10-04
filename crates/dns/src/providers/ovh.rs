use std::collections::HashMap;
use std::sync::{Mutex, PoisonError};

use async_trait::async_trait;
use reqwest::{Client, Method};
use serde_json::{Value, json};
use sha1::{Digest, Sha1};

use crate::config::{OvhConfig, OvhEndpoint, ProviderConfig};
use crate::error::DnsError;
use crate::http::Http;
use crate::provider::{DnsProvider, find_row, row_matches};
use crate::records::{COMMON_TYPES, ensure_type, quote_txt, relative_name, to_fqdn, unquote};
use crate::types::{Record, RecordInput, RecordTarget, Zone};

const DEFAULT_TTL: u32 = 3600;
/// The number of record detail calls run side by side. OVH rate-limits a consumer key, and
/// a large zone is read one record at a time, so the listing stays slow without being rude.
const RECORD_PARALLELISM: usize = 8;

/// OVHcloud DNS. Requests are signed per call with a SHA-1 digest of the application secret,
/// the consumer key, the method, the full URL, the body and a timestamp, and every write has
/// to be followed by a zone refresh before resolvers see it.
pub struct Ovh {
    client: Client,
    clock_skews: Mutex<HashMap<String, i64>>,
    endpoints: Option<(String, String)>,
}

impl Ovh {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
            clock_skews: Mutex::new(HashMap::new()),
            endpoints: None,
        }
    }

    /// Sends the signed calls to `api_base` and the clock probe to `auth_base` instead of the
    /// two hosts the row's endpoint picks, so a test can sign against one local server.
    pub fn with_endpoints(client: &Client, api_base: &str, auth_base: &str) -> Self {
        Self {
            client: client.clone(),
            clock_skews: Mutex::new(HashMap::new()),
            endpoints: Some((api_base.to_string(), auth_base.to_string())),
        }
    }

    fn bases(&self, endpoint: OvhEndpoint) -> (String, String) {
        match &self.endpoints {
            Some((api, auth)) => (api.clone(), auth.clone()),
            None => (
                endpoint.base_url().to_string(),
                endpoint.auth_base_url().to_string(),
            ),
        }
    }

    fn config<'a>(&self, config: &'a ProviderConfig) -> Result<&'a OvhConfig, DnsError> {
        config.ovh()
    }

    /// Signs and sends one call, once more if the first attempt was rejected for a timestamp
    /// the server did not accept — the usual sign of a cached clock skew that has gone stale.
    async fn execute(
        &self,
        credentials: &OvhConfig,
        method: Method,
        path: &str,
        body: Option<Value>,
        operation: &str,
    ) -> Result<Value, DnsError> {
        let first = self
            .signed(credentials, method.clone(), path, body.clone(), operation)
            .await;
        if let Err(error) = first {
            if is_clock_error(&error) {
                self.forget_clock(&self.bases(credentials.endpoint()?).1);
                return self
                    .signed(credentials, method, path, body, operation)
                    .await;
            }
            return Err(error);
        }
        first
    }

    /// The base URL comes from the row's endpoint, so it is rebuilt per call rather than
    /// fixed at construction.
    async fn signed(
        &self,
        credentials: &OvhConfig,
        method: Method,
        path: &str,
        body: Option<Value>,
        operation: &str,
    ) -> Result<Value, DnsError> {
        let (api, auth) = self.bases(credentials.endpoint()?);
        let http = Http::new(&self.client, api);
        let url = format!("{}{path}", http.base_url());
        // `Display` for a JSON value is the same text `to_string` would produce and, unlike
        // it, cannot fail — and the exact bytes are what the signature covers.
        let body = body.map(|value| value.to_string());
        let timestamp = self.timestamp(&auth).await;
        let signature = sign(
            &credentials.application_secret,
            &credentials.consumer_key,
            method.as_str(),
            &url,
            body.as_deref().unwrap_or_default(),
            timestamp,
        );
        // The signature covers the exact URL and body, so both are built once and sent as-is.
        let mut request = http.request(method, path);
        request = request
            .header("Accept", "application/json")
            .header("X-Ovh-Application", &credentials.application_key)
            .header("X-Ovh-Consumer", &credentials.consumer_key)
            .header("X-Ovh-Timestamp", timestamp.to_string())
            .header("X-Ovh-Signature", signature);
        if let Some(body) = body {
            request = request
                .header("Content-Type", "application/json")
                .body(body);
        }
        http.json(request.send().await?, "OVH", operation).await
    }

    fn forget_clock(&self, base: &str) {
        self.clock_skews
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .remove(base);
    }

    /// Local time corrected by the drift OVH reported, cached per endpoint and falling back
    /// to the local clock when the probe fails.
    async fn timestamp(&self, key: &str) -> i64 {
        let now = chrono::Utc::now().timestamp();
        let cached = self
            .clock_skews
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get(key)
            .copied();
        if let Some(delta) = cached {
            return now + delta;
        }
        let Ok(response) = self.client.get(format!("{key}/auth/time")).send().await else {
            return now;
        };
        let Ok(text) = response.text().await else {
            return now;
        };
        match text.trim().parse::<i64>() {
            Ok(server_time) => {
                let delta = server_time - now;
                self.clock_skews
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .insert(key.to_string(), delta);
                now + delta
            }
            Err(_) => now,
        }
    }

    /// Publishes the zone so the change reaches the authoritative nameservers. A failed
    /// refresh does not undo the write, so it is not reported as one.
    async fn refresh(&self, credentials: &OvhConfig, zone_id: &str) {
        let path = format!("/domain/zone/{zone_id}/refresh");
        let _ = self
            .execute(credentials, Method::POST, &path, None, "refresh zone")
            .await;
    }

    async fn resolve_record_id(
        &self,
        config: &ProviderConfig,
        target: &RecordTarget,
    ) -> Result<String, DnsError> {
        if target.has_id() {
            return Ok(target.id.clone());
        }
        let rows = self.list_records(config, &target.zone_id).await?;
        let index = find_row(&rows, target, row_matches)?;
        Ok(rows[index].id.clone())
    }
}

#[async_trait]
impl DnsProvider for Ovh {
    async fn test_connection(&self, config: &ProviderConfig) -> Result<(), DnsError> {
        let credentials = self.config(config)?;
        self.execute(
            credentials,
            Method::GET,
            "/domain/zone",
            None,
            "verify credentials",
        )
        .await?;
        Ok(())
    }

    async fn list_zones(&self, config: &ProviderConfig) -> Result<Vec<Zone>, DnsError> {
        let credentials = self.config(config)?;
        let body = self
            .execute(credentials, Method::GET, "/domain/zone", None, "list zones")
            .await?;
        Ok(string_rows(body)
            .into_iter()
            .map(|name| Zone {
                id: name.clone(),
                name,
                status: None,
            })
            .collect())
    }

    async fn list_records(
        &self,
        config: &ProviderConfig,
        zone_id: &str,
    ) -> Result<Vec<Record>, DnsError> {
        let credentials = self.config(config)?;
        let path = format!("/domain/zone/{zone_id}/record");
        let body = self
            .execute(credentials, Method::GET, &path, None, "list records")
            .await?;
        let ids = string_rows(body);
        let mut records = Vec::with_capacity(ids.len());
        for chunk in ids.chunks(RECORD_PARALLELISM) {
            let fetched: Vec<Result<Value, DnsError>> =
                futures::future::join_all(chunk.iter().map(|id| {
                    let path = format!("/domain/zone/{zone_id}/record/{id}");
                    async move {
                        self.execute(credentials, Method::GET, &path, None, "list records")
                            .await
                    }
                }))
                .await;
            for row in fetched.into_iter().flatten() {
                let record_type = text(&row["fieldType"]).to_ascii_uppercase();
                let sub_domain = text(&row["subDomain"]);
                records.push(Record {
                    id: text(&row["id"]),
                    zone_id: zone_id.to_string(),
                    name: to_fqdn(&sub_domain, zone_id),
                    record_type,
                    content: unquote(&text(&row["target"])),
                    ttl: Some(
                        row["ttl"]
                            .as_u64()
                            .map(|value| value as u32)
                            .filter(|value| *value > 0)
                            .unwrap_or(DEFAULT_TTL),
                    ),
                    proxied: None,
                });
            }
        }
        Ok(records)
    }

    async fn create_record(
        &self,
        config: &ProviderConfig,
        input: &RecordInput,
    ) -> Result<Record, DnsError> {
        let credentials = self.config(config)?;
        let record_type = ensure_type("OVH", &input.record_type, COMMON_TYPES)?;
        let path = format!("/domain/zone/{}/record", input.zone_id);
        let body = self
            .execute(
                credentials,
                Method::POST,
                &path,
                Some(json!({
                    "fieldType": record_type,
                    "subDomain": sub_domain(&input.name, &input.zone_id),
                    "target": quote_txt(&record_type, &input.content),
                    "ttl": input.ttl.unwrap_or(DEFAULT_TTL),
                })),
                "create record",
            )
            .await?;
        self.refresh(credentials, &input.zone_id).await;
        Ok(Record {
            id: text(&body["id"]),
            zone_id: input.zone_id.clone(),
            record_type,
            name: to_fqdn(&input.name, &input.zone_id),
            content: input.content.trim().to_string(),
            ttl: input.ttl.or(Some(DEFAULT_TTL)),
            proxied: None,
        })
    }

    async fn update_record(
        &self,
        config: &ProviderConfig,
        target: &RecordTarget,
        next: &RecordInput,
    ) -> Result<Record, DnsError> {
        let record_id = self.resolve_record_id(config, target).await?;
        let credentials = self.config(config)?;
        let record_type = ensure_type("OVH", &next.record_type, COMMON_TYPES)?;
        let path = format!("/domain/zone/{}/record/{record_id}", next.zone_id);
        // OVH will not change a record's type in place, so the row is replaced instead.
        let existing = self
            .execute(credentials, Method::GET, &path, None, "read record")
            .await
            .ok();
        if let Some(existing) = existing
            && text(&existing["fieldType"]).to_ascii_uppercase() != record_type
        {
            self.delete_record(config, target).await?;
            return self.create_record(config, next).await;
        }
        self.execute(
            credentials,
            Method::PUT,
            &path,
            Some(json!({
                "subDomain": sub_domain(&next.name, &next.zone_id),
                "target": quote_txt(&record_type, &next.content),
                "ttl": next.ttl.unwrap_or(DEFAULT_TTL),
            })),
            "update record",
        )
        .await?;
        self.refresh(credentials, &next.zone_id).await;
        Ok(Record {
            id: record_id,
            zone_id: next.zone_id.clone(),
            record_type,
            name: to_fqdn(&next.name, &next.zone_id),
            content: next.content.trim().to_string(),
            ttl: next.ttl.or(Some(DEFAULT_TTL)),
            proxied: None,
        })
    }

    async fn delete_record(
        &self,
        config: &ProviderConfig,
        target: &RecordTarget,
    ) -> Result<(), DnsError> {
        let record_id = self.resolve_record_id(config, target).await?;
        let credentials = self.config(config)?;
        let path = format!("/domain/zone/{}/record/{record_id}", target.zone_id);
        self.execute(credentials, Method::DELETE, &path, None, "delete record")
            .await?;
        self.refresh(credentials, &target.zone_id).await;
        Ok(())
    }
}

/// OVH stores an TXT record quoted and expects the value to arrive that way.
fn sub_domain(name: &str, zone_id: &str) -> String {
    let relative = relative_name(name, zone_id);
    if relative == "@" {
        String::new()
    } else {
        relative
    }
}

/// A rejected timestamp and a bad credential both answer 401, so a retry costs one request
/// and still surfaces the same message when the key really is wrong.
fn is_clock_error(error: &DnsError) -> bool {
    matches!(error, DnsError::Provider { message } if message.contains("status 401"))
}

fn sign(
    application_secret: &str,
    consumer_key: &str,
    method: &str,
    url: &str,
    body: &str,
    timestamp: i64,
) -> String {
    let mut hasher = Sha1::new();
    hasher.update(format!(
        "{application_secret}+{consumer_key}+{method}+{url}+{body}+{timestamp}"
    ));
    format!("$1${}", hex(&hasher.finalize()))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// OVH returns plain arrays of names or ids for both list calls.
fn string_rows(body: Value) -> Vec<String> {
    body.as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|row| text(&row))
        .filter(|value: &String| !value.is_empty())
        .collect()
}

fn text(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Number(number) => number.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DnsProviderKind;
    use crate::providers::harness::{client, header, hits, ok, ok_text, respond, scenario, sent};
    use wiremock::MockServer;

    const ZONE: &str = "example.com";
    const ZONES: &str = "/domain/zone";
    const RECORDS: &str = "/domain/zone/example.com/record";
    const REFRESH: &str = "/domain/zone/example.com/refresh";
    const KEYS: &str = r#"{"endpoint":"ovh-eu","applicationKey":"ak","applicationSecret":"as","consumerKey":"ck"}"#;
    /// A time far in the past, so the corrected timestamp it produces is distinguishable from
    /// the local clock in the assertion.
    const SERVER_TIME: &str = "1700000000";

    fn config() -> ProviderConfig {
        ProviderConfig::from_json(DnsProviderKind::Ovh, KEYS).unwrap()
    }

    fn provider(server: &MockServer) -> Ovh {
        Ovh::with_endpoints(&client(), &server.uri(), &server.uri())
    }

    #[tokio::test]
    async fn the_signature_digests_the_canonical_ovh_shape() {
        // `secret+consumer+method+url+body+timestamp`, SHA-1, `$1$`-prefixed. A reordered
        // concatenation is invisible locally and a rejected signature on every call in production.
        assert_eq!(
            sign(
                "as",
                "ck",
                "GET",
                "https://example.com/1.0/domain/zone",
                "",
                1_700_000_000
            ),
            "$1$e5f213715e1f3034c9b801d894fe1d38ecf5ca87"
        );
        assert_eq!(
            sign(
                "as",
                "ck",
                "POST",
                "https://example.com/1.0/domain/zone/example.com",
                r#"{"ttl":3600}"#,
                1_700_000_000,
            ),
            "$1$f7d5aa794f3293563a8f1e9383e3c9b8c12097ec"
        );
    }

    #[tokio::test]
    async fn the_signed_request_carries_the_probe_time_and_the_key_headers() {
        let server = MockServer::start().await;
        ok_text(&server, "GET", "/auth/time", SERVER_TIME).await;
        ok(&server, "GET", ZONES, scenario("ovh", "zones")).await;
        provider(&server).list_zones(&config()).await.unwrap();

        let timestamp = header(&server, "GET", ZONES, "X-Ovh-Timestamp")
            .await
            .unwrap();
        assert_eq!(timestamp, SERVER_TIME);
        assert_eq!(
            header(&server, "GET", ZONES, "X-Ovh-Application")
                .await
                .as_deref(),
            Some("ak")
        );
        assert_eq!(
            header(&server, "GET", ZONES, "X-Ovh-Consumer")
                .await
                .as_deref(),
            Some("ck")
        );
        assert_eq!(
            header(&server, "GET", ZONES, "X-Ovh-Signature")
                .await
                .as_deref(),
            Some(
                sign(
                    "as",
                    "ck",
                    "GET",
                    &format!("{}{ZONES}", server.uri()),
                    "",
                    1_700_000_000
                )
                .as_str()
            )
        );
    }

    #[tokio::test]
    async fn the_clock_offset_is_probed_once_and_reused() {
        let server = MockServer::start().await;
        ok_text(&server, "GET", "/auth/time", SERVER_TIME).await;
        ok(&server, "GET", ZONES, scenario("ovh", "zones")).await;
        let provider = provider(&server);
        provider.list_zones(&config()).await.unwrap();
        provider.list_zones(&config()).await.unwrap();
        assert_eq!(hits(&server, "GET", "/auth/time").await, 1);
        assert_eq!(hits(&server, "GET", ZONES).await, 2);
    }

    #[tokio::test]
    async fn a_rejected_timestamp_is_probed_and_signed_again() {
        let server = MockServer::start().await;
        ok_text(&server, "GET", "/auth/time", SERVER_TIME).await;
        respond(
            &server,
            "GET",
            ZONES,
            401,
            json!({ "message": "Invalid signature" }),
        )
        .await;
        let error = provider(&server)
            .list_zones(&config())
            .await
            .unwrap_err()
            .to_string();
        assert!(error.contains("status 401"), "{error}");
        assert!(error.contains("Invalid signature"), "{error}");
        // The stale offset is dropped, so the retry re-probes rather than reusing it.
        assert_eq!(hits(&server, "GET", ZONES).await, 2);
        assert_eq!(hits(&server, "GET", "/auth/time").await, 2);
    }

    #[tokio::test]
    async fn a_zone_is_read_one_record_at_a_time() {
        let server = MockServer::start().await;
        ok(&server, "GET", RECORDS, scenario("ovh", "record_ids")).await;
        ok(
            &server,
            "GET",
            format!("{RECORDS}/7001").as_str(),
            scenario("ovh", "record_a"),
        )
        .await;
        ok(
            &server,
            "GET",
            format!("{RECORDS}/7002").as_str(),
            scenario("ovh", "record_txt"),
        )
        .await;
        let rows = provider(&server)
            .list_records(&config(), ZONE)
            .await
            .unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].id, "7001");
        assert_eq!(rows[0].name, "www.example.com");
        // An empty subDomain is the apex, a zero ttl is the zone default, and a quoted TXT
        // target reads back as the string the operator typed.
        assert_eq!(rows[1].name, ZONE);
        assert_eq!(rows[1].content, "v=spf1 -all");
        assert_eq!(rows[1].ttl, Some(3600));
    }

    #[tokio::test]
    async fn a_write_quotes_txt_and_is_followed_by_a_refresh() {
        let server = MockServer::start().await;
        ok_text(&server, "GET", "/auth/time", SERVER_TIME).await;
        ok(&server, "POST", RECORDS, scenario("ovh", "created")).await;
        ok(&server, "POST", REFRESH, json!(null)).await;
        let created = provider(&server)
            .create_record(
                &config(),
                &RecordInput {
                    zone_id: ZONE.into(),
                    record_type: "TXT".into(),
                    name: "@".into(),
                    content: "v=spf1 -all".into(),
                    ttl: None,
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(
            sent(&server, "POST", RECORDS).await,
            vec![json!({
                "fieldType": "TXT",
                "subDomain": "",
                "target": "\"v=spf1 -all\"",
                "ttl": 3600,
            })]
        );
        assert_eq!(created.id, "7003");
        assert_eq!(hits(&server, "POST", REFRESH).await, 1);
    }

    #[tokio::test]
    async fn a_type_change_replaces_the_row_instead_of_updating_it() {
        let server = MockServer::start().await;
        ok_text(&server, "GET", "/auth/time", SERVER_TIME).await;
        let record = format!("{RECORDS}/7001");
        ok(&server, "GET", record.as_str(), scenario("ovh", "record_a")).await;
        ok(&server, "DELETE", record.as_str(), scenario("ovh", "ack")).await;
        ok(&server, "POST", RECORDS, scenario("ovh", "created")).await;
        ok(&server, "POST", REFRESH, json!(null)).await;
        provider(&server)
            .update_record(
                &config(),
                &RecordTarget {
                    zone_id: ZONE.into(),
                    id: "7001".into(),
                    record_type: "A".into(),
                    ..Default::default()
                },
                &RecordInput {
                    zone_id: ZONE.into(),
                    record_type: "TXT".into(),
                    name: "www".into(),
                    content: "note".into(),
                    ttl: Some(600),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(hits(&server, "PUT", record.as_str()).await, 0);
        assert_eq!(hits(&server, "DELETE", record.as_str()).await, 1);
        assert_eq!(hits(&server, "POST", RECORDS).await, 1);
        // Both the delete and the recreate publish a refresh.
        assert_eq!(hits(&server, "POST", REFRESH).await, 2);
    }

    #[tokio::test]
    async fn a_same_type_update_writes_in_place() {
        let server = MockServer::start().await;
        ok_text(&server, "GET", "/auth/time", SERVER_TIME).await;
        let record = format!("{RECORDS}/7001");
        ok(&server, "GET", record.as_str(), scenario("ovh", "record_a")).await;
        ok(&server, "PUT", record.as_str(), scenario("ovh", "ack")).await;
        ok(&server, "POST", REFRESH, json!(null)).await;
        provider(&server)
            .update_record(
                &config(),
                &RecordTarget {
                    zone_id: ZONE.into(),
                    id: "7001".into(),
                    ..Default::default()
                },
                &RecordInput {
                    zone_id: ZONE.into(),
                    record_type: "A".into(),
                    name: "www".into(),
                    content: "192.0.2.9".into(),
                    ttl: Some(600),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(
            sent(&server, "PUT", record.as_str()).await,
            vec![json!({ "subDomain": "www", "target": "192.0.2.9", "ttl": 600 })]
        );
        assert_eq!(hits(&server, "POST", RECORDS).await, 0);
    }
}
