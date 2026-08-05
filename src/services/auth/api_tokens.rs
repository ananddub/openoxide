use getrandom::fill;
use sha2::{Digest, Sha256};

use crate::{
    api::dto::auth::{CreatedPersonalAccessTokenDto, PersonalAccessTokenDto},
    repository::PersonalAccessTokenRow,
    utils::jwt::claim::{Claims, TokenType},
};

use super::{AuthError, AuthService, subject_from_user, verify_password};

impl AuthService {
    pub async fn create_personal_access_token(
        &self,
        user_id: i64,
        name: String,
        password: String,
        expires_at: Option<i64>,
    ) -> Result<CreatedPersonalAccessTokenDto, AuthError> {
        let user = self.get_user_by_id(user_id).await?;
        verify_password(password, user.password.clone()).await?;
        if expires_at.is_some_and(|value| value <= chrono::Utc::now().timestamp()) {
            return Err(AuthError::InvalidOperation(
                "personal access token expiry must be in the future".into(),
            ));
        }
        let token = generate_token()?;
        let prefix = token.chars().take(15).collect::<String>();
        let row = self
            .repo_api_token
            .create(
                user_id,
                name.trim(),
                &prefix,
                &hash_token(&token),
                expires_at,
            )
            .await?;
        Ok(CreatedPersonalAccessTokenDto {
            token,
            details: row.into(),
        })
    }

    pub async fn list_personal_access_tokens(
        &self,
        user_id: i64,
    ) -> Result<Vec<PersonalAccessTokenDto>, AuthError> {
        Ok(self
            .repo_api_token
            .list(user_id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    pub async fn revoke_personal_access_token(
        &self,
        user_id: i64,
        id: i64,
    ) -> Result<bool, AuthError> {
        self.repo_api_token
            .revoke(user_id, id)
            .await
            .map_err(Into::into)
    }

    pub(super) async fn validate_personal_access_token(
        &self,
        token: &str,
    ) -> Result<Claims, AuthError> {
        let row = self
            .repo_api_token
            .authenticate(&hash_token(token))
            .await?
            .ok_or(AuthError::InvalidToken)?;
        let user = self.get_user_by_id(row.user_id).await?;
        Ok(Claims {
            sub: row.user_id.to_string(),
            user: subject_from_user(&user)?,
            jti: format!("pat:{}", row.id),
            token_type: TokenType::Access,
            iat: row.created_at.max(0) as usize,
            exp: row.expires_at.unwrap_or(i64::MAX).max(0) as usize,
        })
    }
}

impl From<PersonalAccessTokenRow> for PersonalAccessTokenDto {
    fn from(row: PersonalAccessTokenRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            token_prefix: row.token_prefix,
            expires_at: row.expires_at,
            last_used_at: row.last_used_at,
            revoked_at: row.revoked_at,
            created_at: row.created_at,
        }
    }
}

fn generate_token() -> Result<String, AuthError> {
    let mut bytes = [0_u8; 32];
    fill(&mut bytes).map_err(|_| AuthError::Internal)?;
    let secret = bytes
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    Ok(format!("rp_{secret}"))
}

fn hash_token(token: &str) -> String {
    let digest = Sha256::digest(token.trim().as_bytes());
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn personal_tokens_have_distinct_public_prefix_and_hash() {
        let token = generate_token().unwrap();
        assert!(token.starts_with("rp_"));
        assert_eq!(token.len(), 67);
        assert_ne!(token, hash_token(&token));
    }
}
