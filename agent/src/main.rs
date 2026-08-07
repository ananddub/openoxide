mod collector;
mod config;
mod context;
mod docker;
mod error;
mod filter;
mod grpc;
mod logs;
mod rollup;
mod store;
mod tasks;

use std::sync::Arc;
use std::time::Duration;

use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

use config::Config;
use context::MonitorContext;
use docker::api::DockerApi;
use store::Store;

use tasks::{
    container::collect_and_push_containers, grpc::serve_grpc, host::collect_host_metrics,
    retention::prune_old_metrics,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("agent=info")),
        )
        .init();

    let config = match Config::from_env() {
        Ok(config) => Arc::new(config),
        Err(error) => {
            error!(%error, "invalid configuration");
            std::process::exit(1);
        }
    };

    info!(
        server_id = config.server_id,
        refresh_rate = config.refresh_rate,
        grpc_port = config.grpc_port,
        panel = %config.panel_url,
        "starting rustploy monitor agent"
    );

    let store = Arc::new(Store::init(&config.database_url).await?);
    let docker = DockerApi::new(&config.docker_socket);
    let filter = config.container_filter();

    let context = MonitorContext::new(
        Arc::clone(&config),
        Arc::clone(&store),
        docker.clone(),
        filter,
    );

    if let Err(error) = context.docker.ping().await {
        warn!(%error, socket = %context.config.docker_socket, "docker daemon unreachable");
    }

    let shutdown = CancellationToken::new();

    let mut tasks = tokio::task::JoinSet::new();
    tasks.spawn(serve_grpc(context.clone(), shutdown.clone()));
    tasks.spawn(collect_host_metrics(context.clone(), shutdown.clone()));
    tasks.spawn(collect_and_push_containers(
        context.clone(),
        shutdown.clone(),
    ));
    tasks.spawn(prune_old_metrics(context.clone(), shutdown.clone()));

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
