use agent::{app::grpc::GrpcServerTask, core};
use auto_di::Container;

use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

#[tokio::main(worker_threads = 1)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("agent=info")),
        )
        .init();

    let container = Container::new()?;
    let config = container.resolve::<core::config::Config>().await?;

    info!(
        server_id = config.server_id,
        grpc_port = config.grpc_port,
        "starting openoxide agent"
    );

    let context = container.resolve::<core::context::MonitorContext>().await?;

    if let Err(error) = context.docker.ping().await {
        warn!(%error, socket = %context.config.docker_socket, "docker daemon unreachable");
    }

    // One gRPC server is the agent's only listener: every metrics query it
    // answers is sampled live (sysinfo + Docker API), nothing is stored.
    let shutdown = CancellationToken::new();
    let grpc = {
        let container = container.clone();
        let shutdown = shutdown.clone();
        let ctx = (*context).clone();
        tokio::spawn(async move {
            GrpcServerTask::run(ctx, container, shutdown).await;
        })
    };

    wait_for_shutdown_signal().await;
    info!("shutdown signal received, stopping tasks");

    shutdown.cancel();
    if let Err(error) = grpc.await {
        error!(%error, "gRPC server task failed to join");
    }

    Ok(())
}

async fn wait_for_shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};

        let mut sigterm = match signal(SignalKind::terminate()) {
            Ok(sig) => sig,
            Err(error) => {
                error!(%error, "could not listen for SIGTERM, falling back to ctrl-c only");
                let _ = tokio::signal::ctrl_c().await;
                return;
            }
        };

        tokio::select! {
            _ = tokio::signal::ctrl_c() => {}
            _ = sigterm.recv() => {}
        }
    }

    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}
