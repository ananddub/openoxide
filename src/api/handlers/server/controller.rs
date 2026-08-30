use std::sync::Arc;

use auto_di::{resolve, singleton};
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, OperationResult, ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
    ServerControllerService, ServerControllerServiceExt, StreamQuery,
};
use crate::core::config::Config;
use crate::db::queries::FlashQueries;
use crate::services::server::{RemoteServerService, RemoteServerStatus};
use crate::utils::{
    exec::{ExecError, RemoteExecutor, SshAuth, SshHostKey},
    setup::{
        PortAvailability, ServerAudit, ServerSetup, SetupConfig, SetupOutcome, SetupStep,
        ToolState,
    },
};

pub struct ServerController {
    service: Arc<RemoteServerService>,
    queries: Arc<FlashQueries>,
}

#[singleton]
impl ServerController {
    pub fn new(service: Arc<RemoteServerService>, queries: Arc<FlashQueries>) -> Self {
        Self { service, queries }
    }
}

fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("server or SSH key not found"),
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => {
            tracing::warn!(error = %e.message(), "foreign key violation");
            ConnectError::not_found("ssh key not found")
        }
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            tracing::warn!(error = %e.message(), "unique constraint violation");
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        other => {
            tracing::error!(error = %other, "server database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

fn map_exec_error(error: ExecError) -> ConnectError {
    tracing::error!(error=%error, "remote server command failed");
    match error {
        ExecError::CommandFailed { .. } => ConnectError::new(ErrorCode::Unavailable, error.to_string()),
        ExecError::Ssh(_) => ConnectError::new(ErrorCode::Unavailable, error.to_string()),
        ExecError::StreamCancelled => ConnectError::new(ErrorCode::DeadlineExceeded, error.to_string()),
        ExecError::Timeout { .. } => ConnectError::new(ErrorCode::DeadlineExceeded, error.to_string()),
        _ => ConnectError::new(ErrorCode::Internal, error.to_string()),
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl ServerControllerService for ServerController {
    #[permission(Server, CanRead)]
    async fn test_direct_connection<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let body: TestDirectConnectionDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;

        let port = body.port.unwrap_or(22);
        let auth = if let Some(key_id) = body.ssh_key_id {
            let key = match self.queries.ssh_keys_get_by_id(key_id).await {
                Ok(key) => key,
                Err(sqlx::Error::RowNotFound) => {
                    return Err(ConnectError::not_found("SSH Key not found"));
                }
                Err(error) => return Err(map_db_error(error)),
            };
            if key.private_key.trim().is_empty() {
                return Err(ConnectError::invalid_argument(
                    "Selected SSH Key has no private key",
                ));
            }
            SshAuth::key_pair(key.private_key, key.public_key)
        } else {
            return Err(ConnectError::invalid_argument(
                "SSH Key ID is required to test connection",
            ));
        };

        let executor = RemoteExecutor::new(
            body.ip_address,
            port,
            body.username,
            auth,
            SshHostKey::InsecureAcceptAny,
        )
        .with_multiplexing(false);
        executor
            .run("true", std::iter::empty::<&str>())
            .await
            .map_err(map_exec_error)?;

        let payload = serde_json::to_string(&ServerConnectionResponseDto {
            connected: true,
            reused_sessions: 0,
            max_pool_size: 0,
            connections: 0,
            active_channels: 0,
            max_channels_per_session: 0,
        })
        .unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "test_direct_connection".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn test_connection<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let body: ServerConnectionDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let id = request.id;

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
            .map_err(map_db_error)?;
        trigger_monitoring_refresh(id);

        let payload = serde_json::to_string(&ServerConnectionResponseDto {
            connected: true,
            reused_sessions: 0,
            max_pool_size: 0,
            connections: 0,
            active_channels: 0,
            max_channels_per_session: 0,
        })
        .unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "test_connection".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn audit<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let body: ServerConnectionDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let id = request.id;

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
            .map_err(map_db_error)?;
        trigger_monitoring_refresh(id);

        let payload =
            serde_json::to_string(&ServerAuditDto::from(audit)).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "audit".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn setup<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let body: SetupServerDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let id = request.id;

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
        config.build_server = self
            .service
            .get_by_id(id)
            .await
            .map_err(map_db_error)?
            .server_type
            .eq_ignore_ascii_case("BUILD");
        config.agent_server_id = Some(id);
        let panel_config = resolve::<Config>().await.map_err(|_| {
            ConnectError::new(ErrorCode::Internal, "configuration unavailable")
        })?;
        config.agent_panel_url = Some(
            std::env::var("OPENOXIDE_SERVER_URL")
                .or_else(|_| std::env::var("RUSTPLOY_SERVER_URL"))
                .unwrap_or_else(|_| format!("http://127.0.0.1:{}", panel_config.port)),
        );
        config.agent_token = Some(panel_config.metrics_token.clone()).filter(|v| !v.is_empty());
        config.advertise_addr = self
            .service
            .setup_advertise_addr(id, body.advertise_addr)
            .await
            .map_err(map_db_error)?;
        if let Some(email) = body.acme_email {
            config.acme_email = email;
        }
        let outcome = ServerSetup::new_remote(executor, config)
            .setup_all_oneshot(body.install_dependencies)
            .await
            .map_err(map_exec_error)?;
        self.service
            .set_status(id, RemoteServerStatus::Active)
            .await
            .map_err(map_db_error)?;
        self.service
            .touch_test_connection(id)
            .await
            .map_err(map_db_error)?;

        let payload =
            serde_json::to_string(&SetupOutcomeDto::from(outcome)).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "setup".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn stream_setup_logs(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, StreamQuery>,
    ) -> ServiceResult<connectrpc::ServiceStream<impl connectrpc::Encodable<ResourceResponse> + Send + use<>>> {
        // Run the setup to completion and emit a single completion event,
        // then close the stream. True incremental log streaming can be
        // layered on later without changing the proto contract.
        let id = request.id;

        let body: SetupServerDto = serde_json::from_str(request.filter.as_deref().unwrap_or_default())
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;

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
        config.build_server = self
            .service
            .get_by_id(id)
            .await
            .map_err(map_db_error)?
            .server_type
            .eq_ignore_ascii_case("BUILD");
        config.agent_server_id = Some(id);
        let panel_config = resolve::<Config>().await.map_err(|_| {
            ConnectError::new(ErrorCode::Internal, "configuration unavailable")
        })?;
        config.agent_panel_url = Some(
            std::env::var("OPENOXIDE_SERVER_URL")
                .or_else(|_| std::env::var("RUSTPLOY_SERVER_URL"))
                .unwrap_or_else(|_| format!("http://127.0.0.1:{}", panel_config.port)),
        );
        config.agent_token = Some(panel_config.metrics_token.clone()).filter(|v| !v.is_empty());
        config.advertise_addr = self
            .service
            .setup_advertise_addr(id, body.advertise_addr)
            .await
            .map_err(map_db_error)?;
        if let Some(email) = body.acme_email {
            config.acme_email = email;
        }

        let install_dependencies = body.install_dependencies;
        let service = Arc::clone(&self.service);
        let result = ServerSetup::new_remote(executor, config)
            .setup_all_oneshot(install_dependencies)
            .await;

        match result {
            Ok(outcome) => {
                if let Err(error) = service.set_status(id, RemoteServerStatus::Active).await {
                    tracing::warn!(server_id = id, %error, "failed to mark server active after streamed setup");
                }
                if let Err(error) = service.touch_test_connection(id).await {
                    tracing::warn!(server_id = id, %error, "failed to update connection timestamp after streamed setup");
                }
                let payload =
                    serde_json::to_string(&SetupOutcomeDto::from(outcome)).unwrap_or_default();
                let item = ResourceResponse {
                    id,
                    kind: "setup_logs_complete".into(),
                    payload,
                    ..Default::default()
                };
                Response::stream_ok(futures::stream::once(async move { Ok(item) }))
            }
            Err(error) => {
                tracing::error!(error = %error, server_id = id, "streamed server setup failed");
                Err(map_exec_error(error))
            }
        }
    }

    #[permission(Server, CanRead)]
    async fn list_sessions<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        // The original handler was a live-query placeholder that always
        // returned a connected response. Preserve that behaviour.
        let payload = serde_json::to_string(&ServerConnectionResponseDto {
            connected: true,
            reused_sessions: 0,
            max_pool_size: 0,
            connections: 0,
            active_channels: 0,
            max_channels_per_session: 0,
        })
        .unwrap_or_default();
        let item = ResourceResponse {
            id: 0,
            kind: "server_sessions".into(),
            payload,
            ..Default::default()
        };
        Response::ok(ResourceListResponse {
            items: vec![item],
            ..Default::default()
        })
    }

    #[permission(Server, CanDelete)]
    async fn delete_session<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        // Original handler returned NO_CONTENT — in ConnectRPC we return Empty.
        Response::ok(Empty::default())
    }
}

impl ServerController {
    async fn executor(
        &self,
        id: i64,
        host_key: &Option<String>,
        sudo_password: Option<&str>,
        _pool_size: Option<usize>,
        require_sudo: bool,
    ) -> Result<RemoteExecutor, ConnectError> {
        let (server, key) = self
            .service
            .connection_details(id)
            .await
            .map_err(map_db_error)?;
        if key.private_key.trim().is_empty() {
            return Err(ConnectError::invalid_argument(
                "selected SSH key has no private key",
            ));
        }
        let port = u16::try_from(server.port).map_err(|_| {
            ConnectError::invalid_argument("SSH port must be between 0 and 65535")
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

/// Kick the one-shot lifecycle reconciliation after an explicit connection
/// check. The HTTP response is not held open while the agent is contacted.
fn trigger_monitoring_refresh(server_id: i64) {
    tokio::spawn(async move {
        let monitoring =
            match resolve::<crate::services::monitoring::monitoring_service::MonitoringService>()
                .await
            {
                Ok(service) => service,
                Err(error) => {
                    tracing::debug!(server_id, %error, "monitoring refresh service unavailable");
                    return;
                }
            };
        let reconciler = match resolve::<
            crate::services::monitoring::reconciler::MonitoringReconciler,
        >()
        .await
        {
            Ok(reconciler) => reconciler,
            Err(error) => {
                tracing::debug!(server_id, %error, "monitoring reconciler unavailable");
                return;
            }
        };
        match monitoring.container_states(server_id).await {
            Ok(states) => {
                if let Err(error) = reconciler.apply_snapshot(server_id, &states).await {
                    tracing::warn!(server_id, %error, "explicit monitoring refresh reconciliation failed");
                }
            }
            Err(error) => {
                tracing::debug!(server_id, %error, "agent unavailable during explicit monitoring refresh")
            }
        }
    });
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

// ── Helper DTOs for the generic Resource endpoints (moved from the retired
//    api::dto layer; JSON payload contract must not change) ─────────────

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize)]
struct ServerConnectionDto {
    host_key_fingerprint: Option<String>,
    sudo_password: Option<String>,
    pool_size: Option<usize>,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct TestDirectConnectionDto {
    ip_address: String,
    port: Option<u16>,
    username: String,
    ssh_key_id: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct SetupServerDto {
    #[serde(default)]
    install_dependencies: bool,
    advertise_addr: Option<String>,
    acme_email: Option<String>,
    host_key_fingerprint: Option<String>,
    sudo_password: Option<String>,
    pool_size: Option<usize>,
}

#[derive(Clone, Debug, Serialize)]
struct ToolStateDto {
    installed: bool,
    version: Option<String>,
}
impl From<ToolState> for ToolStateDto {
    fn from(v: ToolState) -> Self {
        Self {
            installed: v.installed,
            version: v.version,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct PortAvailabilityDto {
    port: u16,
    available: bool,
}
impl From<PortAvailability> for PortAvailabilityDto {
    fn from(v: PortAvailability) -> Self {
        Self {
            port: v.port,
            available: v.available,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct ServerAuditDto {
    os_id: String,
    architecture: String,
    docker: ToolStateDto,
    git: ToolStateDto,
    rclone: ToolStateDto,
    nixpacks: ToolStateDto,
    railpack: ToolStateDto,
    buildpacks: ToolStateDto,
    swarm_active: bool,
    network_exists: bool,
    base_directory_exists: bool,
    docker_group_member: bool,
    ports: Vec<PortAvailabilityDto>,
}
impl From<ServerAudit> for ServerAuditDto {
    fn from(v: ServerAudit) -> Self {
        Self {
            os_id: v.os_id,
            architecture: v.architecture,
            docker: v.docker.into(),
            git: v.git.into(),
            rclone: v.rclone.into(),
            nixpacks: v.nixpacks.into(),
            railpack: v.railpack.into(),
            buildpacks: v.buildpacks.into(),
            swarm_active: v.swarm_active,
            network_exists: v.network_exists,
            base_directory_exists: v.base_directory_exists,
            docker_group_member: v.docker_group_member,
            ports: v.ports.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct SetupOutcomeDto {
    completed: Vec<String>,
    audit: ServerAuditDto,
}
impl From<SetupOutcome> for SetupOutcomeDto {
    fn from(v: SetupOutcome) -> Self {
        Self {
            completed: v
                .completed
                .into_iter()
                .map(|s| {
                    match s {
                        SetupStep::Dependencies => "dependencies",
                        SetupStep::BuildTools => "build-tools",
                        SetupStep::Directories => "directories",
                        SetupStep::Swarm => "swarm",
                        SetupStep::Network => "network",
                        SetupStep::TraefikConfig => "traefik-config",
                        SetupStep::Traefik => "traefik",
                        SetupStep::Agent => "agent",
                    }
                    .into()
                })
                .collect(),
            audit: v.audit.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct ServerConnectionResponseDto {
    connected: bool,
    reused_sessions: usize,
    max_pool_size: usize,
    connections: usize,
    active_channels: usize,
    max_channels_per_session: usize,
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
