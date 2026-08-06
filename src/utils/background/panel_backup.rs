use std::sync::Arc;

use crate::repository::BackgroundPolicyRepository;
use crate::services::backup::PanelBackupService;

pub fn start(service: Arc<PanelBackupService>, policies: Arc<BackgroundPolicyRepository>) {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(30));
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        let mut last_run_minute = None;
        loop {
            ticker.tick().await;
            let Ok(policy) = policies.get().await else {
                continue;
            };
            let now = chrono::Utc::now();
            let minute = now.timestamp() / 60;
            if !policy.panel_backup_enabled
                || last_run_minute == Some(minute)
                || !super::policy::cron_due(&policy.panel_backup_cron, now)
            {
                continue;
            }
            last_run_minute = Some(minute);
            match service.create().await {
                Ok(backup) => {
                    tracing::info!(path = %backup.path, checksum = %backup.checksum_sha256, "scheduled verified panel backup completed")
                }
                Err(error) => tracing::error!(error = %error, "scheduled panel backup failed"),
            }
        }
    });
}
