use serde::Serialize;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::collector::SystemCollector;
use crate::context::MonitorContext;

#[derive(Serialize)]
struct HostMetricPush<'a> {
    server_id: i64,
    cpu: f64,
    cpu_model: Option<&'a str>,
    cpu_cores: Option<i64>,
    cpu_physical_cores: Option<i64>,
    cpu_speed: Option<f64>,
    os: Option<&'a str>,
    distro: Option<&'a str>,
    kernel: Option<&'a str>,
    arch: Option<&'a str>,
    mem_used: f64,
    mem_used_gb: f64,
    mem_total: f64,
    uptime: i64,
    disk_used: f64,
    total_disk: f64,
    network_in: f64,
    network_out: f64,
}

pub async fn collect_host_metrics(ctx: MonitorContext, shutdown: CancellationToken) {
    let mut collector = SystemCollector::new();
    let interval = SystemCollector::interval_after_sample(ctx.config.refresh_rate);

    loop {
        let metric = collector.sample().await;

        if let Err(error) = ctx.store.save_server_metric(&metric).await {
            error!(%error, "could not persist host metric");
        }
        let payload = HostMetricPush {
            server_id: ctx.config.server_id,
            cpu: metric.cpu,
            cpu_model: Some(&metric.cpu_model),
            cpu_cores: Some(metric.cpu_cores.into()),
            cpu_physical_cores: Some(metric.cpu_physical_cores.into()),
            cpu_speed: Some(metric.cpu_speed),
            os: Some(&metric.os),
            distro: Some(&metric.distro),
            kernel: Some(&metric.kernel),
            arch: Some(&metric.arch),
            mem_used: metric.mem_used,
            mem_used_gb: metric.mem_used_gb,
            mem_total: metric.mem_total,
            uptime: metric.uptime as i64,
            disk_used: metric.disk_used,
            total_disk: metric.total_disk,
            network_in: metric.network_in,
            network_out: metric.network_out,
        };
        if let Err(error) = ctx
            .http_client
            .post(ctx.config.server_metrics_endpoint())
            .header("X-Metrics-Token", &ctx.config.metrics_token)
            .header("X-Server-Id", ctx.config.server_id)
            .json(&payload)
            .send()
            .await
        {
            error!(%error, "could not push host metric to panel");
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
