use axum::{
    Json,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use serde_json::json;
use std::marker::PhantomData;

use crate::{
    services::permission::{PermissionService, PolicyAction},
    utils::jwt::claim::Claims,
};
use auto_di::resolve;

pub trait ActionPermission: Send + Sync + 'static {
    const ACTION: PolicyAction;
}

pub struct RequirePermission<P: ActionPermission>(pub Claims, pub PhantomData<P>);

impl<S, P: ActionPermission> FromRequestParts<S> for RequirePermission<P>
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, state).await?;

        let permission_service = resolve::<PermissionService>()
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "permission service unavailable" })),
                )
            })?
            .clone();
        let requested_org = parts
            .headers
            .get("x-organization-id")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<i64>().ok());
        let org_id = match requested_org {
            Some(org_id) => org_id,
            None => permission_service
                .resolve_organization(claims.user.user_id)
                .await
                .map_err(permission_error)?
                .ok_or_else(|| {
                    (
                        StatusCode::FORBIDDEN,
                        Json(json!({ "error": "user has no organization membership" })),
                    )
                })?,
        };
        let has_permission = permission_service
            .check_permission(claims.user.user_id, org_id, P::ACTION)
            .await
            .map_err(permission_error)?;
        if has_permission {
            Ok(RequirePermission(claims, PhantomData))
        } else {
            Err((
                StatusCode::FORBIDDEN,
                Json(json!({
                    "error": "permission denied",
                    "required_action": P::ACTION.as_str()
                })),
            ))
        }
    }
}

fn permission_error(error: sqlx::Error) -> (StatusCode, Json<serde_json::Value>) {
    tracing::error!(%error, "permission evaluation failed");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": "failed to evaluate permission" })),
    )
}

// Pre-defined Action Permission Markers
pub struct ProjectReadPermission;
impl ActionPermission for ProjectReadPermission {
    const ACTION: PolicyAction = PolicyAction::ProjectRead;
}

pub struct ProjectCreatePermission;
impl ActionPermission for ProjectCreatePermission {
    const ACTION: PolicyAction = PolicyAction::ProjectCreate;
}

pub struct ProjectDeletePermission;
impl ActionPermission for ProjectDeletePermission {
    const ACTION: PolicyAction = PolicyAction::ProjectDelete;
}

pub struct ServerReadPermission;
impl ActionPermission for ServerReadPermission {
    const ACTION: PolicyAction = PolicyAction::ServerRead;
}

pub struct ServerCreatePermission;
impl ActionPermission for ServerCreatePermission {
    const ACTION: PolicyAction = PolicyAction::ServerCreate;
}

pub struct ServerDeletePermission;
impl ActionPermission for ServerDeletePermission {
    const ACTION: PolicyAction = PolicyAction::ServerDelete;
}

pub struct AppReadPermission;
impl ActionPermission for AppReadPermission {
    const ACTION: PolicyAction = PolicyAction::AppRead;
}

pub struct AppCreatePermission;
impl ActionPermission for AppCreatePermission {
    const ACTION: PolicyAction = PolicyAction::AppCreate;
}

pub struct AppDeletePermission;
impl ActionPermission for AppDeletePermission {
    const ACTION: PolicyAction = PolicyAction::AppDelete;
}

pub struct AppDeployPermission;
impl ActionPermission for AppDeployPermission {
    const ACTION: PolicyAction = PolicyAction::AppDeploy;
}

pub struct DatabaseReadPermission;
impl ActionPermission for DatabaseReadPermission {
    const ACTION: PolicyAction = PolicyAction::DatabaseRead;
}

pub struct DatabaseCreatePermission;
impl ActionPermission for DatabaseCreatePermission {
    const ACTION: PolicyAction = PolicyAction::DatabaseCreate;
}

pub struct DatabaseUpdatePermission;
impl ActionPermission for DatabaseUpdatePermission {
    const ACTION: PolicyAction = PolicyAction::DatabaseUpdate;
}

pub struct DatabaseDeletePermission;
impl ActionPermission for DatabaseDeletePermission {
    const ACTION: PolicyAction = PolicyAction::DatabaseDelete;
}

pub struct EnvReadPermission;
impl ActionPermission for EnvReadPermission {
    const ACTION: PolicyAction = PolicyAction::EnvRead;
}

pub struct EnvWritePermission;
impl ActionPermission for EnvWritePermission {
    const ACTION: PolicyAction = PolicyAction::EnvWrite;
}

pub struct OrgWritePermission;
impl ActionPermission for OrgWritePermission {
    const ACTION: PolicyAction = PolicyAction::OrgWrite;
}

pub struct ServerMonitorPermission;
impl ActionPermission for ServerMonitorPermission {
    const ACTION: PolicyAction = PolicyAction::ServerMonitor;
}

pub struct AppMonitorPermission;
impl ActionPermission for AppMonitorPermission {
    const ACTION: PolicyAction = PolicyAction::AppMonitor;
}

pub struct TraefikReadPermission;
impl ActionPermission for TraefikReadPermission {
    const ACTION: PolicyAction = PolicyAction::TraefikRead;
}

pub struct TraefikWritePermission;
impl ActionPermission for TraefikWritePermission {
    const ACTION: PolicyAction = PolicyAction::TraefikWrite;
}

pub struct AlertWritePermission;
impl ActionPermission for AlertWritePermission {
    const ACTION: PolicyAction = PolicyAction::AlertWrite;
}
