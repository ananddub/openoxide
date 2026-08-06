use std::sync::Arc;

use crate::repository::BackgroundPolicyRepository;
use crate::services::deployment::DeploymentService;

pub fn start(service: Arc<DeploymentService>, policies: Arc<BackgroundPolicyRepository>) {
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
            if !policy.log_cleanup_enabled
                || last_run_minute == Some(minute)
                || !super::policy::cron_due(&policy.log_cleanup_cron, now)
            {
                continue;
            }
            last_run_minute = Some(minute);
            let cutoff = now.timestamp() - policy.log_retention_days * 24 * 60 * 60;
            match service.cleanup_logs_before(cutoff).await {
                Ok(0) => {}
                Ok(removed) => tracing::info!(removed, "expired deployment logs removed"),
                Err(error) => tracing::error!(error = %error, "deployment log cleanup failed"),
            }
        }
    });
}
