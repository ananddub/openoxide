use getrandom::fill;
use sha2::{Digest, Sha256};

use crate::{
    api::connect::generated::openoxide::api::v1::EmailVerificationStatus,
    flash_gen::email_verification_tokens::EmailVerificationTokensIssue2Params,
    services::notification::{NotificationMessage, email::send_email_to, senders::send_resend_to},
};

use super::{AuthError, AuthService, notification_configs_for_user};

impl AuthService {
    pub async fn email_verification_status(
        &self,
        user_id: i64,
    ) -> Result<EmailVerificationStatus, AuthError> {
        let user = self.get_user_by_id(user_id).await?;
        Ok(EmailVerificationStatus {
            verified: user.is_email_verify.unwrap_or_default() != 0,
            verified_at: user.email_verify_at,
            ..Default::default()
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
        self.queries
            .email_verification_tokens_issue1(user_id)
            .await?;
        self.queries
            .email_verification_tokens_issue2(&EmailVerificationTokensIssue2Params {
                user_id,
                token_hash: hash_token(&token),
                expires_at: chrono::Utc::now().timestamp() + 24 * 60 * 60,
            })
            .await?;
        let action = std::env::var("OPENOXIDE_PUBLIC_URL")
            .or_else(|_| std::env::var("RUSTPLOY_PUBLIC_URL"))
            .ok()
            .map(|base| format!("{}/verify-email?token={token}", base.trim_end_matches('/')))
            .unwrap_or_else(|| format!("Email verification token: {token}"));
        let message = NotificationMessage::new(
            "Verify your OpenOxide email",
            format!("Verify this email address within 24 hours:\n\n{action}"),
        );

        let (email_config, resend_config) =
            notification_configs_for_user(&self.queries, user_id).await?;
        if let Some(config) = email_config {
            if let Err(error) = send_email_to(&config, &email, &message).await {
                tracing::warn!(user_id, error = %error, "verification SMTP delivery failed");
            }
        } else if let Some(config) = resend_config {
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
        let user_id = match self
            .queries
            .email_verification_tokens_complete1(&hash_token(token))
            .await
        {
            Ok(user_id) => user_id,
            Err(sqlx::Error::RowNotFound) => return Err(AuthError::InvalidVerificationToken),
            Err(error) => return Err(error.into()),
        };
        self.queries
            .email_verification_tokens_complete2(user_id)
            .await?;
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
