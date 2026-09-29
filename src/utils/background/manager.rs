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
        Self::ensure_mtls_material().await;

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

    /// Prepares the local agent's per-server mTLS material under
    /// `OPENOXIDE_MTLS_DIR/server-{id}/`. Each server owns an isolated CA
    /// (generated once, then reused) so no trust is shared between servers.
    /// Remote servers are provisioned during their own setup flow.
    async fn ensure_mtls_material() {
        let base_dir = std::env::var("OPENOXIDE_MTLS_DIR")
            .unwrap_or_else(|_| "/app/data/mtls".to_string());
        let sans = vec![
            "openoxide-agent".to_string(),
            "localhost".to_string(),
            "127.0.0.1".to_string(),
        ];
        let store = match resolve::<crate::utils::mtls::MtlsCaStore>().await {
            Ok(store) => store,
            Err(error) => {
                tracing::warn!(%error, "mTLS store unavailable; agent link stays on token auth");
                return;
            }
        };
        // Provision for the actual local server row so the id matches the one
        // the gRPC client uses. Falls back to SERVER_ID/1 if registration is
        // not ready yet.
        let server_id = match resolve::<MonitoringLifecycleService>().await {
            Ok(lifecycle) => lifecycle.ensure_local_registration().await.unwrap_or_else(|_| {
                std::env::var("SERVER_ID")
                    .ok()
                    .and_then(|v| v.trim().parse().ok())
                    .unwrap_or(1)
            }),
            Err(_) => std::env::var("SERVER_ID")
                .ok()
                .and_then(|v| v.trim().parse().ok())
                .unwrap_or(1),
        };
        let material = match store.load_or_create(server_id, &sans).await {
            Ok(material) => material,
            Err(error) => {
                tracing::warn!(%error, server_id, "could not prepare mTLS material; agent link stays on token auth");
                return;
            }
        };
        let base = std::path::Path::new(&base_dir);
        if let Err(error) = store.write_files(server_id, base, &material) {
            tracing::warn!(%error, server_id, "could not write per-server mTLS files");
            return;
        }
        // The local agent (docker-compose) mounts the mtls dir root read-only,
        // so also drop the local agent's own cert set there for it to read.
        let certs_changed = match write_local_agent_root(base, &material) {
            Ok(changed) => changed,
            Err(error) => {
                tracing::warn!(%error, "could not write local agent mTLS root copy");
                false
            }
        };
        tracing::info!(server_id, dir = %base_dir, "local agent per-server mTLS material ready");
        // The agent loads its certs once at boot. If we just rewrote them (new
        // material or a rotation), restart the agent so it serves the current
        // cert — otherwise the panel would reject it with BadSignature until a
        // manual restart. Skipped on first-ever boot when nothing pre-existed.
        if certs_changed {
            if let Ok(lifecycle) = resolve::<MonitoringLifecycleService>().await {
                match lifecycle.restart_agent_out_of_band(server_id).await {
                    Ok(_) => tracing::info!(server_id, "restarted local agent to load rotated mTLS certs"),
                    Err(error) => tracing::warn!(%error, server_id, "could not restart local agent after cert change"),
                }
            }
        }
    }

    async fn dispatch_startup_notification() {
        let Ok(notifications) = resolve::<NotificationService>().await else {
            tracing::warn!("startup notification service unavailable");
            return;
        };
        super::notification::dispatch_startup(notifications);
    }
}

/// Writes the local agent's own cert set at the mtls dir root. The bundled
/// docker-compose agent mounts this root read-only and reads `agent.crt`/
/// `agent.key`/`ca.crt` directly, so its files live here as well as under
/// `server-{id}/`. Returns true if any file's content changed (or was newly
/// created), so the caller can restart the agent to reload it.
fn write_local_agent_root(
    base: &std::path::Path,
    material: &crate::utils::mtls::ServerMtls,
) -> Result<bool, String> {
    std::fs::create_dir_all(base).map_err(|e| format!("create {}: {e}", base.display()))?;
    let files = [
        ("ca.crt", &material.ca_cert),
        ("agent.crt", &material.agent_cert),
        ("agent.key", &material.agent_key),
    ];
    let mut changed = false;
    for (name, contents) in files {
        let path = base.join(name);
        let current = std::fs::read(&path).ok();
        if current.as_deref() != Some(contents.as_bytes()) {
            changed = true;
            std::fs::write(&path, contents).map_err(|e| format!("write {}: {e}", path.display()))?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
            }
        }
    }
    Ok(changed)
}
