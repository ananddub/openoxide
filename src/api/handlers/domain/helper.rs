use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    Domain as DomainProto, DomainCertificateType as ProtoDomainCertificateType,
    DomainType as ProtoDomainType,
};
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

pub fn domain_type_to_proto(value: &str) -> ProtoDomainType {
    match value.to_ascii_uppercase().as_str() {
        "APPLICATION" => ProtoDomainType::Application,
        "COMPOSE" => ProtoDomainType::Compose,
        _ => ProtoDomainType::Unspecified,
    }
}

pub fn proto_to_domain_type(value: ::buffa::EnumValue<ProtoDomainType>) -> String {
    match value {
        ::buffa::EnumValue::Known(ProtoDomainType::Compose) => "COMPOSE".into(),
        _ => "APPLICATION".into(),
    }
}

pub fn certificate_type_to_proto(value: &str) -> ProtoDomainCertificateType {
    match value.to_ascii_uppercase().as_str() {
        "LETSENCRYPT" => ProtoDomainCertificateType::Letsencrypt,
        "CUSTOM" => ProtoDomainCertificateType::Custom,
        "NONE" => ProtoDomainCertificateType::None,
        _ => ProtoDomainCertificateType::Unspecified,
    }
}

pub fn proto_to_certificate_type(value: ::buffa::EnumValue<ProtoDomainCertificateType>) -> String {
    match value {
        ::buffa::EnumValue::Known(ProtoDomainCertificateType::Letsencrypt) => "LETSENCRYPT".into(),
        ::buffa::EnumValue::Known(ProtoDomainCertificateType::Custom) => "CUSTOM".into(),
        _ => "NONE".into(),
    }
}

pub fn domain_to_proto(record: DomainRecord) -> DomainProto {
    let domain_type = domain_type_to_proto(&record.domain_type);
    let certificate_type = certificate_type_to_proto(&record.certificate_type);
    DomainProto {
        id: record.id,
        host: record.host,
        https: record.https != 0,
        port: record.port,
        path: record.path,
        internal_path: record.internal_path,
        custom_entrypoint: record.custom_entrypoint,
        service_name: record.service_name,
        custom_cert_resolver: record.custom_cert_resolver,
        strip_path: record.strip_path != 0,
        middlewares: record.middlewares,
        domain_type: ::buffa::EnumValue::Known(domain_type),
        certificate_type: ::buffa::EnumValue::Known(certificate_type),
        service_id: record.service_id,
        created_at: record.created_at,
        updated_at: record.updated_at,
        ..Default::default()
    }
}
