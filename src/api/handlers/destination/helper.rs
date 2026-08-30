use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::ResourceResponse;
use crate::flash_gen::models::Destinations;

/// Helper struct to extract the string ID from payload JSON for destination entities.
#[derive(serde::Deserialize)]
pub struct DestinationId {
    pub id: String,
}

pub fn map_sqlx_error(error: sqlx::Error) -> ConnectError {
    match &error {
        sqlx::Error::RowNotFound => ConnectError::not_found("destination not found"),
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        other => {
            tracing::error!(error = %other, "destination database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

pub fn destination_to_resource(dto: DestinationResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id.parse::<i64>().unwrap_or(0),
        kind: "destination".into(),
        payload,
        ..Default::default()
    }
}

// ── Helper DTOs for deserializing ResourceRequest payloads ────────────

#[derive(Debug, serde::Deserialize)]
pub struct TestDestinationDto {
    pub provider: String,
    pub access_key: String,
    pub secret_access_key: String,
    pub bucket: String,
    pub region: String,
    pub endpoint: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DestinationResponseDto {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub access_key: String,
    pub secret_access_key: String,
    pub bucket: String,
    pub region: String,
    pub endpoint: String,
    pub organization_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Destinations> for DestinationResponseDto {
    fn from(val: Destinations) -> Self {
        Self {
            id: val.id.to_string(),
            name: val.name,
            provider: val.provider,
            access_key: val.access_key,
            secret_access_key: val.secret_access_key,
            bucket: val.bucket,
            region: val.region,
            endpoint: val.endpoint,
            organization_id: val.organization_id,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}
