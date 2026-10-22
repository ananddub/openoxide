use std::sync::Arc;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};
use auto_di::singleton;
use sqlx::SqlitePool;

use crate::{
    api::connect::generated::openoxide::api::v1::{
        AuthSession, LoginRequest, SignupRequest, UpdateUserRequest,
    },
    db::queries::FlashQueries,
    flash_gen::{
        jwt_tokens::JwtTokensInsertTokenParams, models::Users,
        notifications::NotificationsFindForUserParams, users::UsersUpdateParams,
    },
    services::integrations::notification::loader::{EmailConfig, ResendConfig},
    utils::jwt::{
        claim::{Claims, JwtSubject},
        service::{JwtService, TokenPair},
    },
};

use super::types::{AuthError, AuthResponseDto};

pub struct AuthService {
    pub(super) db: Arc<SqlitePool>,
    pub(super) jwt: Arc<JwtService>,
    pub(super) queries: Arc<FlashQueries>,
}

#[singleton]
impl AuthService {
    fn new(db: Arc<SqlitePool>, jwt: Arc<JwtService>, queries: Arc<FlashQueries>) -> Self {
        Self { db, jwt, queries }
    }

    pub async fn signup(&self, input: SignupRequest) -> Result<AuthResponseDto, AuthError> {
        let password = hash_password(input.password).await?;
        let email = input.email.trim().to_lowercase();
        if email.is_empty() {
            return Err(AuthError::InvalidOperation("email is required".into()));
        }

        let avatar = input.avatar.unwrap_or_default();
        let first_name = input.first_name.unwrap_or_default();
        let last_name = input.last_name.unwrap_or_default();

        // One transaction: a failure in any step must never leave a
        // half-created account (user without organization, membership
        // without owner, tokens without user). The unique indexes added in
        // 20261022120000 are what make concurrent signups safe.
        let mut tx = self.db.begin().await?;

        sqlx::query(
            "INSERT INTO groups (name, is_system, organization_id)
             SELECT 'owner', 1, NULL
             WHERE NOT EXISTS (SELECT 1 FROM groups WHERE name = 'owner')",
        )
        .execute(&mut *tx)
        .await?;
        let group_id: i64 = sqlx::query_scalar("SELECT id FROM groups WHERE name = 'owner'")
            .fetch_one(&mut *tx)
            .await?;

        let user_id: i64 = sqlx::query_scalar(
            "INSERT INTO users (email, first_name, last_name, avatar, role, password, is_registered, group_id, added_by)
             VALUES (?, ?, ?, ?, 'OWNER', ?, 1, ?, NULL)
             RETURNING id",
        )
        .bind(&email)
        .bind(&first_name)
        .bind(&last_name)
        .bind(&avatar)
        .bind(&password)
        .bind(group_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_user_write_error)?;

        let display_name = if first_name.trim().is_empty() {
            "Default".to_string()
        } else {
            first_name.trim().to_string()
        };
        let org_name = format!("{display_name}'s Organization");
        let base_slug = normalize_slug(&format!("{display_name}-organization"));
        let base_slug = if base_slug.is_empty() {
            "organization".to_string()
        } else {
            base_slug
        };

        // Same first name → same base slug; pick the first free suffix
        // instead of failing the whole signup.
        let mut slug = base_slug.clone();
        let mut suffix = 2;
        let organization_id: i64 = loop {
            let inserted: Option<i64> = sqlx::query_scalar(
                "INSERT INTO organization (name, logo, slug, owner_id)
                 VALUES (?, '', ?, ?)
                 ON CONFLICT DO NOTHING
                 RETURNING id",
            )
            .bind(&org_name)
            .bind(&slug)
            .bind(user_id)
            .fetch_optional(&mut *tx)
            .await?;
            if let Some(id) = inserted {
                break id;
            }
            if suffix > 100 {
                return Err(AuthError::Internal);
            }
            slug = format!("{base_slug}-{suffix}");
            suffix += 1;
        };

        sqlx::query(
            "INSERT INTO organization_members (role, user_id, organization_id)
             VALUES ('ADMIN', ?, ?)",
        )
        .bind(user_id)
        .bind(organization_id)
        .execute(&mut *tx)
        .await?;

        let subject = JwtSubject {
            user_id,
            email: Some(email),
            first_name: Some(first_name),
            last_name: Some(last_name),
            avatar: if avatar.starts_with("data:") {
                "custom".to_string()
            } else {
                avatar
            },
            role: Some("OWNER".to_string()),
            group_id,
        };
        let tokens = self.jwt.generate_token_pair(&subject)?;
        let session_id = uuid::Uuid::new_v4().to_string();
        let access = self.jwt.validate_access_token(&tokens.access_token)?;
        let refresh = self.jwt.validate_refresh_token(&tokens.refresh_token)?;
        for (claims, token_kind) in [(access, "ACCESS"), (refresh, "REFRESH")] {
            sqlx::query(
                "INSERT INTO jwt_tokens (jti, role, user_id, is_blacklist, expired_at, session_id, token_kind)
                 VALUES (?, ?, ?, 0, ?, ?, ?)",
            )
            .bind(&claims.jti)
            .bind(claims.user.role.as_deref().unwrap_or("MEMBER"))
            .bind(claims.user.user_id)
            .bind(claims.exp as i64)
            .bind(&session_id)
            .bind(token_kind)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

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

        // Reissue ONLY the access token; keep the caller's refresh token valid
        // until its own expiry. Rotating (and blacklisting) the refresh token
        // on every refresh caused spurious logouts: any slightly stale copy of
        // the refresh token (concurrent requests, a second tab) would hit the
        // just-revoked token and end the session. Non-rotating refresh avoids
        // that race while the refresh token's own expiry still bounds its life.
        let access_token = self.jwt.generate_access_token(&subject)?;
        let access_claims = self.jwt.validate_access_token(&access_token)?;
        self.queries
            .jwt_tokens_insert_token(&JwtTokensInsertTokenParams {
                jti: access_claims.jti,
                role: access_claims
                    .user
                    .role
                    .as_deref()
                    .unwrap_or("MEMBER")
                    .to_string(),
                user_id: access_claims.user.user_id,
                expired_at: access_claims.exp as i64,
                session_id,
                token_kind: "ACCESS".to_string(),
            })
            .await?;

        Ok(AuthResponseDto {
            user: subject,
            tokens: TokenPair {
                access_token,
                refresh_token: refresh_token.to_string(),
            },
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

    pub(super) async fn get_user_by_id(&self, id: i64) -> sqlx::Result<Users> {
        self.queries.get_user_by_i_d(id).await
    }

    pub async fn update_user(
        &self,
        user_id: i64,
        input: UpdateUserRequest,
        keep_jti: Option<&str>,
    ) -> Result<JwtSubject, AuthError> {
        let mut user = self
            .get_user_by_id(user_id)
            .await
            .map_err(|error| match error {
                sqlx::Error::RowNotFound => AuthError::InvalidCredentials,
                error => error.into(),
            })?;

        let next_email = input
            .email
            .as_deref()
            .map(|email| email.trim().to_lowercase())
            .filter(|email| !email.is_empty());
        let email_changed = next_email
            .as_deref()
            .is_some_and(|email| user.email.as_deref() != Some(email));
        let next_password = input
            .password
            .filter(|password| !password.trim().is_empty());
        let password_changed = next_password.is_some();

        // Changing the address or password is a credential operation: prove
        // the current password first so a stolen access token cannot silently
        // take over the account.
        if email_changed || password_changed {
            let current = input.current_password.unwrap_or_default();
            if current.trim().is_empty() {
                return Err(AuthError::InvalidOperation(
                    "current password is required to change email or password".into(),
                ));
            }
            verify_password(current, user.password.clone()).await?;
        }

        if let Some(email) = next_email {
            user.email = Some(email);
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
        if let Some(password) = next_password {
            user.password = hash_password(password).await?;
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
            .await
            .map_err(map_user_write_error)?;
        if password_changed {
            // Every other session was authenticated with the old password and
            // must sign in again; the caller's own session stays usable.
            self.revoke_other_sessions(user_id, keep_jti).await?;
        }
        subject_from_user(&user)
    }

    /// Revoke every token for the user except the session that owns
    /// `keep_jti`. Credential changes (password, 2FA) must drop other
    /// sessions without logging the user out of the one they just proved.
    pub(crate) async fn revoke_other_sessions(
        &self,
        user_id: i64,
        keep_jti: Option<&str>,
    ) -> Result<(), AuthError> {
        let session_id = match keep_jti {
            Some(jti) => match self.queries.jwt_tokens_session_id_for_jti(jti).await {
                Ok(session_id) => session_id,
                Err(sqlx::Error::RowNotFound) => None,
                Err(error) => return Err(error.into()),
            },
            None => None,
        };
        match session_id {
            Some(session_id) => {
                sqlx::query(
                    "UPDATE jwt_tokens
                     SET is_blacklist = 1, blacklist_at = strftime('%s','now')
                     WHERE user_id = ? AND session_id != ? AND is_blacklist = 0;",
                )
                .bind(user_id)
                .bind(session_id)
                .execute(self.db.as_ref())
                .await?;
            }
            // No usable jti (e.g. a token minted before sessions were
            // tracked): fall back to revoking everything.
            None => {
                self.queries
                    .jwt_tokens_blacklist_all_by_user(user_id)
                    .await?
            }
        }
        Ok(())
    }
}

impl AuthService {
    pub async fn list_sessions(
        &self,
        user_id: i64,
        current_jti: &str,
    ) -> Result<Vec<AuthSession>, AuthError> {
        let current_session = match self
            .queries
            .jwt_tokens_session_id_for_jti(current_jti)
            .await
        {
            Ok(session_id) => session_id,
            Err(sqlx::Error::RowNotFound) => None,
            Err(error) => return Err(error.into()),
        };
        Ok(self
            .queries
            .jwt_tokens_list_sessions(user_id)
            .await?
            .into_iter()
            .map(|row| AuthSession {
                current: current_session.as_deref() == row.session_id.as_deref(),
                session_id: row.session_id.unwrap_or_default(),
                created_at: row.created_at.map(|at| at.timestamp()).unwrap_or_default(),
                expires_at: row.expires_at.map(|at| at.timestamp()),
                active: row.active.map(|at| at.timestamp() != 0).unwrap_or_default(),
            })
            .collect())
    }

    pub async fn revoke_session(&self, user_id: i64, session_id: &str) -> Result<bool, AuthError> {
        let sessions = self.queries.jwt_tokens_list_sessions(user_id).await?;
        let Some(target) = sessions
            .iter()
            .find(|row| row.session_id.as_deref() == Some(session_id))
        else {
            return Ok(false);
        };
        if !target
            .active
            .map(|at| at.timestamp() != 0)
            .unwrap_or_default()
        {
            return Ok(false);
        }
        self.queries
            .jwt_tokens_revoke_session(user_id, session_id)
            .await?;
        Ok(true)
    }
}

pub(crate) fn subject_from_user(user: &Users) -> Result<JwtSubject, AuthError> {
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

fn normalize_slug(value: &str) -> String {
    value
        .trim()
        .to_ascii_lowercase()
        .chars()
        .fold((String::new(), false), |(mut output, separator), ch| {
            if ch.is_ascii_alphanumeric() {
                output.push(ch);
                (output, false)
            } else if !output.is_empty() && !separator {
                output.push('-');
                (output, true)
            } else {
                (output, separator)
            }
        })
        .0
        .trim_end_matches('-')
        .to_owned()
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

pub(crate) async fn hash_password(password: String) -> Result<String, AuthError> {
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

pub(crate) async fn verify_password(password: String, encoded: String) -> Result<(), AuthError> {
    tokio::task::spawn_blocking(move || {
        let hash = PasswordHash::new(&encoded).map_err(|_| AuthError::InvalidCredentials)?;
        Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .map_err(|_| AuthError::InvalidCredentials)
    })
    .await
    .map_err(|_| AuthError::Internal)?
}

/// Turn the users.email unique-index violation into a clean client error
/// instead of leaking a 500 from SQLite.
fn map_user_write_error(error: sqlx::Error) -> AuthError {
    if let sqlx::Error::Database(database_error) = &error
        && database_error
            .message()
            .contains("UNIQUE constraint failed")
    {
        return AuthError::InvalidOperation("an account with this email already exists".into());
    }
    error.into()
}
