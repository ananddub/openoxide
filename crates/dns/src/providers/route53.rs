use async_trait::async_trait;
use aws_credential_types::Credentials;
use aws_sdk_route53::{
    Client as Route53Client,
    types::{Change, ChangeAction, ChangeBatch, ResourceRecord, ResourceRecordSet, RrType},
};

use crate::config::{ProviderConfig, Route53Config};
use crate::error::DnsError;
use crate::provider::DnsProvider;
use crate::records::{COMMON_TYPES, bare, ensure_type, quote, unquote};
use crate::types::{Record, RecordInput, RecordTarget, Zone};

/// Route53 has no per-value endpoint: a record is an rrset (one name + type holding every
/// value), so a mutation resubmits the whole group. The AWS SDK is kept rather than a
/// hand-rolled client because it owns SigV4, the XML wire format and pagination.
pub struct Route53;

impl Route53 {
    pub fn new() -> Self {
        Self
    }

    fn client(config: &ProviderConfig) -> Result<Route53Client, DnsError> {
        let Route53Config {
            access_key_id,
            secret_access_key,
        } = config.route53()?;
        let credentials = Credentials::new(
            access_key_id,
            secret_access_key,
            None,
            None,
            "openoxide-dns",
        );
        Ok(Route53Client::from_conf(
            aws_sdk_route53::Config::builder()
                // Hosted zones and their record APIs are global and only answer in us-east-1.
                .region(aws_sdk_route53::config::Region::new("us-east-1"))
                .credentials_provider(credentials)
                .behavior_version_latest()
                .build(),
        ))
    }
}

impl Default for Route53 {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DnsProvider for Route53 {
    async fn test_connection(&self, config: &ProviderConfig) -> Result<(), DnsError> {
        Self::client(config)?
            .list_hosted_zones()
            .max_items(1)
            .send()
            .await
            .map_err(|error| {
                DnsError::provider(format!("Route53: authentication failed: {error}"))
            })?;
        Ok(())
    }

    async fn list_zones(&self, config: &ProviderConfig) -> Result<Vec<Zone>, DnsError> {
        let client = Self::client(config)?;
        let mut zones = Vec::new();
        let mut marker: Option<String> = None;
        loop {
            let mut request = client.list_hosted_zones().max_items(100);
            if let Some(value) = marker.as_deref() {
                request = request.marker(value);
            }
            let response = request.send().await.map_err(|error| {
                DnsError::provider(format!("Route53: list zones failed: {error}"))
            })?;
            for zone in response.hosted_zones() {
                zones.push(Zone {
                    id: zone.id().trim_start_matches("/hostedzone/").to_string(),
                    name: bare(zone.name()),
                    status: None,
                });
            }
            marker = if response.is_truncated() {
                response.next_marker().map(str::to_string)
            } else {
                None
            };
            if marker.is_none() {
                break;
            }
        }
        Ok(zones)
    }

    async fn list_records(
        &self,
        config: &ProviderConfig,
        zone_id: &str,
    ) -> Result<Vec<Record>, DnsError> {
        let client = Self::client(config)?;
        let mut out = Vec::new();
        let mut next_name: Option<String> = None;
        let mut next_type: Option<RrType> = None;
        loop {
            let mut request = client.list_resource_record_sets().hosted_zone_id(zone_id);
            if let Some(name) = next_name.as_deref() {
                request = request.start_record_name(name);
            }
            if let Some(kind) = next_type.clone() {
                request = request.start_record_type(kind);
            }
            let response = request.send().await.map_err(|error| {
                DnsError::provider(format!("Route53: list records failed: {error}"))
            })?;
            for record in response.resource_record_sets() {
                let name = bare(record.name());
                let kind = record.r#type().as_str().to_string();
                let ttl = record.ttl().map(|value| value as u32);
                // One row per value. Route53 stores an rrset as a group, but the panel edits
                // and deletes single values, so a collapsed row could not be pointed at.
                for value in record.resource_records() {
                    out.push(Record {
                        id: format!("{kind}:{name}:{}", value.value()),
                        zone_id: zone_id.to_string(),
                        record_type: kind.clone(),
                        name: name.clone(),
                        content: display_value(&kind, value.value()),
                        ttl,
                        proxied: None,
                    });
                }
            }
            next_name = response.next_record_name().map(str::to_string);
            next_type = response.next_record_type().cloned();
            if next_name.is_none() {
                break;
            }
        }
        Ok(out)
    }

    async fn create_record(
        &self,
        config: &ProviderConfig,
        input: &RecordInput,
    ) -> Result<Record, DnsError> {
        let client = Self::client(config)?;
        let record_type = record_type(&input.record_type)?;
        let existing = fetch_rrset(&client, &input.zone_id, &input.name, &record_type).await?;
        // UPSERT overwrites the group, so the values already there have to ride along.
        let rrset = match existing {
            Some(rrset) => rrset.with(&input.content, input.ttl)?,
            None => fresh_rrset(&record_type, input),
        };
        apply(
            &client,
            &input.zone_id,
            vec![rrset.change(ChangeAction::Upsert)?],
            "create",
        )
        .await?;
        Ok(row(&record_type, input, input.ttl))
    }

    async fn update_record(
        &self,
        config: &ProviderConfig,
        target: &RecordTarget,
        next: &RecordInput,
    ) -> Result<Record, DnsError> {
        require_value_fields("update", target)?;
        let client = Self::client(config)?;
        let old_type = record_type(&target.record_type)?;
        let new_type = record_type(&next.record_type)?;
        let old = fetch_rrset(&client, &target.zone_id, &target.name, &old_type)
            .await?
            .ok_or_else(|| DnsError::StaleRecord(target.name.clone()))?;

        let mut changes = Vec::new();
        let same_rrset = old.kind == new_type && bare(&old.name) == bare(&next.name);
        let previous_ttl = old.ttl;

        if same_rrset {
            // Dropping the old value and writing the new one back into the same group is one
            // UPSERT; the untouched siblings ride along.
            let rebuilt = old
                .without(&target.content)?
                .with(&next.content, next.ttl)?;
            changes.push(rebuilt.change(ChangeAction::Upsert)?);
        } else {
            let reduced = old.without(&target.content)?;
            changes.push(if reduced.values.is_empty() {
                old.change(ChangeAction::Delete)?
            } else {
                reduced.change(ChangeAction::Upsert)?
            });
            let destination = fetch_rrset(&client, &next.zone_id, &next.name, &new_type).await?;
            let entered = match destination {
                Some(rrset) => rrset.with(&next.content, next.ttl)?,
                None => fresh_rrset(&new_type, next),
            };
            changes.push(entered.change(ChangeAction::Upsert)?);
        }

        apply(&client, &target.zone_id, changes, "update").await?;
        Ok(row(
            &new_type,
            next,
            Some(next.ttl.unwrap_or(previous_ttl.unwrap_or(300))),
        ))
    }

    async fn delete_record(
        &self,
        config: &ProviderConfig,
        target: &RecordTarget,
    ) -> Result<(), DnsError> {
        require_value_fields("delete", target)?;
        let client = Self::client(config)?;
        let kind = record_type(&target.record_type)?;
        let rrset = fetch_rrset(&client, &target.zone_id, &target.name, &kind)
            .await?
            .ok_or_else(|| DnsError::StaleRecord(target.name.clone()))?;
        let reduced = rrset.without(&target.content)?;
        // DELETE only accepts a group listing every current value; removing one member from a
        // larger group means writing back the rest.
        let change = if reduced.values.is_empty() {
            rrset.change(ChangeAction::Delete)?
        } else {
            reduced.change(ChangeAction::Upsert)?
        };
        apply(&client, &target.zone_id, vec![change], "delete").await
    }
}

/// Route53 cannot address a single value inside a group from a name alone, so the type and
/// the exact value have to come back with the request.
fn require_value_fields(what: &str, target: &RecordTarget) -> Result<(), DnsError> {
    if !target.has_type() {
        return Err(DnsError::provider(format!(
            "Route53 {what} needs the record type of the value being changed"
        )));
    }
    if !target.has_content() {
        return Err(DnsError::provider(format!(
            "Route53 {what} needs the exact value — this record set holds several"
        )));
    }
    Ok(())
}

fn record_type(kind: &str) -> Result<String, DnsError> {
    ensure_type("Route53", kind, COMMON_TYPES)
}

fn rr_type(kind: &str) -> Result<RrType, DnsError> {
    Ok(match kind {
        "A" => RrType::A,
        "AAAA" => RrType::Aaaa,
        "CNAME" => RrType::Cname,
        "NS" => RrType::Ns,
        "MX" => RrType::Mx,
        "TXT" => RrType::Txt,
        "SRV" => RrType::Srv,
        "CAA" => RrType::Caa,
        other => {
            return Err(DnsError::provider(format!(
                "Route53: unsupported record type '{other}'"
            )));
        }
    })
}

/// Route53 stores TXT and CAA values wrapped in double quotes; the panel shows and accepts
/// the bare string, so quotes come off on the way out and go back on the way in.
fn display_value(kind: &str, value: &str) -> String {
    match kind {
        "TXT" | "CAA" => unquote(value),
        _ => value.trim().to_string(),
    }
}

fn stored_value(kind: &str, value: &str) -> String {
    match kind {
        "TXT" | "CAA" => quote(value),
        _ => value.trim().to_string(),
    }
}

/// The row the panel gets back after a write, keyed the same way a listed row is.
fn row(kind: &str, input: &RecordInput, ttl: Option<u32>) -> Record {
    Record {
        id: format!("{kind}:{}:{}", bare(&input.name), input.content),
        zone_id: input.zone_id.clone(),
        record_type: kind.to_string(),
        name: bare(&input.name),
        content: input.content.clone(),
        ttl: Some(ttl.unwrap_or(300)),
        proxied: None,
    }
}

#[derive(Clone, Debug)]
struct Route53Rrset {
    name: String,
    kind: String,
    ttl: Option<u32>,
    values: Vec<String>,
    /// Weighted / latency / failover record sets carry a set identifier, which an edit built
    /// without it would silently collapse into a plain record.
    routed: bool,
}

impl Route53Rrset {
    fn position_of(&self, value: &str) -> Option<usize> {
        let want = display_value(&self.kind, value);
        self.values
            .iter()
            .position(|value| display_value(&self.kind, value) == want)
    }

    fn without(&self, value: &str) -> Result<Self, DnsError> {
        if self.routed {
            return Err(DnsError::RoutedRecord(self.name.clone()));
        }
        let index = self
            .position_of(value)
            .ok_or_else(|| DnsError::StaleRecord(self.name.clone()))?;
        let mut copy = self.clone();
        copy.values.remove(index);
        Ok(copy)
    }

    fn with(&self, value: &str, ttl: Option<u32>) -> Result<Self, DnsError> {
        if self.routed {
            return Err(DnsError::RoutedRecord(self.name.clone()));
        }
        let encoded = stored_value(&self.kind, value);
        let mut copy = self.clone();
        if copy.position_of(&encoded).is_none() {
            copy.values.push(encoded);
        }
        copy.ttl = ttl.or(copy.ttl);
        Ok(copy)
    }

    fn into_set(self) -> Result<ResourceRecordSet, DnsError> {
        let mut builder = ResourceRecordSet::builder()
            .name(format!("{}.", self.name))
            .r#type(rr_type(&self.kind)?);
        if let Some(ttl) = self.ttl {
            builder = builder.ttl(ttl as i64);
        }
        for value in self.values {
            builder = builder.resource_records(
                ResourceRecord::builder()
                    .value(value)
                    .build()
                    .map_err(|error| DnsError::provider(error.to_string()))?,
            );
        }
        builder
            .build()
            .map_err(|error| DnsError::provider(error.to_string()))
    }

    fn change(self, action: ChangeAction) -> Result<Change, DnsError> {
        Change::builder()
            .action(action)
            .resource_record_set(self.into_set()?)
            .build()
            .map_err(|error| DnsError::provider(error.to_string()))
    }
}

/// A new value written to an rrset that does not exist yet.
fn fresh_rrset(kind: &str, input: &RecordInput) -> Route53Rrset {
    Route53Rrset {
        name: bare(&input.name),
        kind: kind.to_string(),
        ttl: Some(input.ttl.unwrap_or(300)),
        values: vec![stored_value(kind, &input.content)],
        routed: false,
    }
}

async fn fetch_rrset(
    client: &Route53Client,
    zone_id: &str,
    name: &str,
    kind: &str,
) -> Result<Option<Route53Rrset>, DnsError> {
    let response = client
        .list_resource_record_sets()
        .hosted_zone_id(zone_id)
        .start_record_name(format!("{}.", bare(name)))
        .start_record_type(rr_type(kind)?)
        .max_items(1)
        .send()
        .await
        .map_err(|error| DnsError::provider(format!("Route53: lookup failed: {error}")))?;
    Ok(response
        .resource_record_sets()
        .iter()
        .find(|set| bare(set.name()) == bare(name) && set.r#type().as_str() == kind)
        .map(|set| Route53Rrset {
            name: bare(set.name()),
            kind: set.r#type().as_str().to_string(),
            ttl: set.ttl().map(|value| value as u32),
            values: set
                .resource_records()
                .iter()
                .map(|value| value.value().to_string())
                .collect(),
            routed: set.set_identifier().is_some_and(|id| !id.is_empty()),
        }))
}

async fn apply(
    client: &Route53Client,
    zone_id: &str,
    changes: Vec<Change>,
    what: &str,
) -> Result<(), DnsError> {
    if changes.is_empty() {
        return Ok(());
    }
    let mut builder = ChangeBatch::builder();
    for change in changes {
        builder = builder.changes(change);
    }
    client
        .change_resource_record_sets()
        .hosted_zone_id(zone_id)
        .change_batch(
            builder
                .build()
                .map_err(|error| DnsError::provider(error.to_string()))?,
        )
        .send()
        .await
        .map_err(|error| DnsError::provider(format!("Route53: {what} failed: {error}")))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::records::{quote, unquote};

    /// The list call quotes TXT and CAA; the form shows and accepts the bare string, so a row
    /// read back has to encode to exactly what Route53 already stores.
    #[test]
    fn value_quotes_round_trip() {
        for pair in [("TXT", "\"v=spf1 -all\""), ("CAA", "\"0 issue x\"")] {
            let (kind, stored) = pair;
            let _ = kind;
            assert_eq!(unquote(stored), stored[1..stored.len() - 1]);
            assert_eq!(quote(unquote(stored).as_str()), stored);
            assert_eq!(quote(stored), stored);
        }
        assert_eq!(quote(r#"a"b"#), r#""a\"b""#);
        assert_eq!(unquote("192.0.2.1"), "192.0.2.1");
    }

    /// The group is the unit Route53 stores, so a mutation has to resubmit every value in it.
    fn rrset(kind: &str, values: &[&str]) -> Route53Rrset {
        Route53Rrset {
            name: "www.example.com".into(),
            kind: kind.into(),
            ttl: Some(300),
            values: values
                .iter()
                .map(|value| stored_value(kind, value))
                .collect(),
            routed: false,
        }
    }

    #[test]
    fn adding_a_value_keeps_its_siblings_and_skips_a_duplicate() {
        let group = rrset("A", &["192.0.2.1", "192.0.2.2"]);
        assert_eq!(group.with("192.0.2.3", Some(600)).unwrap().values.len(), 3);
        assert_eq!(group.with("192.0.2.1", None).unwrap().values, group.values);
        // A ttl the caller did not set leaves the stored one alone.
        assert_eq!(group.with("192.0.2.3", None).unwrap().ttl, Some(300));
        assert_eq!(group.with("192.0.2.3", Some(600)).unwrap().ttl, Some(600));
    }

    #[test]
    fn removing_a_value_finds_it_whichever_way_it_is_stored() {
        let group = rrset("TXT", &["v=spf1 -all"]);
        assert_eq!(group.position_of("v=spf1 -all"), Some(0));
        assert_eq!(group.position_of(r#""v=spf1 -all""#), Some(0));
        assert!(group.without("v=spf1 -all").unwrap().values.is_empty());
        // Emptying a group is what makes the caller send DELETE instead of an UPSERT.
        assert!(rrset("A", &["192.0.2.1"]).without("192.0.2.9").is_err());
    }

    #[test]
    fn a_value_that_is_no_longer_listed_is_a_stale_row() {
        let error = rrset("A", &["192.0.2.1"]).without("192.0.2.9").unwrap_err();
        assert!(error.is_stale_record(), "{error}");
    }

    #[test]
    fn a_routed_set_is_never_rewritten() {
        let mut group = rrset("A", &["192.0.2.1"]);
        group.routed = true;
        for error in [
            group.without("192.0.2.1").unwrap_err(),
            group.with("192.0.2.2", None).unwrap_err(),
        ] {
            assert!(matches!(error, DnsError::RoutedRecord(_)), "{error}");
            assert!(error.to_string().contains("Route53"), "{error}");
        }
    }

    #[test]
    fn a_group_is_sent_back_absolute_and_complete() {
        let set = rrset("MX", &["10 mail.example.com", "20 smtp.example.com"])
            .into_set()
            .unwrap();
        assert_eq!(set.name(), "www.example.com.");
        assert_eq!(set.r#type().as_str(), "MX");
        assert_eq!(set.ttl(), Some(300));
        assert_eq!(set.resource_records().len(), 2);
        let change = rrset("A", &["192.0.2.1"])
            .change(ChangeAction::Delete)
            .unwrap();
        assert_eq!(change.action(), &ChangeAction::Delete);
    }

    #[test]
    fn a_new_group_takes_the_default_ttl_and_the_encoded_value() {
        let fresh = fresh_rrset(
            "TXT",
            &RecordInput {
                zone_id: "Z1".into(),
                record_type: "TXT".into(),
                name: "www.example.com".into(),
                content: "note".into(),
                ttl: None,
                proxied: None,
            },
        );
        assert_eq!(fresh.values, vec![r#""note""#]);
        assert_eq!(fresh.ttl, Some(300));
        assert_eq!(bare(&fresh.name), "www.example.com");
    }
}
