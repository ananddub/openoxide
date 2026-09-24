use std::sync::Arc;

use auto_di::resolve;
use connectrpc::{
    ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult,
};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, ServerControllerAuditRequest, ServerControllerAuditResponse,
    ServerControllerDeleteSessionRequest, ServerControllerListSessionsRequest,
    ServerControllerListSessionsResponse, ServerControllerService, ServerControllerServiceExt,
    ServerControllerSetupRequest, ServerControllerSetupResponse,
    ServerControllerStreamSetupLogsRequest, ServerControllerStreamSetupLogsResponse,
    ServerControllerTestConnectionRequest, ServerControllerTestConnectionResponse,
    ServerControllerTestDirectConnectionRequest, ServerControllerTestDirectConnectionResponse,
};
use crate::core::config::Config;
use crate::services::infrastructure::server::RemoteServerStatus;
use crate::utils::exec::{ExecError, RemoteExecutor, SshAuth, SshHostKey};
use crate::utils::setup::{ServerSetup, SetupConfig, SetupOutcome};
use futures::StreamExt;

use super::helper::*;
use super::init::ServerController;

#[auto_grpc::service(connect = true, auth = true)]
impl ServerControllerService for ServerController {
    #[permission(Server, CanRead)]
    async fn test_direct_connection<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerControllerTestDirectConnectionRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ServerControllerTestDirectConnectionResponse> + Send + use<'a>,
    > {
        let port = u16::try_from(request.port.unwrap_or(22))
            .map_err(|_| ConnectError::invalid_argument("SSH port must be between 0 and 65535"))?;
        let key_id = request.ssh_key_id;
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
        let auth = SshAuth::key_pair(key.private_key, key.public_key);

        let executor = RemoteExecutor::new(
            request.ip_address.to_owned(),
            port,
            request.username.to_owned(),
            auth,
            SshHostKey::InsecureAcceptAny,
        )
        .with_multiplexing(false);
        executor
            .run("true", std::iter::empty::<&str>())
            .await
            .map_err(map_exec_error)?;

        Response::ok(ServerControllerTestDirectConnectionResponse {
            connection: ::buffa::MessageField::some(placeholder_connection_info()),
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn test_connection<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerControllerTestConnectionRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ServerControllerTestConnectionResponse> + Send + use<'a>,
    > {
        let id = request.id;
        let host_key = request.host_key_fingerprint.map(str::to_owned);

        let executor = self
            .executor(
                id,
                &host_key,
                request.sudo_password,
                request.pool_size.map(|size| size as usize),
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

        Response::ok(ServerControllerTestConnectionResponse {
            connection: ::buffa::MessageField::some(placeholder_connection_info()),
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn audit<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerControllerAuditRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ServerControllerAuditResponse> + Send + use<'a>>
    {
        let id = request.id;
        let host_key = request.host_key_fingerprint.map(str::to_owned);

        let executor = self
            .executor(
                id,
                &host_key,
                request.sudo_password,
                request.pool_size.map(|size| size as usize),
                false,
            )
            .await?;
        self.capture_and_store_host_key(id, &executor).await;
        let audit = ServerSetup::new_remote(executor, SetupConfig::default())
            .audit()
            .await
            .map_err(map_exec_error)?;
        self.service
            .touch_test_connection(id)
            .await
            .map_err(map_db_error)?;
        trigger_monitoring_refresh(id);

        Response::ok(ServerControllerAuditResponse {
            audit: ::buffa::MessageField::some(audit_to_proto(audit)),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn setup<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerControllerSetupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ServerControllerSetupResponse> + Send + use<'a>>
    {
        let id = request.id;
        let host_key = request.host_key_fingerprint.map(str::to_owned);

        let executor = self
            .executor(
                id,
                &host_key,
                request.sudo_password,
                request.pool_size.map(|size| size as usize),
                true,
            )
            .await?;
        self.capture_and_store_host_key(id, &executor).await;
        let mut config = SetupConfig::default();
        config.build_server = self
            .service
            .get_by_id(id)
            .await
            .map_err(map_db_error)?
            .server_type
            .eq_ignore_ascii_case("BUILD");
        config.agent_server_id = Some(id);
        let panel_config = resolve::<Config>()
            .await
            .map_err(|_| ConnectError::new(ErrorCode::Internal, "configuration unavailable"))?;
        config.agent_panel_url = Some(
            std::env::var("OPENOXIDE_SERVER_URL")
                .or_else(|_| std::env::var("RUSTPLOY_SERVER_URL"))
                .unwrap_or_else(|_| format!("http://127.0.0.1:{}", panel_config.port)),
        );
        config.agent_token = Some(panel_config.metrics_token.clone()).filter(|v| !v.is_empty());
        config.advertise_addr = self
            .service
            .setup_advertise_addr(id, request.advertise_addr.map(str::to_owned))
            .await
            .map_err(map_db_error)?;
        if let Some(email) = request.acme_email {
            config.acme_email = email.to_owned();
        }
        let outcome = ServerSetup::new_remote(executor, config)
            .setup_all_oneshot(request.install_dependencies)
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

        Response::ok(ServerControllerSetupResponse {
            outcome: ::buffa::MessageField::some(outcome_to_proto(outcome)),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn stream_setup_logs(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerControllerStreamSetupLogsRequest>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ServerControllerStreamSetupLogsResponse> + Send + use<>,
        >,
    > {
        // Live streaming: forward each setup log line as its own event, then
        // emit one final event carrying `outcome` (or end the stream with the
        // setup error).
        let id = request.id;
        let host_key = request.host_key_fingerprint.map(str::to_owned);

        let executor = self
            .executor(
                id,
                &host_key,
                request.sudo_password,
                request.pool_size.map(|size| size as usize),
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
        let panel_config = resolve::<Config>()
            .await
            .map_err(|_| ConnectError::new(ErrorCode::Internal, "configuration unavailable"))?;
        config.agent_panel_url = Some(
            std::env::var("OPENOXIDE_SERVER_URL")
                .or_else(|_| std::env::var("RUSTPLOY_SERVER_URL"))
                .unwrap_or_else(|_| format!("http://127.0.0.1:{}", panel_config.port)),
        );
        config.agent_token = Some(panel_config.metrics_token.clone()).filter(|v| !v.is_empty());
        config.advertise_addr = self
            .service
            .setup_advertise_addr(id, request.advertise_addr.map(str::to_owned))
            .await
            .map_err(map_db_error)?;
        if let Some(email) = request.acme_email {
            config.acme_email = email.to_owned();
        }

        let install_dependencies = request.install_dependencies;
        let service = Arc::clone(&self.service);

        enum SetupEvent {
            Line(String),
            Done(Result<SetupOutcome, ExecError>),
        }

        let (line_tx, mut line_rx) = tokio::sync::mpsc::channel::<String>(256);
        let (event_tx, event_rx) = tokio::sync::mpsc::channel::<SetupEvent>(64);

        let line_event_tx = event_tx.clone();
        tokio::spawn(async move {
            while let Some(line) = line_rx.recv().await {
                if line_event_tx.send(SetupEvent::Line(line)).await.is_err() {
                    break;
                }
            }
        });

        tokio::spawn(async move {
            let result = ServerSetup::new_remote(executor, config)
                .setup_all_oneshot_stream(install_dependencies, line_tx)
                .await;
            match &result {
                Ok(_) => {
                    if let Err(error) = service.set_status(id, RemoteServerStatus::Active).await {
                        tracing::warn!(server_id = id, %error, "failed to mark server active after streamed setup");
                    }
                    if let Err(error) = service.touch_test_connection(id).await {
                        tracing::warn!(server_id = id, %error, "failed to update connection timestamp after streamed setup");
                    }
                }
                Err(error) => {
                    tracing::error!(error = %error, server_id = id, "streamed server setup failed");
                }
            }
            let _ = event_tx.send(SetupEvent::Done(result)).await;
        });

        let stream =
            tokio_stream::wrappers::ReceiverStream::new(event_rx).map(move |event| match event {
                SetupEvent::Line(line) => Ok(ServerControllerStreamSetupLogsResponse {
                    server_id: id,
                    lines: vec![line],
                    ..Default::default()
                }),
                SetupEvent::Done(Ok(outcome)) => Ok(ServerControllerStreamSetupLogsResponse {
                    server_id: id,
                    outcome: ::buffa::MessageField::some(outcome_to_proto(outcome)),
                    ..Default::default()
                }),
                SetupEvent::Done(Err(error)) => Err(map_exec_error(error)),
            });
        Response::stream_ok(stream)
    }

    #[permission(Server, CanRead)]
    async fn list_sessions<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ServerControllerListSessionsRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ServerControllerListSessionsResponse> + Send + use<'a>,
    > {
        // The original handler was a live-query placeholder that always
        // returned a connected response. Preserve that behaviour.
        Response::ok(ServerControllerListSessionsResponse {
            sessions: vec![placeholder_connection_info()],
            ..Default::default()
        })
    }

    #[permission(Server, CanDelete)]
    async fn delete_session<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, ServerControllerDeleteSessionRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        // Original handler returned NO_CONTENT — in ConnectRPC we return Empty.
        Response::ok(Empty::default())
    }
}
