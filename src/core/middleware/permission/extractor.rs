use std::marker::PhantomData;

use axum::{extract::FromRequestParts, http::request::Parts};

use crate::core::middleware::auth_context::AuthContext;
use crate::utils::jwt::claim::Claims;

use super::{
    Allows, PermissionOperation, PermissionResource,
    error::{PermissionRejection, denied, internal},
};

pub struct PermissionContext<R, O> {
    organization_id: i64,
    marker: PhantomData<(R, O)>,
}

impl<R, O> PermissionContext<R, O> {
    pub fn organization_id(&self) -> i64 {
        self.organization_id
    }
}

pub struct RequirePermission<R, O>(pub Claims, pub PermissionContext<R, O>)
where
    R: PermissionResource + Allows<O>,
    O: PermissionOperation;

#[derive(Debug, Clone, Copy)]
pub struct PermissionOrganization(pub i64);

impl<S, R, O> FromRequestParts<S> for RequirePermission<R, O>
where
    S: Send + Sync,
    R: PermissionResource + Allows<O>,
    O: PermissionOperation,
{
    type Rejection = PermissionRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // AuthContext handles JWT validation, organization resolution,
        // and PermissionOrganization extension insertion centrally.
        let ctx = AuthContext::from_request_parts(parts, state)
            .await
            .map_err(|(_status, json)| internal(&json.0.get("error").and_then(|v| v.as_str()).unwrap_or("auth failed")))?;

        // Permission check via the same AuthContext that ConnectRPC uses.
        let allowed = ctx
            .check_permission(R::NAME, O::NAME)
            .await
            .map_err(|_| internal("permission evaluation failed"))?;

        if !allowed {
            return Err(denied(PolicyAction::new(R::NAME, O::NAME).as_str()));
        }

        Ok(Self(
            ctx.claims.clone(),
            PermissionContext {
                organization_id: ctx.organization_id,
                marker: PhantomData,
            },
        ))
    }
}

// Re-export PolicyAction for the denial message above.
use crate::services::permission::PolicyAction;
