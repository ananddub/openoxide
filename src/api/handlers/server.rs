use crate::{
    api::dto::server::{
        ServerAuditDto, ServerConnectionDto, ServerConnectionResponseDto, SetupOutcomeDto,
        SetupServerDto, TestDirectConnectionDto,
    },
    core::middleware::permission::{
        RequirePermission, ServerCreatePermission, ServerDeletePermission, ServerReadPermission,
    },
    db::repository::ssh_keys::SshKeyRepository,
    services::remote_server::ServerService,
    utils::{
        exec::{ExecError, RemoteExecutor, SshAuth, SshHostKey},
        setup::{ServerSetup, SetupConfig},
    },
};
use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};
use std::sync::Arc;

type ApiError = (StatusCode, String);

pub struct ServerController {
    service: Arc<ServerService>,
    ssh_key_repo: Arc<SshKeyRepository>,
}

#[controller("/servers")]
impl ServerController {
    fn new(service: Arc<ServerService>, ssh_key_repo: Arc<SshKeyRepository>) -> Self {
        Self {
            service,
            ssh_key_repo,
        }
    }

    #[post("/test-direct-connection")]
    async fn test_direct_connection(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerReadPermission>,
        Json(body): Json<TestDirectConnectionDto>,
    ) -> Result<Json<ServerConnectionResponseDto>, ApiError> {
        let port = body.port.unwrap_or(22);
        let auth = if let Some(key_id) = body.ssh_key_id {
            if let Some(key) = self
                .ssh_key_repo
                .get_by_id(key_id)
                .await
                .map_err(map_sqlx_error)?
            {
                SshAuth::key_pair(key.private_key, key.public_key)
            } else {
                return Err((StatusCode::BAD_REQUEST, "SSH Key not found".into()));
            }
        } else {
            return Err((
                StatusCode::BAD_REQUEST,
                "SSH Key ID is required to test connection".into(),
            ));
        };

        let executor = RemoteExecutor::new(
            body.ip_address,
            port,
            body.username,
            auth,
            SshHostKey::InsecureAcceptAny,
        );
        executor
            .run("true", std::iter::empty::<&str>())
            .await
            .map_err(map_exec_error)?;

        Ok(Json(ServerConnectionResponseDto {
            connected: true,
            reused_sessions: 0,
            max_pool_size: 0,
            connections: 0,
            active_channels: 0,
            max_channels_per_session: 0,
        }))
    }

    #[post("/{id}/test-connection")]
    async fn test_connection(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerReadPermission>,
        Path(id): Path<i64>,
        Json(body): Json<ServerConnectionDto>,
    ) -> Result<Json<ServerConnectionResponseDto>, ApiError> {
        let executor = self
            .executor(
                id,
                &body.host_key_fingerprint,
                body.sudo_password.as_deref(),
                body.pool_size,
                false,
            )
            .await?;
        executor
            .run("true", std::iter::empty::<&str>())
            .await
            .map_err(map_exec_error)?;
        self.service
            .touch_test_connection(id)
            .await
            .map_err(map_sqlx_error)?;
        Ok(Json(ServerConnectionResponseDto {
            connected: true,
            reused_sessions: 0,
            max_pool_size: 0,
            connections: 0,
            active_channels: 0,
            max_channels_per_session: 0,
        }))
    }

    #[post("/{id}/audit")]
    async fn audit(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerReadPermission>,
        Path(id): Path<i64>,
        Json(body): Json<ServerConnectionDto>,
    ) -> Result<Json<ServerAuditDto>, ApiError> {
        let executor = self
            .executor(
                id,
                &body.host_key_fingerprint,
                body.sudo_password.as_deref(),
                body.pool_size,
                false,
            )
            .await?;
        let audit = ServerSetup::new_remote(executor, SetupConfig::default())
            .audit()
            .await
            .map_err(map_exec_error)?;
        self.service
            .touch_test_connection(id)
            .await
            .map_err(map_sqlx_error)?;
        Ok(Json(audit.into()))
    }

    #[post("/{id}/setup")]
    async fn setup(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Path(id): Path<i64>,
        Json(body): Json<SetupServerDto>,
    ) -> Result<Json<SetupOutcomeDto>, ApiError> {
        let executor = self
            .executor(
                id,
                &body.host_key_fingerprint,
                body.sudo_password.as_deref(),
                body.pool_size,
                true,
            )
            .await?;
        let mut config = SetupConfig::default();
        config.advertise_addr = body.advertise_addr;
        if let Some(email) = body.acme_email {
            config.acme_email = email;
        }
        let outcome = ServerSetup::new_remote(executor, config)
            .setup_all_oneshot(body.install_dependencies)
            .await
            .map_err(map_exec_error)?;
        self.service
            .set_status(id, "ACTIVE")
            .await
            .map_err(map_sqlx_error)?;
        self.service
            .touch_test_connection(id)
            .await
            .map_err(map_sqlx_error)?;
        Ok(Json(outcome.into()))
    }

    #[get("/{id}/sessions")]
    async fn sessions(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerReadPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<ServerConnectionResponseDto>, ApiError> {
        self.service
            .connection_details(id)
            .await
            .map_err(map_sqlx_error)?;

        Ok(Json(ServerConnectionResponseDto {
            connected: true,
            reused_sessions: 0,
            max_pool_size: 0,
            connections: 0,
            active_channels: 0,
            max_channels_per_session: 0,
        }))
    }

    #[delete("/{id}/sessions")]
    async fn clear_sessions(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerDeletePermission>,
        Path(_id): Path<i64>,
    ) -> StatusCode {
        StatusCode::NO_CONTENT
    }

    async fn executor(
        &self,
        id: i64,
        host_key: &Option<String>,
        sudo_password: Option<&str>,
        _pool_size: Option<usize>,
        require_sudo: bool,
    ) -> Result<RemoteExecutor, ApiError> {
        let (server, key) = self
            .service
            .connection_details(id)
            .await
            .map_err(map_sqlx_error)?;
        if key.private_key.trim().is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "selected SSH key has no private key".into(),
            ));
        }
        let port = u16::try_from(server.port).map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                "SSH port must be between 0 and 65535".into(),
            )
        })?;
        let host_policy = match host_key {
            Some(fingerprint) => SshHostKey::PinnedSha256(fingerprint.clone()),
            None => {
                tracing::warn!(
                    server_id = id,
                    "SSH host key verification disabled; provide host_key_fingerprint"
                );
                SshHostKey::InsecureAcceptAny
            }
        };
        let auth = SshAuth::key_pair(key.private_key, key.public_key);
        let mut executor =
            RemoteExecutor::new(server.ip_address, port, server.username, auth, host_policy);
        executor = if let Some(password) = sudo_password {
            executor.with_sudo_password(password)
        } else if require_sudo {
            executor.with_sudo()
        } else {
            executor
        };
        Ok(executor)
    }
}

fn map_exec_error(error: ExecError) -> ApiError {
    tracing::error!(error=%error,"remote server command failed");
    match error {
        ExecError::CommandFailed { .. } => (StatusCode::BAD_GATEWAY, error.to_string()),
        ExecError::Ssh(_) => (StatusCode::BAD_GATEWAY, error.to_string()),
        ExecError::StreamCancelled => (StatusCode::REQUEST_TIMEOUT, error.to_string()),
        ExecError::Timeout { .. } => (StatusCode::GATEWAY_TIMEOUT, error.to_string()),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
    }
}
fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "server or SSH key not found".into()),
        other => {
            tracing::error!(error=%other,"server database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
