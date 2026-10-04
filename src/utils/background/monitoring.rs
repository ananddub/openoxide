use auto_di::resolve;
use std::sync::Arc;

use crate::services::observability::monitoring::{
    lifecycle::MonitoringLifecycleService, reconciler::MonitoringReconciler,
    service::MonitoringService,
};

/// Upper bound on one server's update/repair pass. A hung SSH executor (a KEX
/// stall or a wedged remote daemon) would otherwise block this sequential loop
/// indefinitely, freezing automatic updates for every later server. Kept under
/// `AGENT_UPDATE_INTERVAL` so one stuck server can never overrun the next tick.
const AGENT_UPDATE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(4 * 60);

const AGENT_UPDATE_INTERVAL: std::time::Duration = std::time::Duration::from_secs(5 * 60);
/// One full staggered registry-check cycle: each server's `id % 12` slot is
/// its five-minute tick within the hour, so a pull probe (the only check that
/// makes the remote agent contact the registry) fires at most hourly and
/// different servers never pull in the same tick.
const REGISTRY_CHECK_SLOTS: i64 = 12;

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
        // `watched` mirrors the set of servers that currently have a live
        // watcher task. It is reconciled against the discovered server list on
        // every tick so that a removed server's entry is dropped (letting its
        // watcher exit and its cached channel be pruned) and a re-added server
        // gets a fresh watcher.
        let mut watched: std::collections::HashSet<i64> = std::collections::HashSet::new();
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
            let live: std::collections::HashSet<i64> = server_ids.iter().copied().collect();

            // Drop watchers for servers that no longer exist: forget them here
            // so they can be re-watched if re-added, prune the cached gRPC
            // channel, and signal the running watcher to stop.
            let removed: Vec<i64> = watched.difference(&live).copied().collect();
            for server_id in removed {
                watched.remove(&server_id);
                monitoring.forget_agent(server_id);
            }

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
        let mut tick: i64 = 0;

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
            // Only this tick's slot of servers pay the registry-pull probe cost;
            // the rest get the cheap self-inspect, which never reaches the
            // remote registry.
            let slot = tick.rem_euclid(REGISTRY_CHECK_SLOTS);

            for server_id in server_ids {
                let check_registry = server_id.rem_euclid(REGISTRY_CHECK_SLOTS) == slot;
                match tokio::time::timeout(
                    AGENT_UPDATE_TIMEOUT,
                    lifecycle.update_agent_if_needed(server_id, check_registry),
                )
                .await
                {
                    Err(_elapsed) => tracing::warn!(
                        server_id,
                        "automatic monitoring agent update timed out; skipping this tick"
                    ),
                    Ok(Ok(true)) => tracing::info!(
                        server_id,
                        "updated OpenOxide monitoring agent to the latest image"
                    ),
                    Ok(Ok(false)) => {}
                    Ok(Err(error)) => tracing::warn!(
                        server_id,
                        %error,
                        "automatic monitoring agent update failed"
                    ),
                }
            }
            tick = tick.wrapping_add(1);
        }
    });
}

async fn watch_server(
    server_id: i64,
    monitoring: Arc<MonitoringService>,
    reconciler: Arc<MonitoringReconciler>,
) {
    // Reconnect delay doubles per failed cycle up to one minute. Without it a
    // down or flapping server is hammered every few seconds: each cycle is a
    // fresh gRPC/TLS connect, a metrics RPC, and — since the agent is
    // unreachable — a brand-new SSH handshake (KEX + sshd fork) on the remote
    // host, which is itself a CPU spike generator.
    let mut retry_delay = std::time::Duration::from_secs(3);
    loop {
        // Stop only if the server row is actually gone. `server_ids()` excludes
        // the local server, so it must not be used here — otherwise the local
        // watcher would immediately think it was removed and exit.
        if !server_exists(server_id).await {
            tracing::info!(server_id, "server removed; stopping lifecycle watcher");
            monitoring.forget_agent(server_id);
            return;
        }
        let mut healthy = false;
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
                            healthy = true;
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
        if healthy {
            retry_delay = std::time::Duration::from_secs(3);
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        } else {
            if agent_is_reachable(&monitoring, server_id).await {
                tracing::debug!(
                    server_id,
                    "lifecycle stream dropped but agent still answers metrics; keeping server active"
                );
                reconciler.update_server_active(server_id).await;
            } else if server_ssh_is_reachable(server_id).await {
                tracing::debug!(
                    server_id,
                    "monitoring agent unavailable but SSH server is reachable; keeping server active"
                );
            } else if let Err(error) = reconciler.mark_offline(server_id).await {
                tracing::warn!(server_id, %error, "could not mark disconnected server offline");
            }
            let delay = retry_delay;
            retry_delay = (retry_delay * 2).min(std::time::Duration::from_secs(60));
            tokio::time::sleep(delay).await;
        }
    }
}

/// True when the server row still exists (checked directly, so the local
/// server — which `server_ids()` excludes — is not treated as removed).
async fn server_exists(server_id: i64) -> bool {
    let Ok(queries) = resolve::<crate::db::queries::FlashQueries>().await else {
        return true; // transient: don't tear down a healthy watcher
    };
    sqlx::query_scalar::<_, i64>("SELECT id FROM servers WHERE id=?")
        .bind(server_id)
        .fetch_optional(&queries.pool)
        .await
        .map(|row| row.is_some())
        .unwrap_or(true)
}

/// True when the agent still answers a metrics query — the surest sign the
/// host is up, even if the lifecycle stream dropped and SSH is unavailable
/// (as it always is for the local server).
async fn agent_is_reachable(monitoring: &Arc<MonitoringService>, server_id: i64) -> bool {
    tokio::time::timeout(
        std::time::Duration::from_secs(8),
        monitoring.server_history(server_id, 1),
    )
    .await
    .is_ok_and(|result| result.is_ok())
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
