use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::ResourceResponse;
use crate::services::networking::dns::{DnsProviderDto, DnsRecordDto, DnsServiceError, DnsZoneDto};

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

pub fn provider_to_resource(dto: DnsProviderDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "dns_provider".into(),
        payload,
        ..Default::default()
    }
}

pub fn zone_to_resource(dto: DnsZoneDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: 0,
        kind: "dns_zone".into(),
        payload,
        ..Default::default()
    }
}

pub fn record_to_resource(dto: DnsRecordDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: 0,
        kind: "dns_record".into(),
        payload,
        ..Default::default()
    }
}
