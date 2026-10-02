/*
 * Copyright Stalwart Labs LLC See the COPYING
 * file at the top-level directory of this distribution.
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use crate::utils::{build_caa, parse_tlsa, strip_origin_from_name, strip_trailing_dot};
use crate::{
    DnsRecord, DnsRecordType, Error, IntoFqdn, MXRecord, SRVRecord,
    http::{HttpClient, HttpClientBuilder},
};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_API_ENDPOINT: &str = "https://api.simply.com/2";

#[derive(Clone)]
pub struct SimplyComProvider {
    client: HttpClient,
    endpoint: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProductList {
    #[serde(default)]
    pub products: Vec<Product>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Product {
    pub object: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub domain: Option<ProductDomain>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProductDomain {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_idn: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RecordList {
    #[serde(default)]
    pub records: Vec<ExistingDnsRecord>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExistingDnsRecord {
    pub record_id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub record_type: String,
    #[serde(default)]
    pub data: String,
    #[serde(default)]
    pub priority: Option<u16>,
}

#[derive(Deserialize, Debug, Clone)]
struct RecordWithTtl {
    #[serde(flatten)]
    record: ExistingDnsRecord,
    #[serde(default)]
    ttl: Option<u32>,
}

#[derive(Deserialize, Debug)]
struct TtlRecordList {
    #[serde(default)]
    records: Vec<RecordWithTtl>,
}

#[derive(Serialize, Debug, Clone)]
pub struct CreateDnsRecord<'a> {
    #[serde(rename = "type")]
    pub record_type: &'static str,
    pub name: &'a str,
    pub data: &'a str,
    pub ttl: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimplyComRecordContent {
    pub record_type: &'static str,
    pub data: String,
    pub priority: Option<u16>,
}

impl SimplyComProvider {
    pub(crate) fn new(
        account_name: impl AsRef<str>,
        api_key: impl AsRef<str>,
        timeout: Option<Duration>,
    ) -> Self {
        let credentials = format!("{}:{}", account_name.as_ref(), api_key.as_ref());
        let encoded = BASE64.encode(credentials.as_bytes());
        let client = HttpClientBuilder::default()
            .with_header("Authorization", format!("Basic {encoded}"))
            .with_timeout(timeout)
            .build();
        Self {
            client,
            endpoint: DEFAULT_API_ENDPOINT.to_string(),
        }
    }

    #[cfg(test)]
    pub(crate) fn with_endpoint(self, endpoint: impl AsRef<str>) -> Self {
        Self {
            endpoint: endpoint.as_ref().to_string(),
            ..self
        }
    }

    pub(crate) async fn set_rrset(
        &self,
        name: impl IntoFqdn<'_>,
        record_type: DnsRecordType,
        ttl: u32,
        records: Vec<DnsRecord>,
        origin: impl IntoFqdn<'_>,
    ) -> crate::Result<()> {
        let name = name.into_name().into_owned();
        let origin = origin.into_name().into_owned();
        let desired = build_contents(record_type, records)?;
        let (object, zone) = self.find_object(&origin).await?;
        let host = strip_origin_from_name(&name, &zone, Some("@"));
        let existing = self.list_at(&object, &zone, &host, record_type).await?;

        let mut existing_pool = existing;
        let mut to_add: Vec<SimplyComRecordContent> = Vec::new();

        for content in desired {
            if let Some(idx) = existing_pool
                .iter()
                .position(|r| record_matches(r, &content))
            {
                existing_pool.swap_remove(idx);
            } else {
                to_add.push(content);
            }
        }

        for entry in existing_pool {
            self.delete_record(&object, entry.record_id).await?;
        }
        for content in to_add {
            self.create_record(&object, &host, ttl, &content).await?;
        }
        Ok(())
    }

    pub(crate) async fn add_to_rrset(
        &self,
        name: impl IntoFqdn<'_>,
        record_type: DnsRecordType,
        ttl: u32,
        records: Vec<DnsRecord>,
        origin: impl IntoFqdn<'_>,
    ) -> crate::Result<()> {
        if records.is_empty() {
            return Ok(());
        }
        let name = name.into_name().into_owned();
        let origin = origin.into_name().into_owned();
        let desired = build_contents(record_type, records)?;
        let (object, zone) = self.find_object(&origin).await?;
        let host = strip_origin_from_name(&name, &zone, Some("@"));
        let existing = self.list_at(&object, &zone, &host, record_type).await?;

        for content in desired {
            if existing.iter().any(|r| record_matches(r, &content)) {
                continue;
            }
            self.create_record(&object, &host, ttl, &content).await?;
        }
        Ok(())
    }

    pub(crate) async fn remove_from_rrset(
        &self,
        name: impl IntoFqdn<'_>,
        record_type: DnsRecordType,
        records: Vec<DnsRecord>,
        origin: impl IntoFqdn<'_>,
    ) -> crate::Result<()> {
        if records.is_empty() {
            return Ok(());
        }
        let name = name.into_name().into_owned();
        let origin = origin.into_name().into_owned();
        let to_remove = build_contents(record_type, records)?;
        let (object, zone) = self.find_object(&origin).await?;
        let host = strip_origin_from_name(&name, &zone, Some("@"));
        let existing = self.list_at(&object, &zone, &host, record_type).await?;

        for content in to_remove {
            if let Some(entry) = existing.iter().find(|r| record_matches(r, &content)) {
                self.delete_record(&object, entry.record_id).await?;
            }
        }
        Ok(())
    }

    pub(crate) async fn list_rrset(
        &self,
        name: impl IntoFqdn<'_>,
        record_type: DnsRecordType,
        origin: impl IntoFqdn<'_>,
    ) -> crate::Result<Vec<DnsRecord>> {
        let name = name.into_name().into_owned();
        let origin = origin.into_name().into_owned();
        let (object, zone) = self.find_object(&origin).await?;
        let host = strip_origin_from_name(&name, &zone, Some("@"));
        let existing = self.list_at(&object, &zone, &host, record_type).await?;
        existing.into_iter().map(DnsRecord::try_from).collect()
    }

    async fn find_object(&self, origin: &str) -> crate::Result<(String, String)> {
        let response: ProductList = self
            .client
            .get(format!("{endpoint}/my/products/", endpoint = self.endpoint))
            .send_with_retry(3)
            .await?;
        let mut candidate = origin;
        loop {
            if let Some(product) = response
                .products
                .iter()
                .find(|p| product_matches(p, candidate))
            {
                return Ok((product.object.clone(), candidate.to_string()));
            }
            match candidate.split_once('.') {
                Some((_, rest)) if rest.contains('.') => candidate = rest,
                _ => return Err(Error::NotFound),
            }
        }
    }

    async fn list_at(
        &self,
        object: &str,
        zone: &str,
        host: &str,
        record_type: DnsRecordType,
    ) -> crate::Result<Vec<ExistingDnsRecord>> {
        let response: RecordList = self
            .client
            .get(format!(
                "{endpoint}/my/products/{object}/dns/records/",
                endpoint = self.endpoint
            ))
            .send_with_retry(3)
            .await?;
        let type_str = record_type.as_str();
        Ok(response
            .records
            .into_iter()
            .filter(|r| normalize_record_name(&r.name, zone) == host && r.record_type == type_str)
            .collect())
    }

    async fn create_record(
        &self,
        object: &str,
        host: &str,
        ttl: u32,
        content: &SimplyComRecordContent,
    ) -> crate::Result<()> {
        self.client
            .post(format!(
                "{endpoint}/my/products/{object}/dns/records/",
                endpoint = self.endpoint
            ))
            .with_body(CreateDnsRecord {
                record_type: content.record_type,
                name: host,
                data: &content.data,
                ttl,
                priority: content.priority,
            })?
            .send_with_retry::<serde_json::Value>(3)
            .await
            .map(|_| ())
    }

    async fn delete_record(&self, object: &str, record_id: i64) -> crate::Result<()> {
        self.client
            .delete(format!(
                "{endpoint}/my/products/{object}/dns/records/{record_id}/",
                endpoint = self.endpoint
            ))
            .send_with_retry::<serde_json::Value>(3)
            .await
            .map(|_| ())
    }
}

fn product_matches(product: &Product, candidate: &str) -> bool {
    product.object == candidate
        || product.name.as_deref() == Some(candidate)
        || product.domain.as_ref().is_some_and(|d| {
            d.name.as_deref() == Some(candidate) || d.name_idn.as_deref() == Some(candidate)
        })
}

/// The API documents record names both as relative labels and as fully qualified
/// names (e.g. the apex is returned as "example.com"), so normalize every stored
/// name back to the relative form ("@" for the apex) before comparing or listing.
fn normalize_record_name(raw: &str, zone: &str) -> String {
    let raw = raw.trim_end_matches('.');
    if raw.is_empty() || raw == zone {
        "@".to_string()
    } else if let Some(rel) = raw.strip_suffix(&format!(".{zone}")) {
        rel.to_string()
    } else {
        raw.to_string()
    }
}

fn build_contents(
    expected_type: DnsRecordType,
    records: Vec<DnsRecord>,
) -> crate::Result<Vec<SimplyComRecordContent>> {
    let mut out = Vec::with_capacity(records.len());
    for record in records {
        if record.as_type() != expected_type {
            return Err(Error::Api(format!(
                "RRSet record type mismatch: expected {}, got {}",
                expected_type.as_str(),
                record.as_type().as_str(),
            )));
        }
        out.push(SimplyComRecordContent::try_from(record)?);
    }
    Ok(out)
}

fn record_matches(existing: &ExistingDnsRecord, desired: &SimplyComRecordContent) -> bool {
    existing.record_type == desired.record_type
        && host_data_equal(&existing.record_type, &existing.data, &desired.data)
        && (desired.priority.is_none() || existing.priority == desired.priority)
}

/// Simply.com stores host targets with their trailing dot and hands them back that way,
/// while the write path strips it; comparing the two raw forms misses every listed row.
fn host_data_equal(record_type: &str, stored: &str, wanted: &str) -> bool {
    if matches!(record_type, "CNAME" | "NS" | "MX") {
        strip_trailing_dot(stored) == strip_trailing_dot(wanted)
    } else {
        stored == wanted
    }
}

fn parse_simplycom_srv(data: &str, priority: u16) -> crate::Result<DnsRecord> {
    let mut parts = data.split_whitespace();
    let weight = parts.next().and_then(|v| v.parse().ok());
    let port = parts.next().and_then(|v| v.parse().ok());
    let target = parts.next();
    match (weight, port, target) {
        (Some(weight), Some(port), Some(target)) if parts.next().is_none() => {
            Ok(DnsRecord::SRV(SRVRecord {
                priority,
                weight,
                port,
                target: strip_trailing_dot(target).to_string(),
            }))
        }
        _ => Err(Error::Parse(format!("invalid SRV data: {data}"))),
    }
}

fn parse_simplycom_caa(data: &str) -> crate::Result<DnsRecord> {
    let mut parts = data.splitn(3, ' ');
    let flags = parts
        .next()
        .and_then(|v| v.parse::<u8>().ok())
        .ok_or_else(|| Error::Parse(format!("invalid CAA data: {data}")))?;
    let tag = parts
        .next()
        .ok_or_else(|| Error::Parse(format!("invalid CAA data: {data}")))?;
    let value = parts
        .next()
        .map(|v| v.trim().trim_matches('"'))
        .ok_or_else(|| Error::Parse(format!("invalid CAA data: {data}")))?;
    build_caa(flags, tag, value).map(DnsRecord::CAA)
}

impl TryFrom<DnsRecord> for SimplyComRecordContent {
    type Error = Error;

    fn try_from(record: DnsRecord) -> Result<Self, Self::Error> {
        Ok(match record {
            DnsRecord::A(addr) => SimplyComRecordContent {
                record_type: "A",
                data: addr.to_string(),
                priority: None,
            },
            DnsRecord::AAAA(addr) => SimplyComRecordContent {
                record_type: "AAAA",
                data: addr.to_string(),
                priority: None,
            },
            DnsRecord::CNAME(target) => SimplyComRecordContent {
                record_type: "CNAME",
                data: strip_trailing_dot(&target).to_string(),
                priority: None,
            },
            DnsRecord::NS(target) => SimplyComRecordContent {
                record_type: "NS",
                data: strip_trailing_dot(&target).to_string(),
                priority: None,
            },
            DnsRecord::MX(mx) => SimplyComRecordContent {
                record_type: "MX",
                data: strip_trailing_dot(&mx.exchange).to_string(),
                priority: Some(mx.priority),
            },
            DnsRecord::TXT(text) => SimplyComRecordContent {
                record_type: "TXT",
                data: text,
                priority: None,
            },
            DnsRecord::SRV(srv) => SimplyComRecordContent {
                record_type: "SRV",
                data: format!(
                    "{} {} {}",
                    srv.weight,
                    srv.port,
                    strip_trailing_dot(&srv.target)
                ),
                priority: Some(srv.priority),
            },
            DnsRecord::TLSA(tlsa) => SimplyComRecordContent {
                record_type: "TLSA",
                data: tlsa.to_string(),
                priority: None,
            },
            DnsRecord::CAA(caa) => SimplyComRecordContent {
                record_type: "CAA",
                data: caa.to_string(),
                priority: None,
            },
        })
    }
}

impl TryFrom<ExistingDnsRecord> for DnsRecord {
    type Error = Error;

    fn try_from(record: ExistingDnsRecord) -> Result<Self, Self::Error> {
        match record.record_type.as_str() {
            "A" => record
                .data
                .parse()
                .map(DnsRecord::A)
                .map_err(|e| Error::Parse(format!("invalid A data: {e}"))),
            "AAAA" => record
                .data
                .parse()
                .map(DnsRecord::AAAA)
                .map_err(|e| Error::Parse(format!("invalid AAAA data: {e}"))),
            "CNAME" => Ok(DnsRecord::CNAME(record.data)),
            "NS" => Ok(DnsRecord::NS(record.data)),
            "MX" => Ok(DnsRecord::MX(MXRecord {
                exchange: record.data,
                priority: record.priority.unwrap_or_default(),
            })),
            "TXT" => Ok(DnsRecord::TXT(record.data)),
            "SRV" => parse_simplycom_srv(&record.data, record.priority.unwrap_or_default()),
            "TLSA" => parse_tlsa(&record.data),
            "CAA" => parse_simplycom_caa(&record.data),
            other => Err(Error::Parse(format!(
                "Unsupported Simply.com record type: {other}"
            ))),
        }
    }
}

impl SimplyComProvider {
    pub(crate) async fn list_zone_infos(&self) -> crate::Result<Vec<crate::ZoneInfo>> {
        let response = self
            .client
            .get(format!("{endpoint}/my/products/", endpoint = self.endpoint))
            .send_with_retry::<ProductList>(3)
            .await?;
        Ok(response
            .products
            .into_iter()
            .filter_map(|product| {
                let name = product
                    .domain
                    .as_ref()
                    .and_then(|d| d.name.clone())
                    .or_else(|| product.name.clone())?;
                Some(crate::ZoneInfo {
                    id: product.object,
                    name,
                    status: None,
                })
            })
            .collect())
    }

    pub(crate) async fn list_zone_records(
        &self,
        zone: &str,
    ) -> crate::Result<Vec<crate::ZoneRecord>> {
        let (object, zone) = self.find_object(zone.trim_end_matches('.')).await?;
        let response = self
            .client
            .get(format!(
                "{endpoint}/my/products/{object}/dns/records/",
                endpoint = self.endpoint
            ))
            .send_with_retry::<TtlRecordList>(3)
            .await?;
        let mut out = Vec::new();
        for listed in response.records {
            let record = listed.record;
            let ttl = listed.ttl.unwrap_or(0);
            let Ok(typed) = DnsRecord::try_from(record.clone()) else {
                continue;
            };
            out.push(crate::ZoneRecord {
                id: record.record_id.to_string(),
                name: normalize_record_name(&record.name, &zone),
                record_type: record.record_type.to_ascii_uppercase(),
                content: typed.to_string(),
                ttl,
                record: typed,
            });
        }
        Ok(out)
    }
}
