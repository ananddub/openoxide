use connectrpc::{ConnectError, RequestContext};

use crate::core::middleware::auth_context::AuthContext;

/// Extract a fully validated [`AuthContext`] from the request headers.
///
/// This is the ConnectRPC equivalent of the Axum `AuthContext` extractor.
/// It validates the Bearer token and resolves the organization in one call.
pub async fn authenticate(ctx: &RequestContext) -> Result<AuthContext, ConnectError> {
    AuthContext::from_request_context(ctx).await
}

/// Extract only [`Claims`] from the request, without organization resolution.
///
/// Use this when you need the caller's identity but don't need to check
/// organization-scoped permissions.
pub async fn authenticate_claims(
    ctx: &RequestContext,
) -> Result<crate::utils::jwt::claim::Claims, ConnectError> {
    let auth_ctx = AuthContext::from_request_context(ctx).await?;
    Ok(auth_ctx.claims)
}
