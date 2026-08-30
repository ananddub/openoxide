use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{AuthResponse, TokenPair, User};
use crate::services::auth::AuthError;

// ── Error mapping ──────────────────────────────────────────────────

pub fn map_auth_error(error: AuthError) -> ConnectError {
    match &error {
        AuthError::InvalidCredentials => ConnectError::unauthenticated("invalid email or password"),
        AuthError::InvalidToken => ConnectError::unauthenticated("invalid or revoked token"),
        AuthError::TwoFactorRequired => {
            ConnectError::new(ErrorCode::Unauthenticated, "two-factor authentication code is required")
        }
        AuthError::InvalidSecondFactor => {
            ConnectError::unauthenticated("invalid two-factor authentication code")
        }
        AuthError::InvalidResetToken => {
            ConnectError::invalid_argument("invalid or expired password reset token")
        }
        AuthError::InvalidVerificationToken => {
            ConnectError::invalid_argument("invalid or expired email verification token")
        }
        AuthError::InvalidOperation(msg) => ConnectError::new(ErrorCode::FailedPrecondition, msg.clone()),
        AuthError::Database(e) => {
            if let sqlx::Error::Database(db_err) = e {
                if db_err.is_unique_violation() {
                    return ConnectError::new(ErrorCode::AlreadyExists, "email is already registered");
                }
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
        ..Default::default()
    }
}

pub fn token_pair_proto(tokens: crate::utils::jwt::service::TokenPair) -> TokenPair {
    TokenPair {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        ..Default::default()
    }
}

pub fn auth_response_proto(
    user: crate::utils::jwt::claim::JwtSubject,
    tokens: crate::utils::jwt::service::TokenPair,
) -> AuthResponse {
    AuthResponse {
        user: ::buffa::MessageField::some(subject_to_user(user)),
        tokens: ::buffa::MessageField::some(token_pair_proto(tokens)),
        ..Default::default()
    }
}
