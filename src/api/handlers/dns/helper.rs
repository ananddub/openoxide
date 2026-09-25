use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    DnsProvider as DnsProviderProto, DnsProviderType as ProtoDnsProviderType,
    DnsRecord as DnsRecordProto, DnsZone as DnsZoneProto,
};
use crate::services::networking::dns::{
    DnsProviderDto, DnsProviderType, DnsRecordDto, DnsServiceError, DnsZoneDto,
};

pub fn map_dns_error(error: DnsServiceError) -> ConnectError {
    match error {
        DnsServiceError::NotFound => ConnectError::not_found("DNS provider not found"),
        DnsServiceError::Database(e) => match &e {
            sqlx::Error::RowNotFound => ConnectError::not_found("DNS provider not found"),
            sqlx::Error::Database(db) if db.is_unique_violation() => {
                ConnectError::new(ErrorCode::AlreadyExists, db.message())
            }
            _ => {
                tracing::error!(error = %e, "dns database operation failed");
                ConnectError::new(ErrorCode::Internal, "database operation failed")
            }
        },
        DnsServiceError::Http(e) => {
            tracing::error!(error = %e, "dns HTTP request failed");
            ConnectError::new(ErrorCode::Internal, "HTTP request failed")
        }
        DnsServiceError::ProviderError(msg) => ConnectError::invalid_argument(msg),
    }
}

const PROVIDER_TYPE_PREFIX: &str = "DNS_PROVIDER_TYPE_";

pub fn provider_type_to_proto(value: DnsProviderType) -> ProtoDnsProviderType {
    let name = format!("{PROVIDER_TYPE_PREFIX}{}", value.as_str());
    <ProtoDnsProviderType as buffa::Enumeration>::from_proto_name(&name)
        .unwrap_or(ProtoDnsProviderType::DNS_PROVIDER_TYPE_UNSPECIFIED)
}

pub fn proto_to_provider_type(
    value: ::buffa::EnumValue<ProtoDnsProviderType>,
) -> Result<DnsProviderType, ConnectError> {
    let ::buffa::EnumValue::Known(proto) = value else {
        return Err(ConnectError::invalid_argument(
            "unsupported DNS provider type",
        ));
    };
    let name = <ProtoDnsProviderType as buffa::Enumeration>::proto_name(&proto);
    let raw = name
        .strip_prefix(PROVIDER_TYPE_PREFIX)
        .unwrap_or(name)
        .to_ascii_uppercase();
    DnsProviderType::from_str(&raw).ok_or_else(|| {
        ConnectError::invalid_argument(format!("unsupported DNS provider type '{name}'"))
    })
}

pub fn provider_to_proto(dto: DnsProviderDto) -> DnsProviderProto {
    DnsProviderProto {
        id: dto.id,
        name: dto.name,
        provider_type: ::buffa::EnumValue::Known(provider_type_to_proto(dto.provider_type)),
        credentials_json: dto.credentials_json,
        organization_id: dto.organization_id,
        created_at: dto.created_at,
        updated_at: dto.updated_at,
        ..Default::default()
    }
}

pub fn zone_to_proto(dto: DnsZoneDto) -> DnsZoneProto {
    DnsZoneProto {
        id: dto.id,
        name: dto.name,
        status: dto.status,
        ..Default::default()
    }
}

pub fn record_to_proto(dto: DnsRecordDto) -> DnsRecordProto {
    DnsRecordProto {
        id: dto.id,
        zone_id: dto.zone_id,
        record_type: dto.record_type,
        name: dto.name,
        content: dto.content,
        ttl: dto.ttl,
        proxied: dto.proxied,
        ..Default::default()
    }
}
