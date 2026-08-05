use crate::api::dto::auth::AuthSessionDto;

use super::{AuthError, AuthService};

impl AuthService {
    pub async fn list_sessions(
        &self,
        user_id: i64,
        current_jti: &str,
    ) -> Result<Vec<AuthSessionDto>, AuthError> {
        let current_session = self.repo_token.session_id_for_jti(current_jti).await?;
        Ok(self
            .repo_token
            .list_sessions(user_id)
            .await?
            .into_iter()
            .map(|row| AuthSessionDto {
                current: current_session.as_deref() == Some(row.session_id.as_str()),
                session_id: row.session_id,
                created_at: row.created_at,
                expires_at: row.expires_at,
                active: row.active,
            })
            .collect())
    }

    pub async fn revoke_session(&self, user_id: i64, session_id: &str) -> Result<bool, AuthError> {
        self.repo_token
            .revoke_session(user_id, session_id)
            .await
            .map_err(Into::into)
    }
}
