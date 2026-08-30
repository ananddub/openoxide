use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::ResourceResponse;
use crate::flash_gen::models::{CertificateRenewals, Certificates};

pub fn map_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("certificate not found"),
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Protocol(msg) => ConnectError::invalid_argument(msg.clone()),
        other => {
            tracing::error!(error = %other, "certificate database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

pub fn cert_to_resource(dto: CertificateResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: 0,
        kind: "certificate".into(),
        payload,
        ..Default::default()
    }
}

pub fn renewal_to_resource(dto: CertificateRenewalDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "certificate_renewal".into(),
        payload,
        ..Default::default()
    }
}

// ── Helper DTOs for serializing ResourceResponse payloads ────────────

#[derive(Debug, Clone, serde::Serialize)]
pub struct CertificateResponseDto {
    pub id: String,
    pub name: String,
    pub certificate_data: String,
    pub has_private_key: bool,
    pub certificate_path: String,
    pub auto_renew: i64,
    pub server_id: Option<String>,
    pub organization_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Certificates> for CertificateResponseDto {
    fn from(val: Certificates) -> Self {
        Self {
            id: val.id.to_string(),
            name: val.name,
            certificate_data: val.certificate_data,
            has_private_key: !val.private_key.is_empty(),
            certificate_path: val.certificate_path,
            auto_renew: val.auto_renew,
            server_id: val.server_id.map(|id| id.to_string()),
            organization_id: val.organization_id,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CertificateRenewalDto {
    pub id: i64,
    pub certificate_id: i64,
    pub organization_id: i64,
    pub status: String,
    pub previous_expires_at: Option<i64>,
    pub new_expires_at: Option<i64>,
    pub error: Option<String>,
    pub started_at: i64,
    pub finished_at: Option<i64>,
}

impl From<CertificateRenewals> for CertificateRenewalDto {
    fn from(v: CertificateRenewals) -> Self {
        Self {
            id: v.id,
            certificate_id: v.certificate_id,
            organization_id: v.organization_id,
            status: v.status,
            previous_expires_at: v.previous_expires_at,
            new_expires_at: v.new_expires_at,
            error: v.error,
            started_at: v.started_at,
            finished_at: v.finished_at,
        }
    }
}
