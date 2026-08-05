use crate::db::models::certificates::Certificate;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateCertificateDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub certificate_data: String,
    pub private_key: String,
    pub certificate_path: String,
    pub auto_renew: i64,
    pub server_id: Option<String>,
    pub organization_id: i64,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchCertificateDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub certificate_data: Option<String>,
    pub private_key: Option<String>,
    pub certificate_path: Option<String>,
    pub auto_renew: Option<i64>,
    pub server_id: Option<Option<String>>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct CertificateResponseDto {
    pub id: String,
    pub name: String,
    pub certificate_data: String,
    pub private_key: String,
    pub certificate_path: String,
    pub auto_renew: i64,
    pub server_id: Option<String>,
    pub organization_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Certificate> for CertificateResponseDto {
    fn from(val: Certificate) -> Self {
        Self {
            id: val.id.unwrap_or_default().to_string(),
            name: val.name,
            certificate_data: val.certificate_data,
            private_key: val.private_key,
            certificate_path: val.certificate_path,
            auto_renew: val.auto_renew,
            server_id: val.server_id.map(|id| id.to_string()),
            organization_id: val.organization_id,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct RenewCertificateDto {
    pub certificate_data: String,
    pub private_key: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
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

impl From<crate::db::repository::certificate_renewals::CertificateRenewal>
    for CertificateRenewalDto
{
    fn from(v: crate::db::repository::certificate_renewals::CertificateRenewal) -> Self {
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
