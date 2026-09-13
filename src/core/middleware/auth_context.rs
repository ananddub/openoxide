use auto_di::resolve;
use connectrpc::{ConnectError, ErrorCode, RequestContext};

use crate::services::identity::auth::AuthService;
use crate::services::identity::permission::{PermissionService, PolicyAction};
use crate::utils::jwt::claim::Claims;

/// Transport-neutral authentication and authorization context.
///
/// Produced from the same JWT + organization headers regardless of
/// whether the request arrived via Axum or ConnectRPC.  Axum handlers
/// can also extract this directly via `FromRequestParts`; ConnectRPC
/// handlers call [`AuthContext::from_request_context`].
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub claims: Claims,
    pub organization_id: i64,
}

impl AuthContext {
    pub fn user_id(&self) -> i64 {
        self.claims.user.user_id
    }

    // ── Core header-based extraction (shared by both transports) ──────

    /// Validate the Bearer token and resolve the organization from
    /// the `x-organization-id` header (falling back to the user's
    /// first organization membership).
    pub async fn from_headers(
        authorization: Option<&str>,
        organization_id: Option<&str>,
    ) -> Result<Self, AuthError> {
        let token = authorization
            .and_then(|v| v.strip_prefix("Bearer ").or_else(|| v.strip_prefix("bearer ")))
            .filter(|t| !t.is_empty())
            .ok_or(AuthError::MissingToken)?;

        let auth_service = resolve::<AuthService>()
            .await
            .map_err(|_| AuthError::ServiceUnavailable)?
            .clone();

        let claims = auth_service
            .validate_access_token(token)
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let organization_id = Self::resolve_organization(organization_id, claims.user.user_id)
            .await?;

        Ok(Self {
            claims,
            organization_id,
        })
    }

    /// Organization resolution shared by both transports.
    ///
    /// A client-supplied organization is only honored when the caller is
    /// actually a member of it (or owns it); otherwise it falls back to the
    /// user's own first membership. A stale header (e.g. access to that
    /// workspace was revoked) must never hard-block the session — the user
    /// still needs ListOrganizations to switch workspaces — and the fallback
    /// can only ever resolve to an organization they belong to.
    async fn resolve_organization(
        header: Option<&str>,
        user_id: i64,
    ) -> Result<i64, AuthError> {
        let service = resolve::<PermissionService>()
            .await
            .map_err(|_| AuthError::ServiceUnavailable)?
            .clone();
        if let Some(id) = header.and_then(|v| v.parse::<i64>().ok()) {
            if service
                .is_organization_member(user_id, id)
                .await
                .map_err(|_| AuthError::ServiceUnavailable)?
            {
                return Ok(id);
            }
        }
        service
            .resolve_organization(user_id)
            .await
            .map_err(|_| AuthError::NoOrganization)?
            .ok_or(AuthError::NoOrganization)
    }

    // ── ConnectRPC adapter ────────────────────────────────────────────

    /// Extract [`AuthContext`] from a ConnectRPC [`RequestContext`].
    pub async fn from_request_context(
        ctx: &RequestContext,
    ) -> Result<Self, ConnectError> {
        let authorization = ctx
            .header("authorization")
            .and_then(|v| v.to_str().ok());
        let org_id = ctx
            .header("x-organization-id")
            .and_then(|v| v.to_str().ok());
        Self::from_headers(authorization, org_id)
            .await
            .map_err(|e| e.into_connect_error())
    }

    // ── Permission helpers ────────────────────────────────────────────

    /// Check whether the authenticated user holds the given permission.
    pub async fn check_permission(
        &self,
        resource: &'static str,
        operation: &'static str,
    ) -> Result<bool, AuthError> {
        let service = resolve::<PermissionService>()
            .await
            .map_err(|_| AuthError::ServiceUnavailable)?
            .clone();
        service
            .check_permission(self.user_id(), self.organization_id, PolicyAction::new(resource, operation))
            .await
            .map_err(|_| AuthError::EvaluationFailed)
    }

    /// Require a specific permission, returning an error if denied.
    pub async fn require_permission(
        &self,
        resource: &'static str,
        operation: &'static str,
    ) -> Result<(), AuthError> {
        let allowed = self.check_permission(resource, operation).await?;
        if allowed {
            Ok(())
        } else {
            Err(AuthError::PermissionDenied {
                resource,
                operation,
            })
        }
    }
}

// ── Shared error type ─────────────────────────────────────────────

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    ServiceUnavailable,
    NoOrganization,
    EvaluationFailed,
    PermissionDenied {
        resource: &'static str,
        operation: &'static str,
    },
}

impl AuthError {
    /// Map to the canonical HTTP status for Axum rejections.
    pub fn status_code(&self) -> u16 {
        match self {
            Self::MissingToken | Self::InvalidToken => 401,
            Self::PermissionDenied { .. } | Self::NoOrganization => 403,
            Self::ServiceUnavailable | Self::EvaluationFailed => 500,
        }
    }

    /// Map to a ConnectRPC [`ConnectError`].
    pub fn into_connect_error(self) -> ConnectError {
        match &self {
            Self::MissingToken | Self::InvalidToken => {
                ConnectError::unauthenticated(self.message())
            }
            Self::PermissionDenied { .. } | Self::NoOrganization => {
                ConnectError::new(ErrorCode::PermissionDenied, self.message())
            }
            Self::ServiceUnavailable | Self::EvaluationFailed => {
                ConnectError::new(ErrorCode::Internal, self.message())
            }
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::MissingToken => "missing authorization header".into(),
            Self::InvalidToken => "invalid or expired access token".into(),
            Self::ServiceUnavailable => "auth service unavailable".into(),
            Self::NoOrganization => "user has no organization membership".into(),
            Self::EvaluationFailed => "failed to evaluate permission".into(),
            Self::PermissionDenied { resource, operation } => {
                format!("permission denied: {resource}:{operation}")
            }
        }
    }
}

impl From<AuthError> for ConnectError {
    fn from(err: AuthError) -> Self {
        err.into_connect_error()
    }
}
