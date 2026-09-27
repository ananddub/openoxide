use crate::db::queries::FlashQueries;
use crate::flash_gen::{
    dns_providers::{DnsProvidersCreateParams, DnsProvidersUpdateParams},
    models::DnsProviders,
};
use auto_di::singleton;
use os::string_enum;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

mod dispatch;
mod dnsupdate_bridge;
mod providers;
mod validation;

string_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DnsProviderType {
        default = Cloudflare;

        Cloudflare => "CLOUDFLARE",
        Route53 => "ROUTE53",
        Hetzner => "HETZNER",
        DigitalOcean => "DIGITALOCEAN",
        Godaddy => "GODADDY",
        Porkbun => "PORKBUN",
        Infomaniak => "INFOMANIAK",
        Ovh => "OVH",
        Desec => "DESEC",
        Bunny => "BUNNY",
        Constellix => "CONSTELLIX",
        DnsMadeEasy => "DNSMADEEASY",
        Exoscale => "EXOSCALE",
        Nifcloud => "NIFCLOUD",
        WebSupport => "WEBSUPPORT",
        Spaceship => "SPACESHIP",
        Dnsimple => "DNSIMPLE",
        GandiV5 => "GANDIV5",
        NameDotCom => "NAMEDOTCOM",
        NameSilo => "NAMESILO",
        DuckDns => "DUCKDNS",
        FreeMyIp => "FREEMYIP",
        Ipv64 => "IPV64",
        Ddnss => "DDNSS",
        Dynu => "DYNU",
        Plesk => "PLESK",
        Cpanel => "CPANEL",
        GoogleCloudDns => "GOOGLECLOUDDNS",
        Ionos => "IONOS",
        HostingDe => "HOSTINGDE",
        Netcup => "NETCUP",
        Netlify => "NETLIFY",
        HuaweiCloud => "HUAWEICLOUD",
        BaiduCloud => "BAIDUCLOUD",
        AzureDns => "AZUREDNS",
        IbmCloud => "IBMCLOUD",
        Hurricane => "HURRICANE",
        Hostinger => "HOSTINGER",
        Autodns => "AUTODNS",
        Lightsail => "LIGHTSAIL",
        EdgeDns => "EDGEDNS",
        Scaleway => "SCALEWAY",
        Gcore => "GCORE",
        Vercel => "VERCEL",
        Vultr => "VULTR",
        BluecatV2 => "BLUECATV2",
        EasyDns => "EASYDNS",
        Joker => "JOKER",
        MythicBeasts => "MYTHICBEASTS",
        Namecheap => "NAMECHEAP",
        Transip => "TRANSIP",
        Ns1 => "NS1",
        LuaDns => "LUADNS",
        ClouDns => "CLOUDNS",
        Glesys => "GLESYS",
        Dreamhost => "DREAMHOST",
        Alidns => "ALIDNS",
        TencentCloud => "TENCENTCLOUD",
        Volcengine => "VOLCENGINE",
        YandexCloud => "YANDEXCLOUD",
        Domeneshop => "DOMENESHOP",
        Safedns => "SAFEDNS",
        SimplyCom => "SIMPLYCOM",
        ArvanCloud => "ARVANCLOUD",
        Inwx => "INWX",
        UltraDns => "ULTRADNS",
        Infoblox => "INFOBLOX",
        OracleCloud => "ORACLECLOUD",
        Rfc2136 => "RFC2136",
    }
}

// Wire format is the canonical `string_enum` value (SCREAMING_SNAKE but not
// word-split: `DIGITALOCEAN`, not serde's default `DIGITAL_OCEAN`).
impl Serialize for DnsProviderType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for DnsProviderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        Self::from_str(&raw)
            .ok_or_else(|| serde::de::Error::custom(format!("unknown DNS provider type '{raw}'")))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsProviderDto {
    pub id: i64,
    pub name: String,
    pub provider_type: DnsProviderType,
    pub credentials_json: String,
    pub organization_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateDnsProviderDto {
    pub name: String,
    pub provider_type: DnsProviderType,
    pub credentials_json: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDnsProviderDto {
    pub name: Option<String>,
    pub provider_type: Option<DnsProviderType>,
    pub credentials_json: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DnsTestResultDto {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsZoneDto {
    pub id: String,
    pub name: String,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsRecordDto {
    pub id: String,
    pub zone_id: String,
    pub record_type: String, // "A", "AAAA", "CNAME", "TXT", "MX"
    pub name: String,
    pub content: String,
    pub ttl: Option<u32>,
    pub proxied: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertDnsRecordDto {
    pub zone_id: String,
    pub record_type: String,
    pub name: String,
    pub content: String,
    pub ttl: Option<u32>,
    pub proxied: Option<bool>,
}

/// One row the UI wants changed. Every provider keys rows differently — a Cloudflare
/// record id, a Route53 `type:name` pair, an rrset id or just name+type+value — so the
/// whole identity travels with the request and each path uses the strongest hint it
/// understands. Matching on a bare name is never enough: one name holds several types
/// and one type holds several values.
#[derive(Debug, Clone, Default)]
pub struct DnsRecordTargetDto {
    pub zone_id: String,
    pub id: String,
    pub name: String,
    pub record_type: String,
    pub content: String,
}

#[derive(Debug)]
pub struct UpdateDnsRecordDto {
    pub target: DnsRecordTargetDto,
    pub next: UpsertDnsRecordDto,
}

pub const DNS_SECRET_MASK: &str = "********";

#[derive(Debug, Error)]
pub enum DnsServiceError {
    #[error("DNS provider not found")]
    NotFound,
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("HTTP client error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("DNS provider failure: {0}")]
    ProviderError(String),
}

pub struct DnsService {
    queries: Arc<FlashQueries>,
    client: Client,
}

#[singleton]
impl DnsService {
    pub fn new(queries: Arc<FlashQueries>) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();
        Self { queries, client }
    }

    pub async fn list_providers(
        &self,
        organization_id: i64,
    ) -> Result<Vec<DnsProviderDto>, DnsServiceError> {
        let providers = self
            .queries
            .dns_providers_list_by_organization(organization_id)
            .await?;
        Ok(providers.into_iter().map(Self::into_dto).collect())
    }

    pub async fn get_provider(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<DnsProviderDto, DnsServiceError> {
        let provider = match self.queries.dns_providers_get_by_id(id).await {
            Ok(provider) if provider.organization_id == organization_id => provider,
            Ok(_) | Err(sqlx::Error::RowNotFound) => return Err(DnsServiceError::NotFound),
            Err(error) => return Err(error.into()),
        };
        Ok(Self::into_dto(provider))
    }

    pub async fn create_provider(
        &self,
        organization_id: i64,
        body: CreateDnsProviderDto,
    ) -> Result<DnsProviderDto, DnsServiceError> {
        Self::validate_provider_config(body.provider_type, &body.credentials_json)?;
        let now = chrono::Utc::now().timestamp();

        self.queries
            .dns_providers_create(&DnsProvidersCreateParams {
                name: body.name,
                provider_type: body.provider_type.as_str().to_string(),
                credentials_json: body.credentials_json,
                organization_id,
                created_at: now,
                updated_at: now,
            })
            .await?;
        let id = self.queries.dns_providers_get_last_insert_rowid().await?;
        let created = self.queries.dns_providers_get_by_id(id).await?;
        Ok(Self::into_dto(created))
    }

    pub async fn update_provider(
        &self,
        id: i64,
        organization_id: i64,
        body: UpdateDnsProviderDto,
    ) -> Result<DnsProviderDto, DnsServiceError> {
        let mut existing = match self.queries.dns_providers_get_by_id(id).await {
            Ok(provider) if provider.organization_id == organization_id => provider,
            Ok(_) | Err(sqlx::Error::RowNotFound) => return Err(DnsServiceError::NotFound),
            Err(error) => return Err(error.into()),
        };

        if let Some(name) = body.name {
            existing.name = name;
        }
        if let Some(ptype) = body.provider_type {
            Self::ensure_supported_provider(ptype)?;
            existing.provider_type = ptype.as_str().to_string();
        }
        if let Some(creds) = body.credentials_json {
            let merged = merge_masked_credentials(&existing.credentials_json, &creds)?;
            let provider_type = Self::parse_provider_type(&existing.provider_type)?;
            Self::validate_provider_config(provider_type, &merged)?;
            existing.credentials_json = merged;
        }
        existing.updated_at = chrono::Utc::now().timestamp();

        self.queries
            .dns_providers_update(&DnsProvidersUpdateParams {
                name: existing.name.clone(),
                provider_type: existing.provider_type.clone(),
                credentials_json: existing.credentials_json.clone(),
                updated_at: existing.updated_at,
                id,
                organization_id,
            })
            .await?;
        Ok(Self::into_dto(existing))
    }

    pub async fn delete_provider(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<bool, DnsServiceError> {
        match self.queries.dns_providers_delete(id, organization_id).await {
            Ok(_) => Ok(true),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(error) => Err(error.into()),
        }
    }

    async fn provider_for_organization(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<DnsProviders, DnsServiceError> {
        match self.queries.dns_providers_get_by_id(id).await {
            Ok(provider) if provider.organization_id == organization_id => Ok(provider),
            Ok(_) | Err(sqlx::Error::RowNotFound) => Err(DnsServiceError::NotFound),
            Err(error) => Err(error.into()),
        }
    }

    pub(crate) fn into_dto(p: DnsProviders) -> DnsProviderDto {
        let p_type: DnsProviderType = p
            .provider_type
            .parse()
            .unwrap_or(DnsProviderType::Cloudflare);

        DnsProviderDto {
            id: p.id,
            name: p.name,
            provider_type: p_type,
            credentials_json: mask_credentials(&p.credentials_json),
            organization_id: p.organization_id,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}

fn extract_credential_field(json_str: &str, field_name: &str) -> String {
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(json_str) {
        if let Some(s) = val[field_name].as_str() {
            return s.trim().to_string();
        }
    }
    json_str
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_string()
}

// The edit form sends back masked secrets as-is; keep the stored value for
// every field that still carries the mask.
fn merge_masked_credentials(
    existing_json: &str,
    incoming_json: &str,
) -> Result<String, DnsServiceError> {
    let Ok(mut incoming) = serde_json::from_str::<serde_json::Value>(incoming_json) else {
        return Ok(incoming_json.to_string());
    };
    let (Some(obj), Ok(existing)) = (
        incoming.as_object_mut(),
        serde_json::from_str::<serde_json::Value>(existing_json),
    ) else {
        return Ok(incoming_json.to_string());
    };
    for (key, value) in obj.iter_mut() {
        if value.as_str() == Some(DNS_SECRET_MASK) {
            match existing
                .get(key)
                .filter(|v| v.as_str() != Some(DNS_SECRET_MASK))
            {
                Some(stored) => *value = stored.clone(),
                None => {
                    return Err(DnsServiceError::ProviderError(format!(
                        "DNS credentials field '{key}' must be re-entered"
                    )));
                }
            }
        }
    }
    Ok(serde_json::to_string(&incoming).unwrap_or_else(|_| incoming_json.to_string()))
}

fn mask_credentials(json_str: &str) -> String {
    if let Ok(mut val) = serde_json::from_str::<serde_json::Value>(json_str) {
        if let Some(obj) = val.as_object_mut() {
            // Access-key ids and the OVH endpoint are not secrets and stay
            // visible so edit forms can prefill them.
            for key in [
                "apiToken",
                "secretAccessKey",
                "apiKey",
                "authKey",
                "apiSecret",
                "secretApiKey",
                "applicationSecret",
                "consumerKey",
            ] {
                if obj.contains_key(key) {
                    obj.insert(
                        key.to_string(),
                        serde_json::Value::String(DNS_SECRET_MASK.to_string()),
                    );
                }
            }
            return serde_json::to_string(&val).unwrap_or_else(|_| json_str.to_string());
        }
    }
    DNS_SECRET_MASK.to_string()
}
