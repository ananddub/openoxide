use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::DatabaseNetwork;
use crate::services::networking::database_network::DatabaseNetworkResponseDto;

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

pub fn network_to_proto(dto: DatabaseNetworkResponseDto) -> DatabaseNetwork {
    DatabaseNetwork {
        id: dto.id,
        name: dto.name,
        docker_network_name: dto.docker_network_name,
        description: dto.description,
        external: dto.external,
        server_id: dto.server_id,
        created_at: dto.created_at,
        updated_at: dto.updated_at,
    }
}
