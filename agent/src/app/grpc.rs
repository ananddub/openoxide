use std::net::SocketAddr;

use auto_di::Container;
use tokio_util::sync::CancellationToken;
use tonic::service::RoutesBuilder;
use tracing::{error, info};

use crate::context::MonitorContext;

pub struct GrpcServerTask;

impl GrpcServerTask {
    pub async fn run(ctx: MonitorContext, shutdown: CancellationToken) {
        let addr: SocketAddr = match format!("0.0.0.0:{}", ctx.config.grpc_port).parse() {
            Ok(addr) => addr,
            Err(error) => {
                error!(%error, port = ctx.config.grpc_port, "invalid gRPC bind address");
                return;
            }
        };
        info!(%addr, "gRPC query server listening");

        let container = match Container::new() {
            Ok(container) => container,
            Err(error) => {
                error!(%error, "could not create dependency container");
                return;
            }
        };
        let mut routes = RoutesBuilder::default();
        if let Err(error) = auto_grpc::register(&container, &mut routes).await {
            error!(%error, "could not register gRPC services");
            return;
        }
        if let Err(error) = tonic::transport::Server::builder()
            .add_routes(routes.routes())
            .serve_with_shutdown(addr, shutdown.cancelled_owned())
            .await
        {
            error!(%error, "gRPC server stopped unexpectedly");
        }
    }
}
