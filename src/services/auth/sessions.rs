use crate::api::connect::generated::openoxide::api::v1::AuthSession;

use super::{AuthError, AuthService};

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
                ..Default::default()
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
        if !target.active.map(|at| at.timestamp() != 0).unwrap_or_default() {
            return Ok(false);
        }
        self.queries
            .jwt_tokens_revoke_session(user_id, session_id)
            .await?;
        Ok(true)
    }
}
