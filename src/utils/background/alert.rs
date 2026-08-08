use std::sync::Arc;

use crate::services::alert::{AlertService, service::EVALUATION_INTERVAL_SECS};

pub fn start(service: Arc<AlertService>) {
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(
            EVALUATION_INTERVAL_SECS as u64,
        ));
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        tracing::info!(
            interval_secs = EVALUATION_INTERVAL_SECS,
            "alert evaluation loop started"
        );

        loop {
            ticker.tick().await;
            match service.evaluate_once().await {
                Ok(0) => {}
                Ok(count) => tracing::debug!(count, "alerts dispatched"),
                Err(error) => tracing::error!(error = %error, "alert evaluation failed"),
            }
        }
    });
}
