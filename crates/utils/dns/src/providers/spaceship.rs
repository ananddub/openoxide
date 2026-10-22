use async_trait::async_trait;
use reqwest::Client;
use serde_json::{Value, json};

use crate::config::{ProviderConfig, SpaceshipConfig};
use crate::error::DnsError;
use crate::http::Http;
use crate::provider::{DnsProvider, find_row, row_matches};
use crate::records::{COMMON_TYPES, ensure_type, parse_mx, relative_name, to_fqdn, unquote};
use crate::types::{Record, RecordInput, RecordTarget, Zone};

const DEFAULT_API: &str = "https://spaceship.dev";
const PAGE_SIZE: u32 = 100;
const DEFAULT_TTL: u32 = 3600;
const MIN_TTL: u32 = 60;

/// Spaceship DNS. Records have no ids, so a row is addressed as `name:type:content` and both
/// create and update are the same `PUT` of one item — `force` decides whether an existing
/// `name` + `type` set is replaced.
pub struct Spaceship {
    http: Http,
}

impl Spaceship {
    pub fn new(client: &Client) -> Self {
        Self::with_base_url(client, DEFAULT_API)
    }

    pub fn with_base_url(client: &Client, base_url: &str) -> Self {
        Self {
            http: Http::new(client, base_url),
        }
    }

    fn config<'a>(&self, config: &'a ProviderConfig) -> Result<&'a SpaceshipConfig, DnsError> {
        config.spaceship()
    }

    fn get(&self, credentials: &SpaceshipConfig, path: &str) -> reqwest::RequestBuilder {
        self.http
            .get(path)
            .header("X-API-Key", &credentials.api_key)
            .header("X-API-Secret", &credentials.api_secret)
    }

    async fn paged(
        &self,
        credentials: &SpaceshipConfig,
        path: String,
        operation: &str,
    ) -> Result<Vec<Value>, DnsError> {
        let mut items = Vec::new();
        let mut skip: u32 = 0;
        loop {
            let request = self
                .get(credentials, &path)
                .query(&[("take", PAGE_SIZE), ("skip", skip)]);
            let body = self
                .http
                .json(request.send().await?, "Spaceship", operation)
                .await?;
            let rows = body["items"].as_array().cloned().unwrap_or_default();
            let count = rows.len() as u32;
            let total = body["total"].as_u64().unwrap_or_default() as u32;
            items.extend(rows);
            if count == 0 || skip + count >= total {
                break;
            }
            skip += PAGE_SIZE;
        }
        Ok(items)
    }

    /// Addresses a row the panel selected. The id carries the relative name, which is what the
    /// delete endpoint wants; a request without one is matched against the listing.
    async fn resolve(
        &self,
        config: &ProviderConfig,
        target: &RecordTarget,
    ) -> Result<(String, String, String), DnsError> {
        let id = if target.has_id() {
            target.id.clone()
        } else {
            let rows = self.list_records(config, &target.zone_id).await?;
            rows[find_row(&rows, target, row_matches)?].id.clone()
        };
        let mut parts = id.splitn(3, ':');
        let name = parts.next().unwrap_or_default().to_string();
        let record_type = parts.next().unwrap_or_default().to_string();
        let content = parts.next().unwrap_or_default().to_string();
        if name.is_empty() || record_type.is_empty() {
            return Err(DnsError::provider(format!(
                "Spaceship: cannot address record {id:?}, expected \"name:type:content\""
            )));
        }
        Ok((name, record_type, content))
    }

    async fn save(
        &self,
        credentials: &SpaceshipConfig,
        zone_id: &str,
        force: bool,
        item: Value,
        operation: &str,
    ) -> Result<(), DnsError> {
        let path = format!("/v1/dns/records/{zone_id}");
        let response = self
            .http
            .put(&path)
            .header("X-API-Key", &credentials.api_key)
            .header("X-API-Secret", &credentials.api_secret)
            .json(&json!({ "force": force, "items": [item] }))
            .send()
            .await?;
        self.http.json(response, "Spaceship", operation).await?;
        Ok(())
    }

    async fn drop(
        &self,
        credentials: &SpaceshipConfig,
        zone_id: &str,
        item: Value,
    ) -> Result<(), DnsError> {
        let path = format!("/v1/dns/records/{zone_id}");
        let response = self
            .http
            .delete(&path)
            .header("X-API-Key", &credentials.api_key)
            .header("X-API-Secret", &credentials.api_secret)
            .json(&json!([item]))
            .send()
            .await?;
        self.http
            .json(response, "Spaceship", "delete record")
            .await?;
        Ok(())
    }
}

#[async_trait]
impl DnsProvider for Spaceship {
    async fn test_connection(&self, config: &ProviderConfig) -> Result<(), DnsError> {
        let credentials = self.config(config)?;
        let request = self.get(credentials, "/v1/domains").query(&[("take", 1)]);
        self.http
            .json(request.send().await?, "Spaceship", "verify credentials")
            .await?;
        Ok(())
    }

    async fn list_zones(&self, config: &ProviderConfig) -> Result<Vec<Zone>, DnsError> {
        let credentials = self.config(config)?;
        Ok(self
            .paged(credentials, "/v1/domains".to_string(), "list zones")
            .await?
            .into_iter()
            .map(|row| {
                let name = row["name"].as_str().unwrap_or_default().to_string();
                Zone {
                    id: name.clone(),
                    name,
                    status: None,
                }
            })
            .filter(|zone: &Zone| !zone.id.is_empty())
            .collect())
    }

    async fn list_records(
        &self,
        config: &ProviderConfig,
        zone_id: &str,
    ) -> Result<Vec<Record>, DnsError> {
        let credentials = self.config(config)?;
        let path = format!("/v1/dns/records/{zone_id}");
        Ok(self
            .paged(credentials, path, "list records")
            .await?
            .into_iter()
            .map(|row| {
                let record_type = row["type"]
                    .as_str()
                    .unwrap_or_default()
                    .to_ascii_uppercase();
                let label = row["name"].as_str().unwrap_or_default().to_string();
                let content = display_content(&record_type, &row);
                Record {
                    id: format!("{label}:{record_type}:{content}"),
                    zone_id: zone_id.to_string(),
                    name: to_fqdn(&label, zone_id),
                    record_type,
                    ttl: Some(
                        row["ttl"]
                            .as_u64()
                            .map(|value| value as u32)
                            .filter(|value| *value > 0)
                            .unwrap_or(DEFAULT_TTL),
                    ),
                    proxied: None,
                    content,
                }
            })
            .collect())
    }

    async fn create_record(
        &self,
        config: &ProviderConfig,
        input: &RecordInput,
    ) -> Result<Record, DnsError> {
        let credentials = self.config(config)?;
        let item = record_item(input)?;
        self.save(
            credentials,
            &input.zone_id,
            false,
            item.clone(),
            "create record",
        )
        .await?;
        Ok(record_from_item(&item, &input.zone_id))
    }

    async fn update_record(
        &self,
        config: &ProviderConfig,
        target: &RecordTarget,
        next: &RecordInput,
    ) -> Result<Record, DnsError> {
        let credentials = self.config(config)?;
        let (old_name, old_type, old_content) = self.resolve(config, target).await?;
        let item = record_item(next)?;
        let renamed = old_name != item["name"].as_str().unwrap_or_default()
            || !old_type.eq_ignore_ascii_case(item["type"].as_str().unwrap_or_default());
        if renamed {
            // A moved record is not the same row any more, and `force` only replaces the set it
            // lands in, so the old one has to go first.
            self.drop(
                credentials,
                &target.zone_id,
                delete_item(&old_name, &old_type, &old_content),
            )
            .await?;
        }
        self.save(
            credentials,
            &next.zone_id,
            true,
            item.clone(),
            "update record",
        )
        .await?;
        Ok(record_from_item(&item, &next.zone_id))
    }

    async fn delete_record(
        &self,
        config: &ProviderConfig,
        target: &RecordTarget,
    ) -> Result<(), DnsError> {
        let credentials = self.config(config)?;
        let (name, record_type, content) = self.resolve(config, target).await?;
        self.drop(
            credentials,
            &target.zone_id,
            delete_item(&name, &record_type, &content),
        )
        .await
    }
}

/// Builds the `rData`-style flat item Spaceship expects: the value goes into the field that
/// matches the type, and an MX splits into `exchange` plus `preference`.
fn record_item(input: &RecordInput) -> Result<Value, DnsError> {
    let record_type = ensure_type("Spaceship", &input.record_type, COMMON_TYPES)?;
    let mut label = relative_name(&input.name, &input.zone_id);
    if label.is_empty() {
        label = "@".to_string();
    }
    let ttl = match input.ttl {
        Some(value) if value >= MIN_TTL => value,
        _ => DEFAULT_TTL,
    };
    let mut item = json!({ "type": record_type, "name": label, "ttl": ttl });
    fill_value(&mut item, &record_type, &input.content);
    Ok(item)
}

fn delete_item(name: &str, record_type: &str, content: &str) -> Value {
    let mut item = json!({ "type": record_type.to_ascii_uppercase(), "name": name });
    fill_value(&mut item, &record_type.to_ascii_uppercase(), content);
    item
}

fn fill_value(item: &mut Value, record_type: &str, content: &str) {
    let trimmed = content.trim();
    match record_type {
        "A" | "AAAA" => item["address"] = json!(trimmed),
        "CNAME" => item["cname"] = json!(trimmed),
        "NS" => item["nameserver"] = json!(trimmed),
        "MX" => {
            if let Ok(mx) = parse_mx(trimmed) {
                item["exchange"] = json!(mx.host);
                item["preference"] = json!(mx.priority);
            } else {
                item["value"] = json!(trimmed);
            }
        }
        "TXT" => item["value"] = json!(unquote(trimmed)),
        _ => item["value"] = json!(trimmed),
    }
}

fn display_content(record_type: &str, row: &Value) -> String {
    let field = |name: &str| row[name].as_str().unwrap_or_default().to_string();
    match record_type {
        "A" | "AAAA" => field("address"),
        "CNAME" => field("cname"),
        "NS" => field("nameserver"),
        "MX" => match row["preference"].as_u64() {
            Some(preference) => format!("{} {}", preference, field("exchange")),
            None => field("exchange"),
        },
        _ => field("value"),
    }
}

fn record_from_item(item: &Value, zone_id: &str) -> Record {
    let record_type = item["type"].as_str().unwrap_or_default().to_string();
    let label = item["name"].as_str().unwrap_or_default().to_string();
    let content = display_content(&record_type, item);
    Record {
        id: format!("{label}:{record_type}:{content}"),
        zone_id: zone_id.to_string(),
        name: to_fqdn(&label, zone_id),
        record_type,
        ttl: item["ttl"].as_u64().map(|value| value as u32),
        proxied: None,
        content,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DnsProviderKind;
    use crate::providers::harness::{client, header, hits, ok, respond, scenario, sent};
    use serde_json::json;
    use wiremock::MockServer;

    const ZONE: &str = "example.com";
    const DOMAINS: &str = "/v1/domains";
    const RECORDS: &str = "/v1/dns/records/example.com";
    const KEYS: &str = r#"{"apiKey":"k1","apiSecret":"s1"}"#;

    fn config() -> ProviderConfig {
        ProviderConfig::from_json(DnsProviderKind::Spaceship, KEYS).unwrap()
    }

    fn provider(server: &MockServer) -> Spaceship {
        Spaceship::with_base_url(&client(), &server.uri())
    }

    #[tokio::test]
    async fn the_keys_travel_in_headers_and_the_listing_paginates_itself() {
        let server = MockServer::start().await;
        ok(&server, "GET", DOMAINS, scenario("spaceship", "domains")).await;
        let zones = provider(&server).list_zones(&config()).await.unwrap();
        assert_eq!(
            header(&server, "GET", DOMAINS, "X-API-Key")
                .await
                .as_deref(),
            Some("k1")
        );
        assert_eq!(
            header(&server, "GET", DOMAINS, "X-API-Secret")
                .await
                .as_deref(),
            Some("s1")
        );
        assert_eq!(zones.len(), 2);
        // `total` is already covered by the first page, so no second request is made.
        assert_eq!(hits(&server, "GET", DOMAINS).await, 1);
    }

    #[tokio::test]
    async fn a_row_is_addressed_by_name_type_and_content() {
        let server = MockServer::start().await;
        ok(&server, "GET", RECORDS, scenario("spaceship", "records")).await;
        let rows = provider(&server)
            .list_records(&config(), ZONE)
            .await
            .unwrap();
        // MX joins its preference back into the one string the panel shows.
        assert_eq!(rows[0].id, "www:A:192.0.2.1");
        assert_eq!(rows[1].content, "10 smtp.example.com");
        assert_eq!(rows[1].name, ZONE);
        assert_eq!(rows[2].content, "v=spf1 -all");
    }

    #[tokio::test]
    async fn a_write_puts_one_item_with_the_type_specific_field() {
        let server = MockServer::start().await;
        ok(&server, "PUT", RECORDS, scenario("spaceship", "ack")).await;
        let created = provider(&server)
            .create_record(
                &config(),
                &RecordInput {
                    zone_id: ZONE.into(),
                    record_type: "MX".into(),
                    name: "mail".into(),
                    content: "20 smtp.example.com".into(),
                    ttl: Some(30),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(
            sent(&server, "PUT", RECORDS).await,
            vec![json!({
                "force": false,
                "items": [{ "type": "MX", "name": "mail", "ttl": 3600, "exchange": "smtp.example.com", "preference": 20 }],
            })]
        );
        assert_eq!(created.id, "mail:MX:20 smtp.example.com");
    }

    #[tokio::test]
    async fn an_apex_stays_a_dot_and_a_type_change_is_a_drop_and_a_save() {
        let server = MockServer::start().await;
        ok(&server, "PUT", RECORDS, scenario("spaceship", "ack")).await;
        ok(&server, "DELETE", RECORDS, scenario("spaceship", "ack")).await;
        provider(&server)
            .update_record(
                &config(),
                &RecordTarget {
                    zone_id: ZONE.into(),
                    id: "www:A:192.0.2.1".into(),
                    ..Default::default()
                },
                &RecordInput {
                    zone_id: ZONE.into(),
                    record_type: "TXT".into(),
                    name: "@".into(),
                    content: "\"note\"".into(),
                    ttl: Some(600),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(
            sent(&server, "DELETE", RECORDS).await,
            vec![json!([{ "type": "A", "name": "www", "address": "192.0.2.1" }])]
        );
        assert_eq!(
            sent(&server, "PUT", RECORDS).await,
            vec![json!({
                "force": true,
                "items": [{ "type": "TXT", "name": "@", "ttl": 600, "value": "note" }],
            })]
        );
    }

    #[tokio::test]
    async fn a_delete_without_an_id_is_resolved_from_the_listing() {
        let server = MockServer::start().await;
        ok(&server, "GET", RECORDS, scenario("spaceship", "records")).await;
        ok(&server, "DELETE", RECORDS, scenario("spaceship", "ack")).await;
        provider(&server)
            .delete_record(
                &config(),
                &RecordTarget {
                    zone_id: ZONE.into(),
                    name: "www.example.com".into(),
                    record_type: "A".into(),
                    content: "192.0.2.1".into(),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(hits(&server, "DELETE", RECORDS).await, 1);
    }

    #[tokio::test]
    async fn a_rejected_call_keeps_the_providers_words() {
        let server = MockServer::start().await;
        respond(
            &server,
            "GET",
            DOMAINS,
            401,
            scenario("spaceship", "rejected"),
        )
        .await;
        let error = provider(&server)
            .list_zones(&config())
            .await
            .unwrap_err()
            .to_string();
        assert!(error.contains("invalid credentials"), "{error}");
    }
}
