mod collector;
mod config;
mod grpc;
mod logs;
mod store;

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

use collector::{SystemCollector, collect_container_metrics};
use config::Config;
use grpc::{MonitoringGrpc, MonitoringServiceServer};
use store::Store;

/// How often the retention sweep runs.
const CLEANUP_INTERVAL: Duration = Duration::from_secs(86_400);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("rustploy_monitor=info")),
        )
        .init();

    // Config problems are fatal: running half-configured produces metrics that
    // silently go nowhere, which is worse than not starting.
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

    if config.metrics_token.is_empty() {
        warn!("METRICS_TOKEN is unset — the panel cannot authenticate pushes from this agent");
    }

    let store = Arc::new(Store::init(&config.database_url).await?);

    // One token shuts every task down together on SIGINT/SIGTERM.
    let shutdown = CancellationToken::new();

    let mut tasks = tokio::task::JoinSet::new();
    tasks.spawn(serve_grpc(
        Arc::clone(&config),
        Arc::clone(&store),
        shutdown.clone(),
    ));
    tasks.spawn(collect_host_metrics(
        Arc::clone(&config),
        Arc::clone(&store),
        shutdown.clone(),
    ));
    tasks.spawn(collect_and_push_containers(
        Arc::clone(&config),
        Arc::clone(&store),
        shutdown.clone(),
    ));
    tasks.spawn(prune_old_metrics(
        Arc::clone(&config),
        Arc::clone(&store),
        shutdown.clone(),
    ));

    wait_for_shutdown_signal().await;
    info!("shutdown signal received, stopping tasks");
    shutdown.cancel();

    // Give tasks a moment to unwind; abort whatever is still running so a stuck
    // docker call can't hold the process open.
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

/// Serves the panel's metric queries.
async fn serve_grpc(config: Arc<Config>, store: Arc<Store>, shutdown: CancellationToken) {
    let addr: SocketAddr = match format!("0.0.0.0:{}", config.grpc_port).parse() {
        Ok(addr) => addr,
        Err(error) => {
            error!(%error, port = config.grpc_port, "invalid gRPC bind address");
            return;
        }
    };

    info!(%addr, "gRPC query server listening");

    let service = MonitoringGrpc::new(store, config.server_id);
    let result = tonic::transport::Server::builder()
        .add_service(MonitoringServiceServer::new(service))
        .serve_with_shutdown(addr, shutdown.cancelled_owned())
        .await;

    if let Err(error) = result {
        error!(%error, "gRPC server stopped unexpectedly");
    }
}

/// Samples host CPU/memory/disk/network and persists each reading.
async fn collect_host_metrics(config: Arc<Config>, store: Arc<Store>, shutdown: CancellationToken) {
    let mut collector = SystemCollector::new();
    let interval = SystemCollector::interval_after_sample(config.refresh_rate);

    loop {
        let metric = collector.sample().await;

        if let Err(error) = store.save_server_metric(&metric).await {
            error!(%error, "could not persist host metric");
        }

        tokio::select! {
            _ = shutdown.cancelled() => {
                info!("host metric collector stopped");
                return;
            }
            _ = tokio::time::sleep(interval) => {}
        }
    }
}

/// Samples per-container stats, persists them, and forwards them to the panel
/// so it can fan them out over SSE.
async fn collect_and_push_containers(
    config: Arc<Config>,
    store: Arc<Store>,
    shutdown: CancellationToken,
) {
    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
    {
        Ok(client) => client,
        Err(error) => {
            error!(%error, "could not build HTTP client, container push disabled");
            return;
        }
    };

    let endpoint = config.container_metrics_endpoint();
    let mut ticker = tokio::time::interval(Duration::from_secs(config.refresh_rate));

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                info!("container metric collector stopped");
                return;
            }
            _ = ticker.tick() => {}
        }

        let metrics = collect_container_metrics().await;
        if metrics.is_empty() {
            continue;
        }

        for metric in &metrics {
            if let Err(error) = store.save_container_metric(metric).await {
                error!(%error, container = %metric.name, "could not persist container metric");
            }
        }

        push_container_metrics(&client, &config, &endpoint, &metrics).await;
    }
}

/// Forwards a batch of container metrics to the panel's ingest endpoint.
async fn push_container_metrics(
    client: &reqwest::Client,
    config: &Config,
    endpoint: &str,
    metrics: &[store::ContainerMetricRow],
) {
    let timestamp = chrono::Utc::now().timestamp();
    let mut failures = 0usize;

    for metric in metrics {
        // application_id / compose_id are zero because the agent only sees
        // container names; the panel resolves them to owning resources.
        let payload = serde_json::json!({
            "server_id": config.server_id,
            "application_id": 0,
            "compose_id": 0,
            "container_id": metric.container_id,
            "container_name": metric.name,
            "cpu_percent": metric.cpu_perc,
            "memory_used_mb": metric.mem_used_mb,
            "memory_limit_mb": metric.mem_total_mb,
            // Cumulative totals since container start, converted to KB — not a
            // per-second rate, despite the field name the panel expects.
            "net_rx_kbps": metric.net_in_mb * 1024.0,
            "net_tx_kbps": metric.net_out_mb * 1024.0,
            "timestamp": timestamp,
        });

        let mut request = client.post(endpoint).json(&payload);
        if !config.metrics_token.is_empty() {
            request = request.header("X-Metrics-Token", &config.metrics_token);
        }

        match request.send().await {
            Ok(response) if response.status().is_success() => {}
            Ok(response) => {
                failures += 1;
                warn!(
                    status = response.status().as_u16(),
                    container = %metric.name,
                    "panel rejected container metric"
                );
            }
            Err(error) => {
                failures += 1;
                warn!(%error, container = %metric.name, "could not reach panel");
            }
        }
    }

    if failures == 0 {
        info!(count = metrics.len(), "pushed container metrics to panel");
    } else {
        warn!(
            failed = failures,
            total = metrics.len(),
            "some container metrics could not be pushed"
        );
    }
}

/// Deletes metrics older than the configured retention window.
async fn prune_old_metrics(config: Arc<Config>, store: Arc<Store>, shutdown: CancellationToken) {
    let mut ticker = tokio::time::interval(CLEANUP_INTERVAL);

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                info!("retention sweeper stopped");
                return;
            }
            _ = ticker.tick() => {}
        }

        match store.cleanup_old_metrics(config.retention_days).await {
            Ok(0) => {}
            Ok(deleted) => info!(
                deleted,
                retention_days = config.retention_days,
                "pruned old metrics"
            ),
            Err(error) => error!(%error, "retention sweep failed"),
        }
    }
}
