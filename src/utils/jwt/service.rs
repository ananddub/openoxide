use super::config::JwtConfig;
use super::error::TokenError;
use crate::utils::jwt::claim::{Claims, JwtSubject, TokenType};
use auto_di::singleton;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct JwtService {
    config: Arc<JwtConfig>,
}
#[singleton]
impl JwtService {
    pub fn new(config: Arc<JwtConfig>) -> Self {
        if config.debug_skip_time_check {
            tracing::warn!(
                "JWT_DEBUG_SKIP_TIME_CHECK is enabled: JWT expiry validation will be skipped"
            );
        }
        Self { config }
    }

    pub fn generate_token_pair(&self, subject: &JwtSubject) -> Result<TokenPair, TokenError> {
        let access_token = self.generate_token(subject, TokenType::Access)?;
        let refresh_token = self.generate_token(subject, TokenType::Refresh)?;
        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }

    fn generate_token(
        &self,
        subject: &JwtSubject,
        token_type: TokenType,
    ) -> Result<String, TokenError> {
        let now = Utc::now();
        let (secret, expiry) = match token_type {
            TokenType::Access => (
                &self.config.access_secret,
                now + Duration::minutes(self.config.access_expiry_mins),
            ),
            TokenType::Refresh => (
                &self.config.refresh_secret,
                now + Duration::days(self.config.refresh_expiry_days),
            ),
        };

        let claims = Claims {
            sub: subject.user_id.to_string(),
            user: subject.clone(),
            jti: uuid::Uuid::new_v4().to_string(),
            token_type,
            iat: now.timestamp() as usize,
            exp: expiry.timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|_| TokenError::EncodingFailed)
    }

    pub fn validate_access_token(&self, token: &str) -> Result<Claims, TokenError> {
        self.validate(token, &self.config.access_secret, TokenType::Access)
    }

    pub fn validate_refresh_token(&self, token: &str) -> Result<Claims, TokenError> {
        self.validate(token, &self.config.refresh_secret, TokenType::Refresh)
    }

    fn validate(
        &self,
        token: &str,
        secret: &str,
        expected: TokenType,
    ) -> Result<Claims, TokenError> {
        let mut validation = Validation::new(Algorithm::HS256);
        if self.config.debug_skip_time_check {
            // dbg!(self.config.debug_skip_time_check,token);
            validation.validate_exp = false;
        }

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => TokenError::Expired,
            _ => TokenError::Invalid,
        })?;

        if token_data.claims.token_type != expected {
            return Err(TokenError::WrongType);
        }

        Ok(token_data.claims)
    }

    pub fn refresh_access_token(&self, refresh_token: &str) -> Result<String, TokenError> {
        let claims = self.validate_refresh_token(refresh_token)?;
        self.generate_token(&claims.user, TokenType::Access)
    }
}
