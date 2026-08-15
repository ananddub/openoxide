use os::string_enum;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use validator::Validate;

string_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, poem_openapi::Enum, ts_rs::TS)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[oai(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum DnsProviderType {
        default = Cloudflare;

        Cloudflare => "CLOUDFLARE",
        Route53 => "ROUTE53",
        Hetzner => "HETZNER",
        DigitalOcean => "DIGITALOCEAN",
        Godaddy => "GODADDY",
    }
}

#[derive(Debug, Serialize, Deserialize, Object, ts_rs::TS)]
pub struct DnsProviderDto {
    pub id: i64,
    pub name: String,
    pub provider_type: DnsProviderType,
    pub credentials_json: String,
    pub organization_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize, Validate, Object)]
pub struct CreateDnsProviderDto {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub provider_type: DnsProviderType,
    pub credentials_json: String,
}

#[derive(Debug, Deserialize, Validate, Object)]
pub struct UpdateDnsProviderDto {
    pub name: Option<String>,
    pub provider_type: Option<DnsProviderType>,
    pub credentials_json: Option<String>,
}

#[derive(Debug, Serialize, Object, ts_rs::TS)]
pub struct DnsTestResultDto {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Object, ts_rs::TS)]
pub struct DnsZoneDto {
    pub id: String,
    pub name: String,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Object, ts_rs::TS)]
pub struct DnsRecordDto {
    pub id: String,
    pub zone_id: String,
    pub record_type: String, // "A", "AAAA", "CNAME", "TXT", "MX"
    pub name: String,
    pub content: String,
    pub ttl: Option<u32>,
    pub proxied: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, Object)]
pub struct UpsertDnsRecordDto {
    pub zone_id: String,
    pub record_type: String,
    pub name: String,
    pub content: String,
    pub ttl: Option<u32>,
    pub proxied: Option<bool>,
}
