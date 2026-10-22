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

/// `Organization` message with the caller's effective role attached
/// (ListOrganizations; other callers leave it unset).
pub fn org_with_role_to_proto(
    org: crate::flash_gen::models::Organization,
    role: String,
) -> OrganizationProto {
    let mut proto = org_to_proto(org);
    proto.role = Some(role);
    proto
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
        // Service-level guards (e.g. an owner trying to leave instead of
        // deleting the organization) surface as Protocol errors; without
        // this they became an opaque 500 "database operation failed".
        sqlx::Error::Protocol(message) => {
            ConnectError::new(ErrorCode::FailedPrecondition, message.clone())
        }
        other => {
            tracing::error!(error = %other, "organization database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}
