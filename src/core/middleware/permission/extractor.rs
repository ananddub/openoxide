use std::marker::PhantomData;

use auto_di::resolve;
use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{
    services::permission::{PermissionService, PolicyAction},
    utils::jwt::claim::Claims,
};

use super::{
    Allows, PermissionOperation, PermissionResource,
    error::{PermissionRejection, denied, evaluation, internal, no_organization},
};

pub struct RequirePermission<R, O>(pub Claims, pub PhantomData<(R, O)>)
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
        let claims = Claims::from_request_parts(parts, state).await?;
        let service = resolve::<PermissionService>()
            .await
            .map_err(|_| internal("permission service unavailable"))?
            .clone();

        let requested_organization = parts
            .headers
            .get("x-organization-id")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<i64>().ok());

        let organization_id = match requested_organization {
            Some(id) => id,
            None => service
                .resolve_organization(claims.user.user_id)
                .await
                .map_err(evaluation)?
                .ok_or_else(no_organization)?,
        };

        let required = PolicyAction::new(R::NAME, O::NAME);
        let allowed = service
            .check_permission(claims.user.user_id, organization_id, required)
            .await
            .map_err(evaluation)?;

        if !allowed {
            return Err(denied(required.as_str()));
        }

        parts
            .extensions
            .insert(PermissionOrganization(organization_id));
        Ok(Self(claims, PhantomData))
    }
}
