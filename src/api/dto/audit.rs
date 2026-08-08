use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize, poem_openapi::Object)]
pub struct AuditLogQueryDto {
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct AuditLogDto {
    pub id: Option<i64>,
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

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct AuditLogPageDto {
    pub items: Vec<AuditLogDto>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

impl From<crate::db::models::audit_logs::AuditLog> for AuditLogDto {
    fn from(v: crate::db::models::audit_logs::AuditLog) -> Self {
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
