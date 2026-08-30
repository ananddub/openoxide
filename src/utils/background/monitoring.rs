use auto_di::resolve;
use std::sync::Arc;

use crate::services::observability::monitoring::{
    lifecycle::MonitoringLifecycleService, monitoring_service::MonitoringService,
    reconciler::MonitoringReconciler,
};

const AGENT_UPDATE_INTERVAL: std::time::Duration = std::time::Duration::from_secs(5 * 60);

pub fn start(
    monitoring: Arc<MonitoringService>,
    reconciler: Arc<MonitoringReconciler>,
    lifecycle: Arc<MonitoringLifecycleService>,
) {
    let local_lifecycle = Arc::clone(&lifecycle);
    let local_monitoring = Arc::clone(&monitoring);
    let local_reconciler = Arc::clone(&reconciler);
    tokio::spawn(async move {
        match local_lifecycle.ensure_local_registration().await {
            Ok(server_id) => {
                tracing::info!(server_id, "starting local Agent lifecycle watcher");
                watch_server(server_id, local_monitoring, local_reconciler).await;
            }
            Err(error) => tracing::warn!(%error, "could not start local Agent lifecycle watcher"),
        }
    });
    start_agent_updates(Arc::clone(&monitoring), lifecycle);
    tokio::spawn(async move {
        let mut watched = std::collections::HashSet::new();
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(10));
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            ticker.tick().await;
            let server_ids = match monitoring.server_ids().await {
                Ok(ids) => ids,
                Err(error) => {
                    tracing::warn!(%error, "could not discover monitoring servers");
                    continue;
                }
            };
            for server_id in server_ids {
                if !watched.insert(server_id) {
                    continue;
                }
                let monitoring = Arc::clone(&monitoring);
                let reconciler = Arc::clone(&reconciler);
                tokio::spawn(async move {
                    watch_server(server_id, monitoring, reconciler).await;
                });
            }
        }
    });
}

fn start_agent_updates(
    monitoring: Arc<MonitoringService>,
    lifecycle: Arc<MonitoringLifecycleService>,
) {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(AGENT_UPDATE_INTERVAL);
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        loop {
            ticker.tick().await;
            // The panel runtime is intentionally Docker-CLI-free. Local agent
            // lifecycle/update work is owned by the Agent deployment, so do
            // not try to pull or recreate it from this process.
            let server_ids = match monitoring.remote_server_ids().await {
                Ok(server_ids) => server_ids,
                Err(error) => {
                    tracing::warn!(%error, "could not discover agents for automatic update");
                    continue;
                }
            };

            for server_id in server_ids {
                match lifecycle.update_agent_if_needed(server_id).await {
                    Ok(true) => tracing::info!(
                        server_id,
                        "updated OpenOxide monitoring agent to the latest image"
                    ),
                    Ok(false) => {}
                    Err(error) => tracing::warn!(
                        server_id,
                        %error,
                        "automatic monitoring agent update failed"
                    ),
                }
            }
        }
    });
}

async fn watch_server(
    server_id: i64,
    monitoring: Arc<MonitoringService>,
    reconciler: Arc<MonitoringReconciler>,
) {
    loop {
        match tokio::time::timeout(
            std::time::Duration::from_secs(10),
            monitoring.watch_container_states(server_id),
        )
        .await
        {
            Ok(result) => match result {
                Ok(mut stream) => loop {
                    match tokio::time::timeout(std::time::Duration::from_secs(30), stream.message())
                        .await
                    {
                        Ok(Ok(Some(snapshot))) => {
                            if let Err(error) = reconciler
                                .apply_snapshot(server_id, &snapshot.containers)
                                .await
                            {
                                tracing::warn!(server_id, %error, "could not apply lifecycle snapshot");
                            }
                        }
                        Ok(Ok(None)) => {
                            tracing::warn!(server_id, "monitoring lifecycle stream ended");
                            break;
                        }
                        Ok(Err(error)) => {
                            tracing::warn!(server_id, %error, "monitoring lifecycle stream failed");
                            break;
                        }
                        Err(error) => {
                            tracing::warn!(server_id, %error, "monitoring lifecycle stream timed out");
                            break;
                        }
                    }
                },
                Err(error) => tracing::warn!(server_id, %error, "lifecycle stream unavailable"),
            },
            Err(error) => {
                tracing::warn!(server_id, %error, "lifecycle stream connection timed out")
            }
        }
        if server_ssh_is_reachable(server_id).await {
            tracing::debug!(
                server_id,
                "monitoring agent unavailable but SSH server is reachable; keeping server active"
            );
        } else if let Err(error) = reconciler.mark_offline(server_id).await {
            tracing::warn!(server_id, %error, "could not mark disconnected server offline");
        }
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}

async fn server_ssh_is_reachable(server_id: i64) -> bool {
    let Ok(queries) = resolve::<crate::db::queries::FlashQueries>().await else {
        return false;
    };
    let Ok(Some((host, port, username, private_key, public_key))) =
        crate::services::infrastructure::server::ssh_credentials(&queries, server_id).await
    else {
        return false;
    };
    let Ok(port) = u16::try_from(port) else {
        return false;
    };
    let executor = crate::utils::exec::RemoteExecutor::new(
        host,
        port,
        username,
        crate::utils::exec::SshAuth::key_pair(private_key, public_key),
        crate::utils::exec::SshHostKey::InsecureAcceptAny,
    )
    .with_multiplexing(false);
    tokio::time::timeout(
        std::time::Duration::from_secs(8),
        executor.run("true", std::iter::empty::<&str>()),
    )
    .await
    .is_ok_and(|result| result.is_ok())
}
