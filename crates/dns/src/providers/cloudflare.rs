use async_trait::async_trait;
use serde_json::{Value, json};

use crate::config::{CloudflareConfig, ProviderConfig};
use crate::error::DnsError;
use crate::http::Http;
use crate::provider::{DnsProvider, find_row, row_matches};
use crate::records::{
    COMMON_TYPES, ensure_type, is_proxyable, parse_caa, parse_mx, parse_srv, render_mx,
};
use crate::types::{Record, RecordInput, RecordTarget, Zone};

const DEFAULT_API: &str = "https://api.cloudflare.com/client/v4";
const PAGE_SIZE: u32 = 50;

/// Cloudflare DNS, on the v4 zone API with a bearer API token.
pub struct Cloudflare {
    http: Http,
}

impl Cloudflare {
    pub fn new(client: &reqwest::Client) -> Self {
        Self::with_base_url(client, DEFAULT_API)
    }

    pub fn with_base_url(client: &reqwest::Client, base_url: &str) -> Self {
        Self {
            http: Http::new(client, base_url),
        }
    }

    fn config<'a>(&'a self, config: &'a ProviderConfig) -> Result<&'a CloudflareConfig, DnsError> {
        config.cloudflare()
    }
}

#[async_trait]
impl DnsProvider for Cloudflare {
    async fn test_connection(&self, config: &ProviderConfig) -> Result<(), DnsError> {
        let credentials = self.config(config)?;
        let request = self
            .http
            .get("/zones")
            .query(&[("per_page", "1")])
            .bearer_auth(&credentials.api_token);
        result(
            self.http
                .json(request.send().await?, "Cloudflare", "verify credentials")
                .await?,
        )
        .map(|_| ())
    }

    async fn list_zones(&self, config: &ProviderConfig) -> Result<Vec<Zone>, DnsError> {
        let credentials = self.config(config)?;
        let mut zones = Vec::new();
        let mut page = 1;
        loop {
            let request = self
                .http
                .get("/zones")
                .query(&[("per_page", PAGE_SIZE), ("page", page)])
                .bearer_auth(&credentials.api_token);
            let rows = rows(
                self.http
                    .json(request.send().await?, "Cloudflare", "list zones")
                    .await?,
            )?;
            let count = rows.len();
            zones.extend(rows.into_iter().map(|zone| Zone {
                id: text(&zone["id"]),
                name: text(&zone["name"]),
                status: zone["status"].as_str().map(str::to_string),
            }));
            if count < PAGE_SIZE as usize {
                break;
            }
            page += 1;
        }
        Ok(zones)
    }

    async fn list_records(
        &self,
        config: &ProviderConfig,
        zone_id: &str,
    ) -> Result<Vec<Record>, DnsError> {
        let credentials = self.config(config)?;
        let path = format!("/zones/{zone_id}/dns_records");
        let mut records = Vec::new();
        let mut page = 1;
        loop {
            let request = self
                .http
                .get(&path)
                .query(&[("per_page", PAGE_SIZE), ("page", page)])
                .bearer_auth(&credentials.api_token);
            let rows = rows(
                self.http
                    .json(request.send().await?, "Cloudflare", "list records")
                    .await?,
            )?;
            let count = rows.len();
            records.extend(rows.into_iter().map(|row| Record {
                id: text(&row["id"]),
                zone_id: zone_id.to_string(),
                record_type: text(&row["type"]),
                name: text(&row["name"]),
                content: record_content(&row),
                ttl: row["ttl"].as_u64().map(|value| value as u32),
                proxied: row["proxied"].as_bool(),
            }));
            if count < PAGE_SIZE as usize {
                break;
            }
            page += 1;
        }
        Ok(records)
    }

    async fn create_record(
        &self,
        config: &ProviderConfig,
        input: &RecordInput,
    ) -> Result<Record, DnsError> {
        let credentials = self.config(config)?;
        let path = format!("/zones/{}/dns_records", input.zone_id);
        // Create always POSTs. Searching for a name+type match and PUTting to it would
        // overwrite an unrelated value instead of adding a second one.
        let request = self
            .http
            .post(&path)
            .bearer_auth(&credentials.api_token)
            .json(&payload(input)?);
        let created = result(
            self.http
                .json(request.send().await?, "Cloudflare", "create record")
                .await?,
        )?;
        Ok(Record {
            id: text(&created["id"]),
            zone_id: input.zone_id.clone(),
            record_type: text(&created["type"]),
            name: text(&created["name"]),
            content: record_content(&created),
            ttl: created["ttl"]
                .as_u64()
                .map(|value| value as u32)
                .or(input.ttl),
            proxied: created["proxied"].as_bool().or(input.proxied),
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
        let path = format!("/zones/{}/dns_records/{record_id}", next.zone_id);
        let request = self
            .http
            .put(&path)
            .bearer_auth(&credentials.api_token)
            .json(&payload(next)?);
        let updated = result(
            self.http
                .json(request.send().await?, "Cloudflare", "update record")
                .await?,
        )?;
        Ok(Record {
            id: text_or(&updated["id"], &record_id),
            zone_id: next.zone_id.clone(),
            record_type: text(&updated["type"]),
            name: text(&updated["name"]),
            content: record_content(&updated),
            ttl: updated["ttl"]
                .as_u64()
                .map(|value| value as u32)
                .or(next.ttl),
            proxied: updated["proxied"].as_bool().or(next.proxied),
        })
    }

    async fn delete_record(
        &self,
        config: &ProviderConfig,
        target: &RecordTarget,
    ) -> Result<(), DnsError> {
        // The delete endpoint takes a record id, never a name.
        let record_id = self.resolve_record_id(config, target).await?;
        let credentials = self.config(config)?;
        let path = format!("/zones/{}/dns_records/{record_id}", target.zone_id);
        let request = self.http.delete(&path).bearer_auth(&credentials.api_token);
        result(
            self.http
                .json(request.send().await?, "Cloudflare", "delete record")
                .await?,
        )?;
        Ok(())
    }
}

impl Cloudflare {
    /// The strongest hint wins: a supplied id goes straight through, otherwise the row is
    /// found in the provider's own listing so a rename or a multi-value name stays
    /// unambiguous.
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

/// Builds a record body. Cloudflare wants `content` for simple types, a `priority` beside
/// the content for MX, and a `data` object for SRV and CAA.
fn payload(input: &RecordInput) -> Result<Value, DnsError> {
    let record_type = ensure_type("Cloudflare", &input.record_type, COMMON_TYPES)?;
    let mut payload = json!({
        "type": record_type,
        "name": input.name,
        "content": input.content,
        "ttl": input.ttl.unwrap_or(1),
    });
    match record_type.as_str() {
        "MX" => {
            let mx = parse_mx(&input.content)?;
            payload["content"] = json!(mx.host);
            payload["priority"] = json!(mx.priority);
        }
        "SRV" => {
            let srv = parse_srv(&input.content)?;
            payload["data"] = json!({
                "priority": srv.priority,
                "weight": srv.weight,
                "port": srv.port,
                "target": srv.target,
            });
            payload
                .as_object_mut()
                .map(|object| object.remove("content"));
        }
        "CAA" => {
            let caa = parse_caa(&input.content)?;
            payload["data"] = json!({
                "flags": caa.flags,
                "tag": caa.tag,
                "value": caa.value,
            });
            payload
                .as_object_mut()
                .map(|object| object.remove("content"));
        }
        _ => {}
    }
    if input.proxied.is_some() && is_proxyable(&record_type) {
        payload["proxied"] = json!(input.proxied.unwrap_or(false));
    }
    Ok(payload)
}

/// Renders a listed row back into the single-string form the panel edits.
fn record_content(row: &Value) -> String {
    let record_type = row["type"].as_str().unwrap_or_default();
    match record_type {
        "MX" => {
            let host = row["content"].as_str().unwrap_or_default();
            match row["priority"].as_u64() {
                Some(priority) => render_mx(&crate::records::MxValue {
                    priority: priority as u16,
                    host: host.to_string(),
                }),
                None => host.to_string(),
            }
        }
        "SRV" => row["data"]
            .as_object()
            .map(|data| {
                format!(
                    "{} {} {} {}",
                    data["priority"].as_u64().unwrap_or_default(),
                    data["weight"].as_u64().unwrap_or_default(),
                    data["port"].as_u64().unwrap_or_default(),
                    data["target"].as_str().unwrap_or_default(),
                )
            })
            .unwrap_or_else(|| text(&row["content"])),
        "CAA" => row["data"]
            .as_object()
            .map(|data| {
                format!(
                    "{} {} \"{}\"",
                    data["flags"].as_u64().unwrap_or_default(),
                    data["tag"].as_str().unwrap_or_default(),
                    data["value"].as_str().unwrap_or_default(),
                )
            })
            .unwrap_or_else(|| text(&row["content"])),
        _ => text(&row["content"]),
    }
}

/// Unwraps Cloudflare's `{success, errors, result}` envelope. A 2xx with `success: false`
/// still counts as a failure, because that is how the API reports a rejected token.
fn result(body: Value) -> Result<Value, DnsError> {
    if body["success"].as_bool() != Some(true) {
        return Err(DnsError::provider(format!(
            "Cloudflare: {}",
            error_detail(&body)
        )));
    }
    Ok(body["result"].clone())
}

fn rows(body: Value) -> Result<Vec<Value>, DnsError> {
    match result(body)? {
        Value::Array(rows) => Ok(rows),
        other => Err(DnsError::provider(format!(
            "Cloudflare returned an unexpected response: {other}"
        ))),
    }
}

/// Joins `errors[].message` with its `error_chain`, which is where Cloudflare puts the part
/// that actually explains a rejection.
fn error_detail(body: &Value) -> String {
    let Some(errors) = body["errors"].as_array() else {
        return "request rejected without an error list".to_string();
    };
    let messages: Vec<String> = errors
        .iter()
        .filter_map(|error| {
            let message = error["message"].as_str()?.trim();
            if message.is_empty() {
                return None;
            }
            let chained: Vec<&str> = error["error_chain"]
                .as_array()
                .map(|chain| {
                    chain
                        .iter()
                        .filter_map(|entry| entry["message"].as_str())
                        .filter(|nested| !nested.trim().is_empty())
                        .collect()
                })
                .unwrap_or_default();
            if chained.is_empty() {
                Some(message.to_string())
            } else {
                Some(format!("{message} ({})", chained.join(", ")))
            }
        })
        .collect();
    if messages.is_empty() {
        "request failed".to_string()
    } else {
        messages.join(", ")
    }
}

fn text(value: &Value) -> String {
    value.as_str().unwrap_or_default().to_string()
}

/// Reads a string the provider may have left out, keeping the value we already know.
fn text_or(value: &Value, fallback: &str) -> String {
    match value.as_str() {
        Some(text) if !text.is_empty() => text.to_string(),
        _ => fallback.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DnsProviderKind, ProviderConfig};
    use crate::providers::harness::{header, hits, ok, respond, scenario, sent};
    use serde_json::json;
    use wiremock::MockServer;

    const ZONE: &str = "0ss4vkxTzjBuHWcnq2khO636I1ZRGkmi";

    fn config() -> ProviderConfig {
        ProviderConfig::from_json(DnsProviderKind::Cloudflare, r#"{"apiToken":"test-token"}"#)
            .unwrap()
    }

    fn provider(server: &MockServer) -> Cloudflare {
        Cloudflare::with_base_url(&crate::providers::harness::client(), &server.uri())
    }

    #[tokio::test]
    async fn reads_the_result_envelope_and_structured_rows() {
        let server = MockServer::start().await;
        ok(&server, "GET", "/zones", scenario("cloudflare", "zones")).await;
        ok(
            &server,
            "GET",
            &format!("/zones/{ZONE}/dns_records"),
            scenario("cloudflare", "records"),
        )
        .await;
        let config = config();
        let provider = provider(&server);

        let zones = provider.list_zones(&config).await.unwrap();
        assert_eq!(zones.len(), 1);
        assert_eq!(zones[0].id, ZONE);
        assert_eq!(zones[0].name, "example.com");
        assert_eq!(zones[0].status.as_deref(), Some("active"));
        assert_eq!(
            header(&server, "GET", "/zones", "Authorization").await,
            Some("Bearer test-token".to_string())
        );

        // Every structured type has to read back as the single string the form showed.
        let rows = provider.list_records(&config, ZONE).await.unwrap();
        assert_eq!(rows[0].content, "192.0.2.1");
        assert_eq!(rows[0].proxied, Some(false));
        assert_eq!(rows[1].content, "10 smtp.example.com");
        assert_eq!(rows[1].ttl, Some(1));
        assert_eq!(rows[2].content, "10 60 5060 sip.example.com");
        assert_eq!(rows[3].content, r#"0 issue "letsencrypt.org""#);
    }

    #[tokio::test]
    async fn an_mx_create_splits_priority_out_of_the_value() {
        let server = MockServer::start().await;
        ok(
            &server,
            "POST",
            &format!("/zones/{ZONE}/dns_records"),
            scenario("cloudflare", "created"),
        )
        .await;
        let record = provider(&server)
            .create_record(
                &config(),
                &RecordInput {
                    zone_id: ZONE.into(),
                    record_type: "MX".into(),
                    name: "mail.example.com".into(),
                    content: "10 smtp.example.com".into(),
                    ttl: Some(300),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        assert_eq!(
            sent(&server, "POST", &format!("/zones/{ZONE}/dns_records")).await,
            vec![json!({
                "type": "MX",
                "name": "mail.example.com",
                "content": "smtp.example.com",
                "priority": 10,
                "ttl": 300,
            })]
        );
        assert_eq!(record.id, "rec-new");
        assert_eq!(record.content, "10 smtp.example.com");
    }

    #[tokio::test]
    async fn proxied_only_rides_along_for_proxyable_types() {
        let server = MockServer::start().await;
        let route = format!("/zones/{ZONE}/dns_records");
        ok(&server, "POST", &route, scenario("cloudflare", "created")).await;
        let provider = provider(&server);
        let config = config();
        for record_type in ["A", "MX"] {
            provider
                .create_record(
                    &config,
                    &RecordInput {
                        zone_id: ZONE.into(),
                        record_type: record_type.into(),
                        name: "www.example.com".into(),
                        content: "192.0.2.1".into(),
                        ttl: Some(300),
                        proxied: Some(true),
                    },
                )
                .await
                .unwrap();
        }
        let bodies = sent(&server, "POST", &route).await;
        assert_eq!(bodies[0]["proxied"], json!(true));
        assert!(bodies[1].get("proxied").is_none());
    }

    #[tokio::test]
    async fn a_delete_without_an_id_is_resolved_from_the_listing() {
        let server = MockServer::start().await;
        let listing = format!("/zones/{ZONE}/dns_records");
        let deleted = format!("/zones/{ZONE}/dns_records/rec-a");
        ok(&server, "GET", &listing, scenario("cloudflare", "records")).await;
        ok(
            &server,
            "DELETE",
            &deleted,
            scenario("cloudflare", "deleted"),
        )
        .await;
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
        assert_eq!(hits(&server, "DELETE", &deleted).await, 1);
    }

    #[tokio::test]
    async fn a_success_false_body_is_a_failure_even_at_http_200() {
        let server = MockServer::start().await;
        ok(&server, "GET", "/zones", scenario("cloudflare", "rejected")).await;
        let error = provider(&server)
            .test_connection(&config())
            .await
            .unwrap_err()
            .to_string();
        assert!(error.contains("Invalid API Token"), "{error}");
        // The chain is where Cloudflare puts the part that actually explains a rejection.
        assert!(
            error.contains("could not read any zones with this token"),
            "{error}"
        );
    }

    #[tokio::test]
    async fn an_error_status_keeps_the_providers_own_words() {
        let server = MockServer::start().await;
        respond(
            &server,
            "GET",
            "/zones",
            403,
            scenario("cloudflare", "rejected"),
        )
        .await;
        let error = provider(&server)
            .list_zones(&config())
            .await
            .unwrap_err()
            .to_string();
        assert!(error.contains("status 403"), "{error}");
        assert!(error.contains("Invalid API Token"), "{error}");
    }
}
