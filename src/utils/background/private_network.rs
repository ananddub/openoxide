use std::{sync::Arc, time::Duration};

use crate::services::server::ServerPrivateNetworkService;

pub fn start(service: Arc<ServerPrivateNetworkService>) {
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(45)).await;
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            service.check_all_health().await;
        }
    });
}
