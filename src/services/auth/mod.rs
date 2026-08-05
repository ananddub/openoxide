use std::{fmt, sync::Arc};

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};
use auto_di::singleton;
use sqlx::{Sqlite, SqlitePool, Transaction};

use crate::{
    api::dto::auth::{AuthResponseDto, LoginDto, SignupDto, UpdateUserDto},
    db::models::users::User,
    repository::{
        EmailVerificationTokenRepository, GroupRepository, JwtTokenRepository,
        NotifEmailRepository, NotifResendRepository, OrganizationMemberRepository,
        OrganizationRepository, PasswordResetTokenRepository, PersonalAccessTokenRepository,
        TwoFactorRepository, UserRepository,
    },
    utils::jwt::{
        claim::{Claims, JwtSubject},
        error::TokenError,
        service::{JwtService, TokenPair},
    },
};

pub mod api_tokens;
pub mod email_verification;
pub mod password_reset;
pub mod sessions;
pub mod two_factor;

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

pub struct AuthService {
    db: Arc<SqlitePool>,
    jwt: Arc<JwtService>,
    repo_user: Arc<UserRepository>,
    repo_token: Arc<JwtTokenRepository>,
    repo_group: Arc<GroupRepository>,
    repo_org: Arc<OrganizationRepository>,
    repo_member: Arc<OrganizationMemberRepository>,
    pub(super) repo_two_factor: Arc<TwoFactorRepository>,
    pub(super) repo_password_reset: Arc<PasswordResetTokenRepository>,
    pub(super) repo_notif_email: Arc<NotifEmailRepository>,
    pub(super) repo_notif_resend: Arc<NotifResendRepository>,
    pub(super) repo_api_token: Arc<PersonalAccessTokenRepository>,
    pub(super) repo_email_verification: Arc<EmailVerificationTokenRepository>,
}

#[singleton]
impl AuthService {
    fn new(
        db: Arc<SqlitePool>,
        jwt: Arc<JwtService>,
        repo_user: Arc<UserRepository>,
        repo_token: Arc<JwtTokenRepository>,
        repo_group: Arc<GroupRepository>,
        repo_org: Arc<OrganizationRepository>,
        repo_member: Arc<OrganizationMemberRepository>,
        repo_two_factor: Arc<TwoFactorRepository>,
        repo_password_reset: Arc<PasswordResetTokenRepository>,
        repo_notif_email: Arc<NotifEmailRepository>,
        repo_notif_resend: Arc<NotifResendRepository>,
        repo_api_token: Arc<PersonalAccessTokenRepository>,
        repo_email_verification: Arc<EmailVerificationTokenRepository>,
    ) -> Self {
        Self {
            db,
            jwt,
            repo_user,
            repo_token,
            repo_group,
            repo_org,
            repo_member,
            repo_two_factor,
            repo_password_reset,
            repo_notif_email,
            repo_notif_resend,
            repo_api_token,
            repo_email_verification,
        }
    }

    pub async fn signup(&self, input: SignupDto) -> Result<AuthResponseDto, AuthError> {
        let password = hash_password(input.password).await?;
        let email = input.email.trim().to_lowercase();
        let mut tx = self.db.begin().await?;

        let group_id = self
            .repo_group
            .create_owner_group_if_not_exists(&mut tx)
            .await?;

        let avatar = input.avatar.unwrap_or_default();
        let user = self
            .repo_user
            .create_owner_and_return(
                &mut tx,
                email,
                input.first_name,
                input.last_name,
                avatar,
                password,
                group_id,
            )
            .await?;

        let user_id = user.id.ok_or_else(|| AuthError::Internal)?;
        let first_name = user
            .first_name
            .clone()
            .unwrap_or_else(|| "Default".to_string());
        let org_name = format!("{}'s Organization", first_name);
        let org_slug = format!("{}-organization", first_name.to_lowercase());

        let org = self
            .repo_org
            .create_in_transaction(&mut tx, org_name, None, org_slug, user_id)
            .await?;

        let org_id = org.id.ok_or_else(|| AuthError::Internal)?;

        self.repo_member
            .add_member_in_transaction(&mut tx, "ADMIN", user_id, org_id)
            .await?;

        let subject = subject_from_user(&user)?;
        let tokens = self.jwt.generate_token_pair(&subject)?;
        let session_id = uuid::Uuid::new_v4().to_string();
        self.store_token_pair(&mut tx, &tokens, &session_id).await?;
        tx.commit().await?;

        Ok(AuthResponseDto {
            user: subject,
            tokens,
        })
    }

    pub async fn login(&self, input: LoginDto) -> Result<AuthResponseDto, AuthError> {
        let email = input.email.trim().to_lowercase();
        let user = self
            .repo_user
            .get_by_email(&email)
            .await?
            .ok_or(AuthError::InvalidCredentials)?;

        verify_password(input.password, user.password.clone()).await?;
        self.verify_login_second_factor(
            &user,
            input.two_factor_code.as_deref(),
            input.recovery_code.as_deref(),
        )
        .await?;
        let subject = subject_from_user(&user)?;
        let tokens = self.issue_token_pair(&subject).await?;
        Ok(AuthResponseDto {
            user: subject,
            tokens,
        })
    }

    pub async fn refresh(&self, refresh_token: &str) -> Result<AuthResponseDto, AuthError> {
        let old_claims = self.jwt.validate_refresh_token(refresh_token)?;
        self.ensure_token_active(&old_claims.jti).await?;
        let session_id = self
            .repo_token
            .session_id_for_jti(&old_claims.jti)
            .await?
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let user = self.get_user_by_id(old_claims.user.user_id).await?;
        let subject = subject_from_user(&user)?;
        let tokens = self.jwt.generate_token_pair(&subject)?;
        let mut tx = self.db.begin().await?;

        self.repo_token
            .blacklist_by_jti(&mut tx, &old_claims.jti)
            .await?;
        self.store_token_pair(&mut tx, &tokens, &session_id).await?;
        tx.commit().await?;

        Ok(AuthResponseDto {
            user: subject,
            tokens,
        })
    }

    pub async fn validate_access_token(&self, token: &str) -> Result<Claims, AuthError> {
        if token.starts_with("rp_") {
            return self.validate_personal_access_token(token).await;
        }
        let claims = self.jwt.validate_access_token(token)?;
        self.ensure_token_active(&claims.jti).await?;
        Ok(claims)
    }

    pub async fn logout_all(&self, user_id: i64) -> Result<(), AuthError> {
        self.repo_token.blacklist_all_by_user(user_id).await?;
        Ok(())
    }

    pub async fn is_owner_present(&self) -> Result<bool, AuthError> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(self.db.as_ref())
            .await?;
        Ok(count > 0)
    }

    async fn issue_token_pair(&self, subject: &JwtSubject) -> Result<TokenPair, AuthError> {
        let tokens = self.jwt.generate_token_pair(subject)?;
        let session_id = uuid::Uuid::new_v4().to_string();
        let mut tx = self.db.begin().await?;
        self.store_token_pair(&mut tx, &tokens, &session_id).await?;
        tx.commit().await?;
        Ok(tokens)
    }

    async fn store_token_pair(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        tokens: &TokenPair,
        session_id: &str,
    ) -> Result<(), AuthError> {
        let access = self.jwt.validate_access_token(&tokens.access_token)?;
        let refresh = self.jwt.validate_refresh_token(&tokens.refresh_token)?;
        for (claims, token_kind) in [(access, "ACCESS"), (refresh, "REFRESH")] {
            let role = claims.user.role.as_deref().unwrap_or("MEMBER");
            let expired_at = claims.exp as i64;
            self.repo_token
                .insert_token(
                    tx,
                    claims.jti,
                    role.to_string(),
                    claims.user.user_id,
                    expired_at,
                    session_id,
                    token_kind,
                )
                .await?;
        }
        Ok(())
    }

    async fn ensure_token_active(&self, jti: &str) -> Result<(), AuthError> {
        let active = self.repo_token.is_token_active(jti).await?;
        if !active {
            return Err(AuthError::InvalidToken);
        }
        Ok(())
    }

    async fn get_user_by_id(&self, id: i64) -> sqlx::Result<User> {
        self.repo_user
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn update_user(
        &self,
        user_id: i64,
        input: UpdateUserDto,
    ) -> Result<JwtSubject, AuthError> {
        let mut user = self
            .repo_user
            .get_by_id(user_id)
            .await?
            .ok_or(AuthError::InvalidCredentials)?;

        if let Some(email) = input.email {
            if !email.trim().is_empty() {
                user.email = Some(email.trim().to_lowercase());
            }
        }
        if let Some(first_name) = input.first_name {
            user.first_name = Some(first_name);
        }
        if let Some(last_name) = input.last_name {
            user.last_name = Some(last_name);
        }
        if let Some(avatar) = input.avatar {
            user.avatar = avatar;
        }
        if let Some(password) = input.password {
            if !password.trim().is_empty() {
                user.password = hash_password(password).await?;
            }
        }

        user.updated_at = chrono::Utc::now().timestamp();
        self.repo_user.update(user_id, &user).await?;
        subject_from_user(&user)
    }
}

pub(super) fn subject_from_user(user: &User) -> Result<JwtSubject, AuthError> {
    Ok(JwtSubject {
        user_id: user.id.ok_or(AuthError::Internal)?,
        email: user.email.clone(),
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        avatar: user.avatar.clone(),
        role: user.role.clone(),
        group_id: user.group_id,
    })
}

pub(super) async fn hash_password(password: String) -> Result<String, AuthError> {
    tokio::task::spawn_blocking(move || {
        Argon2::default()
            .hash_password(password.as_bytes())
            .map(|hash| hash.to_string())
            .map_err(|_| AuthError::Internal)
    })
    .await
    .map_err(|_| AuthError::Internal)?
    .map_err(|_| AuthError::Internal)
}

pub(super) async fn verify_password(password: String, encoded: String) -> Result<(), AuthError> {
    tokio::task::spawn_blocking(move || {
        let hash = PasswordHash::new(&encoded).map_err(|_| AuthError::InvalidCredentials)?;
        Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .map_err(|_| AuthError::InvalidCredentials)
    })
    .await
    .map_err(|_| AuthError::Internal)?
}
