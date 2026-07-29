use crate::{
    api::dto::deployment::DeploymentSseEventDto,
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
use axum::{
    Json,
    extract::Path,
    http::StatusCode,
    response::sse::{Event, Sse},
};
use futures::Stream;
use serde_json::json;
use std::{convert::Infallible, pin::Pin, sync::Arc};
use tokio::sync::mpsc;
use tokio::time::{Duration, MissedTickBehavior};

type ApiError = (StatusCode, String);
type ServerSetupStream = Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>>;
type ServerSetupSse = Sse<ServerSetupStream>;

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

    #[post("/{id}/setup/logs", sse = DeploymentSseEventDto)]
    async fn setup_logs(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Path(id): Path<i64>,
        Json(body): Json<SetupServerDto>,
    ) -> Result<ServerSetupSse, ApiError> {
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

        let install_dependencies = body.install_dependencies;
        let service = Arc::clone(&self.service);
        let (sender, receiver) = mpsc::channel::<String>(128);

        tokio::spawn(async move {
            let _ = sender.send("Starting server setup...".into()).await;
            let result = ServerSetup::new_remote(executor, config)
                .setup_all_oneshot_stream(install_dependencies, sender.clone())
                .await;

            match result {
                Ok(_) => {
                    if let Err(error) = service.set_status(id, "ACTIVE").await {
                        let _ = sender
                            .send(format!("Failed to mark server active: {error}"))
                            .await;
                    }
                    if let Err(error) = service.touch_test_connection(id).await {
                        let _ = sender
                            .send(format!("Failed to update connection timestamp: {error}"))
                            .await;
                    }
                    let _ = sender.send("Setup Server: ✅".into()).await;
                }
                Err(error) => {
                    tracing::error!(error = %error, server_id = id, "streamed server setup failed");
                    let _ = sender
                        .send(format!("[STDERR] Setup Server failed: {error} ❌"))
                        .await;
                }
            }
        });

        Ok(Sse::new(server_setup_log_stream(receiver)))
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
        let host_policy = host_key_policy(id, host_key);
        let auth = SshAuth::key_pair(key.private_key, key.public_key);
        let is_root = server.username == "root";
        let mut executor =
            RemoteExecutor::new(server.ip_address, port, server.username, auth, host_policy);
        executor = if is_root {
            executor
        } else if let Some(password) = sudo_password {
            executor.with_sudo_password(password)
        } else if require_sudo {
            executor.with_sudo()
        } else {
            executor
        };
        Ok(executor)
    }
}

fn host_key_policy(server_id: i64, host_key: &Option<String>) -> SshHostKey {
    match host_key
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        Some(fingerprint) => SshHostKey::PinnedSha256(fingerprint.to_owned()),
        None => {
            tracing::warn!(
                server_id,
                "SSH host key verification disabled; provide host_key_fingerprint"
            );
            SshHostKey::InsecureAcceptAny
        }
    }
}

fn server_setup_log_stream(receiver: mpsc::Receiver<String>) -> ServerSetupStream {
    let mut keep_alive = tokio::time::interval(Duration::from_secs(15));
    keep_alive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    Box::pin(futures::stream::unfold(
        (receiver, keep_alive),
        |(mut receiver, mut keep_alive)| async move {
            loop {
                tokio::select! {
                    _ = keep_alive.tick() => {
                        let event = Event::default().event("keep-alive").data(json_payload(json!({
                            "type": "keep-alive",
                        })));
                        return Some((Ok(event), (receiver, keep_alive)));
                    }
                    received = receiver.recv() => {
                        match received {
                            Some(line) => {
                                let event = Event::default().event("log").data(json_payload(json!({
                                    "type": "log",
                                    "line": line,
                                })));
                                return Some((Ok(event), (receiver, keep_alive)));
                            }
                            None => return None,
                        }
                    }
                }
            }
        },
    ))
}

fn json_payload(value: serde_json::Value) -> String {
    serde_json::to_string(&value).unwrap_or_else(|_| "{}".into())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_host_key_fingerprint_disables_pinning() {
        assert!(matches!(
            host_key_policy(1, &None),
            SshHostKey::InsecureAcceptAny
        ));
        assert!(matches!(
            host_key_policy(1, &Some(String::new())),
            SshHostKey::InsecureAcceptAny
        ));
        assert!(matches!(
            host_key_policy(1, &Some("   ".to_owned())),
            SshHostKey::InsecureAcceptAny
        ));
    }

    #[test]
    fn non_empty_host_key_fingerprint_is_trimmed_and_pinned() {
        match host_key_policy(1, &Some(" SHA256:abc ".to_owned())) {
            SshHostKey::PinnedSha256(value) => assert_eq!(value, "SHA256:abc"),
            SshHostKey::InsecureAcceptAny => panic!("expected pinned host key"),
        }
    }
}
