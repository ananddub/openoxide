use auto_di::resolve;
use std::sync::Arc;

use crate::db::queries::FlashQueries;
use crate::{
    services::{
        automation::backup::PanelBackupService, automation::schedule::ScheduleRunner,
        compute::engine::DeploymentService,
        infrastructure::server::ServerPrivateNetworkService,
        integrations::notification::NotificationService, observability::alert::AlertService,
        observability::monitoring::{
            lifecycle::MonitoringLifecycleService, reconciler::MonitoringReconciler,
        },
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

        // The panel host's edge (Traefik config + container) is repaired in
        // the background: it depends on the local Agent gRPC channel, which
        // may still be booting. Retrying keeps a transient failure from
        // leaving domains unrouted until the next restart.
        let traefik = resolve::<crate::services::networking::traefik::TraefikService>()
            .await
            .map_err(|error| format!("failed to resolve traefik service: {error}"))?;
        super::edge::start(traefik);

        let alerts = resolve::<AlertService>()
            .await
            .map_err(|error| format!("failed to resolve alert service: {error}"))?;
        super::alert::start(alerts);

        let monitoring = resolve::<MonitoringReconciler>()
            .await
            .map_err(|error| format!("failed to resolve monitoring reconciler: {error}"))?;
        let monitoring_service =
            resolve::<crate::services::observability::monitoring::service::MonitoringService>()
                .await
                .map_err(|error| format!("failed to resolve monitoring service: {error}"))?;
        let monitoring_lifecycle = resolve::<MonitoringLifecycleService>()
            .await
            .map_err(|error| format!("failed to resolve monitoring lifecycle: {error}"))?;
        monitoring_lifecycle
            .ensure_local_registration()
            .await
            .map_err(|error| format!("failed to register local monitoring agent: {error}"))?;
        super::monitoring::start(monitoring_service, monitoring, monitoring_lifecycle);

        let deployments = resolve::<DeploymentService>()
            .await
            .map_err(|error| format!("failed to resolve deployment service: {error}"))?;
        let queries = resolve::<FlashQueries>()
            .await
            .map_err(|error| format!("failed to resolve database queries: {error}"))?;
        super::log_cleanup::start(deployments, Arc::clone(&queries));

        let panel_backup = resolve::<PanelBackupService>()
            .await
            .map_err(|error| format!("failed to resolve panel backup service: {error}"))?;
        super::panel_backup::start(panel_backup, queries);

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
