use getrandom::fill;
use sha2::{Digest, Sha256};

use crate::services::notification::{
    NotificationMessage, email::send_email_to, senders::send_resend_to,
};

use super::{AuthError, AuthService, hash_password};

impl AuthService {
    pub async fn request_password_reset(&self, email: &str) -> Result<(), AuthError> {
        let email = email.trim().to_lowercase();
        let Some(user) = self.repo_user.get_by_email(&email).await? else {
            return Ok(());
        };
        let user_id = user.id.ok_or(AuthError::Internal)?;
        let token = random_token()?;
        self.repo_password_reset
            .issue(
                user_id,
                &hash_token(&token),
                chrono::Utc::now().timestamp() + 30 * 60,
            )
            .await?;

        let action = std::env::var("OPENOXIDE_PUBLIC_URL")
            .or_else(|_| std::env::var("RUSTPLOY_PUBLIC_URL"))
            .ok()
            .map(|base| {
                format!(
                    "{}/reset-password?token={token}",
                    base.trim_end_matches('/')
                )
            })
            .unwrap_or_else(|| format!("Password reset token: {token}"));
        let message = NotificationMessage::new(
            "Reset your OpenOxide password",
            format!(
                "A password reset was requested for your account. Use this within 30 minutes:\n\n{action}\n\nIf you did not request this, ignore this message."
            ),
        );

        if let Some(config) = self.repo_notif_email.find_for_user(user_id).await? {
            if let Err(error) = send_email_to(&config, &email, &message).await {
                tracing::warn!(user_id, error = %error, "password reset SMTP delivery failed");
            }
        } else if let Some(config) = self.repo_notif_resend.find_for_user(user_id).await? {
            if let Err(error) =
                send_resend_to(&reqwest::Client::new(), &config, &email, &message).await
            {
                tracing::warn!(user_id, error = %error, "password reset Resend delivery failed");
            }
        } else {
            tracing::warn!(
                user_id,
                "password reset requested but no email provider is configured"
            );
        }
        Ok(())
    }

    pub async fn reset_password(&self, token: &str, password: String) -> Result<(), AuthError> {
        let password_hash = hash_password(password).await?;
        if !self
            .repo_password_reset
            .complete(&hash_token(token), &password_hash)
            .await?
        {
            return Err(AuthError::InvalidResetToken);
        }
        Ok(())
    }
}

fn random_token() -> Result<String, AuthError> {
    let mut bytes = [0_u8; 32];
    fill(&mut bytes).map_err(|_| AuthError::Internal)?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}

fn hash_token(token: &str) -> String {
    let digest = Sha256::digest(token.trim().as_bytes());
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_tokens_are_random_and_only_hashes_need_storage() {
        let first = random_token().unwrap();
        let second = random_token().unwrap();
        assert_eq!(first.len(), 64);
        assert_ne!(first, second);
        assert_ne!(first, hash_token(&first));
    }
}
