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

use crate::utils::{
    decode_hex, strip_origin_from_name, tlsa_cert_usage_from_u8, tlsa_matching_from_u8,
    tlsa_selector_from_u8,
};
use crate::{
    DnsRecord, DnsRecordType, Error, IntoFqdn, MXRecord, SRVRecord, TLSARecord,
    http::{HttpClient, HttpClientBuilder},
};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_API_ENDPOINT: &str = "https://api.domeneshop.no/v0";
const TTL_MIN: u32 = 60;
const TTL_MAX: u32 = 604_800;

#[derive(Clone)]
pub struct DomeneshopProvider {
    client: HttpClient,
    endpoint: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Domain {
    pub id: i64,
    pub domain: String,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DnsRecordPayload {
    pub host: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub data: String,
    pub ttl: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<u8>,
    #[serde(rename = "dtype", skip_serializing_if = "Option::is_none")]
    pub matching_type: Option<u8>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExistingDnsRecord {
    pub id: i64,
    pub host: String,
    #[serde(rename = "type")]
    pub record_type: String,
    #[serde(default)]
    pub ttl: u32,
    #[serde(default)]
    pub data: String,
    #[serde(default)]
    pub priority: Option<u16>,
    #[serde(default)]
    pub weight: Option<u16>,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub usage: Option<u8>,
    #[serde(default)]
    pub selector: Option<u8>,
    #[serde(rename = "dtype", default)]
    pub matching_type: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomeneshopRecordContent {
    pub record_type: &'static str,
    pub data: String,
    pub priority: Option<u16>,
    pub weight: Option<u16>,
    pub port: Option<u16>,
    pub usage: Option<u8>,
    pub selector: Option<u8>,
    pub matching_type: Option<u8>,
}

impl DomeneshopProvider {
    pub(crate) fn new(
        api_token: impl AsRef<str>,
        api_secret: impl AsRef<str>,
        timeout: Option<Duration>,
    ) -> Self {
        let credentials = format!("{}:{}", api_token.as_ref(), api_secret.as_ref());
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
        let domain = origin.into_name().into_owned();
        let host = strip_origin_from_name(&name, &domain, Some("@"));
        let desired = build_contents(record_type, records)?;
        let domain_id = self.find_domain_id(&domain).await?;
        let existing = self.list_at(domain_id, &host, record_type).await?;

        let mut existing_pool = existing;
        let mut to_add: Vec<DomeneshopRecordContent> = Vec::new();

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
            self.delete_record(domain_id, entry.id).await?;
        }
        for content in to_add {
            self.create_record(domain_id, &host, ttl, &content).await?;
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
        let domain = origin.into_name().into_owned();
        let host = strip_origin_from_name(&name, &domain, Some("@"));
        let desired = build_contents(record_type, records)?;
        let domain_id = self.find_domain_id(&domain).await?;
        let existing = self.list_at(domain_id, &host, record_type).await?;

        for content in desired {
            if existing.iter().any(|r| record_matches(r, &content)) {
                continue;
            }
            self.create_record(domain_id, &host, ttl, &content).await?;
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
        let domain = origin.into_name().into_owned();
        let host = strip_origin_from_name(&name, &domain, Some("@"));
        let to_remove = build_contents(record_type, records)?;
        let domain_id = self.find_domain_id(&domain).await?;
        let existing = self.list_at(domain_id, &host, record_type).await?;

        for content in to_remove {
            if let Some(entry) = existing.iter().find(|r| record_matches(r, &content)) {
                self.delete_record(domain_id, entry.id).await?;
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
        let domain = origin.into_name().into_owned();
        let host = strip_origin_from_name(&name, &domain, Some("@"));
        let domain_id = self.find_domain_id(&domain).await?;
        let existing = self.list_at(domain_id, &host, record_type).await?;
        existing.into_iter().map(DnsRecord::try_from).collect()
    }

    async fn find_domain_id(&self, domain: &str) -> crate::Result<i64> {
        let domains: Vec<Domain> = self
            .client
            .get(format!("{endpoint}/domains", endpoint = self.endpoint))
            .send()
            .await?;
        domains
            .into_iter()
            .find(|d| d.domain == domain)
            .map(|d| d.id)
            .ok_or_else(|| Error::Api(format!("Domain {domain} not found")))
    }

    async fn list_at(
        &self,
        domain_id: i64,
        host: &str,
        record_type: DnsRecordType,
    ) -> crate::Result<Vec<ExistingDnsRecord>> {
        let query = serde_urlencoded::to_string([("host", host), ("type", record_type.as_str())])
            .unwrap_or_default();
        let records: Vec<ExistingDnsRecord> = self
            .client
            .get(format!(
                "{endpoint}/domains/{domain_id}/dns?{query}",
                endpoint = self.endpoint
            ))
            .send()
            .await?;
        let type_str = record_type.as_str();
        Ok(records
            .into_iter()
            .filter(|r| r.host == host && r.record_type == type_str)
            .collect())
    }

    async fn create_record(
        &self,
        domain_id: i64,
        host: &str,
        ttl: u32,
        content: &DomeneshopRecordContent,
    ) -> crate::Result<()> {
        let body = build_payload(host, ttl, content);
        self.client
            .post(format!(
                "{endpoint}/domains/{domain_id}/dns",
                endpoint = self.endpoint
            ))
            .with_body(&body)?
            .send_with_retry::<serde_json::Value>(3)
            .await
            .map(|_| ())
    }

    async fn delete_record(&self, domain_id: i64, record_id: i64) -> crate::Result<()> {
        self.client
            .delete(format!(
                "{endpoint}/domains/{domain_id}/dns/{record_id}",
                endpoint = self.endpoint
            ))
            .send_with_retry::<serde_json::Value>(3)
            .await
            .map(|_| ())
    }
}

fn build_payload(host: &str, ttl: u32, content: &DomeneshopRecordContent) -> DnsRecordPayload {
    DnsRecordPayload {
        host: host.to_string(),
        record_type: content.record_type.to_string(),
        data: content.data.clone(),
        ttl: clamp_ttl(ttl),
        priority: content.priority,
        weight: content.weight,
        port: content.port,
        usage: content.usage,
        selector: content.selector,
        matching_type: content.matching_type,
    }
}

/// The Domeneshop API requires the TTL to be a multiple of 60 seconds, within
/// 60..604800; round up to the next valid value.
fn clamp_ttl(ttl: u32) -> u32 {
    let clamped = ttl.clamp(TTL_MIN, TTL_MAX);
    ((clamped + 59) / 60) * 60
}

fn build_contents(
    expected_type: DnsRecordType,
    records: Vec<DnsRecord>,
) -> crate::Result<Vec<DomeneshopRecordContent>> {
    let mut out = Vec::with_capacity(records.len());
    for record in records {
        if record.as_type() != expected_type {
            return Err(Error::Api(format!(
                "RRSet record type mismatch: expected {}, got {}",
                expected_type.as_str(),
                record.as_type().as_str(),
            )));
        }
        out.push(DomeneshopRecordContent::try_from(record)?);
    }
    Ok(out)
}

fn record_matches(existing: &ExistingDnsRecord, desired: &DomeneshopRecordContent) -> bool {
    let data_matches = if desired.record_type == "TLSA" {
        // TLSA data is a hex digest; the API may return it in upper case.
        existing.data.eq_ignore_ascii_case(&desired.data)
    } else {
        existing.data == desired.data
    };
    existing.record_type == desired.record_type
        && data_matches
        && existing.priority == desired.priority
        && existing.weight == desired.weight
        && existing.port == desired.port
        && existing.usage == desired.usage
        && existing.selector == desired.selector
        && existing.matching_type == desired.matching_type
}

impl TryFrom<DnsRecord> for DomeneshopRecordContent {
    type Error = Error;

    fn try_from(record: DnsRecord) -> Result<Self, Self::Error> {
        match record {
            DnsRecord::A(addr) => Ok(DomeneshopRecordContent {
                record_type: "A",
                data: addr.to_string(),
                priority: None,
                weight: None,
                port: None,
                usage: None,
                selector: None,
                matching_type: None,
            }),
            DnsRecord::AAAA(addr) => Ok(DomeneshopRecordContent {
                record_type: "AAAA",
                data: addr.to_string(),
                priority: None,
                weight: None,
                port: None,
                usage: None,
                selector: None,
                matching_type: None,
            }),
            DnsRecord::CNAME(target) => Ok(DomeneshopRecordContent {
                record_type: "CNAME",
                data: target,
                priority: None,
                weight: None,
                port: None,
                usage: None,
                selector: None,
                matching_type: None,
            }),
            DnsRecord::NS(_) => Err(Error::Unsupported(
                "NS records are not supported by the Domeneshop DNS API".to_string(),
            )),
            DnsRecord::MX(mx) => Ok(DomeneshopRecordContent {
                record_type: "MX",
                data: mx.exchange,
                priority: Some(mx.priority),
                weight: None,
                port: None,
                usage: None,
                selector: None,
                matching_type: None,
            }),
            DnsRecord::TXT(text) => Ok(DomeneshopRecordContent {
                record_type: "TXT",
                data: text,
                priority: None,
                weight: None,
                port: None,
                usage: None,
                selector: None,
                matching_type: None,
            }),
            DnsRecord::SRV(srv) => Ok(DomeneshopRecordContent {
                record_type: "SRV",
                data: srv.target,
                priority: Some(srv.priority),
                weight: Some(srv.weight),
                port: Some(srv.port),
                usage: None,
                selector: None,
                matching_type: None,
            }),
            DnsRecord::TLSA(tlsa) => Ok(DomeneshopRecordContent {
                record_type: "TLSA",
                data: tlsa.cert_data.iter().map(|b| format!("{b:02x}")).collect(),
                priority: None,
                weight: None,
                port: None,
                usage: Some(u8::from(tlsa.cert_usage)),
                selector: Some(u8::from(tlsa.selector)),
                matching_type: Some(u8::from(tlsa.matching)),
            }),
            DnsRecord::CAA(_) => Err(Error::Unsupported(
                "CAA records are not supported by the Domeneshop DNS API".to_string(),
            )),
        }
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
            "SRV" => Ok(DnsRecord::SRV(SRVRecord {
                priority: record.priority.unwrap_or_default(),
                weight: record.weight.unwrap_or_default(),
                port: record.port.unwrap_or_default(),
                target: record.data,
            })),
            "TLSA" => Ok(DnsRecord::TLSA(TLSARecord {
                cert_usage: tlsa_cert_usage_from_u8(record.usage.unwrap_or_default())?,
                selector: tlsa_selector_from_u8(record.selector.unwrap_or_default())?,
                matching: tlsa_matching_from_u8(record.matching_type.unwrap_or_default())?,
                cert_data: decode_hex(&record.data)?,
            })),
            other => Err(Error::Parse(format!(
                "Unsupported Domeneshop record type: {other}"
            ))),
        }
    }
}

impl DomeneshopProvider {
    pub(crate) async fn list_zone_infos(&self) -> crate::Result<Vec<crate::ZoneInfo>> {
        let domains: Vec<Domain> = self
            .client
            .get(format!("{endpoint}/domains", endpoint = self.endpoint))
            .send()
            .await?;
        Ok(domains
            .into_iter()
            .map(|d| crate::ZoneInfo {
                id: d.id.to_string(),
                name: d.domain,
                status: d.status,
            })
            .collect())
    }

    pub(crate) async fn list_zone_records(
        &self,
        zone: &str,
    ) -> crate::Result<Vec<crate::ZoneRecord>> {
        let domain_id = self.find_domain_id(zone).await?;
        let records: Vec<ExistingDnsRecord> = self
            .client
            .get(format!(
                "{endpoint}/domains/{domain_id}/dns",
                endpoint = self.endpoint
            ))
            .send()
            .await?;
        let mut out = Vec::with_capacity(records.len());
        for record in records {
            let record_type = record.record_type.clone();
            let id = record.id.to_string();
            let ttl = record.ttl;
            let mut name = strip_origin_from_name(&record.host, zone, Some("@"));
            if name.is_empty() {
                name = "@".to_string();
            }
            let Ok(dns_record) = DnsRecord::try_from(record) else {
                continue;
            };
            out.push(crate::ZoneRecord {
                id,
                name,
                record_type: record_type.to_uppercase(),
                content: dns_record.to_string(),
                ttl,
                record: dns_record,
            });
        }
        Ok(out)
    }
}
