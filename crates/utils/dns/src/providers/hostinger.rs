use async_trait::async_trait;
use serde_json::{Value, json};

use crate::config::{HostingerConfig, ProviderConfig};
use crate::error::DnsError;
use crate::http::Http;
use crate::provider::{DnsProvider, find_row, row_matches};
use crate::records::{COMMON_TYPES, ensure_type, parse_mx, to_fqdn};
use crate::types::{Record, RecordInput, RecordTarget, Zone};

const DEFAULT_API: &str = "https://developers.hostinger.com";
const PAGE_SIZE: u32 = 100;
const DEFAULT_TTL: u32 = 300;

/// Hostinger DNS. Its zone API has no record ids: a zone is a list of groups keyed by
/// `name` + `type`, each holding one or more values, so a row is addressed as
/// `name:type:content` and an edit is a `PUT` of the whole group.
pub struct Hostinger {
    http: Http,
}

impl Hostinger {
    pub fn new(client: &reqwest::Client) -> Self {
        Self::with_base_url(client, DEFAULT_API)
    }

    pub fn with_base_url(client: &reqwest::Client, base_url: &str) -> Self {
        Self {
            http: Http::new(client, base_url),
        }
    }

    fn config<'a>(&self, config: &'a ProviderConfig) -> Result<&'a HostingerConfig, DnsError> {
        config.hostinger()
    }

    async fn fetch_groups(
        &self,
        credentials: &HostingerConfig,
        zone_id: &str,
    ) -> Result<Vec<Group>, DnsError> {
        let path = format!("/api/dns/v1/zones/{zone_id}");
        let response = self
            .http
            .get(&path)
            .bearer_auth(&credentials.api_token)
            .send()
            .await?;
        let body = self
            .http
            .json(response, "Hostinger", "list records")
            .await?;
        Ok(rows_of(body).into_iter().map(Group::from_value).collect())
    }

    async fn put(
        &self,
        credentials: &HostingerConfig,
        zone_id: &str,
        overwrite: bool,
        groups: &[Group],
        operation: &str,
    ) -> Result<(), DnsError> {
        let path = format!("/api/dns/v1/zones/{zone_id}");
        let payload = json!({
            "overwrite": overwrite,
            "zone": groups.iter().map(Group::to_value).collect::<Vec<_>>(),
        });
        let response = self
            .http
            .put(&path)
            .bearer_auth(&credentials.api_token)
            .json(&payload)
            .send()
            .await?;
        self.http.json(response, "Hostinger", operation).await?;
        Ok(())
    }

    async fn drop_group(
        &self,
        credentials: &HostingerConfig,
        zone_id: &str,
        group: &Group,
    ) -> Result<(), DnsError> {
        let path = format!("/api/dns/v1/zones/{zone_id}");
        let payload = json!({ "filters": [{ "name": group.name, "type": group.record_type }] });
        let response = self
            .http
            .delete(&path)
            .bearer_auth(&credentials.api_token)
            .json(&payload)
            .send()
            .await?;
        self.http
            .json(response, "Hostinger", "delete record")
            .await?;
        Ok(())
    }

    /// Which existing group and value a request points at. The synthetic id is authoritative
    /// because the panel got it from the listing; a hand-built request still works through
    /// the same fields.
    async fn locate(
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
        let (name, record_type, content) = split_id(&id);
        if name.is_empty() || record_type.is_empty() {
            return Err(DnsError::provider(format!(
                "Hostinger: cannot address record {id:?}, expected \"name:type:content\""
            )));
        }
        Ok((name, record_type, content))
    }
}

#[async_trait]
impl DnsProvider for Hostinger {
    async fn test_connection(&self, config: &ProviderConfig) -> Result<(), DnsError> {
        let credentials = self.config(config)?;
        let response = self
            .http
            .get("/api/domains/v1/portfolio")
            .query(&[("page", 1)])
            .bearer_auth(&credentials.api_token)
            .send()
            .await?;
        self.http
            .json(response, "Hostinger", "verify credentials")
            .await?;
        Ok(())
    }

    async fn list_zones(&self, config: &ProviderConfig) -> Result<Vec<Zone>, DnsError> {
        let credentials = self.config(config)?;
        let mut zones = Vec::new();
        let mut page = 1;
        loop {
            let response = self
                .http
                .get("/api/domains/v1/portfolio")
                .query(&[("page", page), ("per_page", PAGE_SIZE)])
                .bearer_auth(&credentials.api_token)
                .send()
                .await?;
            let body = self.http.json(response, "Hostinger", "list zones").await?;
            let rows = rows_of(body);
            let count = rows.len();
            for row in rows {
                let name = first_text(&row, &["domain", "name"]);
                if !name.is_empty() {
                    zones.push(Zone {
                        id: name.clone(),
                        name,
                        status: None,
                    });
                }
            }
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
        let groups = self.fetch_groups(credentials, zone_id).await?;
        let mut records = Vec::new();
        for group in groups {
            for value in &group.values {
                let content = display_value(&group.record_type, value);
                records.push(Record {
                    id: format!("{}:{}:{content}", group.name, group.record_type),
                    zone_id: zone_id.to_string(),
                    record_type: group.record_type.clone(),
                    name: to_fqdn(&group.name, zone_id),
                    ttl: Some(group.ttl),
                    content,
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
        let group = group_from(input, input.ttl.unwrap_or(DEFAULT_TTL))?;
        self.put(
            credentials,
            &input.zone_id,
            false,
            std::slice::from_ref(&group),
            "create record",
        )
        .await?;
        Ok(record_from_group(&group, &input.zone_id))
    }

    async fn update_record(
        &self,
        config: &ProviderConfig,
        target: &RecordTarget,
        next: &RecordInput,
    ) -> Result<Record, DnsError> {
        let credentials = self.config(config)?;
        let (old_name, old_type, old_value) = self.locate(config, target).await?;
        let groups = self.fetch_groups(credentials, &next.zone_id).await?;
        let wanted = group_from(next, next.ttl.unwrap_or(DEFAULT_TTL))?;
        let renamed =
            old_name != wanted.name || !old_type.eq_ignore_ascii_case(&wanted.record_type);

        if renamed {
            // The new group is written first so a rename never leaves the zone without the
            // record, then the old value is taken out of the group it came from.
            self.put(
                credentials,
                &next.zone_id,
                true,
                std::slice::from_ref(&wanted),
                "update record",
            )
            .await?;
            if let Some(old) = groups
                .iter()
                .find(|group| group.name == old_name && group.record_type == old_type)
            {
                self.remove_value(credentials, &next.zone_id, old, &old_value)
                    .await?;
            }
        } else {
            let ttl = next.ttl.unwrap_or(DEFAULT_TTL);
            let mut merged = groups
                .iter()
                .find(|group| group.name == wanted.name && group.record_type == wanted.record_type)
                .cloned()
                .unwrap_or_else(|| Group {
                    name: wanted.name.clone(),
                    record_type: wanted.record_type.clone(),
                    ttl,
                    values: Vec::new(),
                });
            merged.ttl = ttl;
            let record_type = merged.record_type.clone();
            let replacement = wanted.values[0].clone();
            match merged
                .values
                .iter_mut()
                .find(|value| display_value(&record_type, value) == old_value)
            {
                // Replacing one value of a multi-value group has to keep the others, because
                // the PUT writes the whole group.
                Some(slot) => *slot = replacement,
                None => merged.values.push(replacement),
            }
            self.put(credentials, &next.zone_id, true, &[merged], "update record")
                .await?;
        }
        Ok(record_from_group(&wanted, &next.zone_id))
    }

    async fn delete_record(
        &self,
        config: &ProviderConfig,
        target: &RecordTarget,
    ) -> Result<(), DnsError> {
        let credentials = self.config(config)?;
        let (name, record_type, value) = self.locate(config, target).await?;
        let groups = self.fetch_groups(credentials, &target.zone_id).await?;
        let Some(group) = groups.iter().find(|group| {
            group.name == name && group.record_type.eq_ignore_ascii_case(&record_type)
        }) else {
            return Err(DnsError::StaleRecord(name));
        };
        self.remove_value(credentials, &target.zone_id, group, &value)
            .await
    }
}

impl Hostinger {
    /// Hostinger can only delete a whole `name` + `type` group, so a group that still holds
    /// other values is rewritten without the row the panel selected — otherwise deleting one
    /// of two A records would silently drop the other.
    async fn remove_value(
        &self,
        credentials: &HostingerConfig,
        zone_id: &str,
        group: &Group,
        value: &str,
    ) -> Result<(), DnsError> {
        if value.trim().is_empty() {
            return self.drop_group(credentials, zone_id, group).await;
        }
        let record_type = group.record_type.clone();
        let remaining: Vec<Value> = group
            .values
            .iter()
            .filter(|entry| display_value(&record_type, entry) != value)
            .cloned()
            .collect();
        if remaining.is_empty() {
            return self.drop_group(credentials, zone_id, group).await;
        }
        if remaining.len() == group.values.len() {
            return Err(DnsError::StaleRecord(group.name.clone()));
        }
        let mut shrunk = group.clone();
        shrunk.values = remaining;
        self.put(credentials, zone_id, true, &[shrunk], "delete record")
            .await
    }
}

#[derive(Debug, Clone)]
struct Group {
    name: String,
    record_type: String,
    ttl: u32,
    values: Vec<Value>,
}

impl Group {
    fn from_value(row: Value) -> Self {
        Self {
            name: first_text(&row, &["name"]),
            record_type: first_text(&row, &["type"]).to_ascii_uppercase(),
            ttl: row["ttl"].as_u64().map(|v| v as u32).unwrap_or(DEFAULT_TTL),
            values: row["records"].as_array().cloned().unwrap_or_default(),
        }
    }

    fn to_value(&self) -> Value {
        json!({
            "name": self.name,
            "type": self.record_type,
            "ttl": self.ttl,
            "records": self.values,
        })
    }
}

fn group_from(input: &RecordInput, ttl: u32) -> Result<Group, DnsError> {
    let record_type = ensure_type("Hostinger", &input.record_type, COMMON_TYPES)?;
    let (content, priority) = stored_value(&record_type, &input.content)?;
    let mut value = json!({ "content": content });
    if let Some(priority) = priority {
        value["priority"] = json!(priority);
    }
    Ok(Group {
        name: to_fqdn(&input.name, &input.zone_id),
        record_type,
        ttl,
        values: vec![value],
    })
}

/// Hostinger keeps an MX preference beside the host, so the panel's `"10 mail.example.com"`
/// splits into the two fields and [`display_value`] joins them again.
fn stored_value(record_type: &str, content: &str) -> Result<(String, Option<u16>), DnsError> {
    if record_type == "MX" {
        let mx = parse_mx(content)?;
        return Ok((mx.host, Some(mx.priority)));
    }
    Ok((content.trim().to_string(), None))
}

fn display_value(record_type: &str, value: &Value) -> String {
    let content = value["content"].as_str().unwrap_or_default().to_string();
    if record_type == "MX"
        && let Some(priority) = value["priority"].as_u64()
    {
        return format!("{priority} {content}");
    }
    content
}

fn record_from_group(group: &Group, zone_id: &str) -> Record {
    let content = group
        .values
        .first()
        .map(|value| display_value(&group.record_type, value))
        .unwrap_or_default();
    Record {
        id: format!("{}:{}:{content}", group.name, group.record_type),
        zone_id: zone_id.to_string(),
        record_type: group.record_type.clone(),
        name: group.name.clone(),
        content,
        ttl: Some(group.ttl),
        proxied: None,
    }
}

/// A Hostinger row id is `name:type:content`; the content may itself contain colons (a CAA or
/// TXT value), so only the first two fields are split off.
fn split_id(id: &str) -> (String, String, String) {
    let mut parts = id.splitn(3, ':');
    (
        parts.next().unwrap_or_default().to_string(),
        parts.next().unwrap_or_default().to_string(),
        parts.next().unwrap_or_default().to_string(),
    )
}

/// Hostinger returns either a bare array or an object with a `data` list, depending on the
/// endpoint version, and both mean the same thing here.
fn rows_of(body: Value) -> Vec<Value> {
    match body {
        Value::Array(rows) => rows,
        body => body["data"].as_array().cloned().unwrap_or_default(),
    }
}

fn first_text(row: &Value, keys: &[&str]) -> String {
    for key in keys {
        let text = row[*key].as_str().unwrap_or_default().trim().to_string();
        if !text.is_empty() {
            return text;
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DnsProviderKind, ProviderConfig};
    use crate::providers::harness::{client, hits, ok, respond, scenario, sent};
    use serde_json::json;
    use wiremock::MockServer;

    const ZONE: &str = "example.com";
    const RECORDS: &str = "/api/dns/v1/zones/example.com";
    const PORTFOLIO: &str = "/api/domains/v1/portfolio";

    fn config() -> ProviderConfig {
        ProviderConfig::from_json(DnsProviderKind::Hostinger, r#"{"apiToken":"test-token"}"#)
            .unwrap()
    }

    fn provider(server: &MockServer) -> Hostinger {
        Hostinger::with_base_url(&client(), &server.uri())
    }

    #[tokio::test]
    async fn zones_come_from_the_portfolio_in_either_envelope() {
        let bare = MockServer::start().await;
        ok(&bare, "GET", PORTFOLIO, scenario("hostinger", "portfolio")).await;
        let zones = provider(&bare).list_zones(&config()).await.unwrap();
        assert_eq!(
            zones
                .iter()
                .map(|zone| zone.name.as_str())
                .collect::<Vec<_>>(),
            ["example.com", "second.com"]
        );
        assert_eq!(zones[0].id, "example.com");

        // Some endpoints wrap the list in `data`; the same page has to read the same way.
        let wrapped = MockServer::start().await;
        ok(
            &wrapped,
            "GET",
            PORTFOLIO,
            scenario("hostinger", "portfolio_data"),
        )
        .await;
        let zones = provider(&wrapped).list_zones(&config()).await.unwrap();
        assert_eq!(zones.len(), 1);
    }

    #[tokio::test]
    async fn one_row_per_value_and_the_mx_preference_joined_back() {
        let server = MockServer::start().await;
        ok(&server, "GET", RECORDS, scenario("hostinger", "zone")).await;
        let rows = provider(&server)
            .list_records(&config(), ZONE)
            .await
            .unwrap();

        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].id, "example.com:A:192.0.2.1");
        assert_eq!(rows[1].id, "example.com:A:192.0.2.2");
        assert_eq!(rows[2].record_type, "MX");
        assert_eq!(rows[2].content, "10 mail.example.com");
        assert_eq!(rows[2].name, "smtp.example.com");
        assert_eq!(rows[2].ttl, Some(300));
    }

    #[tokio::test]
    async fn a_create_puts_one_group_without_overwriting() {
        let server = MockServer::start().await;
        ok(&server, "PUT", RECORDS, scenario("hostinger", "ack")).await;
        let record = provider(&server)
            .create_record(
                &config(),
                &RecordInput {
                    zone_id: ZONE.into(),
                    record_type: "A".into(),
                    name: "www".into(),
                    content: "192.0.2.3".into(),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        assert_eq!(
            sent(&server, "PUT", RECORDS).await,
            vec![json!({
                "overwrite": false,
                "zone": [{ "name": "www.example.com", "type": "A", "ttl": 300,
                           "records": [{ "content": "192.0.2.3" }] }],
            })]
        );
        assert_eq!(record.id, "www.example.com:A:192.0.2.3");
    }

    #[tokio::test]
    async fn deleting_one_value_of_a_group_keeps_its_sibling() {
        let server = MockServer::start().await;
        ok(&server, "GET", RECORDS, scenario("hostinger", "zone")).await;
        ok(&server, "PUT", RECORDS, scenario("hostinger", "ack")).await;
        provider(&server)
            .delete_record(
                &config(),
                &RecordTarget {
                    zone_id: ZONE.into(),
                    id: "example.com:A:192.0.2.1".into(),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        // Hostinger can only replace a whole group, so the untouched value has to ride along.
        assert_eq!(
            sent(&server, "PUT", RECORDS).await,
            vec![json!({
                "overwrite": true,
                "zone": [{ "name": "example.com", "type": "A", "ttl": 300,
                           "records": [{ "content": "192.0.2.2" }] }],
            })]
        );
        assert_eq!(hits(&server, "DELETE", RECORDS).await, 0);
    }

    #[tokio::test]
    async fn deleting_the_last_value_of_a_group_drops_the_group() {
        let server = MockServer::start().await;
        ok(&server, "GET", RECORDS, scenario("hostinger", "zone")).await;
        ok(&server, "DELETE", RECORDS, scenario("hostinger", "ack")).await;
        provider(&server)
            .delete_record(
                &config(),
                &RecordTarget {
                    zone_id: ZONE.into(),
                    id: "smtp.example.com:MX:10 mail.example.com".into(),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        assert_eq!(
            sent(&server, "DELETE", RECORDS).await,
            vec![json!({ "filters": [{ "name": "smtp.example.com", "type": "MX" }] })]
        );
        assert_eq!(hits(&server, "PUT", RECORDS).await, 0);
    }

    #[tokio::test]
    async fn a_rename_writes_the_new_group_before_clearing_the_old_one() {
        let server = MockServer::start().await;
        ok(&server, "GET", RECORDS, scenario("hostinger", "zone")).await;
        ok(&server, "PUT", RECORDS, scenario("hostinger", "ack")).await;
        ok(&server, "DELETE", RECORDS, scenario("hostinger", "ack")).await;
        provider(&server)
            .update_record(
                &config(),
                &RecordTarget {
                    zone_id: ZONE.into(),
                    id: "example.com:A:192.0.2.1".into(),
                    ..Default::default()
                },
                &RecordInput {
                    zone_id: ZONE.into(),
                    record_type: "CNAME".into(),
                    name: "www".into(),
                    content: "example.com".into(),
                    ttl: Some(600),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        let writes = sent(&server, "PUT", RECORDS).await;
        assert_eq!(
            writes[0],
            json!({
                "overwrite": true,
                "zone": [{ "name": "www.example.com", "type": "CNAME", "ttl": 600,
                           "records": [{ "content": "example.com" }] }],
            })
        );
        // The A group keeps its other value; only the moved one comes out.
        assert_eq!(
            writes[1],
            json!({
                "overwrite": true,
                "zone": [{ "name": "example.com", "type": "A", "ttl": 300,
                           "records": [{ "content": "192.0.2.2" }] }],
            })
        );
    }

    #[tokio::test]
    async fn a_rejected_token_surfaces_the_providers_message() {
        let server = MockServer::start().await;
        respond(
            &server,
            "GET",
            PORTFOLIO,
            401,
            scenario("hostinger", "unauthorized"),
        )
        .await;
        let error = provider(&server)
            .test_connection(&config())
            .await
            .unwrap_err()
            .to_string();
        assert!(error.contains("Invalid API token"), "{error}");
        assert!(error.contains("status 401"), "{error}");
    }
}
