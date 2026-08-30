use connectrpc::{ConnectError, ErrorCode};
use serde::Serialize;

pub fn map_db_error(err: sqlx::Error) -> ConnectError {
    tracing::error!(error = %err, "audit log database operation failed");
    ConnectError::new(ErrorCode::Internal, "audit log database operation failed")
}

pub fn csv_field(value: &str) -> String {
    value.replace('"', "\"\"").replace(['\r', '\n'], " ")
}

// ── Helper DTOs for ResourceResponse payloads ────────────

#[derive(Debug, Clone, Serialize)]
pub struct AuditLogDto {
    pub id: i64,
    pub user_email: String,
    pub user_role: String,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub resource_name: Option<String>,
    pub metadata: Option<String>,
    pub organization_id: Option<i64>,
    pub user_id: Option<i64>,
    pub created_at: i64,
}

impl From<crate::flash_gen::models::AuditLogs> for AuditLogDto {
    fn from(v: crate::flash_gen::models::AuditLogs) -> Self {
        Self {
            id: v.id,
            user_email: v.user_email,
            user_role: v.user_role,
            action: v.action,
            resource_type: v.resource_type,
            resource_id: v.resource_id,
            resource_name: v.resource_name,
            metadata: v.metadata,
            organization_id: v.organization_id,
            user_id: v.user_id,
            created_at: v.created_at,
        }
    }
}
