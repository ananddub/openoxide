use std::{fmt, sync::Arc};

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};
use auto_di::singleton;
use sqlx::{Sqlite, SqlitePool, Transaction};

use crate::{
    api::dto::auth::{AuthResponseDto, LoginDto, SignupDto},
    db::models::users::User,
    repository::{
        GroupRepository, JwtTokenRepository, OrganizationMemberRepository,
        OrganizationRepository, UserRepository,
    },
    utils::jwt::{
        claim::{Claims, JwtSubject},
        error::TokenError,
        service::{JwtService, TokenPair},
    },
};

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    InvalidToken,
    Database(sqlx::Error),
    Internal,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCredentials => write!(f, "invalid email or password"),
            Self::InvalidToken => write!(f, "invalid or revoked token"),
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
    ) -> Self {
        Self {
            db,
            jwt,
            repo_user,
            repo_token,
            repo_group,
            repo_org,
            repo_member,
        }
    }

    pub async fn signup(&self, input: SignupDto) -> Result<AuthResponseDto, AuthError> {
        let password = hash_password(input.password).await?;
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
                input.email,
                input.first_name,
                input.last_name,
                avatar,
                password,
                group_id,
            )
            .await?;

        let user_id = user.id.ok_or_else(|| AuthError::Internal)?;
        let first_name = user.first_name.clone().unwrap_or_else(|| "Default".to_string());
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
        self.store_token_pair(&mut tx, &tokens).await?;
        tx.commit().await?;

        Ok(AuthResponseDto {
            user: subject,
            tokens,
        })
    }

    pub async fn login(&self, input: LoginDto) -> Result<AuthResponseDto, AuthError> {
        let user = self
            .repo_user
            .get_by_email(&input.email)
            .await?
            .ok_or(AuthError::InvalidCredentials)?;

        verify_password(input.password, user.password.clone()).await?;
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

        let user = self.get_user_by_id(old_claims.user.user_id).await?;
        let subject = subject_from_user(&user)?;
        let tokens = self.jwt.generate_token_pair(&subject)?;
        let mut tx = self.db.begin().await?;

        self.repo_token
            .blacklist_by_jti(&mut tx, &old_claims.jti)
            .await?;
        self.store_token_pair(&mut tx, &tokens).await?;
        tx.commit().await?;

        Ok(AuthResponseDto {
            user: subject,
            tokens,
        })
    }

    pub async fn validate_access_token(&self, token: &str) -> Result<Claims, AuthError> {
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
        let mut tx = self.db.begin().await?;
        self.store_token_pair(&mut tx, &tokens).await?;
        tx.commit().await?;
        Ok(tokens)
    }

    async fn store_token_pair(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        tokens: &TokenPair,
    ) -> Result<(), AuthError> {
        let access = self.jwt.validate_access_token(&tokens.access_token)?;
        let refresh = self.jwt.validate_refresh_token(&tokens.refresh_token)?;
        for claims in [access, refresh] {
            let role = claims.user.role.as_deref().unwrap_or("MEMBER");
            let expired_at = claims.exp as i64;
            self.repo_token
                .insert_token(
                    tx,
                    claims.jti,
                    role.to_string(),
                    claims.user.user_id,
                    expired_at,
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
}

fn subject_from_user(user: &User) -> Result<JwtSubject, AuthError> {
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

async fn hash_password(password: String) -> Result<String, AuthError> {
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

async fn verify_password(password: String, encoded: String) -> Result<(), AuthError> {
    tokio::task::spawn_blocking(move || {
        let hash = PasswordHash::new(&encoded).map_err(|_| AuthError::InvalidCredentials)?;
        Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .map_err(|_| AuthError::InvalidCredentials)
    })
    .await
    .map_err(|_| AuthError::Internal)?
}

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use super::*;
    use crate::utils::jwt::config::JwtConfig;

    // async fn service() -> AuthService {
    //     let pool = SqlitePoolOptions::new()
    //         .max_connections(1)
    //         .connect("sqlite::memory:")
    //         .await
    //         .unwrap();
    //     sqlx::query("PRAGMA foreign_keys = ON")
    //         .execute(&pool)
    //         .await
    //         .unwrap();
    //     sqlx::query("CREATE TABLE groups (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL UNIQUE, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))) STRICT").execute(&pool).await.unwrap();
    //     sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, email TEXT UNIQUE, last_name TEXT, first_name TEXT, avatar TEXT NOT NULL, role TEXT DEFAULT 'OWNER', about_me TEXT, password TEXT NOT NULL, is_email_verify INTEGER DEFAULT 0, email_verify_at INTEGER, two_factor_enable INTEGER DEFAULT 0, is_registered INTEGER NOT NULL DEFAULT 0, added_by INTEGER REFERENCES users(id), group_id INTEGER NOT NULL REFERENCES groups(id), created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))) STRICT").execute(&pool).await.unwrap();
    //     sqlx::query("CREATE TABLE jwt_tokens (id INTEGER PRIMARY KEY AUTOINCREMENT, jti TEXT NOT NULL, role TEXT NOT NULL, user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE, is_blacklist INTEGER DEFAULT 0, blacklist_at INTEGER, expired_at INTEGER, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))) STRICT").execute(&pool).await.unwrap();
    //
    //     let db = Arc::new(pool);

        // AuthService {
        //     db: db.clone(),
        //     jwt: Arc::new(JwtService::new(Arc::new(JwtConfig::default()))),
        //     repo_user: Arc::new(UserRepository::new(db.clone())),
        //     repo_token: Arc::new(JwtTokenRepository::new(db.clone())),
        //     repo_group: Arc::new(GroupRepository::new(db.clone())),
        // }
    // }

    // #[tokio::test]
    // async fn signup_login_refresh_and_logout_flow() {
    //     let service = service().await;
    //     let signup = service
    //         .signup(SignupDto {
    //             email: "owner@example.com".into(),
    //             password: "strong-password".into(),
    //             first_name: Some("Owner".into()),
    //             last_name: None,
    //             avatar: None,
    //         })
    //         .await
    //         .unwrap();
    //
    //     let stored_password: String = sqlx::query_scalar("SELECT password FROM users WHERE id = ?")
    //         .bind(signup.user.user_id)
    //         .fetch_one(service.db.as_ref())
    //         .await
    //         .unwrap();
    //     assert!(stored_password.starts_with("$argon2"));
    //     assert_ne!(stored_password, "strong-password");
    //
    //     let login = service
    //         .login(LoginDto {
    //             email: "owner@example.com".into(),
    //             password: "strong-password".into(),
    //         })
    //         .await
    //         .unwrap();
    //     service
    //         .validate_access_token(&login.tokens.access_token)
    //         .await
    //         .unwrap();
    //
    //     let refreshed = service.refresh(&login.tokens.refresh_token).await.unwrap();
    //     assert!(service.refresh(&login.tokens.refresh_token).await.is_err());
    //     service.logout_all(refreshed.user.user_id).await.unwrap();
    //     assert!(
    //         service
    //             .validate_access_token(&refreshed.tokens.access_token)
    //             .await
    //             .is_err()
    //     );
    // }
}
