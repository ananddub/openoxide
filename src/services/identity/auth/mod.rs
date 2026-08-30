use std::{fmt, sync::Arc};

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};
use auto_di::singleton;
use sqlx::SqlitePool;

use serde::Serialize;

use crate::{
    api::connect::generated::openoxide::api::v1::{
        LoginRequest, SignupRequest, UpdateUserRequest,
    },
    db::queries::FlashQueries,
    flash_gen::{
        jwt_tokens::JwtTokensInsertTokenParams,
        models::Users,
        notifications::NotificationsFindForUserParams,
        organization_members::OrganizationMembersAddMemberInTransactionParams,
        organizations::CreateOrganizationParams,
        users::{UsersCreateOwnerAndReturnRow, UsersCreateOwnerAndReturnParams, UsersUpdateParams},
    },
    services::integrations::notification::loader::{EmailConfig, ResendConfig},
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

#[derive(Debug, Serialize)]
pub struct AuthResponseDto {
    pub user: JwtSubject,
    pub tokens: TokenPair,
}

pub struct AuthService {
    db: Arc<SqlitePool>,
    jwt: Arc<JwtService>,
    queries: Arc<FlashQueries>,
}

#[singleton]
impl AuthService {
    fn new(db: Arc<SqlitePool>, jwt: Arc<JwtService>, queries: Arc<FlashQueries>) -> Self {
        Self { db, jwt, queries }
    }

    pub async fn signup(&self, input: SignupRequest) -> Result<AuthResponseDto, AuthError> {
        let password = hash_password(input.password).await?;
        let email = input.email.trim().to_lowercase();

        self.queries.groups_create_owner_group_if_not_exists1().await?;
        let group_id = self.queries.groups_create_owner_group_if_not_exists2().await?;

        let avatar = input.avatar.unwrap_or_default();
        let user = self
            .queries
            .users_create_owner_and_return(&UsersCreateOwnerAndReturnParams {
                email,
                first_name: input.first_name.unwrap_or_default(),
                last_name: input.last_name.unwrap_or_default(),
                avatar,
                password,
                group_id,
            })
            .await
            .map(user_from_owner_row)?;

        let first_name = user
            .first_name
            .clone()
            .unwrap_or_else(|| "Default".to_string());
        let org_name = format!("{}'s Organization", first_name);
        let org_slug = format!("{}-organization", first_name.to_lowercase());

        let org = self
            .queries
            .create_organization(&CreateOrganizationParams {
                name: org_name,
                logo: String::new(),
                slug: org_slug,
                owner_id: user.id,
            })
            .await?;

        self.queries
            .organization_members_add_member_in_transaction(
                &OrganizationMembersAddMemberInTransactionParams {
                    role: "ADMIN".into(),
                    user_id: user.id,
                    organization_id: org.id,
                },
            )
            .await?;

        let subject = subject_from_user(&user)?;
        let tokens = self.jwt.generate_token_pair(&subject)?;
        let session_id = uuid::Uuid::new_v4().to_string();
        self.store_token_pair(&tokens, &session_id).await?;

        Ok(AuthResponseDto {
            user: subject,
            tokens,
        })
    }

    pub async fn login(&self, input: LoginRequest) -> Result<AuthResponseDto, AuthError> {
        let email = input.email.trim().to_lowercase();
        let user = match self.queries.get_user_by_email(&email).await {
            Ok(user) => user,
            Err(sqlx::Error::RowNotFound) => return Err(AuthError::InvalidCredentials),
            Err(error) => return Err(error.into()),
        };

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
        let session_id = match self
            .queries
            .jwt_tokens_session_id_for_jti(&old_claims.jti)
            .await
        {
            Ok(session_id) => session_id,
            Err(sqlx::Error::RowNotFound) => None,
            Err(error) => return Err(error.into()),
        }
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let user = self.get_user_by_id(old_claims.user.user_id).await?;
        let subject = subject_from_user(&user)?;
        let tokens = self.jwt.generate_token_pair(&subject)?;

        self.queries
            .jwt_tokens_blacklist_by_jti(&old_claims.jti)
            .await?;
        self.store_token_pair(&tokens, &session_id).await?;

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
        self.queries
            .jwt_tokens_blacklist_all_by_user(user_id)
            .await?;
        Ok(())
    }

    pub async fn is_owner_present(&self) -> Result<bool, AuthError> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(self.db.as_ref())
            .await?;
        Ok(count > 0)
    }

    pub async fn get_whoami(&self, user_id: i64) -> Result<JwtSubject, AuthError> {
        let user = self
            .get_user_by_id(user_id)
            .await
            .map_err(|error| match error {
                sqlx::Error::RowNotFound => AuthError::InvalidToken,
                error => error.into(),
            })?;
        Ok(JwtSubject {
            user_id: user.id,
            email: user.email.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            avatar: user.avatar,
            role: user.role,
            group_id: user.group_id,
        })
    }

    async fn issue_token_pair(&self, subject: &JwtSubject) -> Result<TokenPair, AuthError> {
        let tokens = self.jwt.generate_token_pair(subject)?;
        let session_id = uuid::Uuid::new_v4().to_string();
        self.store_token_pair(&tokens, &session_id).await?;
        Ok(tokens)
    }

    async fn store_token_pair(
        &self,
        tokens: &TokenPair,
        session_id: &str,
    ) -> Result<(), AuthError> {
        let access = self.jwt.validate_access_token(&tokens.access_token)?;
        let refresh = self.jwt.validate_refresh_token(&tokens.refresh_token)?;
        for (claims, token_kind) in [(access, "ACCESS"), (refresh, "REFRESH")] {
            let role = claims.user.role.as_deref().unwrap_or("MEMBER");
            let expired_at = claims.exp as i64;
            self.queries
                .jwt_tokens_insert_token(&JwtTokensInsertTokenParams {
                    jti: claims.jti,
                    role: role.to_string(),
                    user_id: claims.user.user_id,
                    expired_at,
                    session_id: session_id.to_string(),
                    token_kind: token_kind.to_string(),
                })
                .await?;
        }
        Ok(())
    }

    async fn ensure_token_active(&self, jti: &str) -> Result<(), AuthError> {
        let active = self.queries.jwt_tokens_is_token_active(jti).await?;
        if !active {
            return Err(AuthError::InvalidToken);
        }
        Ok(())
    }

    async fn get_user_by_id(&self, id: i64) -> sqlx::Result<Users> {
        self.queries.get_user_by_i_d(id).await
    }

    pub async fn update_user(
        &self,
        user_id: i64,
        input: UpdateUserRequest,
    ) -> Result<JwtSubject, AuthError> {
        let mut user = self
            .get_user_by_id(user_id)
            .await
            .map_err(|error| match error {
                sqlx::Error::RowNotFound => AuthError::InvalidCredentials,
                error => error.into(),
            })?;

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
        self.queries
            .users_update(&UsersUpdateParams {
                email: user.email.clone().unwrap_or_default(),
                last_name: user.last_name.clone().unwrap_or_default(),
                first_name: user.first_name.clone().unwrap_or_default(),
                avatar: user.avatar.clone(),
                role: user.role.clone().unwrap_or_default(),
                about_me: user.about_me.clone().unwrap_or_default(),
                password: user.password.clone(),
                is_email_verify: user.is_email_verify.unwrap_or_default(),
                email_verify_at: user.email_verify_at.unwrap_or_default(),
                two_factor_enable: user.two_factor_enable.unwrap_or_default(),
                is_registered: user.is_registered,
                added_by: user.added_by.unwrap_or_default(),
                group_id: user.group_id,
                created_at: user.created_at,
                updated_at: user.updated_at,
                id: user_id,
            })
            .await?;
        subject_from_user(&user)
    }
}

pub(super) fn subject_from_user(user: &Users) -> Result<JwtSubject, AuthError> {
    let avatar = if user.avatar.starts_with("data:") {
        "custom".to_string()
    } else {
        user.avatar.clone()
    };

    Ok(JwtSubject {
        user_id: user.id,
        email: user.email.clone(),
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        avatar,
        role: user.role.clone(),
        group_id: user.group_id,
    })
}

fn user_from_owner_row(row: UsersCreateOwnerAndReturnRow) -> Users {
    Users {
        id: row.id,
        email: row.email,
        last_name: row.last_name,
        first_name: row.first_name,
        avatar: row.avatar,
        role: row.role,
        about_me: row.about_me,
        password: row.password,
        is_email_verify: row.is_email_verify,
        email_verify_at: row.email_verify_at,
        two_factor_enable: row.two_factor_enable,
        is_registered: row.is_registered,
        added_by: row.added_by,
        group_id: row.group_id,
        created_at: row.created_at,
        updated_at: row.updated_at,
        is_owner: None,
    }
}

pub(crate) async fn notification_configs_for_user(
    queries: &FlashQueries,
    user_id: i64,
) -> Result<(Option<EmailConfig>, Option<ResendConfig>), sqlx::Error> {
    let email = match queries
        .notifications_find_for_user(&NotificationsFindForUserParams {
            user_id,
            notification_type: "EMAIL".into(),
            owner_id: user_id,
        })
        .await
    {
        Ok(row) => email_config_from_json(&row.config_json),
        Err(sqlx::Error::RowNotFound) => None,
        Err(error) => return Err(error),
    };
    let resend = match queries
        .notifications_find_for_user(&NotificationsFindForUserParams {
            user_id,
            notification_type: "RESEND".into(),
            owner_id: user_id,
        })
        .await
    {
        Ok(row) => resend_config_from_json(&row.config_json),
        Err(sqlx::Error::RowNotFound) => None,
        Err(error) => return Err(error),
    };
    Ok((email, resend))
}

fn json_string(value: &serde_json::Value, key: &str) -> String {
    value
        .get(key)
        .and_then(|field| field.as_str())
        .unwrap_or_default()
        .to_string()
}

fn json_string_array(value: &serde_json::Value, key: &str) -> Vec<String> {
    value
        .get(key)
        .and_then(|field| field.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn email_config_from_json(config_json: &str) -> Option<EmailConfig> {
    let value: serde_json::Value = serde_json::from_str(config_json).ok()?;
    Some(EmailConfig {
        smtp_server: json_string(&value, "smtp_server"),
        smtp_port: value
            .get("smtp_port")
            .and_then(|field| field.as_i64())
            .unwrap_or(587),
        username: json_string(&value, "username"),
        password: json_string(&value, "password"),
        from_address: json_string(&value, "from_address"),
        to_addresses: json_string_array(&value, "to_addresses"),
    })
}

fn resend_config_from_json(config_json: &str) -> Option<ResendConfig> {
    let value: serde_json::Value = serde_json::from_str(config_json).ok()?;
    Some(ResendConfig {
        api_key: json_string(&value, "api_key"),
        from_address: json_string(&value, "from_address"),
        to_addresses: json_string_array(&value, "to_addresses"),
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
