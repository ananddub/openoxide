use async_trait::async_trait;
use serde_json::{Value, json};

use crate::config::{PorkbunConfig, ProviderConfig};
use crate::error::DnsError;
use crate::http::Http;
use crate::provider::{DnsProvider, find_row, row_matches};
use crate::records::{COMMON_TYPES, ensure_type, parse_mx, parse_srv, relative_name, to_fqdn};
use crate::types::{Record, RecordInput, RecordTarget, Zone};

const DEFAULT_API: &str = "https://api.porkbun.com/api/json/v3";
/// Porkbun rejects anything below 600s, so a shorter TTL from the form is raised rather than
/// rejected.
const DEFAULT_TTL: u32 = 600;

/// Porkbun DNS. Every call is a POST with the credentials in the body — there are no headers
/// and no GETs — and the reply is `{status, message, ...}` rather than an HTTP error code.
pub struct Porkbun {
    http: Http,
}

impl Porkbun {
    pub fn new(client: &reqwest::Client) -> Self {
        Self::with_base_url(client, DEFAULT_API)
    }

    pub fn with_base_url(client: &reqwest::Client, base_url: &str) -> Self {
        Self {
            http: Http::new(client, base_url),
        }
    }

    fn config<'a>(&self, config: &'a ProviderConfig) -> Result<&'a PorkbunConfig, DnsError> {
        config.porkbun()
    }

    /// Posts `payload` on top of the credential body and hands back the response document
    /// once the envelope says the call actually worked.
    async fn call(
        &self,
        credentials: &PorkbunConfig,
        path: &str,
        operation: &str,
        payload: Value,
    ) -> Result<Value, DnsError> {
        let mut body = json!({
            "apikey": credentials.api_key,
            "secretapikey": credentials.secret_api_key,
        });
        if let (Some(target), Some(fields)) = (body.as_object_mut(), payload.as_object()) {
            for (key, value) in fields {
                target.insert(key.clone(), value.clone());
            }
        }
        let response = self.http.post(path).json(&body).send().await?;
        let body = self.http.json(response, "Porkbun", operation).await?;
        if body["status"].as_str() == Some("SUCCESS") {
            Ok(body)
        } else {
            Err(DnsError::provider(format!(
                "Porkbun: {} failed: {}",
                operation,
                match body["message"].as_str().filter(|m| !m.trim().is_empty()) {
                    Some(message) => message.to_string(),
                    None => "the API reported a non-success status".to_string(),
                }
            )))
        }
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
impl DnsProvider for Porkbun {
    async fn test_connection(&self, config: &ProviderConfig) -> Result<(), DnsError> {
        let credentials = self.config(config)?;
        self.call(credentials, "/ping", "verify credentials", json!({}))
            .await?;
        Ok(())
    }

    async fn list_zones(&self, config: &ProviderConfig) -> Result<Vec<Zone>, DnsError> {
        let credentials = self.config(config)?;
        let body = self
            .call(credentials, "/domain/listAll", "list zones", json!({}))
            .await?;
        Ok(body["domains"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|domain| {
                let name = domain["domain"].as_str().unwrap_or_default().to_string();
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
        let path = format!("/dns/retrieve/{zone_id}");
        let body = self
            .call(credentials, &path, "list records", json!({}))
            .await?;
        Ok(body["records"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|row| record_from(&row, zone_id))
            .collect())
    }

    async fn create_record(
        &self,
        config: &ProviderConfig,
        input: &RecordInput,
    ) -> Result<Record, DnsError> {
        let credentials = self.config(config)?;
        let path = format!("/dns/create/{}", input.zone_id);
        let body = self
            .call(
                credentials,
                &path,
                "create record",
                record_payload(input, &input.zone_id)?,
            )
            .await?;
        Ok(record_from_input(input, text(&body["id"]), &input.zone_id))
    }

    async fn update_record(
        &self,
        config: &ProviderConfig,
        target: &RecordTarget,
        next: &RecordInput,
    ) -> Result<Record, DnsError> {
        let record_id = self.resolve_record_id(config, target).await?;
        let credentials = self.config(config)?;
        let path = format!("/dns/edit/{}/{}", next.zone_id, record_id);
        self.call(
            credentials,
            &path,
            "update record",
            record_payload(next, &next.zone_id)?,
        )
        .await?;
        Ok(record_from_input(next, record_id, &next.zone_id))
    }

    async fn delete_record(
        &self,
        config: &ProviderConfig,
        target: &RecordTarget,
    ) -> Result<(), DnsError> {
        let record_id = self.resolve_record_id(config, target).await?;
        let credentials = self.config(config)?;
        let path = format!("/dns/delete/{}/{}", target.zone_id, record_id);
        self.call(credentials, &path, "delete record", json!({}))
            .await?;
        Ok(())
    }
}

/// Porkbun keeps the priority of MX and SRV in its own `prio` field, so the single string the
/// panel stores is split on the way in and re-joined by [`record_from`] on the way out.
fn record_payload(input: &RecordInput, zone_id: &str) -> Result<Value, DnsError> {
    let record_type = ensure_type("Porkbun", &input.record_type, COMMON_TYPES)?;
    let (content, priority) = split_value(&record_type, &input.content)?;
    let mut payload = json!({
        "name": label(&input.name, zone_id),
        "type": record_type,
        "content": content,
        "ttl": normalize_ttl(input.ttl),
    });
    if let Some(priority) = priority {
        payload["prio"] = json!(priority);
    }
    Ok(payload)
}

fn split_value(record_type: &str, content: &str) -> Result<(String, Option<u16>), DnsError> {
    match record_type {
        "MX" => {
            let mx = parse_mx(content)?;
            Ok((mx.host, Some(mx.priority)))
        }
        "SRV" => {
            let srv = parse_srv(content)?;
            Ok((
                format!("{} {} {}", srv.weight, srv.port, srv.target),
                Some(srv.priority),
            ))
        }
        _ => Ok((content.trim().to_string(), None)),
    }
}

fn record_from(row: &Value, zone_id: &str) -> Record {
    let record_type = row["type"].as_str().unwrap_or_default().to_string();
    let content = row["content"].as_str().unwrap_or_default().to_string();
    let priority = matches!(record_type.as_str(), "MX" | "SRV")
        .then(|| text(&row["prio"]))
        .filter(|raw| !raw.is_empty());
    let content = match priority {
        Some(raw) => format!("{raw} {content}"),
        None => content,
    };
    Record {
        id: text(&row["id"]),
        zone_id: zone_id.to_string(),
        name: to_fqdn(row["name"].as_str().unwrap_or_default(), zone_id),
        record_type,
        content,
        ttl: ttl_of(row).or(Some(DEFAULT_TTL)),
        proxied: None,
    }
}

fn record_from_input(input: &RecordInput, id: String, zone_id: &str) -> Record {
    let record_type = input.record_type.trim().to_ascii_uppercase();
    Record {
        id,
        zone_id: zone_id.to_string(),
        record_type: record_type.clone(),
        name: to_fqdn(&input.name, zone_id),
        content: input.content.trim().to_string(),
        ttl: Some(normalize_ttl(input.ttl)),
        proxied: None,
    }
}

/// Porkbun answers `ttl` as a string and occasionally as a number.
fn ttl_of(row: &Value) -> Option<u32> {
    text(&row["ttl"]).trim().parse().ok()
}

/// Porkbun wants the label rather than the FQDN, and an empty label for the apex.
fn label(name: &str, zone_id: &str) -> String {
    let relative = relative_name(name, zone_id);
    if relative == "@" {
        String::new()
    } else {
        relative
    }
}

fn normalize_ttl(ttl: Option<u32>) -> u32 {
    match ttl {
        Some(value) if value >= DEFAULT_TTL => value,
        _ => DEFAULT_TTL,
    }
}

/// Porkbun mixes strings and numbers across the same fields (`id`, `ttl`, `prio`), so both
/// have to read as text.
fn text(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Number(number) => number.to_string(),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DnsProviderKind, ProviderConfig};
    use crate::providers::harness::{client, hits, ok, respond, scenario, sent};
    use serde_json::json;
    use wiremock::MockServer;

    const ZONE: &str = "example.com";
    const KEYS: &str = r#"{"apiKey":"pk-1","secretApiKey":"sk-1"}"#;

    fn config() -> ProviderConfig {
        ProviderConfig::from_json(DnsProviderKind::Porkbun, KEYS).unwrap()
    }

    fn provider(server: &MockServer) -> Porkbun {
        Porkbun::with_base_url(&client(), &server.uri())
    }

    #[tokio::test]
    async fn credentials_travel_in_the_body_because_there_are_no_headers() {
        let server = MockServer::start().await;
        ok(&server, "POST", "/ping", scenario("porkbun", "ping")).await;
        provider(&server).test_connection(&config()).await.unwrap();
        assert_eq!(
            sent(&server, "POST", "/ping").await,
            vec![json!({ "apikey": "pk-1", "secretapikey": "sk-1" })]
        );
    }

    #[tokio::test]
    async fn zones_are_read_from_the_domain_list() {
        let server = MockServer::start().await;
        ok(
            &server,
            "POST",
            "/domain/listAll",
            scenario("porkbun", "domains"),
        )
        .await;
        let zones = provider(&server).list_zones(&config()).await.unwrap();
        assert_eq!(
            zones
                .iter()
                .map(|zone| zone.id.as_str())
                .collect::<Vec<_>>(),
            ["example.com", "second.com"]
        );
    }

    #[tokio::test]
    async fn ttl_and_priority_arrive_as_strings() {
        let server = MockServer::start().await;
        ok(
            &server,
            "POST",
            "/dns/retrieve/example.com",
            scenario("porkbun", "records"),
        )
        .await;
        let rows = provider(&server)
            .list_records(&config(), ZONE)
            .await
            .unwrap();

        assert_eq!(rows[0].id, "111");
        assert_eq!(rows[0].ttl, Some(600));
        assert_eq!(rows[0].content, "192.0.2.1");
        assert_eq!(rows[1].id, "222");
        assert_eq!(rows[1].content, "10 smtp.example.com");
        assert_eq!(rows[1].ttl, Some(600));
    }

    #[tokio::test]
    async fn a_create_sends_the_relative_label_and_raises_a_short_ttl() {
        let server = MockServer::start().await;
        ok(
            &server,
            "POST",
            "/dns/create/example.com",
            scenario("porkbun", "created"),
        )
        .await;
        let record = provider(&server)
            .create_record(
                &config(),
                &RecordInput {
                    zone_id: ZONE.into(),
                    record_type: "A".into(),
                    name: "www.example.com".into(),
                    content: "192.0.2.1".into(),
                    ttl: Some(60),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        assert_eq!(
            sent(&server, "POST", "/dns/create/example.com").await,
            vec![json!({
                "apikey": "pk-1",
                "secretapikey": "sk-1",
                "name": "www",
                "type": "A",
                "content": "192.0.2.1",
                "ttl": 600,
            })]
        );
        assert_eq!(record.id, "333");
        assert_eq!(record.name, "www.example.com");
    }

    #[tokio::test]
    async fn an_srv_priority_and_an_apex_label_are_the_other_two_quirks() {
        let server = MockServer::start().await;
        ok(
            &server,
            "POST",
            "/dns/create/example.com",
            scenario("porkbun", "created"),
        )
        .await;
        provider(&server)
            .create_record(
                &config(),
                &RecordInput {
                    zone_id: ZONE.into(),
                    record_type: "SRV".into(),
                    name: "example.com".into(),
                    content: "10 60 5060 sip.example.com".into(),
                    ttl: Some(600),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        assert_eq!(
            sent(&server, "POST", "/dns/create/example.com").await,
            vec![json!({
                "apikey": "pk-1",
                "secretapikey": "sk-1",
                // The apex is an empty label and SRV keeps its priority in `prio`.
                "name": "",
                "type": "SRV",
                "content": "60 5060 sip.example.com",
                "ttl": 600,
                "prio": 10,
            })]
        );
    }

    #[tokio::test]
    async fn edit_and_delete_find_the_id_in_the_listing() {
        let server = MockServer::start().await;
        ok(
            &server,
            "POST",
            "/dns/retrieve/example.com",
            scenario("porkbun", "records"),
        )
        .await;
        ok(
            &server,
            "POST",
            "/dns/edit/example.com/111",
            scenario("porkbun", "edited"),
        )
        .await;
        ok(
            &server,
            "POST",
            "/dns/delete/example.com/111",
            scenario("porkbun", "deleted"),
        )
        .await;
        let provider = provider(&server);
        let config = config();
        let target = RecordTarget {
            zone_id: ZONE.into(),
            name: "www.example.com".into(),
            record_type: "A".into(),
            content: "192.0.2.1".into(),
            ..Default::default()
        };

        let updated = provider
            .update_record(
                &config,
                &target,
                &RecordInput {
                    zone_id: ZONE.into(),
                    record_type: "A".into(),
                    name: "www.example.com".into(),
                    content: "192.0.2.9".into(),
                    ttl: Some(600),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(updated.id, "111");
        provider.delete_record(&config, &target).await.unwrap();

        assert_eq!(hits(&server, "POST", "/dns/edit/example.com/111").await, 1);
        assert_eq!(
            hits(&server, "POST", "/dns/delete/example.com/111").await,
            1
        );
    }

    #[tokio::test]
    async fn an_error_answered_at_http_200_is_still_a_failure() {
        let server = MockServer::start().await;
        respond(
            &server,
            "POST",
            "/domain/listAll",
            200,
            scenario("porkbun", "rejected"),
        )
        .await;
        let error = provider(&server)
            .list_zones(&config())
            .await
            .unwrap_err()
            .to_string();
        assert!(error.contains("API key is incorrect"), "{error}");
    }
}
