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

pub fn provider_type_to_proto(value: DnsProviderType) -> ProtoDnsProviderType {
    match value {
        DnsProviderType::Cloudflare => ProtoDnsProviderType::Cloudflare,
        DnsProviderType::Route53 => ProtoDnsProviderType::Route53,
        DnsProviderType::Hetzner => ProtoDnsProviderType::Hetzner,
        DnsProviderType::DigitalOcean => ProtoDnsProviderType::Digitalocean,
        DnsProviderType::Godaddy => ProtoDnsProviderType::Godaddy,
        DnsProviderType::Porkbun => ProtoDnsProviderType::Porkbun,
        DnsProviderType::Infomaniak => ProtoDnsProviderType::Infomaniak,
        DnsProviderType::Ovh => ProtoDnsProviderType::Ovh,
    }
}

pub fn proto_to_provider_type(
    value: ::buffa::EnumValue<ProtoDnsProviderType>,
) -> Result<DnsProviderType, ConnectError> {
    match value {
        ::buffa::EnumValue::Known(ProtoDnsProviderType::Cloudflare) => {
            Ok(DnsProviderType::Cloudflare)
        }
        ::buffa::EnumValue::Known(ProtoDnsProviderType::Route53) => Ok(DnsProviderType::Route53),
        ::buffa::EnumValue::Known(ProtoDnsProviderType::Hetzner) => Ok(DnsProviderType::Hetzner),
        ::buffa::EnumValue::Known(ProtoDnsProviderType::Digitalocean) => {
            Ok(DnsProviderType::DigitalOcean)
        }
        ::buffa::EnumValue::Known(ProtoDnsProviderType::Godaddy) => Ok(DnsProviderType::Godaddy),
        ::buffa::EnumValue::Known(ProtoDnsProviderType::Porkbun) => Ok(DnsProviderType::Porkbun),
        ::buffa::EnumValue::Known(ProtoDnsProviderType::Infomaniak) => {
            Ok(DnsProviderType::Infomaniak)
        }
        ::buffa::EnumValue::Known(ProtoDnsProviderType::Ovh) => Ok(DnsProviderType::Ovh),
        _ => Err(ConnectError::invalid_argument(
            "unsupported DNS provider type",
        )),
    }
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
