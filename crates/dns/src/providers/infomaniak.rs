use async_trait::async_trait;
use serde_json::{Value, json};

use crate::config::{InfomaniakConfig, ProviderConfig};
use crate::error::DnsError;
use crate::http::Http;
use crate::provider::{DnsProvider, find_row, row_matches};
use crate::records::{COMMON_TYPES, ensure_type, quote_txt, relative_name, to_fqdn, unquote};
use crate::types::{Record, RecordInput, RecordTarget, Zone};

const DEFAULT_API: &str = "https://api.infomaniak.com";
const PAGE_SIZE: u32 = 100;
const DEFAULT_TTL: u32 = 300;
const MIN_TTL: u32 = 60;
const MAX_TTL: u32 = 86400;

/// Infomaniak DNS. Zones are addressed by domain name, records by numeric id, and every
/// reply — success or not — arrives as `{result, data, error}` with an HTTP 200.
pub struct Infomaniak {
    http: Http,
}

impl Infomaniak {
    pub fn new(client: &reqwest::Client) -> Self {
        Self::with_base_url(client, DEFAULT_API)
    }

    pub fn with_base_url(client: &reqwest::Client, base_url: &str) -> Self {
        Self {
            http: Http::new(client, base_url),
        }
    }

    fn config<'a>(&self, config: &'a ProviderConfig) -> Result<&'a InfomaniakConfig, DnsError> {
        config.infomaniak()
    }

    /// Sends one call and unwraps the envelope, so a `{result: "error"}` body answered with
    /// 200 still surfaces as the provider's own message rather than an empty result.
    async fn call(
        &self,
        credentials: &InfomaniakConfig,
        request: reqwest::RequestBuilder,
        operation: &str,
    ) -> Result<Value, DnsError> {
        let response = request.bearer_auth(&credentials.api_token).send().await?;
        let body = self.http.json(response, "Infomaniak", operation).await?;
        if body["result"].as_str() == Some("success") {
            return Ok(body);
        }
        let error = &body["error"];
        let detail = first_text(error, &["description", "code"])
            .unwrap_or_else(|| "the API reported an error".to_string());
        Err(DnsError::provider(format!(
            "Infomaniak: {operation} failed: {detail}"
        )))
    }

    /// Pages through a list endpoint until `pages` is exhausted.
    async fn paged(
        &self,
        credentials: &InfomaniakConfig,
        path: String,
        operation: &str,
        query: &[(&str, &str)],
    ) -> Result<Vec<Value>, DnsError> {
        let mut rows = Vec::new();
        let mut page = 1;
        loop {
            let mut request = self.http.get(&path).query(query);
            request = request.query(&[("per_page", PAGE_SIZE), ("page", page)]);
            let body = self.call(credentials, request, operation).await?;
            let last_page = body["pages"].as_u64().unwrap_or(1) as u32;
            rows.extend(body["data"].as_array().cloned().unwrap_or_default());
            if page >= last_page {
                break;
            }
            page += 1;
        }
        Ok(rows)
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
impl DnsProvider for Infomaniak {
    async fn test_connection(&self, config: &ProviderConfig) -> Result<(), DnsError> {
        let credentials = self.config(config)?;
        let request = self
            .http
            .get("/1/products")
            .query(&[("service_name", "domain"), ("per_page", "1")]);
        self.call(credentials, request, "verify credentials")
            .await?;
        Ok(())
    }

    async fn list_zones(&self, config: &ProviderConfig) -> Result<Vec<Zone>, DnsError> {
        let credentials = self.config(config)?;
        let rows = self
            .paged(
                credentials,
                "/1/products".to_string(),
                "list zones",
                &[("service_name", "domain")],
            )
            .await?;
        Ok(rows
            .into_iter()
            .filter_map(|row| first_text(&row, &["customer_name"]))
            .filter(|name: &String| !name.is_empty())
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
        let path = format!("/2/zones/{zone_id}/records");
        let rows = self
            .paged(
                credentials,
                path,
                "list records",
                &[("with", "records_description")],
            )
            .await?;
        Ok(rows
            .into_iter()
            .map(|row| Record {
                id: text(&row["id"]),
                zone_id: zone_id.to_string(),
                record_type: text(&row["type"]).to_ascii_uppercase(),
                name: to_fqdn(&text(&row["source"]), zone_id),
                content: unquote(&text(&row["target"])),
                ttl: number(&row["ttl"]),
                proxied: None,
            })
            .collect())
    }

    async fn create_record(
        &self,
        config: &ProviderConfig,
        input: &RecordInput,
    ) -> Result<Record, DnsError> {
        let credentials = self.config(config)?;
        let path = format!("/2/zones/{}/records", input.zone_id);
        let payload = payload(input)?;
        let body = self
            .call(
                credentials,
                self.http.post(&path).json(&payload),
                "create record",
            )
            .await?;
        Ok(Record {
            id: text(&body["data"]["id"]),
            zone_id: input.zone_id.clone(),
            record_type: payload["type"].as_str().unwrap_or_default().to_string(),
            name: to_fqdn(&input.name, &input.zone_id),
            content: input.content.trim().to_string(),
            ttl: number(&payload["ttl"]),
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
        let path = format!("/2/zones/{}/records/{record_id}", next.zone_id);
        let payload = payload(next)?;
        // The update endpoint takes only the mutable fields; the name and type of an existing
        // row cannot be changed in place, so a rename has to delete and recreate.
        let existing_type = payload["type"].as_str().unwrap_or_default();
        if target.has_type()
            && !target.record_type.trim().is_empty()
            && !target
                .record_type
                .trim()
                .eq_ignore_ascii_case(existing_type)
        {
            self.delete_record(config, target).await?;
            return self.create_record(config, next).await;
        }
        let body = json!({
            "source": payload["source"],
            "target": payload["target"],
            "ttl": payload["ttl"],
        });
        self.call(
            credentials,
            self.http.put(&path).json(&body),
            "update record",
        )
        .await?;
        Ok(Record {
            id: record_id,
            zone_id: next.zone_id.clone(),
            record_type: payload["type"].as_str().unwrap_or_default().to_string(),
            name: to_fqdn(&next.name, &next.zone_id),
            content: next.content.trim().to_string(),
            ttl: number(&payload["ttl"]),
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
        let path = format!("/2/zones/{}/records/{record_id}", target.zone_id);
        self.call(credentials, self.http.delete(&path), "delete record")
            .await?;
        Ok(())
    }
}

fn payload(input: &RecordInput) -> Result<Value, DnsError> {
    let record_type = ensure_type("Infomaniak", &input.record_type, COMMON_TYPES)?;
    let mut source = relative_name(&input.name, &input.zone_id);
    if source == "@" {
        source = ".".to_string();
    }
    Ok(json!({
        "type": record_type,
        "source": source,
        "target": quote_txt(&record_type, &input.content),
        "ttl": normalize_ttl(input.ttl),
    }))
}

fn normalize_ttl(ttl: Option<u32>) -> u32 {
    match ttl {
        Some(value) => value.clamp(MIN_TTL, MAX_TTL),
        None => DEFAULT_TTL,
    }
}

fn first_text(row: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .filter_map(|key| row[*key].as_str())
        .map(str::trim)
        .find(|text| !text.is_empty())
        .map(str::to_string)
}

/// Infomaniak mixes strings and numbers in `id` and `ttl`.
fn text(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Number(number) => number.to_string(),
        _ => String::new(),
    }
}

fn number(value: &Value) -> Option<u32> {
    match value {
        Value::Number(number) => number.as_u64().map(|v| v as u32),
        Value::String(text) => text.trim().parse().ok(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DnsProviderKind, ProviderConfig};
    use crate::providers::harness::{client, header, hits, ok, scenario, sent};
    use serde_json::json;
    use wiremock::MockServer;

    const ZONE: &str = "example.com";
    const RECORDS: &str = "/2/zones/example.com/records";

    fn config() -> ProviderConfig {
        ProviderConfig::from_json(DnsProviderKind::Infomaniak, r#"{"apiToken":"tk-1"}"#).unwrap()
    }

    fn provider(server: &MockServer) -> Infomaniak {
        Infomaniak::with_base_url(&client(), &server.uri())
    }

    #[tokio::test]
    async fn a_success_envelope_carries_the_bearer_token() {
        let server = MockServer::start().await;
        ok(
            &server,
            "GET",
            "/1/products",
            scenario("infomaniak", "products"),
        )
        .await;
        provider(&server).list_zones(&config()).await.unwrap();
        assert_eq!(
            header(&server, "GET", "/1/products", "Authorization")
                .await
                .as_deref(),
            Some("Bearer tk-1")
        );
    }

    #[tokio::test]
    async fn ids_and_ttls_arrive_as_either_strings_or_numbers() {
        let server = MockServer::start().await;
        ok(&server, "GET", RECORDS, scenario("infomaniak", "records")).await;
        let rows = provider(&server)
            .list_records(&config(), ZONE)
            .await
            .unwrap();
        assert_eq!(rows[0].id, "1001");
        assert_eq!(rows[0].name, "www.example.com");
        // A dotted source is the apex, and TXT targets come back quoted.
        assert_eq!(rows[1].name, ZONE);
        assert_eq!(rows[1].content, "v=spf1 -all");
        assert_eq!(rows[1].ttl, Some(3600));
    }

    #[tokio::test]
    async fn a_write_quotes_txt_and_clamps_the_ttl() {
        let server = MockServer::start().await;
        ok(&server, "POST", RECORDS, scenario("infomaniak", "created")).await;
        let created = provider(&server)
            .create_record(
                &config(),
                &RecordInput {
                    zone_id: ZONE.into(),
                    record_type: "TXT".into(),
                    name: "@".into(),
                    content: "v=spf1 -all".into(),
                    ttl: Some(10),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(
            sent(&server, "POST", RECORDS).await,
            vec![json!({
                "type": "TXT",
                "source": ".",
                "target": "\"v=spf1 -all\"",
                "ttl": 60,
            })]
        );
        assert_eq!(created.id, "1003");
        assert_eq!(created.name, ZONE);
    }

    #[tokio::test]
    async fn an_unchanged_type_updates_in_place() {
        let server = MockServer::start().await;
        ok(
            &server,
            "PUT",
            format!("{RECORDS}/1001").as_str(),
            scenario("infomaniak", "updated"),
        )
        .await;
        let record = provider(&server)
            .update_record(
                &config(),
                &RecordTarget {
                    zone_id: ZONE.into(),
                    id: "1001".into(),
                    record_type: "A".into(),
                    ..Default::default()
                },
                &RecordInput {
                    zone_id: ZONE.into(),
                    record_type: "A".into(),
                    name: "www".into(),
                    content: "192.0.2.9".into(),
                    ttl: Some(300),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        // The name and type are immutable here, so only the mutable fields are sent.
        assert_eq!(
            sent(&server, "PUT", format!("{RECORDS}/1001").as_str()).await,
            vec![json!({ "source": "www", "target": "192.0.2.9", "ttl": 300 })]
        );
        assert_eq!(record.content, "192.0.2.9");
    }

    #[tokio::test]
    async fn a_type_change_is_a_delete_and_a_recreate() {
        let server = MockServer::start().await;
        let path = format!("{RECORDS}/1001");
        ok(
            &server,
            "DELETE",
            path.as_str(),
            scenario("infomaniak", "deleted"),
        )
        .await;
        ok(&server, "POST", RECORDS, scenario("infomaniak", "created")).await;
        provider(&server)
            .update_record(
                &config(),
                &RecordTarget {
                    zone_id: ZONE.into(),
                    id: "1001".into(),
                    record_type: "A".into(),
                    ..Default::default()
                },
                &RecordInput {
                    zone_id: ZONE.into(),
                    record_type: "TXT".into(),
                    name: "www".into(),
                    content: "note".into(),
                    ttl: Some(300),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(hits(&server, "DELETE", path.as_str()).await, 1);
        assert_eq!(hits(&server, "POST", RECORDS).await, 1);
        assert_eq!(hits(&server, "PUT", path.as_str()).await, 0);
    }

    #[tokio::test]
    async fn a_delete_without_an_id_is_resolved_from_the_listing() {
        let server = MockServer::start().await;
        ok(&server, "GET", RECORDS, scenario("infomaniak", "records")).await;
        let path = format!("{RECORDS}/1001");
        ok(
            &server,
            "DELETE",
            path.as_str(),
            scenario("infomaniak", "deleted"),
        )
        .await;
        provider(&server)
            .delete_record(
                &config(),
                &RecordTarget {
                    zone_id: ZONE.into(),
                    name: "www.example.com".into(),
                    record_type: "a".into(),
                    content: "192.0.2.1".into(),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(hits(&server, "DELETE", path.as_str()).await, 1);
    }

    #[tokio::test]
    async fn an_error_envelope_at_http_200_is_still_a_failure() {
        let server = MockServer::start().await;
        ok(
            &server,
            "GET",
            "/1/products",
            scenario("infomaniak", "rejected"),
        )
        .await;
        let error = provider(&server)
            .list_zones(&config())
            .await
            .unwrap_err()
            .to_string();
        assert!(error.contains("The token is not valid"), "{error}");
    }
}
