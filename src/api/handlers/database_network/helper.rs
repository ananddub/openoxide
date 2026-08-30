use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::ResourceResponse;
use crate::services::database_network::DatabaseNetworkResponseDto;

pub fn map_sqlx_error(error: sqlx::Error) -> ConnectError {
    match &error {
        sqlx::Error::RowNotFound => ConnectError::not_found("database network not found"),
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Protocol(msg) => ConnectError::new(ErrorCode::AlreadyExists, msg.clone()),
        other => {
            tracing::error!(error = %other, "database network operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

pub fn network_to_resource(dto: DatabaseNetworkResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "database_network".into(),
        payload,
        ..Default::default()
    }
}
