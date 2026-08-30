use connectrpc::{ConnectError, ErrorCode};
use serde::{Deserialize, Serialize};

use crate::api::connect::generated::openoxide::api::v1 as proto;
use crate::services::permission::group_service::{PermissionGroupDto, PermissionPolicyDto};
use crate::services::permission::PermissionGroupError;

pub fn map_error(err: PermissionGroupError) -> ConnectError {
    match &err {
        PermissionGroupError::Invalid(msg) => ConnectError::invalid_argument(msg.clone()),
        PermissionGroupError::Escalation(msg) => {
            ConnectError::new(ErrorCode::PermissionDenied, msg.clone())
        }
        PermissionGroupError::NotFound => ConnectError::not_found(err.to_string()),
        PermissionGroupError::Database(e) => {
            if let sqlx::Error::Database(db_err) = e {
                if db_err.is_unique_violation() {
                    return ConnectError::new(ErrorCode::AlreadyExists, "group name already exists");
                }
            }
            tracing::error!(error = %err, "permission group operation failed");
            ConnectError::new(ErrorCode::Internal, "permission operation failed")
        }
    }
}

pub fn group_to_resource(dto: PermissionGroupDto) -> proto::ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    proto::ResourceResponse {
        id: dto.id,
        kind: "permission_group".into(),
        payload,
        ..Default::default()
    }
}

pub fn member_to_resource(dto: OrganizationMemberDto) -> proto::ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    proto::ResourceResponse {
        id: dto.id.unwrap_or(0),
        kind: "organization_member".into(),
        payload,
        ..Default::default()
    }
}

pub fn invite_to_resource(dto: OrganizationInviteDto) -> proto::ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    proto::ResourceResponse {
        id: dto.id,
        kind: "organization_invite".into(),
        payload,
        ..Default::default()
    }
}

pub fn policy_to_resource(dto: PermissionPolicyDto) -> proto::ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    proto::ResourceResponse {
        id: dto.id,
        kind: "permission_policy".into(),
        payload,
        ..Default::default()
    }
}

// ── Permission payload DTOs ───────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AssignPermissionGroupDto {
    pub group_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberRoleDto {
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct OrganizationMemberDto {
    pub id: Option<i64>,
    pub user_id: i64,
    pub email: String,
    pub avatar: Option<String>,
    pub organization_id: i64,
    pub role: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize)]
pub struct OrganizationInviteDto {
    pub id: i64,
    pub email: String,
    pub role: Option<String>,
    pub group_id: i64,
    pub organization_id: i64,
    pub invited_by: i64,
    pub expired_at: i64,
    pub created_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct AcceptOrganizationInviteDto {
    pub token: String,
}

impl From<crate::flash_gen::models::OrganizationInvites> for OrganizationInviteDto {
    fn from(value: crate::flash_gen::models::OrganizationInvites) -> Self {
        Self {
            id: value.id,
            email: value.email,
            role: value.role,
            group_id: value.group_id,
            organization_id: value.organization_id,
            invited_by: value.invited_by,
            expired_at: value.expired_at,
            created_at: value.created_at,
        }
    }
}
