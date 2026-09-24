use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    AuditLog as AuditLogProto,
};
use crate::flash_gen::models::AuditLogs;

pub fn map_db_error(err: sqlx::Error) -> ConnectError {
    tracing::error!(error = %err, "audit log database operation failed");
    ConnectError::new(ErrorCode::Internal, "audit log database operation failed")
}

pub fn csv_field(value: &str) -> String {
    value.replace('"', "\"\"").replace(['\r', '\n'], " ")
}

pub fn log_to_proto(row: AuditLogs) -> AuditLogProto {
    AuditLogProto {
        id: row.id,
        user_email: row.user_email,
        user_role: row.user_role,
        action: row.action,
        resource_type: row.resource_type,
        resource_id: row.resource_id,
        resource_name: row.resource_name,
        metadata: row.metadata,
        organization_id: row.organization_id,
        user_id: row.user_id,
        created_at: row.created_at,
        ..Default::default()
    }
}
