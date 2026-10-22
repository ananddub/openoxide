use std::time::Duration;

use connectrpc::{ConnectError, ErrorCode, RequestContext};

use crate::api::connect::generated::openoxide::api::v1::{AuthResponse, TokenPair, User};
use crate::services::identity::auth::AuthError;
use crate::services::identity::auth::rate_limit::AuthRateLimiter;

// ── Rate limiting ──────────────────────────────────────────────────

/// Login is limited per account (online password/TOTP guessing) and per
/// source IP (spraying many accounts).
pub const LOGIN_WINDOW: Duration = Duration::from_secs(15 * 60);
pub const LOGIN_ACCOUNT_LIMIT: u32 = 10;
pub const LOGIN_IP_LIMIT: u32 = 60;

pub const SIGNUP_WINDOW: Duration = Duration::from_secs(60 * 60);
pub const SIGNUP_EMAIL_LIMIT: u32 = 5;
pub const SIGNUP_IP_LIMIT: u32 = 10;

pub const RESET_WINDOW: Duration = Duration::from_secs(15 * 60);
pub const RESET_EMAIL_LIMIT: u32 = 3;
pub const RESET_IP_LIMIT: u32 = 20;

/// TOTP/recovery-code checks for an authenticated account.
pub const TWO_FACTOR_WINDOW: Duration = Duration::from_secs(15 * 60);
pub const TWO_FACTOR_LIMIT: u32 = 10;

/// Best-effort client address: the reverse proxy's forwarded header first,
/// then the socket peer injected by the TLS listener.
pub fn client_ip(ctx: &RequestContext) -> String {
    if let Some(ip) = ctx
        .headers()
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(',').next())
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        return ip.to_owned();
    }
    if let Some(ip) = ctx
        .headers()
        .get("x-real-ip")
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        return ip.to_owned();
    }
    ctx.peer_addr()
        .map(|address| address.ip().to_string())
        .unwrap_or_else(|| "unknown".to_owned())
}

pub fn enforce_rate_limit(
    limiter: &AuthRateLimiter,
    key: String,
    limit: u32,
    window: Duration,
) -> Result<(), ConnectError> {
    limiter.check(&key, limit, window).map_err(|retry_after| {
        ConnectError::resource_exhausted(format!(
            "too many attempts; retry in {} seconds",
            retry_after.as_secs().max(1)
        ))
    })
}

// ── Input validation ──────────────────────────────────────────────

const MAX_EMAIL_LEN: usize = 320;

/// Structural check for the proto's `string.email` rule. buf.validate
/// annotations are not enforced by the Connect router, so handlers must
/// validate before touching the auth service.
pub fn validated_email(raw: &str) -> Result<String, ConnectError> {
    let email = raw.trim();
    let valid = email.chars().count() <= MAX_EMAIL_LEN
        && !email.chars().any(char::is_whitespace)
        && email.split_once('@').is_some_and(|(local, domain)| {
            !local.is_empty()
                && !local.starts_with('.')
                && !local.ends_with('.')
                && !local.contains("..")
                && !domain.is_empty()
                && !domain.contains('@')
                && domain.contains('.')
                && !domain.starts_with('.')
                && !domain.ends_with('.')
                && !domain.contains("..")
        });
    if !valid {
        return Err(ConnectError::invalid_argument(
            "a valid email address is required",
        ));
    }
    Ok(email.to_owned())
}

/// Mirrors `string.min_len = 8` on every password-setting field.
pub fn validated_new_password(password: &str) -> Result<(), ConnectError> {
    if password.chars().count() < 8 {
        return Err(ConnectError::invalid_argument(
            "password must be at least 8 characters",
        ));
    }
    Ok(())
}

/// Login only requires a non-empty password (proto `min_len: 1`); the
/// stored-password comparison decides the rest.
pub fn validated_login_password(password: &str) -> Result<(), ConnectError> {
    if password.is_empty() {
        return Err(ConnectError::invalid_argument("password is required"));
    }
    Ok(())
}

// ── Error mapping ──────────────────────────────────────────────────

pub fn map_auth_error(error: AuthError) -> ConnectError {
    match &error {
        AuthError::InvalidCredentials => ConnectError::unauthenticated("invalid email or password"),
        AuthError::InvalidToken => ConnectError::unauthenticated("invalid or revoked token"),
        AuthError::TwoFactorRequired => ConnectError::new(
            ErrorCode::Unauthenticated,
            "two-factor authentication code is required",
        ),
        AuthError::InvalidSecondFactor => {
            ConnectError::unauthenticated("invalid two-factor authentication code")
        }
        AuthError::InvalidResetToken => {
            ConnectError::invalid_argument("invalid or expired password reset token")
        }
        AuthError::InvalidVerificationToken => {
            ConnectError::invalid_argument("invalid or expired email verification token")
        }
        AuthError::InvalidOperation(msg) => {
            ConnectError::new(ErrorCode::FailedPrecondition, msg.clone())
        }
        AuthError::Database(e) => {
            if let sqlx::Error::Database(db_err) = e
                && db_err.is_unique_violation()
            {
                return ConnectError::new(ErrorCode::AlreadyExists, "email is already registered");
            }
            tracing::error!(error = %error, "auth database operation failed");
            ConnectError::new(ErrorCode::Internal, "authentication operation failed")
        }
        AuthError::Internal => {
            tracing::error!("internal auth operation failed");
            ConnectError::new(ErrorCode::Internal, "authentication operation failed")
        }
    }
}

// ── Proto conversion helpers ───────────────────────────────────────

pub fn subject_to_user(sub: crate::utils::jwt::claim::JwtSubject) -> User {
    User {
        id: sub.user_id,
        email: sub.email,
        first_name: sub.first_name,
        last_name: sub.last_name,
        avatar: sub.avatar,
        role: sub.role,
        group_id: sub.group_id,
    }
}

pub fn token_pair_proto(tokens: crate::utils::jwt::service::TokenPair) -> TokenPair {
    TokenPair {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
    }
}

pub fn auth_response_proto(
    user: crate::utils::jwt::claim::JwtSubject,
    tokens: crate::utils::jwt::service::TokenPair,
) -> AuthResponse {
    AuthResponse {
        user: ::buffa::MessageField::some(subject_to_user(user)),
        tokens: ::buffa::MessageField::some(token_pair_proto(tokens)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_validation_accepts_normal_addresses() {
        assert_eq!(
            validated_email("  User@Example.COM ").unwrap(),
            "User@Example.COM"
        );
        assert!(validated_email("first.last+tag@sub.example.org").is_ok());
    }

    #[test]
    fn email_validation_rejects_malformed_addresses() {
        for email in [
            "",
            "   ",
            "plain",
            "@example.com",
            "user@",
            "user@example",
            "user@@example.com",
            "user name@example.com",
            "user@.example.com",
            "user@example..com",
            "user..name@example.com",
            ".user@example.com",
            "user.@example.com",
        ] {
            assert!(
                validated_email(email).is_err(),
                "{email:?} should be rejected"
            );
        }
        assert!(validated_email(&format!("{}@example.com", "x".repeat(320))).is_err());
    }

    #[test]
    fn new_password_requires_eight_characters() {
        assert!(validated_new_password("1234567").is_err());
        assert!(validated_new_password("12345678").is_ok());
        assert!(validated_new_password("पासवर्ड१२३").is_ok());
    }

    #[test]
    fn login_password_must_not_be_empty() {
        assert!(validated_login_password("").is_err());
        assert!(validated_login_password("x").is_ok());
    }
}
