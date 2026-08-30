use std::sync::Arc;

use auto_di::resolve;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, OperationResult, ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
    ServerControllerService, ServerControllerServiceExt, StreamQuery,
};
use crate::core::config::Config;
use crate::services::server::RemoteServerStatus;
use crate::utils::{
    exec::{RemoteExecutor, SshAuth, SshHostKey},
    setup::{ServerSetup, SetupConfig},
};

use super::helper::*;
use super::init::ServerController;

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

