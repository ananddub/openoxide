use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::Destination as DestinationProto;
use crate::flash_gen::models::Destinations;

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

pub fn destination_to_proto(destination: Destinations) -> DestinationProto {
    DestinationProto {
        id: destination.id.to_string(),
        name: destination.name,
        provider: destination.provider,
        access_key: destination.access_key,
        secret_access_key: destination.secret_access_key,
        bucket: destination.bucket,
        region: destination.region,
        endpoint: destination.endpoint,
        organization_id: destination.organization_id,
        created_at: destination.created_at,
        updated_at: destination.updated_at,
    }
}
