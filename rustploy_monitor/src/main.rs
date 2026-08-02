mod collector;
mod config;
mod docker;
mod filter;
mod rollup;
mod grpc;
mod logs;
mod store;

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

use collector::{CgroupCollector, SystemCollector};
use config::Config;
use docker::api::DockerApi;
use docker::stream::ContainerStreamer;
use grpc::{MonitoringGrpc, MonitoringServiceServer};
use rollup::Rollup;
use store::Store;

/// How often the retention sweep runs.
const CLEANUP_INTERVAL: Duration = Duration::from_secs(86_400);
/// How often the cgroup collector re-lists containers. The set changes far more
/// slowly than the metrics do, so this is deliberately slower than sampling.
const CONTAINER_REFRESH_INTERVAL: Duration = Duration::from_secs(30);

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

    let docker = DockerApi::new(&config.docker_socket);
    // Probe once at startup so a missing socket mount is reported here rather
    // than as an identical error on every collection cycle.
    if let Err(error) = docker.ping().await {
        warn!(%error, socket = %config.docker_socket, "docker daemon unreachable — container metrics will be empty");
    }

    // One token shuts every task down together on SIGINT/SIGTERM.
    let shutdown = CancellationToken::new();

    let mut tasks = tokio::task::JoinSet::new();
    tasks.spawn(serve_grpc(
        Arc::clone(&config),
        Arc::clone(&store),
        docker.clone(),
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
        docker.clone(),
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
async fn serve_grpc(
    config: Arc<Config>,
    store: Arc<Store>,
    docker: DockerApi,
    shutdown: CancellationToken,
) {
    let addr: SocketAddr = match format!("0.0.0.0:{}", config.grpc_port).parse() {
        Ok(addr) => addr,
        Err(error) => {
            error!(%error, port = config.grpc_port, "invalid gRPC bind address");
            return;
        }
    };

    info!(%addr, "gRPC query server listening");

    let service = MonitoringGrpc::new(store, config.server_id, docker);
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

/// Collects container metrics and forwards them to the panel.
///
/// Two collection modes, picked at startup:
///
/// - **cgroup** (default when cgroup v2 is mounted) — reads each container's
///   metrics straight from the filesystem. Costs ~35 µs per container and puts
///   no load on dockerd, so it holds up at thousands of containers. No network
///   counters, since those live in the container's network namespace.
/// - **stream** — one persistent docker API connection per container, pushing a
///   frame per second. Gives network I/O and sub-second freshness, but needs a
///   connection and dockerd work per container, so it does not scale past a few
///   hundred.
///
/// `COLLECTION_MODE` overrides the choice.
async fn collect_and_push_containers(
    config: Arc<Config>,
    store: Arc<Store>,
    docker: DockerApi,
    shutdown: CancellationToken,
) {
    let filter = config.container_filter();

    let use_cgroup = match config.collection_mode.as_str() {
        "stream" => false,
        "cgroup" => true,
        // auto: prefer cgroup, fall back to streaming if it is unavailable.
        _ => CgroupCollector::new(filter.clone()).is_some(),
    };

    if use_cgroup {
        if let Some(collector) = CgroupCollector::new(filter.clone()) {
            info!(
                mode = "cgroup",
                filter = %filter.describe(),
                "collecting container metrics"
            );
            poll_cgroups(config, store, docker, collector, shutdown).await;
            return;
        }
        warn!("cgroup v2 unavailable, falling back to the streaming collector");
    }

    info!(
        mode = "stream",
        filter = %filter.describe(),
        "collecting container metrics"
    );
    stream_and_push_containers(config, store, docker, shutdown).await;
}

/// Samples every monitored container from cgroup on a fixed cadence.
async fn poll_cgroups(
    config: Arc<Config>,
    store: Arc<Store>,
    docker: DockerApi,
    mut collector: CgroupCollector,
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
    let filter_is_unset = config.container_filter().is_unset();

    // Density control: when ROLLUP_SAMPLES > 1, raw samples stay in the
    // accumulator and only average+peak rows reach the store and panel.
    let mut rollup = Rollup::new(config.rollup_samples);
    let rollup_enabled = !rollup.is_passthrough();
    if rollup_enabled {
        info!(
            window_samples = config.rollup_samples,
            "container rollup enabled — only average + peak rows will be stored"
        );
    }

    let mut sample = tokio::time::interval(Duration::from_secs(config.refresh_rate));
    sample.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    // The container list changes far more slowly than the metrics do, so it is
    // refreshed on its own slower cadence rather than every sample.
    let mut refresh = tokio::time::interval(CONTAINER_REFRESH_INTERVAL);
    refresh.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                info!("cgroup collector stopped");
                return;
            }
            _ = refresh.tick() => {
                match collector.refresh_containers(&docker).await {
                    Ok(count) => collector::cgroup::warn_if_dense(count, filter_is_unset),
                    Err(error) => error!(%error, "could not refresh container list"),
                }
            }
            _ = sample.tick() => {
                let started = std::time::Instant::now();
                let rows = collector.sample();

                if rows.is_empty() {
                    continue;
                }

                debug!(
                    containers = rows.len(),
                    monitored = collector.monitored_count(),
                    took_ms = started.elapsed().as_millis(),
                    "sampled cgroups"
                );

                // With rollup on, most ticks accumulate and emit nothing.
                let to_store = if rollup_enabled {
                    rollup.add(&rows)
                } else {
                    rows
                };

                if to_store.is_empty() {
                    continue;
                }

                for row in &to_store {
                    if let Err(error) = store.save_container_metric(row).await {
                        error!(%error, container = %row.name, "could not persist container metric");
                    }
                }

                push_container_metrics(&client, &config, &endpoint, &to_store).await;
            }
        }
    }
}

/// Streams per-container stats from the docker daemon and forwards them to the
/// panel so it can fan them out over SSE.
///
/// The daemon pushes one frame per second per container. Those are buffered
/// here and flushed to the panel on a cooldown, so a burst of metrics becomes a
/// single batched request rather than N per-second POSTs.
async fn stream_and_push_containers(
    config: Arc<Config>,
    store: Arc<Store>,
    docker: DockerApi,
    shutdown: CancellationToken,
) {
    let (mut streamer, mut samples) =
        ContainerStreamer::new(docker.clone(), config.container_filter());

    // The streamer reconciles and streams in its own task.
    let streamer_shutdown = shutdown.clone();
    let streamer_task = tokio::spawn(async move {
        streamer.run(streamer_shutdown).await;
    });

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

    // Batch metrics on this cadence. The stream is live underneath; this only
    // controls how often we flush accumulated samples, not how fresh they are.
    let endpoint = config.container_metrics_endpoint();
    let mut flush = tokio::time::interval(Duration::from_secs(config.refresh_rate));
    flush.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    let mut latest: std::collections::HashMap<String, store::ContainerMetricRow> =
        std::collections::HashMap::new();

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                info!("container streamer stopped");
                streamer_task.abort();
                return;
            }
            _ = flush.tick() => {
                if latest.is_empty() {
                    continue;
                }

                let rows: Vec<store::ContainerMetricRow> = latest.values().cloned().collect();
                latest.clear();

                for metric in &rows {
                    if let Err(error) = store.save_container_metric(metric).await {
                        error!(%error, container = %metric.name, "could not persist container metric");
                    }
                }

                push_container_metrics(&client, &config, &endpoint, &rows).await;
            }
            sample = samples.recv() => {
                match sample {
                    Ok(sample) => {
                        // Hold the newest reading per container; the flush sends them all.
                        latest.insert(sample.container_id.clone(), to_metric_row(&sample));
                    }
                    // A slow flush can fall behind a fast stream. Only the newest
                    // reading matters, so skipped frames are not an error.
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(skipped)) => {
                        debug!(skipped, "dropped stale container samples");
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }
}

/// Converts a streamed sample into the row shape used for persistence and push.
fn to_metric_row(sample: &docker::stream::ContainerSample) -> store::ContainerMetricRow {
    let stats = &sample.stats;
    let (net_rx_bytes, net_tx_bytes) = stats.network_bytes();
    let (block_read_bytes, block_write_bytes) = stats.block_io_bytes();

    store::ContainerMetricRow {
        id: None,
        timestamp: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        container_id: sample.container_id.clone(),
        name: sample.name.clone(),
        cpu_perc: stats.cpu_percent(),
        mem_perc: stats.memory.used_percent(),
        mem_used_mb: stats.memory.used_bytes() as f64 / 1_048_576.0,
        mem_total_mb: stats.memory.limit as f64 / 1_048_576.0,
        net_in_mb: net_rx_bytes as f64 / 1_048_576.0,
        net_out_mb: net_tx_bytes as f64 / 1_048_576.0,
        block_read_mb: block_read_bytes as f64 / 1_048_576.0,
        block_write_mb: block_write_bytes as f64 / 1_048_576.0,
    }
}

/// Forwards a batch of container metrics to the panel's ingest endpoint in a
/// single request, instead of one POST per container.
async fn push_container_metrics(
    client: &reqwest::Client,
    config: &Config,
    endpoint: &str,
    metrics: &[store::ContainerMetricRow],
) {
    let timestamp = chrono::Utc::now().timestamp();

    let payload = serde_json::json!({
        "metrics": metrics.iter().map(|metric| {
            serde_json::json!({
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
            })
        }).collect::<Vec<_>>()
    });

    let mut request = client.post(endpoint).json(&payload);
    if !config.metrics_token.is_empty() {
        request = request.header("X-Metrics-Token", &config.metrics_token);
    }

    match request.send().await {
        Ok(response) if response.status().is_success() => {
            info!(count = metrics.len(), "pushed container metrics to panel");
        }
        Ok(response) => {
            warn!(
                status = response.status().as_u16(),
                count = metrics.len(),
                "panel rejected container metrics"
            );
        }
        Err(error) => {
            warn!(%error, count = metrics.len(), "could not reach panel");
        }
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
