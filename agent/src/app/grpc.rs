use std::net::SocketAddr;

use auto_di::Container;
use tokio_util::sync::CancellationToken;
use tonic::service::RoutesBuilder;
use tonic::transport::{Identity, ServerTlsConfig};
use tracing::{error, info};

use crate::context::MonitorContext;

pub struct GrpcServerTask;

impl GrpcServerTask {
    pub async fn run(
        ctx: MonitorContext,
        container: Container,
        shutdown: CancellationToken,
    ) {
        let addr: SocketAddr = match format!("0.0.0.0:{}", ctx.config.grpc_port).parse() {
            Ok(addr) => addr,
            Err(error) => {
                error!(%error, port = ctx.config.grpc_port, "invalid gRPC bind address");
                return;
            }
        };
        info!(%addr, "gRPC query server listening");

        // Register against the process's existing container so the gRPC
        // handlers share the same singleton Store the collectors write to.
        // A fresh Container here built a second, empty store, and every
        // metrics query returned [] forever.
        let mut routes = RoutesBuilder::default();
        if let Err(error) = auto_grpc::register(&container, &mut routes).await {
            error!(%error, "could not register gRPC services");
            return;
        }
        let mut server = tonic::transport::Server::builder();
        if let (Some(cert_path), Some(key_path)) = (&ctx.config.tls_cert, &ctx.config.tls_key) {
            let cert = match tokio::fs::read(cert_path).await {
                Ok(value) => value,
                Err(error) => {
                    error!(%error, path = %cert_path, "could not read gRPC TLS certificate");
                    return;
                }
            };
            let key = match tokio::fs::read(key_path).await {
                Ok(value) => value,
                Err(error) => {
                    error!(%error, path = %key_path, "could not read gRPC TLS private key");
                    return;
                }
            };
            server = match server
                .tls_config(ServerTlsConfig::new().identity(Identity::from_pem(cert, key)))
            {
                Ok(value) => value,
                Err(error) => {
                    error!(%error, "could not configure gRPC TLS");
                    return;
                }
            };
            info!("gRPC TLS enabled");
        } else {
            info!(
                "gRPC TLS disabled; configure AGENT_TLS_CERT and AGENT_TLS_KEY for encrypted transport"
            );
        }
        if let Err(error) = server
            .add_routes(routes.routes())
            .serve_with_shutdown(addr, shutdown.cancelled_owned())
            .await
        {
            error!(%error, "gRPC server stopped unexpectedly");
        }
    }
}
