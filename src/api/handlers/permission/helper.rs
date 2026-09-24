use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    PermissionGroup as PermissionGroupProto, PermissionInvite as PermissionInviteProto,
    PermissionMember as PermissionMemberProto, PermissionPolicy as PermissionPolicyProto,
    PermissionPolicyEffect as ProtoPermissionPolicyEffect,
    PermissionUserRole as ProtoPermissionUserRole,
};
use crate::services::identity::permission::group_service::{
    PermissionGroupDto, PermissionPolicyDto,
};
use crate::services::identity::permission::PermissionGroupError;

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

// ── Enum mapping ──────────────────────────────────────────────────

/// Map a `UserRole` string (`OWNER`/`ADMIN`/`MEMBER`, any case) to the proto
/// enum; unknown values collapse to `UNSPECIFIED`.
pub fn user_role_str_to_proto(value: &str) -> ProtoPermissionUserRole {
    match value.to_ascii_uppercase().as_str() {
        "OWNER" => ProtoPermissionUserRole::Owner,
        "ADMIN" => ProtoPermissionUserRole::Admin,
        "MEMBER" => ProtoPermissionUserRole::Member,
        _ => ProtoPermissionUserRole::Unspecified,
    }
}

/// Optional projection of the above: unknown or unset roles stay absent.
pub fn user_role_str_to_opt(
    value: Option<&String>,
) -> Option<::buffa::EnumValue<ProtoPermissionUserRole>> {
    let role = value?;
    match role.to_ascii_uppercase().as_str() {
        "OWNER" => Some(::buffa::EnumValue::Known(ProtoPermissionUserRole::Owner)),
        "ADMIN" => Some(::buffa::EnumValue::Known(ProtoPermissionUserRole::Admin)),
        "MEMBER" => Some(::buffa::EnumValue::Known(ProtoPermissionUserRole::Member)),
        _ => None,
    }
}

/// Map the request role enum to the service's `USER_ROLE` string. Only
/// `ADMIN`/`MEMBER` are assignable; the proto validation already rejects
/// `UNSPECIFIED`/`OWNER`, but stay defensive here.
pub fn proto_to_user_role(
    value: ::buffa::EnumValue<ProtoPermissionUserRole>,
) -> Result<String, ConnectError> {
    match value {
        ::buffa::EnumValue::Known(ProtoPermissionUserRole::Owner) => Ok("OWNER".into()),
        ::buffa::EnumValue::Known(ProtoPermissionUserRole::Admin) => Ok("ADMIN".into()),
        ::buffa::EnumValue::Known(ProtoPermissionUserRole::Member) => Ok("MEMBER".into()),
        _ => Err(ConnectError::invalid_argument("role must be ADMIN or MEMBER")),
    }
}

/// Map the request effect enum to the service's `PolicyEffect` string.
pub fn proto_to_policy_effect(
    value: ::buffa::EnumValue<ProtoPermissionPolicyEffect>,
) -> Result<String, ConnectError> {
    match value {
        ::buffa::EnumValue::Known(ProtoPermissionPolicyEffect::Grant) => Ok("GRANT".into()),
        ::buffa::EnumValue::Known(ProtoPermissionPolicyEffect::Deny) => Ok("DENY".into()),
        _ => Err(ConnectError::invalid_argument("effect must be GRANT or DENY")),
    }
}

// ── Response builders ─────────────────────────────────────────────

pub fn group_to_proto(dto: PermissionGroupDto) -> PermissionGroupProto {
    PermissionGroupProto {
        id: dto.id,
        name: dto.name,
        is_system: dto.is_system,
        organization_id: dto.organization_id,
        actions: dto.actions,
        created_at: dto.created_at,
        updated_at: dto.updated_at,
        ..Default::default()
    }
}

pub fn policy_to_proto(dto: PermissionPolicyDto) -> PermissionPolicyProto {
    PermissionPolicyProto {
        id: dto.id,
        action: dto.action,
        ..Default::default()
    }
}

pub fn member_to_proto(row: crate::flash_gen::organization_members::OrganizationMembersListWithUserForOrganizationRow) -> PermissionMemberProto {
    PermissionMemberProto {
        id: row.id,
        user_id: row.user_id.unwrap_or_default(),
        email: row.email.unwrap_or_default(),
        avatar: (!row.avatar.is_empty()).then_some(row.avatar),
        organization_id: row.organization_id.unwrap_or_default(),
        role: user_role_str_to_opt(row.role.as_ref()),
        created_at: row.created_at.unwrap_or_default(),
        updated_at: row.updated_at.unwrap_or_default(),
        ..Default::default()
    }
}

pub fn invite_to_proto(invite: crate::flash_gen::models::OrganizationInvites) -> PermissionInviteProto {
    PermissionInviteProto {
        id: invite.id,
        email: invite.email,
        role: user_role_str_to_opt(invite.role.as_ref()),
        group_id: invite.group_id,
        organization_id: invite.organization_id,
        invited_by: invite.invited_by,
        expired_at: invite.expired_at,
        created_at: invite.created_at,
        ..Default::default()
    }
}
