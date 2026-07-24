mod containers;
mod db;
mod grpc_client;
mod grpc_server;
mod monitoring;

use db::Db;
use grpc_server::{MonitoringGrpcServer, MonitoringServiceServer};
use monitoring::ServerMetricsMonitor;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("rustploy_monitor=info".parse()?),
        )
        .init();

    info!("Starting Rustploy Dedicated gRPC Monitoring Service...");

    let db_url =
        std::env::var("MONITOR_DATABASE_URL").unwrap_or_else(|_| "sqlite://monitor.db".to_string());

    let grpc_port = std::env::var("GRPC_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(50051);

    let server_token = std::env::var("METRICS_TOKEN").unwrap_or_default();
    let callback_url = std::env::var("METRICS_URL_CALLBACK").unwrap_or_default();
    let cpu_threshold = std::env::var("CPU_THRESHOLD")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(0.0);
    let mem_threshold = std::env::var("MEMORY_THRESHOLD")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(0.0);
    let refresh_rate = std::env::var("REFRESH_RATE")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(60);

    let db = Arc::new(Db::init(&db_url).await?);
    let server_monitor = Arc::new(ServerMetricsMonitor::new());

    // 1. gRPC Server on 0.0.0.0:50051
    let grpc_db = db.clone();
    let grpc_addr: SocketAddr = format!("0.0.0.0:{}", grpc_port).parse()?;

    let grpc_server_handle = tokio::spawn(async move {
        info!(
            "Starting gRPC Telemetry Server listening on gRPC://{}",
            grpc_addr
        );
        let grpc_service = MonitoringGrpcServer::new(grpc_db);
        if let Err(err) = tonic::transport::Server::builder()
            .add_service(MonitoringServiceServer::new(grpc_service))
            .serve(grpc_addr)
            .await
        {
            error!("gRPC Telemetry Server failed: {:?}", err);
        }
    });

    // 2. Background loop for Server Metrics collection & Alert threshold checks
    let db_clone = db.clone();
    let monitor_clone = server_monitor.clone();
    let cb_url = callback_url.clone();
    let token = server_token.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(refresh_rate));
        loop {
            interval.tick().await;
            let metric = monitor_clone.get_server_metrics();
            if let Err(err) = db_clone.save_server_metric(&metric).await {
                error!("Error saving server metrics to SQLite: {:?}", err);
            }
            monitor_clone
                .check_thresholds(
                    &metric,
                    cpu_threshold,
                    mem_threshold,
                    &cb_url,
                    &token,
                    "DOKPLOY",
                )
                .await;
        }
    });

    // 3. Background loop for Docker Container Metrics collection & real-time SSE stream forwarding
    let db_clone2 = db.clone();
    let rustploy_url = std::env::var("RUSTPLOY_SERVER_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:4000".to_string());
    let req_client = reqwest::Client::new();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(refresh_rate));
        loop {
            interval.tick().await;
            let container_metrics = containers::collect_docker_container_metrics();
            for c_metric in container_metrics {
                if let Err(err) = db_clone2.save_container_metric(&c_metric).await {
                    error!("Error saving container metric: {:?}", err);
                }

                let payload = serde_json::json!({
                    "server_id": 1,
                    "application_id": 0,
                    "compose_id": 0,
                    "container_id": c_metric.container_id,
                    "container_name": c_metric.name,
                    "cpu_percent": c_metric.cpu_perc,
                    "memory_used_mb": c_metric.mem_used_mb,
                    "memory_limit_mb": c_metric.mem_total_mb,
                    "net_rx_kbps": c_metric.net_in_mb * 1024.0,
                    "net_tx_kbps": c_metric.net_out_mb * 1024.0,
                    "timestamp": chrono::Utc::now().timestamp(),
                });

                let _ = req_client
                    .post(format!("{}/api/monitoring/containers", rustploy_url))
                    .json(&payload)
                    .send()
                    .await;
            }
        }
    });

    // 4. Background metrics cleanup task (Runs every 24h)
    let db_clone3 = db.clone();
    let retention_days = std::env::var("RETENTION_DAYS")
        .ok()
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(7);
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(86400));
        loop {
            interval.tick().await;
            if let Ok(affected) = db_clone3.cleanup_old_metrics(retention_days).await {
                info!(
                    "Cleaned up {} old metrics records older than {} days",
                    affected, retention_days
                );
            }
        }
    });

    // Wait for gRPC server to finish (runs forever)
    let _ = grpc_server_handle.await;
    Ok(())
}
