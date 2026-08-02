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

