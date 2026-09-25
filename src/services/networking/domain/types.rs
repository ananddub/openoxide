use serde::Deserialize;

use crate::flash_gen::models::Domains;

#[derive(Debug, Deserialize)]
pub struct CreateDomainDto {
    pub host: String,
    #[serde(default)]
    pub https: bool,
    pub port: Option<i64>,
    #[serde(default = "default_path")]
    pub path: String,
    #[serde(default = "default_path")]
    pub internal_path: String,
    pub custom_entrypoint: Option<String>,
    pub service_name: Option<String>,
    pub custom_cert_resolver: Option<String>,
    #[serde(default)]
    pub strip_path: bool,
    #[serde(default = "default_middlewares")]
    pub middlewares: String,
    #[serde(default = "default_domain_type")]
    pub domain_type: String,
    #[serde(default = "default_certificate_type")]
    pub certificate_type: String,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PatchDomainDto {
    pub host: Option<String>,
    pub https: Option<bool>,
    pub port: Option<i64>,
    pub path: Option<String>,
    pub internal_path: Option<String>,
    pub custom_entrypoint: Option<String>,
    pub service_name: Option<String>,
    pub custom_cert_resolver: Option<String>,
    pub strip_path: Option<bool>,
    pub middlewares: Option<String>,
    pub certificate_type: Option<String>,
}

fn default_path() -> String {
    "/".into()
}

fn default_middlewares() -> String {
    "[]".into()
}

fn default_domain_type() -> String {
    "APPLICATION".into()
}

fn default_certificate_type() -> String {
    "NONE".into()
}

#[derive(Debug, Clone)]
pub struct DomainRecord {
    pub id: i64,
    pub host: String,
    pub https: i64,
    pub port: Option<i64>,
    pub path: Option<String>,
    pub internal_path: Option<String>,
    pub custom_entrypoint: Option<String>,
    pub service_name: Option<String>,
    pub custom_cert_resolver: Option<String>,
    pub strip_path: i64,
    pub middlewares: String,
    pub domain_type: String,
    pub certificate_type: String,
    pub service_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Domains> for DomainRecord {
    fn from(v: Domains) -> Self {
        Self {
            id: v.id,
            host: v.host,
            https: v.https,
            port: v.port,
            path: v.path,
            internal_path: v.internal_path,
            custom_entrypoint: v.custom_entrypoint,
            service_name: v.service_name,
            custom_cert_resolver: v.custom_cert_resolver,
            strip_path: v.strip_path,
            middlewares: v.middlewares,
            domain_type: v.domain_type,
            certificate_type: v.certificate_type,
            service_id: v.service_id,
            created_at: v.created_at,
            updated_at: v.updated_at,
        }
    }
}

/// Result of a live DNS check for a domain host.
#[derive(Debug, Clone)]
pub struct DnsValidation {
    pub is_valid: bool,
    pub resolved_ips: Vec<String>,
    pub expected_ips: Vec<String>,
    pub error: Option<String>,
}
