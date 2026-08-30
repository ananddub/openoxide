use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::Organization as OrganizationProto;

pub fn org_to_proto(org: crate::flash_gen::models::Organization) -> OrganizationProto {
    OrganizationProto {
        id: org.id,
        name: org.name,
        logo: org.logo,
        slug: org.slug,
        owner_id: org.owner_id,
        created_at: org.created_at,
        updated_at: org.updated_at,
        ..Default::default()
    }
}

pub fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("organization not found"),
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => {
            ConnectError::not_found("owner not found")
        }
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        other => {
            tracing::error!(error = %other, "organization database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}
