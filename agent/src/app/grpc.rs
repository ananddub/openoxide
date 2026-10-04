use std::net::SocketAddr;

use auto_di::Container;
use tokio_util::sync::CancellationToken;
use tonic::service::RoutesBuilder;
use tonic::transport::{Identity, ServerTlsConfig};
use tracing::{error, info};

use crate::context::MonitorContext;

pub struct GrpcServerTask;

impl GrpcServerTask {
    pub async fn run(ctx: MonitorContext, container: Container, shutdown: CancellationToken) {
        let addr: SocketAddr = match format!("0.0.0.0:{}", ctx.config.grpc_port).parse() {
            Ok(addr) => addr,
            Err(error) => {
                error!(%error, port = ctx.config.grpc_port, "invalid gRPC bind address");
                return;
            }
        };
        info!(%addr, "gRPC query server listening");

        // Register against the process's existing container so the handlers
        // share the same singleton MonitoringService (live Docker API and
        // host collector). A fresh Container here would build a second set
        // of singletons and duplicate the gRPC listeners.
        let mut routes = RoutesBuilder::default();
        if let Err(error) = auto_grpc::register(&container, &mut routes).await {
            error!(%error, "could not register gRPC services");
            return;
        }
        let mut server = tonic::transport::Server::builder();
        if let (Some(cert_path), Some(key_path)) = (&ctx.config.tls_cert, &ctx.config.tls_key) {
            // The panel writes these into the shared volume at startup; the
            // agent may boot first, so wait for them rather than fail fast.
            let cert = match read_with_wait(cert_path).await {
                Ok(value) => value,
                Err(error) => {
                    error!(%error, path = %cert_path, "gRPC TLS certificate never appeared");
                    return;
                }
            };
            let key = match read_with_wait(key_path).await {
                Ok(value) => value,
                Err(error) => {
                    error!(%error, path = %key_path, "gRPC TLS private key never appeared");
                    return;
                }
            };
            server = match server.tls_config({
                let mut tls = ServerTlsConfig::new().identity(Identity::from_pem(cert, key));
                // mTLS: require and verify the panel's client certificate
                // against the configured CA.
                if let Some(ca_path) = &ctx.config.tls_client_ca {
                    match read_with_wait(ca_path).await {
                        Ok(ca) => {
                            tls = tls.client_ca_root(tonic::transport::Certificate::from_pem(ca));
                        }
                        Err(error) => {
                            error!(%error, path = %ca_path, "gRPC client CA never appeared");
                            return;
                        }
                    }
                }
                tls
            }) {
                Ok(value) => value,
                Err(error) => {
                    error!(%error, "could not configure gRPC TLS");
                    return;
                }
            };
            info!(
                mtls = ctx.config.tls_client_ca.is_some(),
                "gRPC TLS enabled"
            );
        } else {
            info!(
                "gRPC TLS disabled; configure AGENT_TLS_CERT and AGENT_TLS_KEY for encrypted transport"
            );
        }
        // Match the panel's client-side h2 tuning: 16 KiB per inbound stream
        // window, 64 KiB connection window. Inbound traffic here is uploads,
        // shell keystrokes and small unary requests — none of it needs the
        // default ~64 KiB-per-stream buffering, and every open stream would
        // otherwise charge that to this process's RSS.
        let mut server = server
            .initial_stream_window_size(16 * 1024)
            .initial_connection_window_size(64 * 1024);
        if let Err(error) = server
            .add_routes(routes.routes())
            .serve_with_shutdown(addr, shutdown.cancelled_owned())
            .await
        {
            error!(%error, "gRPC server stopped unexpectedly");
        }
    }
}

/// Reads a cert file, waiting for it to appear (the panel writes the shared
/// mTLS material at its own startup, which may lag the agent's boot).
async fn read_with_wait(path: &str) -> std::io::Result<Vec<u8>> {
    const ATTEMPTS: u32 = 60; // ~60s at 1s intervals
    for attempt in 0..ATTEMPTS {
        match tokio::fs::read(path).await {
            Ok(bytes) if !bytes.is_empty() => return Ok(bytes),
            _ => {
                if attempt == 0 {
                    info!(path, "waiting for mTLS material to be provisioned");
                }
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    }
    tokio::fs::read(path).await
}
