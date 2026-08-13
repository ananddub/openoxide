use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::services::domain::DomainRecord;

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateDomainDto {
    #[validate(length(min = 1, max = 255))]
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

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchDomainDto {
    #[validate(length(min = 1, max = 255))]
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

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct DomainResponseDto {
    pub id: i64,
    pub host: String,
    pub https: bool,
    pub port: Option<i64>,
    pub path: Option<String>,
    pub internal_path: Option<String>,
    pub custom_entrypoint: Option<String>,
    pub service_name: Option<String>,
    pub custom_cert_resolver: Option<String>,
    pub strip_path: bool,
    pub middlewares: String,
    pub domain_type: String,
    pub certificate_type: String,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<DomainRecord> for DomainResponseDto {
    fn from(value: DomainRecord) -> Self {
        Self {
            id: value.id,
            host: value.host,
            https: value.https != 0,
            port: value.port,
            path: value.path,
            internal_path: value.internal_path,
            custom_entrypoint: value.custom_entrypoint,
            service_name: value.service_name,
            custom_cert_resolver: value.custom_cert_resolver,
            strip_path: value.strip_path != 0,
            middlewares: value.middlewares,
            domain_type: value.domain_type,
            certificate_type: value.certificate_type,
            application_id: value.application_id,
            compose_id: value.compose_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
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
