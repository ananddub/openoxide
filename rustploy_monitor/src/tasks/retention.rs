use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::context::MonitorContext;

const CLEANUP_INTERVAL: Duration = Duration::from_secs(86_400);

pub async fn prune_old_metrics(ctx: MonitorContext, shutdown: CancellationToken) {
    let mut ticker = tokio::time::interval(CLEANUP_INTERVAL);

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                info!("retention sweeper stopped");
                return;
            }
            _ = ticker.tick() => {}
        }

        match ctx.store.cleanup_old_metrics(ctx.config.retention_days).await {
            Ok(0) => {}
            Ok(deleted) => info!(
                deleted,
                retention_days = ctx.config.retention_days,
                "pruned old metrics"
            ),
            Err(error) => error!(%error, "retention sweep failed"),
        }
    }
}
