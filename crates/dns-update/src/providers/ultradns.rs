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

use crate::utils::same_value;
use crate::utils::split_caa_value;
use crate::utils::strip_trailing_dot;
use crate::utils::{parse_mx, parse_srv, parse_tlsa};
use crate::{
    CAARecord, DnsRecord, DnsRecordType, Error, IntoFqdn, Result,
    http::{HttpClient, HttpClientBuilder, HttpRequest},
};
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const DEFAULT_ENDPOINT: &str = "https://api.ultradns.com";
const TOKEN_PATH: &str = "/v1/authorization/token";
const RETRIES: u32 = 3;
const TOKEN_REFRESH_RETRIES: u32 = 1;

#[derive(Clone)]
pub struct UltraDnsProvider {
    client: HttpClient,
    username: String,
    password: String,
    endpoint: String,
    token: Arc<Mutex<Option<TokenState>>>,
}

#[derive(Clone)]
struct TokenState {
    access_token: String,
    refresh_token: Option<String>,
    expires: Instant,
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    #[serde(rename = "accessToken", alias = "access_token")]
    access_token: String,
    #[serde(rename = "refreshToken", alias = "refresh_token", default)]
    refresh_token: Option<String>,
    #[serde(rename = "expiresIn", alias = "expires_in", default)]
    expires_in: Option<serde_json::Value>,
}

#[derive(Serialize, Debug)]
struct RrsetBody {
    ttl: u32,
    rdata: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct ApiError {
    #[serde(default, rename = "errorCode")]
    error_code: Option<u64>,
    #[serde(default, rename = "errorMessage")]
    error_message: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ListRrsetsResponse {
    #[serde(default, rename = "rrSets")]
    rrsets: Vec<ListedRrset>,
}

#[derive(Deserialize, Debug)]
struct ListedRrset {
    #[serde(default)]
    rrtype: String,
    #[serde(default)]
    ttl: u32,
    #[serde(default)]
    rdata: Vec<String>,
}

impl UltraDnsProvider {
    pub(crate) fn new(
        username: impl Into<String>,
        password: impl Into<String>,
        endpoint: Option<String>,
        timeout: Option<Duration>,
    ) -> Result<Self> {
        let client = HttpClientBuilder::default().with_timeout(timeout).build();
        Ok(Self {
            client,
            username: username.into(),
            password: password.into(),
            endpoint: endpoint
                .map(|value| value.trim_end_matches('/').to_string())
                .unwrap_or_else(|| DEFAULT_ENDPOINT.to_string()),
            token: Arc::new(Mutex::new(None)),
        })
    }

    #[cfg(test)]
    pub(crate) fn with_endpoint(mut self, endpoint: impl AsRef<str>) -> Self {
        self.endpoint = endpoint.as_ref().trim_end_matches('/').to_string();
        self
    }

    #[cfg(test)]
    pub(crate) fn with_cached_token(self, token: impl Into<String>) -> Self {
        *self.token.lock().expect("UltraDNS test token lock") = Some(TokenState {
            access_token: token.into(),
            refresh_token: None,
            expires: Instant::now() + Duration::from_secs(55 * 60),
        });
        self
    }

    pub(crate) async fn set_rrset(
        &self,
        name: impl IntoFqdn<'_>,
        record_type: DnsRecordType,
        ttl: u32,
        records: Vec<DnsRecord>,
        origin: impl IntoFqdn<'_>,
    ) -> Result<()> {
        check_record_types(record_type, &records)?;
        let owner = name.into_fqdn().into_owned();
        let zone = origin.into_fqdn().into_owned();

        if records.is_empty() {
            return self.delete_rrset(&zone, record_type, &owner).await;
        }

        let rdata = build_rdata(records)?;
        self.put_rrset(&zone, record_type, &owner, ttl, rdata).await
    }

    pub(crate) async fn add_to_rrset(
        &self,
        name: impl IntoFqdn<'_>,
        record_type: DnsRecordType,
        ttl: u32,
        records: Vec<DnsRecord>,
        origin: impl IntoFqdn<'_>,
    ) -> Result<()> {
        check_record_types(record_type, &records)?;
        if records.is_empty() {
            return Ok(());
        }
        let owner = name.into_fqdn().into_owned();
        let zone = origin.into_fqdn().into_owned();
        let desired = build_rdata(records)?;

        let (mut merged, effective_ttl) =
            match self.fetch_rrset_full(&zone, record_type, &owner).await? {
                Some((existing, existing_ttl)) => (existing, existing_ttl),
                None => (Vec::new(), ttl),
            };
        let before = merged.len();
        for value in desired {
            if !merged.iter().any(|existing| same_value(existing, &value)) {
                merged.push(value);
            }
        }
        if merged.len() == before {
            return Ok(());
        }
        self.put_rrset(&zone, record_type, &owner, effective_ttl, merged)
            .await
    }

    pub(crate) async fn remove_from_rrset(
        &self,
        name: impl IntoFqdn<'_>,
        record_type: DnsRecordType,
        records: Vec<DnsRecord>,
        origin: impl IntoFqdn<'_>,
    ) -> Result<()> {
        check_record_types(record_type, &records)?;
        if records.is_empty() {
            return Ok(());
        }
        let owner = name.into_fqdn().into_owned();
        let zone = origin.into_fqdn().into_owned();
        let to_remove = build_rdata(records)?;

        let (existing_rdata, existing_ttl) =
            match self.fetch_rrset_full(&zone, record_type, &owner).await? {
                Some(v) => v,
                None => return Ok(()),
            };

        let remaining: Vec<String> = existing_rdata
            .into_iter()
            .filter(|value| !to_remove.iter().any(|wanted| same_value(value, wanted)))
            .collect();

        if remaining.is_empty() {
            return self.delete_rrset(&zone, record_type, &owner).await;
        }
        self.put_rrset(&zone, record_type, &owner, existing_ttl, remaining)
            .await
    }

    pub(crate) async fn list_rrset(
        &self,
        name: impl IntoFqdn<'_>,
        record_type: DnsRecordType,
        origin: impl IntoFqdn<'_>,
    ) -> Result<Vec<DnsRecord>> {
        let owner = name.into_fqdn().into_owned();
        let zone = origin.into_fqdn().into_owned();
        let rdata = match self.fetch_rrset(&zone, record_type, &owner).await? {
            Some(v) => v,
            None => return Ok(Vec::new()),
        };
        rdata
            .into_iter()
            .map(|value| parse_rdata(record_type, &value))
            .collect()
    }

    async fn put_rrset(
        &self,
        zone: &str,
        record_type: DnsRecordType,
        owner: &str,
        ttl: u32,
        rdata: Vec<String>,
    ) -> Result<()> {
        let url = self.rrset_url(zone, record_type.as_str(), owner);
        let body = RrsetBody { ttl, rdata };
        self.send_with_token::<serde_json::Value, _>(Method::PUT, &url, Some(&body))
            .await
            .map(|_| ())
    }

    async fn delete_rrset(
        &self,
        zone: &str,
        record_type: DnsRecordType,
        owner: &str,
    ) -> Result<()> {
        let url = self.rrset_url(zone, record_type.as_str(), owner);
        match self
            .send_with_token::<serde_json::Value, ()>(Method::DELETE, &url, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(Error::NotFound) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn fetch_rrset(
        &self,
        zone: &str,
        record_type: DnsRecordType,
        owner: &str,
    ) -> Result<Option<Vec<String>>> {
        Ok(self
            .fetch_rrset_full(zone, record_type, owner)
            .await?
            .map(|(rdata, _)| rdata))
    }

    async fn fetch_rrset_full(
        &self,
        zone: &str,
        record_type: DnsRecordType,
        owner: &str,
    ) -> Result<Option<(Vec<String>, u32)>> {
        let url = self.rrset_url(zone, record_type.as_str(), owner);
        let expected_type = record_type.as_str();
        let response: ListRrsetsResponse = match self
            .send_with_token::<ListRrsetsResponse, ()>(Method::GET, &url, None)
            .await
        {
            Ok(r) => r,
            Err(Error::NotFound) => return Ok(None),
            Err(e) => return Err(e),
        };

        let mut combined: Vec<String> = Vec::new();
        let mut ttl: u32 = 0;
        for rrset in response.rrsets {
            if !type_matches(&rrset.rrtype, expected_type) {
                continue;
            }
            if rrset.ttl > 0 {
                ttl = rrset.ttl;
            }
            for value in rrset.rdata {
                combined.push(value);
            }
        }
        if combined.is_empty() {
            Ok(None)
        } else {
            Ok(Some((combined, ttl)))
        }
    }

    fn rrset_url(&self, zone: &str, record_type: &str, owner: &str) -> String {
        format!(
            "{base}/v3/zones/{zone}/rrsets/{record_type}/{owner}",
            base = self.endpoint,
            zone = path_escape(zone),
            owner = path_escape(owner),
        )
    }

    async fn send_with_token<T, B>(&self, method: Method, url: &str, body: Option<&B>) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let mut attempts = 0u32;
        loop {
            let token = self.ensure_token(false).await?;
            let request = self.build_request(method.clone(), url, &token, body)?;
            let raw = match request.send_raw().await {
                Ok(text) => text,
                Err(Error::Unauthorized) if attempts < TOKEN_REFRESH_RETRIES => {
                    self.invalidate_token();
                    attempts += 1;
                    continue;
                }
                Err(Error::Api(ref msg))
                    if attempts < TOKEN_REFRESH_RETRIES && contains_token_expired(msg) =>
                {
                    self.invalidate_token();
                    attempts += 1;
                    continue;
                }
                Err(e) => return Err(e),
            };
            if raw.is_empty() {
                return serde_json::from_str("{}").map_err(|err| {
                    Error::Serialize(format!("Failed to create empty response: {err}"))
                });
            }
            if let Some(err_msg) = parse_api_error(&raw)
                && attempts < TOKEN_REFRESH_RETRIES
                && contains_token_expired(&err_msg)
            {
                self.invalidate_token();
                attempts += 1;
                continue;
            }
            return serde_json::from_str(&raw)
                .map_err(|err| Error::Serialize(format!("Failed to deserialize response: {err}")));
        }
    }

    fn build_request<B>(
        &self,
        method: Method,
        url: &str,
        token: &str,
        body: Option<&B>,
    ) -> Result<HttpRequest>
    where
        B: Serialize,
    {
        let bearer = format!("Bearer {token}");
        let mut request = self
            .client
            .request(method, url)
            .set_header("Authorization", &bearer);
        if let Some(body) = body {
            request = request.with_body(body)?;
        }
        Ok(request)
    }

    fn invalidate_token(&self) {
        if let Ok(mut guard) = self.token.lock() {
            *guard = None;
        }
    }

    async fn ensure_token(&self, force_password: bool) -> Result<String> {
        if !force_password
            && let Some(state) = self.token.lock().ok().and_then(|guard| guard.clone())
            && Instant::now() < state.expires
        {
            return Ok(state.access_token);
        }

        let refresh = if force_password {
            None
        } else {
            self.token
                .lock()
                .ok()
                .and_then(|guard| guard.as_ref().and_then(|state| state.refresh_token.clone()))
        };

        let body = if let Some(refresh) = refresh.as_deref() {
            serde_urlencoded::to_string([
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh),
            ])
        } else {
            serde_urlencoded::to_string([
                ("grant_type", "password"),
                ("username", self.username.as_str()),
                ("password", self.password.as_str()),
            ])
        }
        .map_err(|err| Error::Api(format!("UltraDNS token body encode failed: {err}")))?;

        let url = format!("{}{TOKEN_PATH}", self.endpoint);
        let response = self
            .client
            .post(&url)
            .set_header("Content-Type", "application/x-www-form-urlencoded")
            .with_raw_body(body)
            .send_with_retry::<TokenResponse>(RETRIES)
            .await;

        let token = match response {
            Ok(t) => t,
            Err(Error::Unauthorized) if refresh.is_some() => {
                self.invalidate_token();
                return Box::pin(self.ensure_token(true)).await;
            }
            Err(e) => return Err(e),
        };

        let lifetime = token
            .expires_in
            .as_ref()
            .and_then(|value| value.as_u64().or_else(|| value.as_str()?.parse().ok()))
            .unwrap_or(55 * 60);

        let access_token = token.access_token.clone();
        let state = TokenState {
            access_token: token.access_token,
            refresh_token: token.refresh_token,
            expires: Instant::now() + Duration::from_secs(lifetime.saturating_sub(30)),
        };

        if let Ok(mut guard) = self.token.lock() {
            *guard = Some(state);
        }
        Ok(access_token)
    }
}

fn type_matches(reported: &str, expected: &str) -> bool {
    let head = reported
        .split_whitespace()
        .next()
        .unwrap_or(reported)
        .trim_end_matches(',');
    head.eq_ignore_ascii_case(expected)
}

fn check_record_types(expected: DnsRecordType, records: &[DnsRecord]) -> Result<()> {
    for r in records {
        if r.as_type() != expected {
            return Err(Error::Api(format!(
                "RRSet record type mismatch: expected {}, got {}",
                expected.as_str(),
                r.as_type().as_str(),
            )));
        }
    }
    Ok(())
}

fn build_rdata(records: Vec<DnsRecord>) -> Result<Vec<String>> {
    let mut out = Vec::with_capacity(records.len());
    for record in records {
        out.push(render_rdata(record)?);
    }
    Ok(out)
}

fn render_rdata(record: DnsRecord) -> Result<String> {
    Ok(match record {
        DnsRecord::A(ip) => ip.to_string(),
        DnsRecord::AAAA(ip) => ip.to_string(),
        DnsRecord::CNAME(target) => (&target).into_fqdn().into_owned(),
        DnsRecord::NS(target) => (&target).into_fqdn().into_owned(),
        DnsRecord::MX(mx) => format!(
            "{} {}",
            mx.priority,
            (&mx.exchange).into_fqdn().into_owned()
        ),
        DnsRecord::TXT(value) => value,
        DnsRecord::SRV(srv) => format!(
            "{} {} {} {}",
            srv.priority,
            srv.weight,
            srv.port,
            (&srv.target).into_fqdn().into_owned(),
        ),
        DnsRecord::TLSA(tlsa) => tlsa.to_string(),
        DnsRecord::CAA(caa) => caa.to_string(),
    })
}

fn parse_rdata(record_type: DnsRecordType, value: &str) -> Result<DnsRecord> {
    Ok(match record_type {
        DnsRecordType::A => DnsRecord::A(
            value
                .parse()
                .map_err(|e| Error::Parse(format!("invalid A value '{value}': {e}")))?,
        ),
        DnsRecordType::AAAA => DnsRecord::AAAA(
            value
                .parse()
                .map_err(|e| Error::Parse(format!("invalid AAAA value '{value}': {e}")))?,
        ),
        DnsRecordType::CNAME => DnsRecord::CNAME(strip_trailing_dot(value).to_string()),
        DnsRecordType::NS => DnsRecord::NS(strip_trailing_dot(value).to_string()),
        DnsRecordType::MX => parse_mx(value)?,
        DnsRecordType::TXT => DnsRecord::TXT(parse_txt(value)),
        DnsRecordType::SRV => parse_srv(value)?,
        DnsRecordType::TLSA => parse_tlsa(value)?,
        DnsRecordType::CAA => parse_caa(value)?,
    })
}

fn parse_txt(value: &str) -> String {
    let trimmed = value.trim();
    if !trimmed.starts_with('"') {
        return trimmed.to_string();
    }
    let mut out = String::with_capacity(trimmed.len());
    let mut bytes = trimmed.bytes().peekable();
    while let Some(&b) = bytes.peek() {
        if b != b'"' {
            bytes.next();
            continue;
        }
        bytes.next();
        loop {
            match bytes.next() {
                Some(b'"') => break,
                Some(b'\\') => {
                    if let Some(next) = bytes.next() {
                        out.push(next as char);
                    }
                }
                Some(other) => out.push(other as char),
                None => break,
            }
        }
    }
    out
}

fn parse_caa(value: &str) -> Result<DnsRecord> {
    let mut parts = value.splitn(3, char::is_whitespace);
    let flags: u8 = parts
        .next()
        .ok_or_else(|| Error::Parse(format!("invalid CAA value '{value}'")))?
        .parse()
        .map_err(|e| Error::Parse(format!("invalid CAA flags in '{value}': {e}")))?;
    let tag = parts
        .next()
        .ok_or_else(|| Error::Parse(format!("invalid CAA value '{value}'")))?
        .to_ascii_lowercase();
    let raw_value = parts
        .next()
        .ok_or_else(|| Error::Parse(format!("invalid CAA value '{value}'")))?
        .trim();
    let unquoted = raw_value
        .strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .map(|s| s.replace("\\\"", "\""))
        .unwrap_or_else(|| raw_value.to_string());

    let issuer_critical = flags & 0x80 != 0;
    match tag.as_str() {
        "issue" => {
            let (name, options) = split_caa_value(&unquoted);
            Ok(DnsRecord::CAA(CAARecord::Issue {
                issuer_critical,
                name,
                options,
            }))
        }
        "issuewild" => {
            let (name, options) = split_caa_value(&unquoted);
            Ok(DnsRecord::CAA(CAARecord::IssueWild {
                issuer_critical,
                name,
                options,
            }))
        }
        "iodef" => Ok(DnsRecord::CAA(CAARecord::Iodef {
            issuer_critical,
            url: unquoted,
        })),
        other => Err(Error::Parse(format!("unknown CAA tag: {other}"))),
    }
}

fn parse_api_error(body: &str) -> Option<String> {
    if body.trim().is_empty() {
        return None;
    }
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(array) = value.as_array() {
            let messages: Vec<String> = array
                .iter()
                .filter_map(|item| serde_json::from_value::<ApiError>(item.clone()).ok())
                .map(format_api_error)
                .filter(|msg| !msg.is_empty())
                .collect();
            if !messages.is_empty() {
                return Some(messages.join("; "));
            }
        }
        if let Ok(item) = serde_json::from_value::<ApiError>(value) {
            let msg = format_api_error(item);
            if !msg.is_empty() {
                return Some(msg);
            }
        }
    }
    None
}

fn format_api_error(item: ApiError) -> String {
    match (item.error_code, item.error_message) {
        (Some(code), Some(msg)) => format!("[{code}] {msg}"),
        (Some(code), None) => format!("[{code}]"),
        (None, Some(msg)) => msg,
        (None, None) => String::new(),
    }
}

fn contains_token_expired(message: &str) -> bool {
    message.contains("60001") || message.to_ascii_lowercase().contains("invalid token")
}

fn path_escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        if is_path_safe(byte) {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}

fn is_path_safe(byte: u8) -> bool {
    matches!(
        byte,
        b'A'..=b'Z'
            | b'a'..=b'z'
            | b'0'..=b'9'
            | b'-'
            | b'.'
            | b'_'
            | b'~'
            | b'!'
            | b'$'
            | b'&'
            | b'\''
            | b'('
            | b')'
            | b'*'
            | b'+'
            | b','
            | b';'
            | b'='
            | b':'
            | b'@'
    )
}

#[derive(Deserialize, Debug)]
struct PaginationInfo {
    #[serde(default, rename = "totalCount")]
    total_results: Option<u64>,
}

#[derive(Deserialize, Debug)]
struct ZonesListResponse {
    #[serde(default)]
    zones: Vec<ZoneEntry>,
    #[serde(default, rename = "resultInfo")]
    pagination: Option<PaginationInfo>,
}

#[derive(Deserialize, Debug, Default)]
struct ZoneProperties {
    #[serde(default)]
    name: String,
    #[serde(default, rename = "type")]
    zone_type: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ZoneEntry {
    #[serde(default)]
    properties: ZoneProperties,
}

#[derive(Deserialize, Debug)]
struct ZoneRrsetsResponse {
    #[serde(default, rename = "rrSets")]
    rrsets: Vec<ZoneRrset>,
    #[serde(default, rename = "resultInfo")]
    pagination: Option<PaginationInfo>,
}

#[derive(Deserialize, Debug)]
struct ZoneRrset {
    #[serde(default, rename = "ownerName")]
    owner_name: String,
    #[serde(default, rename = "rrtype", alias = "rrType")]
    rrtype: String,
    #[serde(default)]
    ttl: u32,
    #[serde(default)]
    rdata: Vec<String>,
}

const ULTRADNS_PAGE_SIZE: u64 = 1000;
const ULTRADNS_MAX_PAGES: u64 = 100;

fn ultradns_record_type_from_str(value: &str) -> Option<DnsRecordType> {
    let head = value
        .split_whitespace()
        .next()
        .unwrap_or(value)
        .trim_end_matches(',');
    match head.to_ascii_uppercase().as_str() {
        "A" => Some(DnsRecordType::A),
        "AAAA" => Some(DnsRecordType::AAAA),
        "CNAME" => Some(DnsRecordType::CNAME),
        "NS" => Some(DnsRecordType::NS),
        "MX" => Some(DnsRecordType::MX),
        "TXT" => Some(DnsRecordType::TXT),
        "SRV" => Some(DnsRecordType::SRV),
        "TLSA" => Some(DnsRecordType::TLSA),
        "CAA" => Some(DnsRecordType::CAA),
        _ => None,
    }
}

impl UltraDnsProvider {
    pub(crate) async fn list_zone_infos(&self) -> crate::Result<Vec<crate::ZoneInfo>> {
        let mut out = Vec::new();
        let mut offset = 0u64;
        loop {
            let url = format!(
                "{}/v3/zones?limit={ULTRADNS_PAGE_SIZE}&offset={offset}",
                self.endpoint
            );
            let response: ZonesListResponse = self
                .send_with_token::<ZonesListResponse, ()>(Method::GET, &url, None)
                .await?;
            let count = response.zones.len() as u64;
            for entry in response.zones {
                let name = entry.properties.name.trim_end_matches('.').to_string();
                if name.is_empty() {
                    continue;
                }
                out.push(crate::ZoneInfo {
                    id: name.clone(),
                    name,
                    status: entry.properties.zone_type,
                });
            }
            let total = response
                .pagination
                .as_ref()
                .and_then(|p| p.total_results)
                .unwrap_or(0);
            offset += ULTRADNS_PAGE_SIZE;
            if count == 0 || offset >= total || offset / ULTRADNS_PAGE_SIZE > ULTRADNS_MAX_PAGES {
                break;
            }
        }
        Ok(out)
    }

    pub(crate) async fn list_zone_records(
        &self,
        zone: &str,
    ) -> crate::Result<Vec<crate::ZoneRecord>> {
        let zone_fqdn = zone.into_fqdn().into_owned();
        let zone_name = zone.into_name().to_ascii_lowercase();
        let mut out = Vec::new();
        let mut offset = 0u64;
        loop {
            let url = format!(
                "{base}/v3/zones/{zone}/rrsets?limit={ULTRADNS_PAGE_SIZE}&offset={offset}",
                base = self.endpoint,
                zone = path_escape(&zone_fqdn),
            );
            let response: ZoneRrsetsResponse = self
                .send_with_token::<ZoneRrsetsResponse, ()>(Method::GET, &url, None)
                .await?;
            let count = response.rrsets.len() as u64;
            for rrset in response.rrsets {
                let Some(record_type) = ultradns_record_type_from_str(&rrset.rrtype) else {
                    continue;
                };
                let owner = rrset.owner_name.trim_end_matches('.').to_ascii_lowercase();
                let name = if owner.is_empty() {
                    "@".to_string()
                } else {
                    crate::utils::strip_origin_from_name(&owner, &zone_name, Some("@"))
                };
                let type_str = rrset
                    .rrtype
                    .split_whitespace()
                    .next()
                    .unwrap_or(&rrset.rrtype)
                    .trim_end_matches(',')
                    .to_ascii_uppercase();
                for value in rrset.rdata {
                    let Ok(record) = parse_rdata(record_type, &value) else {
                        continue;
                    };
                    let content = record.to_string();
                    out.push(crate::ZoneRecord {
                        id: format!("{owner}|{type_str}|{content}"),
                        name: name.clone(),
                        record_type: type_str.clone(),
                        content,
                        ttl: rrset.ttl,
                        record,
                    });
                }
            }
            let total = response
                .pagination
                .as_ref()
                .and_then(|p| p.total_results)
                .unwrap_or(0);
            offset += ULTRADNS_PAGE_SIZE;
            if count == 0 || offset >= total || offset / ULTRADNS_PAGE_SIZE > ULTRADNS_MAX_PAGES {
                break;
            }
        }
        Ok(out)
    }
}
