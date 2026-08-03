use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::collector::SystemCollector;
use crate::context::MonitorContext;

pub async fn collect_host_metrics(ctx: MonitorContext, shutdown: CancellationToken) {
    let mut collector = SystemCollector::new();
    let interval = SystemCollector::interval_after_sample(ctx.config.refresh_rate);

    loop {
        let metric = collector.sample().await;

        if let Err(error) = ctx.store.save_server_metric(&metric).await {
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
