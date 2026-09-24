use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    Certificate as CertificateProto, CertificateDependencies as CertificateDependenciesProto,
    CertificateRenewal as CertificateRenewalProto,
    CertificateRenewalStatus as ProtoCertificateRenewalStatus,
};
use crate::flash_gen::resource_dependencies::CertificateDependencyCountsRow;
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

pub fn renewal_status_to_proto(value: &str) -> ProtoCertificateRenewalStatus {
    match value.to_ascii_uppercase().as_str() {
        "RUNNING" => ProtoCertificateRenewalStatus::Running,
        "SUCCEEDED" => ProtoCertificateRenewalStatus::Succeeded,
        "FAILED" => ProtoCertificateRenewalStatus::Failed,
        _ => ProtoCertificateRenewalStatus::Unspecified,
    }
}

pub fn cert_to_proto(value: Certificates) -> CertificateProto {
    CertificateProto {
        id: value.id,
        name: value.name,
        certificate_data: value.certificate_data,
        has_private_key: !value.private_key.is_empty(),
        certificate_path: value.certificate_path,
        auto_renew: value.auto_renew != 0,
        server_id: value.server_id.map(|id| id.to_string()),
        organization_id: value.organization_id,
        created_at: value.created_at,
        updated_at: value.updated_at,
        ..Default::default()
    }
}

pub fn renewal_to_proto(value: CertificateRenewals) -> CertificateRenewalProto {
    let status = renewal_status_to_proto(&value.status);
    CertificateRenewalProto {
        id: value.id,
        certificate_id: value.certificate_id,
        organization_id: value.organization_id,
        status: ::buffa::EnumValue::Known(status),
        previous_expires_at: value.previous_expires_at,
        new_expires_at: value.new_expires_at,
        error: value.error,
        started_at: value.started_at,
        finished_at: value.finished_at,
        ..Default::default()
    }
}

pub fn dependencies_to_proto(row: CertificateDependencyCountsRow) -> CertificateDependenciesProto {
    CertificateDependenciesProto {
        renewals: row.renewals,
        running_renewals: row.running_renewals,
        ..Default::default()
    }
}
