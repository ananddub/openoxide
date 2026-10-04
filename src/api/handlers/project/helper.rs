use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::Project as ProjectProto;

pub fn project_to_proto(p: crate::flash_gen::models::Projects) -> ProjectProto {
    ProjectProto {
        id: p.id,
        name: p.name,
        description: p.description,
        environment_variables: p.env_var,
        organization_id: p.organization_id,
        created_at: p.created_at,
        updated_at: p.updated_at,
    }
}

pub fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("project not found"),
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        other => {
            tracing::error!(error = %other, "project database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}
