use std::collections::HashMap;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use crate::collector::{CgroupCollector, cgroup};
use crate::config::{CollectionMode, Config};
use crate::context::MonitorContext;
use crate::docker::stream::ContainerStreamer;
use crate::docker::types::{ContainerId, ContainerSample};
use crate::rollup::Rollup;
use crate::store::ContainerMetricRow;

const CONTAINER_REFRESH_INTERVAL: Duration = Duration::from_secs(30);

#[derive(serde::Serialize)]
struct PushMetricsPayload<'a> {
    metrics: Vec<PushContainerMetricPoint<'a>>,
}

#[derive(serde::Serialize)]
struct PushContainerMetricPoint<'a> {
    server_id: i64,
    application_id: i64,
    compose_id: i64,
    container_id: &'a str,
    container_name: &'a str,
    cpu_percent: f64,
    memory_used_mb: f64,
    memory_limit_mb: f64,
    net_rx_kbps: f64,
    net_tx_kbps: f64,
    timestamp: i64,
}

pub async fn collect_and_push_containers(ctx: MonitorContext, shutdown: CancellationToken) {
    let filter = &ctx.filter;

    let use_cgroup = match ctx.config.collection_mode {
        CollectionMode::Stream => false,
        CollectionMode::Cgroup => true,
        CollectionMode::Auto => CgroupCollector::new(filter.clone()).is_some(),
    };

    if use_cgroup {
        if let Some(collector) = CgroupCollector::new(filter.clone()) {
            info!(
                mode = "cgroup",
                filter = %filter.describe(),
                "collecting container metrics"
            );
            poll_cgroups(ctx, collector, shutdown).await;
            return;
        }
        warn!("cgroup v2 unavailable, falling back to streaming collector");
    }

    info!(
        mode = "stream",
        filter = %filter.describe(),
        "collecting container metrics"
    );
    stream_and_push_containers(ctx, shutdown).await;
}

async fn poll_cgroups(
    ctx: MonitorContext,
    mut collector: CgroupCollector,
    shutdown: CancellationToken,
) {
    let endpoint = ctx.config.container_metrics_endpoint();
    let filter_is_unset = ctx.filter.is_unset();

    let mut rollup = Rollup::new(ctx.config.rollup_samples);
    let rollup_enabled = !rollup.is_passthrough();
    if rollup_enabled {
        info!(
            window_samples = ctx.config.rollup_samples,
            "container rollup enabled — storing average + peak rows"
        );
    }

    let mut sample = tokio::time::interval(Duration::from_secs(ctx.config.refresh_rate));
    sample.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    let mut refresh = tokio::time::interval(CONTAINER_REFRESH_INTERVAL);
    refresh.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                info!("cgroup collector stopped");
                return;
            }
            _ = refresh.tick() => {
                match collector.refresh_containers(&ctx.docker).await {
                    Ok(count) => cgroup::warn_if_dense(count, filter_is_unset),
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

                let to_store = if rollup_enabled {
                    rollup.add(&rows)
                } else {
                    rows
                };

                if to_store.is_empty() {
                    continue;
                }

                if let Err(error) = ctx.store.save_container_metrics_batch(&to_store).await {
                    error!(%error, "could not persist batch container metrics");
                }

                push_container_metrics(&ctx.http_client, &ctx.config, &endpoint, &to_store).await;
            }
        }
    }
}

async fn stream_and_push_containers(ctx: MonitorContext, shutdown: CancellationToken) {
    let (mut streamer, mut samples) =
        ContainerStreamer::new(ctx.docker.clone(), ctx.filter.clone());

    let streamer_shutdown = shutdown.clone();
    let streamer_task = tokio::spawn(async move {
        streamer.run(streamer_shutdown).await;
    });

    let endpoint = ctx.config.container_metrics_endpoint();
    let mut flush = tokio::time::interval(Duration::from_secs(ctx.config.refresh_rate));
    flush.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    let mut latest: HashMap<ContainerId, ContainerMetricRow> = HashMap::new();

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

                let rows: Vec<ContainerMetricRow> = latest.values().cloned().collect();
                latest.clear();

                if let Err(error) = ctx.store.save_container_metrics_batch(&rows).await {
                    error!(%error, "could not persist batch container metrics");
                }

                push_container_metrics(&ctx.http_client, &ctx.config, &endpoint, &rows).await;
            }
            sample = samples.recv() => {
                match sample {
                    Ok(sample) => {
                        latest.insert(sample.container_id.clone(), to_metric_row(&sample));
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(skipped)) => {
                        debug!(skipped, "dropped stale container samples");
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }
}

fn to_metric_row(sample: &ContainerSample) -> ContainerMetricRow {
    let stats = &sample.stats;
    let (net_rx_bytes, net_tx_bytes) = stats.network_bytes();
    let (block_read_bytes, block_write_bytes) = stats.block_io_bytes();

    ContainerMetricRow {
        id: None,
        timestamp: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        container_id: sample.container_id.short().to_string(),
        name: sample.name.as_str().to_string(),
        cpu_perc: stats.cpu_percent(),
        mem_perc: stats.memory.used_percent(),
        mem_used_mb: stats.memory.used_bytes() as f64 / 1_048_576.0,
        mem_total_mb: stats.memory.limit as f64 / 1_048_576.0,
        net_in_mb: net_rx_bytes as f64 / 1_048_576.0,
        net_out_mb: net_tx_bytes as f64 / 1_048_576.0,
        block_read_mb: block_read_bytes as f64 / 1_048_576.0,
        block_write_mb: block_write_bytes as f64 / 1_048_576.0,
        application_id: sample.labels.get("com.rustploy.application-id").and_then(|v| v.parse().ok()),
        compose_id: sample.labels.get("com.rustploy.compose-id").and_then(|v| v.parse().ok()),
    }
}

pub async fn push_container_metrics(
    client: &reqwest::Client,
    config: &Config,
    endpoint: &str,
    metrics: &[ContainerMetricRow],
) {
    let timestamp = chrono::Utc::now().timestamp();

    let points: Vec<_> = metrics
        .iter()
        .map(|metric| PushContainerMetricPoint {
            server_id: config.server_id,
            application_id: metric.application_id.unwrap_or(0),
            compose_id: metric.compose_id.unwrap_or(0),
            container_id: &metric.container_id,
            container_name: &metric.name,
            cpu_percent: metric.cpu_perc,
            memory_used_mb: metric.mem_used_mb,
            memory_limit_mb: metric.mem_total_mb,
            net_rx_kbps: metric.net_in_mb * 1024.0,
            net_tx_kbps: metric.net_out_mb * 1024.0,
            timestamp,
        })
        .collect();

    let payload = PushMetricsPayload { metrics: points };

    let request = client
        .post(endpoint)
        .header("X-Metrics-Token", &config.metrics_token)
        .header("X-Server-Id", config.server_id)
        .json(&payload);

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
