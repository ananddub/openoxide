use std::fmt;

use serde::Serialize;

use crate::utils::jwt::{
    claim::JwtSubject,
    error::TokenError,
    service::TokenPair,
};

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    InvalidToken,
    TwoFactorRequired,
    InvalidSecondFactor,
    InvalidResetToken,
    InvalidVerificationToken,
    InvalidOperation(String),
    Database(sqlx::Error),
    Internal,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCredentials => write!(f, "invalid email or password"),
            Self::InvalidToken => write!(f, "invalid or revoked token"),
            Self::TwoFactorRequired => write!(f, "two-factor authentication code is required"),
            Self::InvalidSecondFactor => write!(f, "invalid two-factor authentication code"),
            Self::InvalidResetToken => write!(f, "invalid or expired password reset token"),
            Self::InvalidVerificationToken => {
                write!(f, "invalid or expired email verification token")
            }
            Self::InvalidOperation(message) => write!(f, "{message}"),
            Self::Database(error) => write!(f, "{error}"),
            Self::Internal => write!(f, "authentication operation failed"),
        }
    }
}

impl std::error::Error for AuthError {}

impl From<sqlx::Error> for AuthError {
    fn from(value: sqlx::Error) -> Self {
        Self::Database(value)
    }
}

impl From<TokenError> for AuthError {
    fn from(_: TokenError) -> Self {
        Self::InvalidToken
    }
}

#[derive(Debug, Serialize)]
pub struct AuthResponseDto {
    pub user: JwtSubject,
    pub tokens: TokenPair,
}
