use std::sync::Arc;

use crate::services::deployment::DeploymentService;

pub fn start(service: Arc<DeploymentService>) {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(24 * 60 * 60));
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            ticker.tick().await;
            let cutoff = chrono::Utc::now().timestamp() - 30 * 24 * 60 * 60;
            match service.cleanup_logs_before(cutoff).await {
                Ok(0) => {}
                Ok(removed) => tracing::info!(removed, "expired deployment logs removed"),
                Err(error) => tracing::error!(error = %error, "deployment log cleanup failed"),
            }
        }
    });
}
