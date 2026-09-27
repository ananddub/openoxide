use super::super::{
    DnsRecordDto, DnsRecordTargetDto, DnsService, DnsServiceError, UpsertDnsRecordDto,
};
use super::route53::{api_value, plain_value};
use aws_sdk_route53::{
    Client as Route53Client,
    types::{Change, ChangeAction, ChangeBatch, ResourceRecord, ResourceRecordSet, RrType},
};

fn provider_error(message: impl Into<String>) -> DnsServiceError {
    DnsServiceError::ProviderError(message.into())
}

fn rr_type(kind: &str) -> Result<RrType, DnsServiceError> {
    Ok(match kind {
        "A" => RrType::A,
        "AAAA" => RrType::Aaaa,
        "CNAME" => RrType::Cname,
        "NS" => RrType::Ns,
        "MX" => RrType::Mx,
        "TXT" => RrType::Txt,
        "SRV" => RrType::Srv,
        "CAA" => RrType::Caa,
        _ => return Err(provider_error("Unsupported record type for Route53")),
    })
}

fn bare(name: &str) -> String {
    name.trim().trim_end_matches('.').to_string()
}

fn stale(name: &str) -> DnsServiceError {
    provider_error(format!(
        "record '{name}' is no longer listed by this provider — refresh the record list"
    ))
}

/// One Route53 rrset: the (name, type) group holding every value written to it. Route53
/// has no per-value endpoint, so a mutation resubmits the whole group.
#[derive(Clone, Debug)]
struct Route53Rrset {
    name: String,
    kind: String,
    ttl: Option<u32>,
    values: Vec<String>,
    /// Weighted / latency / failover record sets carry a set identifier, which an edit
    /// built without it would silently collapse into a plain record.
    routed: bool,
}

impl Route53Rrset {
    fn position_of(&self, value: &str) -> Option<usize> {
        let want = plain_value(&self.kind, value);
        self.values
            .iter()
            .position(|value| plain_value(&self.kind, value) == want)
    }

    fn without(&self, value: &str) -> Result<Self, DnsServiceError> {
        if self.routed {
            return Err(routed_error(&self.name));
        }
        let index = self.position_of(value).ok_or_else(|| stale(&self.name))?;
        let mut copy = self.clone();
        copy.values.remove(index);
        Ok(copy)
    }

    fn with(&self, value: &str, ttl: Option<u32>) -> Result<Self, DnsServiceError> {
        if self.routed {
            return Err(routed_error(&self.name));
        }
        let encoded = api_value(&self.kind, value);
        let mut copy = self.clone();
        if copy.position_of(&encoded).is_none() {
            copy.values.push(encoded);
        }
        copy.ttl = ttl.or(copy.ttl);
        Ok(copy)
    }

    fn into_set(self) -> Result<ResourceRecordSet, DnsServiceError> {
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
                    .map_err(|e| provider_error(e.to_string()))?,
            );
        }
        builder.build().map_err(|e| provider_error(e.to_string()))
    }

    fn change(self, action: ChangeAction) -> Result<Change, DnsServiceError> {
        Change::builder()
            .action(action)
            .resource_record_set(self.into_set()?)
            .build()
            .map_err(|e| provider_error(e.to_string()))
    }
}

fn routed_error(name: &str) -> DnsServiceError {
    provider_error(format!(
        "'{name}' is part of a routing-policy record set — edit it in Route53"
    ))
}

async fn fetch_rrset(
    client: &Route53Client,
    zone_id: &str,
    name: &str,
    kind: &str,
) -> Result<Option<Route53Rrset>, DnsServiceError> {
    let response = client
        .list_resource_record_sets()
        .hosted_zone_id(zone_id)
        .start_record_name(format!("{}.", bare(name)))
        .start_record_type(rr_type(kind)?)
        .max_items(1)
        .send()
        .await
        .map_err(|e| provider_error(format!("Route53 lookup failed: {e}")))?;
    Ok(response
        .resource_record_sets()
        .iter()
        .find(|set| {
            bare(set.name()) == bare(name) && set.r#type().as_str().eq_ignore_ascii_case(kind)
        })
        .map(|set| Route53Rrset {
            name: bare(set.name()),
            kind: set.r#type().as_str().to_string(),
            ttl: set.ttl().map(|v| v as u32),
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
) -> Result<(), DnsServiceError> {
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
        .change_batch(builder.build().map_err(|e| provider_error(e.to_string()))?)
        .send()
        .await
        .map_err(|e| provider_error(format!("Route53 {what} failed: {e}")))?;
    Ok(())
}

/// A new value written to an rrset that does not exist yet.
fn fresh_rrset(body: &UpsertDnsRecordDto) -> Route53Rrset {
    Route53Rrset {
        name: bare(&body.name),
        kind: body.record_type.clone(),
        ttl: Some(body.ttl.unwrap_or(300)),
        values: vec![api_value(&body.record_type, &body.content)],
        routed: false,
    }
}

fn dto(body: &UpsertDnsRecordDto, ttl: Option<u32>) -> DnsRecordDto {
    DnsRecordDto {
        id: format!("{}:{}:{}", body.record_type, bare(&body.name), body.content),
        zone_id: body.zone_id.clone(),
        record_type: body.record_type.clone(),
        name: body.name.clone(),
        content: body.content.clone(),
        ttl: Some(ttl.or(body.ttl).unwrap_or(300)),
        proxied: None,
    }
}

impl DnsService {
    pub(in crate::services::networking::dns) async fn upsert_route53_record(
        &self,
        credentials_json: &str,
        body: &UpsertDnsRecordDto,
    ) -> Result<DnsRecordDto, DnsServiceError> {
        let client = Self::route53_client(credentials_json)?;
        let existing = fetch_rrset(&client, &body.zone_id, &body.name, &body.record_type).await?;
        // UPSERT overwrites the group, so the values already there have to ride along.
        let rrset = match existing {
            Some(rrset) => rrset.with(&body.content, body.ttl)?,
            None => fresh_rrset(body),
        };
        apply(
            &client,
            &body.zone_id,
            vec![rrset.change(ChangeAction::Upsert)?],
            "create",
        )
        .await?;
        Ok(dto(body, None))
    }

    pub(in crate::services::networking::dns) async fn update_route53_record(
        &self,
        credentials_json: &str,
        target: &DnsRecordTargetDto,
        next: &UpsertDnsRecordDto,
    ) -> Result<DnsRecordDto, DnsServiceError> {
        if target.record_type.trim().is_empty() {
            return Err(provider_error(
                "Route53 update needs the record type of the value being changed",
            ));
        }
        if target.content.trim().is_empty() {
            return Err(provider_error(
                "Route53 update needs the exact value — this record set holds several",
            ));
        }
        let client = Self::route53_client(credentials_json)?;
        let old = fetch_rrset(&client, &target.zone_id, &target.name, &target.record_type)
            .await?
            .ok_or_else(|| stale(&target.name))?;

        let mut changes = Vec::new();
        let same_rrset = old.kind.eq_ignore_ascii_case(next.record_type.trim())
            && bare(&old.name) == bare(&next.name);
        let previous_ttl = old.ttl;

        if same_rrset {
            // Dropping the old value and writing the new one back into the same group is
            // one UPSERT; the untouched siblings ride along.
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
            let destination =
                fetch_rrset(&client, &next.zone_id, &next.name, &next.record_type).await?;
            let entered = match destination {
                Some(rrset) => rrset.with(&next.content, next.ttl)?,
                None => fresh_rrset(next),
            };
            changes.push(entered.change(ChangeAction::Upsert)?);
        }

        apply(&client, &target.zone_id, changes, "update").await?;
        Ok(dto(next, previous_ttl))
    }

    pub(in crate::services::networking::dns) async fn delete_route53_record(
        &self,
        credentials_json: &str,
        target: &DnsRecordTargetDto,
    ) -> Result<bool, DnsServiceError> {
        if target.record_type.trim().is_empty() {
            return Err(provider_error(
                "Route53 delete needs the record type of the value being removed",
            ));
        }
        if target.content.trim().is_empty() {
            return Err(provider_error(
                "Route53 delete needs the exact value — this record set holds several",
            ));
        }
        let client = Self::route53_client(credentials_json)?;
        let rrset = fetch_rrset(&client, &target.zone_id, &target.name, &target.record_type)
            .await?
            .ok_or_else(|| stale(&target.name))?;
        let reduced = rrset.without(&target.content)?;
        // DELETE only accepts a group listing every current value; removing one member
        // from a larger group means writing back the rest.
        let change = if reduced.values.is_empty() {
            rrset.change(ChangeAction::Delete)?
        } else {
            reduced.change(ChangeAction::Upsert)?
        };
        apply(&client, &target.zone_id, vec![change], "delete").await?;
        Ok(true)
    }
}
