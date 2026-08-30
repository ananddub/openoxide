use getrandom::fill;
use sha2::{Digest, Sha256};

use crate::{
    api::connect::generated::openoxide::api::v1::{
        CreatedPersonalAccessToken, PersonalAccessToken,
    },
    flash_gen::personal_access_tokens::{
        PersonalAccessTokensCreateParams, PersonalAccessTokensCreateRow,
        PersonalAccessTokensListRow,
    },
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
    ) -> Result<CreatedPersonalAccessToken, AuthError> {
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
            .queries
            .personal_access_tokens_create(&PersonalAccessTokensCreateParams {
                user_id,
                name: name.trim().to_string(),
                token_prefix: prefix,
                token_hash: hash_token(&token),
                expires_at: expires_at.unwrap_or(i64::MAX),
            })
            .await?;
        Ok(CreatedPersonalAccessToken {
            token,
            details: ::buffa::MessageField::some(row.into()),
            ..Default::default()
        })
    }

    pub async fn list_personal_access_tokens(
        &self,
        user_id: i64,
    ) -> Result<Vec<PersonalAccessToken>, AuthError> {
        Ok(self
            .queries
            .personal_access_tokens_list(user_id)
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
        let tokens = self.queries.personal_access_tokens_list(user_id).await?;
        if !tokens
            .iter()
            .any(|token| token.id == id && token.revoked_at.is_none())
        {
            return Ok(false);
        }
        self.queries
            .personal_access_tokens_revoke(id, user_id)
            .await?;
        Ok(true)
    }

    pub(super) async fn validate_personal_access_token(
        &self,
        token: &str,
    ) -> Result<Claims, AuthError> {
        let row = match self
            .queries
            .personal_access_tokens_authenticate1(&hash_token(token))
            .await
        {
            Ok(row) => row,
            Err(sqlx::Error::RowNotFound) => return Err(AuthError::InvalidToken),
            Err(error) => return Err(error.into()),
        };
        self.queries.personal_access_tokens_authenticate2(row.id).await?;
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

impl From<PersonalAccessTokensCreateRow> for PersonalAccessToken {
    fn from(row: PersonalAccessTokensCreateRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            token_prefix: row.token_prefix,
            expires_at: row.expires_at,
            last_used_at: row.last_used_at,
            revoked_at: row.revoked_at,
            created_at: row.created_at,
            ..Default::default()
        }
    }
}

impl From<PersonalAccessTokensListRow> for PersonalAccessToken {
    fn from(row: PersonalAccessTokensListRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            token_prefix: row.token_prefix,
            expires_at: row.expires_at,
            last_used_at: row.last_used_at,
            revoked_at: row.revoked_at,
            created_at: row.created_at,
            ..Default::default()
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
