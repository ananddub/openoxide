use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::ResourceResponse;
use crate::services::networking::domain::DomainRecord;

pub fn map_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("domain not found"),
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Protocol(msg) => ConnectError::new(ErrorCode::AlreadyExists, msg.clone()),
        other => {
            tracing::error!(error = %other, "domain database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

pub fn domain_to_resource(dto: DomainResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "domain".into(),
        payload,
        ..Default::default()
    }
}

// ── Helper DTOs for serializing ResourceResponse payloads ────────────

#[derive(Debug, Clone, serde::Serialize)]
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
    pub service_id: Option<i64>,
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
            service_id: value.service_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
