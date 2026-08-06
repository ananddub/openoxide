use auto_di::resolve;

use crate::{
    services::{
        alert::AlertService, notification::NotificationService, schedule::ScheduleRunner,
        server_management::ServerPrivateNetworkService,
    },
    utils::builder::queue::BuilderQueue,
};

pub struct BackgroundManager;

impl BackgroundManager {
    pub async fn start_all() -> Result<(), String> {
        let schedule = resolve::<ScheduleRunner>()
            .await
            .map_err(|error| format!("failed to resolve schedule runner: {error}"))?;
        super::schedule::start(schedule).await?;

        let builder = resolve::<BuilderQueue>()
            .await
            .map_err(|error| format!("failed to resolve builder queue: {error}"))?;
        super::builder::start(builder).await?;

        let alerts = resolve::<AlertService>()
            .await
            .map_err(|error| format!("failed to resolve alert service: {error}"))?;
        super::alert::start(alerts);

        let private_network = resolve::<ServerPrivateNetworkService>()
            .await
            .map_err(|error| format!("failed to resolve private-network service: {error}"))?;
        super::private_network::start(private_network);

        Self::dispatch_startup_notification().await;
        tracing::info!("all background systems started");
        Ok(())
    }

    async fn dispatch_startup_notification() {
        let Ok(notifications) = resolve::<NotificationService>().await else {
            tracing::warn!("startup notification service unavailable");
            return;
        };
        super::notification::dispatch_startup(notifications);
    }
}
