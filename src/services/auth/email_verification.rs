use getrandom::fill;
use sha2::{Digest, Sha256};

use crate::{
    api::dto::auth::EmailVerificationStatusDto,
    services::notification::{NotificationMessage, email::send_email_to, senders::send_resend_to},
};

use super::{AuthError, AuthService};

impl AuthService {
    pub async fn email_verification_status(
        &self,
        user_id: i64,
    ) -> Result<EmailVerificationStatusDto, AuthError> {
        let user = self.get_user_by_id(user_id).await?;
        Ok(EmailVerificationStatusDto {
            verified: user.is_email_verify.unwrap_or_default() != 0,
            verified_at: user.email_verify_at,
        })
    }

    pub async fn request_email_verification(&self, user_id: i64) -> Result<(), AuthError> {
        let user = self.get_user_by_id(user_id).await?;
        if user.is_email_verify.unwrap_or_default() != 0 {
            return Ok(());
        }
        let email = user.email.ok_or_else(|| {
            AuthError::InvalidOperation("account does not have an email address".into())
        })?;
        let token = random_token()?;
        self.repo_email_verification
            .issue(
                user_id,
                &hash_token(&token),
                chrono::Utc::now().timestamp() + 24 * 60 * 60,
            )
            .await?;
        let action = std::env::var("RUSTPLOY_PUBLIC_URL")
            .ok()
            .map(|base| format!("{}/verify-email?token={token}", base.trim_end_matches('/')))
            .unwrap_or_else(|| format!("Email verification token: {token}"));
        let message = NotificationMessage::new(
            "Verify your Rustploy email",
            format!("Verify this email address within 24 hours:\n\n{action}"),
        );

        if let Some(config) = self.repo_notif_email.find_for_user(user_id).await? {
            if let Err(error) = send_email_to(&config, &email, &message).await {
                tracing::warn!(user_id, error = %error, "verification SMTP delivery failed");
            }
        } else if let Some(config) = self.repo_notif_resend.find_for_user(user_id).await? {
            if let Err(error) =
                send_resend_to(&reqwest::Client::new(), &config, &email, &message).await
            {
                tracing::warn!(user_id, error = %error, "verification Resend delivery failed");
            }
        } else {
            tracing::warn!(
                user_id,
                "email verification requested but no email provider is configured"
            );
        }
        Ok(())
    }

    pub async fn verify_email(&self, token: &str) -> Result<(), AuthError> {
        if !self
            .repo_email_verification
            .complete(&hash_token(token))
            .await?
        {
            return Err(AuthError::InvalidVerificationToken);
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
