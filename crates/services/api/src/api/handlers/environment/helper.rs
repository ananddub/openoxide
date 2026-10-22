use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::Environment as EnvironmentProto;

pub fn env_to_proto(e: crate::flash_gen::models::Environments) -> EnvironmentProto {
    EnvironmentProto {
        id: e.id,
        name: e.name,
        description: e.description,
        environment_variables: e.env_var,
        is_default: e.is_default != 0,
        project_id: e.project_id,
        created_at: e.created_at,
        updated_at: e.updated_at,
    }
}

pub fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("environment not found"),
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => {
            ConnectError::not_found("project not found")
        }
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Protocol(msg) => ConnectError::invalid_argument(msg.clone()),
        other => {
            tracing::error!(error = %other, "environment database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}
