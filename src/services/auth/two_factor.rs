use getrandom::fill;
use sha2::{Digest, Sha256};
use totp_rs::{Algorithm, Secret, TOTP};

use crate::{
    api::dto::auth::{RecoveryCodesResponseDto, TwoFactorSetupResponseDto, TwoFactorStatusDto},
    db::models::{two_factor::TwoFactor, users::User},
};

use super::{AuthError, AuthService, verify_password};

const RECOVERY_CODE_COUNT: usize = 10;

impl AuthService {
    pub async fn two_factor_status(&self, user_id: i64) -> Result<TwoFactorStatusDto, AuthError> {
        let user = self.get_user_by_id(user_id).await?;
        let configured = self
            .repo_two_factor
            .get_by_user_id(user_id)
            .await?
            .is_some();
        Ok(TwoFactorStatusDto {
            enabled: user.two_factor_enable.unwrap_or_default() != 0,
            configured,
        })
    }

    pub async fn setup_two_factor(
        &self,
        user_id: i64,
        password: String,
    ) -> Result<TwoFactorSetupResponseDto, AuthError> {
        let user = self.get_user_by_id(user_id).await?;
        verify_password(password, user.password.clone()).await?;
        if user.two_factor_enable.unwrap_or_default() != 0 {
            return Err(AuthError::InvalidOperation(
                "two-factor authentication is already enabled".into(),
            ));
        }

        let secret = Secret::generate_secret().to_encoded().to_string();
        let recovery_codes = generate_recovery_codes()?;
        let backup_codes = encode_recovery_codes(&recovery_codes)?;
        self.repo_two_factor
            .upsert(&TwoFactor {
                id: None,
                secret: secret.clone(),
                backup_codes,
                user_id,
            })
            .await?;

        let email = user.email.as_deref().unwrap_or("user");
        let otpauth_url = totp(&secret, email)?.get_url();
        Ok(TwoFactorSetupResponseDto {
            secret,
            otpauth_url,
            recovery_codes,
        })
    }

    pub async fn enable_two_factor(&self, user_id: i64, code: &str) -> Result<(), AuthError> {
        let user = self.get_user_by_id(user_id).await?;
        let setting = self
            .repo_two_factor
            .get_by_user_id(user_id)
            .await?
            .ok_or_else(|| AuthError::InvalidOperation("two-factor setup is not started".into()))?;
        if !check_totp(
            &setting.secret,
            user.email.as_deref().unwrap_or("user"),
            code,
        )? {
            return Err(AuthError::InvalidSecondFactor);
        }
        self.repo_user.set_two_factor_enabled(user_id, true).await?;
        self.repo_token.blacklist_all_by_user(user_id).await?;
        Ok(())
    }

    pub async fn disable_two_factor(
        &self,
        user_id: i64,
        password: String,
        code: &str,
    ) -> Result<(), AuthError> {
        let user = self.get_user_by_id(user_id).await?;
        verify_password(password, user.password.clone()).await?;
        let setting = self
            .repo_two_factor
            .get_by_user_id(user_id)
            .await?
            .ok_or(AuthError::InvalidSecondFactor)?;
        self.verify_stored_second_factor(&user, &setting, code, code)
            .await?;
        self.repo_two_factor.delete_by_user_id(user_id).await?;
        self.repo_user
            .set_two_factor_enabled(user_id, false)
            .await?;
        self.repo_token.blacklist_all_by_user(user_id).await?;
        Ok(())
    }

    pub async fn regenerate_recovery_codes(
        &self,
        user_id: i64,
        password: String,
        code: &str,
    ) -> Result<RecoveryCodesResponseDto, AuthError> {
        let user = self.get_user_by_id(user_id).await?;
        verify_password(password, user.password.clone()).await?;
        let setting = self
            .repo_two_factor
            .get_by_user_id(user_id)
            .await?
            .ok_or(AuthError::InvalidSecondFactor)?;
        self.verify_stored_second_factor(&user, &setting, code, code)
            .await?;
        let recovery_codes = generate_recovery_codes()?;
        let encoded = encode_recovery_codes(&recovery_codes)?;
        self.repo_two_factor
            .replace_recovery_codes(user_id, &encoded)
            .await?;
        Ok(RecoveryCodesResponseDto { recovery_codes })
    }

    pub(super) async fn verify_login_second_factor(
        &self,
        user: &User,
        code: Option<&str>,
        recovery_code: Option<&str>,
    ) -> Result<(), AuthError> {
        if user.two_factor_enable.unwrap_or_default() == 0 {
            return Ok(());
        }
        if code.is_none() && recovery_code.is_none() {
            return Err(AuthError::TwoFactorRequired);
        }
        let setting = self
            .repo_two_factor
            .get_by_user_id(user.id.ok_or(AuthError::Internal)?)
            .await?
            .ok_or(AuthError::InvalidSecondFactor)?;
        self.verify_stored_second_factor(
            user,
            &setting,
            code.unwrap_or_default(),
            recovery_code.unwrap_or_default(),
        )
        .await
    }

    async fn verify_stored_second_factor(
        &self,
        user: &User,
        setting: &TwoFactor,
        code: &str,
        recovery_code: &str,
    ) -> Result<(), AuthError> {
        if !code.trim().is_empty()
            && check_totp(
                &setting.secret,
                user.email.as_deref().unwrap_or("user"),
                code,
            )?
        {
            return Ok(());
        }
        if !recovery_code.trim().is_empty()
            && self
                .repo_two_factor
                .consume_recovery_code(
                    user.id.ok_or(AuthError::Internal)?,
                    &setting.backup_codes,
                    &hash_recovery_code(recovery_code),
                )
                .await?
        {
            return Ok(());
        }
        Err(AuthError::InvalidSecondFactor)
    }
}

fn totp(secret: &str, account: &str) -> Result<TOTP, AuthError> {
    let bytes = Secret::Encoded(secret.to_owned())
        .to_bytes()
        .map_err(|_| AuthError::Internal)?;
    TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        bytes,
        Some("Rustploy".into()),
        account.into(),
    )
    .map_err(|_| AuthError::Internal)
}

fn check_totp(secret: &str, account: &str, code: &str) -> Result<bool, AuthError> {
    totp(secret, account)?
        .check_current(code.trim())
        .map_err(|_| AuthError::Internal)
}

fn generate_recovery_codes() -> Result<Vec<String>, AuthError> {
    (0..RECOVERY_CODE_COUNT)
        .map(|_| {
            let mut bytes = [0_u8; 10];
            fill(&mut bytes).map_err(|_| AuthError::Internal)?;
            let raw = bytes
                .iter()
                .map(|byte| format!("{byte:02X}"))
                .collect::<String>();
            Ok(format!(
                "{}-{}-{}-{}",
                &raw[0..5],
                &raw[5..10],
                &raw[10..15],
                &raw[15..20]
            ))
        })
        .collect()
}

fn encode_recovery_codes(codes: &[String]) -> Result<String, AuthError> {
    serde_json::to_string(
        &codes
            .iter()
            .map(|code| hash_recovery_code(code))
            .collect::<Vec<_>>(),
    )
    .map_err(|_| AuthError::Internal)
}

fn hash_recovery_code(code: &str) -> String {
    let normalized = code
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_uppercase)
        .collect::<String>();
    let digest = Sha256::digest(normalized.as_bytes());
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recovery_code_hash_is_format_insensitive() {
        assert_eq!(
            hash_recovery_code("ABCDE-12345-FGHIJ-67890"),
            hash_recovery_code("abcde12345fghij67890")
        );
    }

    #[test]
    fn recovery_codes_are_unique_and_hashed_for_storage() {
        let codes = generate_recovery_codes().unwrap();
        assert_eq!(codes.len(), RECOVERY_CODE_COUNT);
        let unique = codes.iter().collect::<std::collections::HashSet<_>>();
        assert_eq!(unique.len(), RECOVERY_CODE_COUNT);
        let stored: Vec<String> =
            serde_json::from_str(&encode_recovery_codes(&codes).unwrap()).unwrap();
        assert_eq!(stored.len(), RECOVERY_CODE_COUNT);
        assert!(!stored.iter().any(|value| codes.contains(value)));
    }
}
