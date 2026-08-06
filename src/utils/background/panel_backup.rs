use std::sync::Arc;

use crate::services::backup::PanelBackupService;

pub fn start(service: Arc<PanelBackupService>) {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(24 * 60 * 60));
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        ticker.tick().await;
        loop {
            ticker.tick().await;
            match service.create().await {
                Ok(backup) => {
                    tracing::info!(path = %backup.path, checksum = %backup.checksum_sha256, "scheduled verified panel backup completed")
                }
                Err(error) => tracing::error!(error = %error, "scheduled panel backup failed"),
            }
        }
    });
}
