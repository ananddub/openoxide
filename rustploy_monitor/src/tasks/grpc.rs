use std::net::SocketAddr;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::context::MonitorContext;
use crate::grpc::{MonitoringGrpc, MonitoringServiceServer};

fn authenticate(
    token: &str,
    request: tonic::Request<()>,
) -> Result<tonic::Request<()>, tonic::Status> {
    let supplied = request
        .metadata()
        .get("x-metrics-token")
        .and_then(|value| value.to_str().ok());
    if supplied == Some(token) {
        Ok(request)
    } else {
        Err(tonic::Status::unauthenticated("invalid monitoring token"))
    }
}

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
    let token = ctx.config.metrics_token.clone();
    let result = tonic::transport::Server::builder()
        .add_service(MonitoringServiceServer::with_interceptor(service, move |request| {
            authenticate(&token, request)
        }))
        .serve_with_shutdown(addr, shutdown.cancelled_owned())
        .await;

    if let Err(error) = result {
        error!(%error, "gRPC server stopped unexpectedly");
    }
}
