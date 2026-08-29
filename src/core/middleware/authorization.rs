use axum::{
    Json, RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use serde_json::json;

use super::auth_context::{AuthContext, AuthError};
use crate::services::auth::AuthService;
use crate::utils::jwt::claim::Claims;
use auto_di::resolve;

// ── Claims extractor (unchanged public API) ───────────────────────

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = if let Ok(TypedHeader(Authorization(bearer))) =
            parts.extract::<TypedHeader<Authorization<Bearer>>>().await
        {
            bearer.token().to_string()
        } else {
            let query = parts.uri.query().unwrap_or_default();
            query
                .split('&')
                .find_map(|pair| {
                    let mut kv = pair.splitn(2, '=');
                    if kv.next()? == "token" {
                        Some(kv.next()?.to_string())
                    } else {
                        None
                    }
                })
                .ok_or_else(|| {
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(json!({ "error": "missing authorization header or query token" })),
                    )
                })?
        };

        let auth_service = resolve::<AuthService>()
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "auth service unavailable" })),
                )
            })?
            .clone();

        auth_service
            .validate_access_token(&token)
            .await
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "invalid token" })),
                )
            })
    }
}

// ── AuthContext extractor (transport-neutral, Axum adapter) ───────

fn auth_error_to_rejection(err: &AuthError) -> (StatusCode, Json<serde_json::Value>) {
    let status = StatusCode::from_u16(err.status_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    (status, Json(json!({ "error": err.message() })))
}

impl<S> FromRequestParts<S> for AuthContext
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let authorization = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_owned());

        // Axum-specific fallback: ?token= query parameter
        let authorization = authorization.or_else(|| {
            parts.uri.query().unwrap_or_default().split('&').find_map(|pair| {
                let mut kv = pair.splitn(2, '=');
                if kv.next()? == "token" {
                    Some(format!("Bearer {}", kv.next()?))
                } else {
                    None
                }
            })
        });

        let org_header = parts
            .headers
            .get("x-organization-id")
            .and_then(|v| v.to_str().ok());

        let ctx = AuthContext::from_headers(authorization.as_deref(), org_header)
            .await
            .map_err(|e| auth_error_to_rejection(&e))?;

        // Preserve PermissionOrganization extension for downstream code.
        parts.extensions.insert(super::permission::PermissionOrganization(ctx.organization_id));

        Ok(ctx)
    }
}
