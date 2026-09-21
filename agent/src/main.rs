use agent::utils::background::tasks;
use agent::{app::grpc::GrpcServerTask, core};
use auto_di::Container;
use std::time::Duration;

use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

use tasks::{
    collectors::{container::ContainerCollectorTask, host::HostCollectorTask},
    maintenance::retention::RetentionTask,
};

/// The agent's workload is a handful of interval loops plus a low-QPS gRPC
/// server. The default multi-thread runtime spawns one worker + one blocking
/// thread per core (64 threads on a 32-core host, all futex-idle at 0.1% CPU);
/// each idle thread costs stack and TLS pages. Two workers cover the collector
/// cadence and the gRPC accept loop with room to spare.
#[tokio::main(worker_threads = 2)]
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
        refresh_rate = config.refresh_rate,
        grpc_port = config.grpc_port,
        panel = %config.panel_url,
        "starting openoxide agent"
    );

    let context = container.resolve::<core::context::MonitorContext>().await?;

    if let Err(error) = context.docker.ping().await {
        warn!(%error, socket = %context.config.docker_socket, "docker daemon unreachable");
    }

    let shutdown = CancellationToken::new();

    let mut tasks = tokio::task::JoinSet::new();
    tasks.spawn(GrpcServerTask::run(
        (*context).clone(),
        container.clone(),
        shutdown.clone(),
    ));
    tasks.spawn(HostCollectorTask::run((*context).clone(), shutdown.clone()));
    tasks.spawn(ContainerCollectorTask::run(
        (*context).clone(),
        shutdown.clone(),
    ));
    tasks.spawn(RetentionTask::run((*context).clone(), shutdown.clone()));

    wait_for_shutdown_signal().await;
    info!("shutdown signal received, stopping tasks");
    shutdown.cancel();

    match tokio::time::timeout(Duration::from_secs(5), async {
        while tasks.join_next().await.is_some() {}
    })
    .await
    {
        Ok(()) => info!("all tasks stopped cleanly"),
        Err(_) => {
            warn!("tasks did not stop within 5s, aborting");
            tasks.abort_all();
        }
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
