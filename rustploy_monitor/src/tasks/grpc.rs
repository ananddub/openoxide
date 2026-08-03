use std::net::SocketAddr;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::context::MonitorContext;
use crate::grpc::{MonitoringGrpc, MonitoringServiceServer};

pub async fn serve_grpc(ctx: MonitorContext, shutdown: CancellationToken) {
    let addr: SocketAddr = match format!("0.0.0.0:{}", ctx.config.grpc_port).parse() {
        Ok(addr) => addr,
        Err(error) => {
            error!(%error, port = ctx.config.grpc_port, "invalid gRPC bind address");
            return;
        }
    };

    info!(%addr, "gRPC query server listening");

    let service = MonitoringGrpc::new(ctx.store.clone(), ctx.config.server_id, ctx.docker.clone());
    let result = tonic::transport::Server::builder()
        .add_service(MonitoringServiceServer::new(service))
        .serve_with_shutdown(addr, shutdown.cancelled_owned())
        .await;

    if let Err(error) = result {
        error!(%error, "gRPC server stopped unexpectedly");
    }
}
