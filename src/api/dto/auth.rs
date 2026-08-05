use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::jwt::{claim::JwtSubject, service::TokenPair};

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct SignupDto {
    #[validate(email, length(max = 320))]
    pub email: String,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
    #[validate(length(min = 1, max = 100))]
    pub first_name: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub last_name: Option<String>,
    #[validate(length(max = 2_048))]
    pub avatar: Option<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct LoginDto {
    #[validate(email, length(max = 320))]
    pub email: String,
    #[validate(length(min = 1, max = 128))]
    pub password: String,
    #[validate(length(min = 6, max = 32))]
    pub two_factor_code: Option<String>,
    #[validate(length(min = 8, max = 64))]
    pub recovery_code: Option<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct RefreshTokenDto {
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct AuthResponseDto {
    pub user: JwtSubject,
    pub tokens: TokenPair,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct UpdateUserDto {
    #[validate(email, length(max = 320))]
    pub email: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct TwoFactorSetupDto {
    #[validate(length(min = 1, max = 128))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct TwoFactorCodeDto {
    #[validate(length(min = 6, max = 32))]
    pub code: String,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct TwoFactorDisableDto {
    #[validate(length(min = 1, max = 128))]
    pub password: String,
    #[validate(length(min = 6, max = 64))]
    pub code: String,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct TwoFactorSetupResponseDto {
    pub secret: String,
    pub otpauth_url: String,
    pub recovery_codes: Vec<String>,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct TwoFactorStatusDto {
    pub enabled: bool,
    pub configured: bool,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct RecoveryCodesResponseDto {
    pub recovery_codes: Vec<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PasswordResetRequestDto {
    #[validate(email, length(max = 320))]
    pub email: String,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PasswordResetConfirmDto {
    #[validate(length(min = 32, max = 128))]
    pub token: String,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct AuthSessionDto {
    pub session_id: String,
    pub created_at: i64,
    pub expires_at: Option<i64>,
    pub active: bool,
    pub current: bool,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreatePersonalAccessTokenDto {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1, max = 128))]
    pub password: String,
    pub expires_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct PersonalAccessTokenDto {
    pub id: i64,
    pub name: String,
    pub token_prefix: String,
    pub expires_at: Option<i64>,
    pub last_used_at: Option<i64>,
    pub revoked_at: Option<i64>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct CreatedPersonalAccessTokenDto {
    pub token: String,
    pub details: PersonalAccessTokenDto,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct EmailVerificationConfirmDto {
    #[validate(length(min = 32, max = 128))]
    pub token: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct EmailVerificationStatusDto {
    pub verified: bool,
    pub verified_at: Option<i64>,
}
