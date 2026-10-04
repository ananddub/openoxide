use std::sync::Arc;

use auto_di::singleton;
use serde::{Deserialize, Serialize};

use crate::{
    db::queries::FlashQueries,
    flash_gen::monitoring_lifecycle::{
        MonitoringLifecycleAcknowledgeEventParams, MonitoringLifecycleCreateGlobalWindowParams,
        MonitoringLifecycleCreateWindowParams, MonitoringLifecycleSilenceEventParams,
        MonitoringLifecycleUpdatePolicyParams,
    },
    services::observability::monitoring::agent_auth::MonitoringAgentAuth,
    utils::{
        docker::DockerCli,
        setup::{AGENT_CONTAINER_NAME, ServerSetup, SetupConfig, agent_supports_direct_host_shell},
    },
};

pub struct MonitoringLifecycleService {
    queries: Arc<FlashQueries>,
    agent_auth: Arc<MonitoringAgentAuth>,
    mtls: Arc<crate::utils::mtls::MtlsCaStore>,
}

#[singleton]
impl MonitoringLifecycleService {
    fn new(
        queries: Arc<FlashQueries>,
        agent_auth: Arc<MonitoringAgentAuth>,
        mtls: Arc<crate::utils::mtls::MtlsCaStore>,
    ) -> Self {
        Self {
            queries,
            agent_auth,
            mtls,
        }
    }

    /// Loads (creating on first use) this server's isolated mTLS material and
    /// stamps it onto a setup config so `ensure_agent` distributes the agent's
    /// CA + cert/key to the remote host.
    async fn apply_agent_mtls(&self, server_id: i64, config: &mut SetupConfig) {
        let sans = self.agent_sans(server_id).await;
        match self.mtls.load_or_create(server_id, &sans).await {
            Ok(material) => {
                config.agent_mtls_ca_cert = Some(material.ca_cert);
                config.agent_mtls_cert = Some(material.agent_cert);
                config.agent_mtls_key = Some(material.agent_key);
            }
            Err(error) => tracing::warn!(
                server_id,
                %error,
                "per-server mTLS unavailable; agent stays on token-only auth"
            ),
        }
    }

    /// SANs the agent server cert must cover so the panel's TLS check passes:
    /// the server's own address plus the in-network agent name.
    async fn agent_sans(&self, server_id: i64) -> Vec<String> {
        let ip: Option<String> = sqlx::query_scalar("SELECT ip_address FROM servers WHERE id = ?")
            .bind(server_id)
            .fetch_optional(&self.queries.pool)
            .await
            .ok()
            .flatten();
        let mut sans = vec![
            "openoxide-agent".to_string(),
            "localhost".to_string(),
            "127.0.0.1".to_string(),
        ];
        if let Some(ip) = ip {
            let ip = ip.trim();
            if !ip.is_empty() && !sans.iter().any(|s| s == ip) {
                sans.push(ip.to_string());
            }
        }
        sans
    }

    pub async fn policy(&self, organization_id: i64) -> sqlx::Result<MonitoringPolicyDto> {
        self.queries
            .monitoring_lifecycle_ensure_policy(organization_id)
            .await?;
        self.queries
            .monitoring_lifecycle_policy(organization_id)
            .await
            .map(Into::into)
    }

    pub async fn update_policy(
        &self,
        organization_id: i64,
        input: UpdateMonitoringPolicyDto,
    ) -> sqlx::Result<MonitoringPolicyDto> {
        if !(1..=3650).contains(&input.retention_days) {
            return Err(sqlx::Error::Protocol(
                "retention_days must be between 1 and 3650".into(),
            ));
        }
        self.queries
            .monitoring_lifecycle_update_policy(&MonitoringLifecycleUpdatePolicyParams {
                organization_id,
                desired_agent_version: input.desired_agent_version.unwrap_or_default(),
                retention_days: input.retention_days,
            })
            .await
            .map(Into::into)
    }

    /// Restarts the agent out-of-band (local Docker socket / remote SSH), which
    /// works even when the gRPC link is down — e.g. right after rotating the
    /// agent's mTLS certs, when the old in-memory cert no longer verifies.
    pub async fn restart_agent_out_of_band(
        &self,
        server_id: i64,
    ) -> sqlx::Result<MonitoringActionDto> {
        let docker = self.out_of_band_docker(server_id).await?;
        docker
            .stop_agent_container()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        docker
            .start_agent_container()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(action("restart", "monitoring agent restarted"))
    }

    pub async fn restart_agent(&self, server_id: i64) -> sqlx::Result<MonitoringActionDto> {
        let executor = crate::services::compute::compose::remote::agent_executor(server_id)
            .await
            .map_err(sqlx::Error::Protocol)?;
        DockerCli::from_executor(executor)
            .containers()
            .restart(AGENT_CONTAINER_NAME)
            .run()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(action("restart", "monitoring agent restarted"))
    }

    /// Stops the agent container. The agent's own gRPC channel cannot be used
    /// for this (stopping it kills the channel mid-request), so this always
    /// goes out-of-band: the local agent is stopped through the Docker socket
    /// mounted in the panel container, remote agents through their SSH
    /// executor. `restart: unless-stopped` respects a manual stop, so the
    /// agent stays down until `start_agent` is called.
    pub async fn stop_agent(&self, server_id: i64) -> sqlx::Result<MonitoringActionDto> {
        self.out_of_band_docker(server_id)
            .await?
            .stop_agent_container()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(action("stop", "monitoring agent stopped"))
    }

    /// Starts a stopped agent container. Out-of-band by necessity: a stopped
    /// agent has no gRPC channel to receive the request through.
    pub async fn start_agent(&self, server_id: i64) -> sqlx::Result<MonitoringActionDto> {
        self.out_of_band_docker(server_id)
            .await?
            .start_agent_container()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(action("start", "monitoring agent started"))
    }

    /// Returns the local Docker socket API when `server_id` is the panel's own
    /// host, or an SSH-backed docker client for a remote server.
    async fn out_of_band_docker(&self, server_id: i64) -> sqlx::Result<OutOfBandDocker> {
        let ip: Option<String> = sqlx::query_scalar("SELECT ip_address FROM servers WHERE id = ?")
            .bind(server_id)
            .fetch_optional(&self.queries.pool)
            .await?;
        let is_local = ip
            .as_deref()
            .is_some_and(|value| {
                matches!(value, "openoxide-agent" | "localhost" | "127.0.0.1" | "openoxide-monitor")
            })
            // A missing server row can only be the local sentinel.
            || ip.is_none();
        if is_local {
            Ok(OutOfBandDocker::Local)
        } else {
            let executor = crate::services::compute::compose::remote::remote_executor(
                &self.queries.pool,
                server_id,
            )
            .await
            .map_err(sqlx::Error::Protocol)?;
            Ok(OutOfBandDocker::Remote(Box::new(
                DockerCli::from_remote_executor(executor),
            )))
        }
    }

    /// Pull the published agent image and recreate the monitor only when the
    /// running container is backed by an older image.
    ///
    /// `check_registry` selects the probe depth. A registry check asks the
    /// agent to pull the image (the only way to learn the upstream digest),
    /// which is expensive on the remote host; the caller therefore runs it on
    /// a staggered hourly schedule per server. Cheap ticks only self-inspect —
    /// an agent that answers consistently with the same identity is assumed
    /// current until its next registry check, so the five-minute loop never
    /// touches the remote Docker daemon beyond one container inspect.
    pub async fn update_agent_if_needed(
        &self,
        server_id: i64,
        check_registry: bool,
    ) -> sqlx::Result<bool> {
        let image = crate::utils::setup::agent_image();
        // Prefer the agent's own socket probe over the CLI path: an
        // up-to-date host answers entirely through its existing gRPC channel,
        // so no docker CLI process (a 28 MiB RSS charge in the agent's
        // cgroup) is ever spawned on the five-minute check. Any older agent,
        // a failed probe, or an out-of-date image falls through to the
        // out-of-band CLI repair below.
        if let Ok(monitoring) = auto_di::resolve::<
            crate::services::observability::monitoring::service::MonitoringService,
        >()
        .await
        {
            let probed_image = if check_registry { image } else { "" };
            if let Ok(info) = monitoring.agent_update_probe(server_id, probed_image).await
                && info.self_running
                && info.self_server_id == server_id
                && !info.self_image_id.is_empty()
            {
                if !check_registry {
                    return Ok(false);
                }
                if !info.latest_image_id.is_empty() && info.self_image_id == info.latest_image_id {
                    tracing::debug!(server_id, "agent update probe confirms latest image");
                    return Ok(false);
                }
            }
        }

        let executor = crate::services::compute::compose::remote::agent_executor(server_id)
            .await
            .map_err(sqlx::Error::Protocol)?;
        let docker = DockerCli::from_executor(executor);

        let inspected = match docker.container(AGENT_CONTAINER_NAME).inspect().await {
            Ok(container) => Some(container),
            Err(error) => {
                // If the Agent transport itself is unavailable (or is too old to
                // answer the Docker operation), use SSH only as bootstrap recovery.
                tracing::warn!(server_id, error = %error, "Agent inspection failed; falling back to SSH bootstrap repair");
                self.reinstall_agent(server_id).await?;
                return Ok(true);
            }
        };
        let mut running_image_id = inspected.as_ref().map(|container| container.image.clone());
        let configured_server_id = inspected.as_ref().and_then(|container| {
            container
                .config
                .env
                .iter()
                .find_map(|value| value.strip_prefix("SERVER_ID=")?.parse::<i64>().ok())
        });
        if configured_server_id != Some(server_id) {
            let _ = docker
                .container(AGENT_CONTAINER_NAME)
                .remove()
                .force()
                .run()
                .await;
            running_image_id = None;
        }
        if let Some(container) = inspected.as_ref()
            && configured_server_id == Some(server_id)
            && !container.state.running
        {
            docker
                .container(AGENT_CONTAINER_NAME)
                .start()
                .run()
                .await
                .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
            return Ok(true);
        }

        docker
            .images()
            .pull(image)
            .pull()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;

        let latest_image_id = docker
            .images()
            .inspect(image)
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?
            .id;

        // Matching image IDs are only "current" when the container was also
        // created by a spec that image needs: an agent running the newest image
        // under an older spec still has to be recreated for the direct host
        // shell, which reads its namespaces through /host.
        let spec_ready = inspected.as_ref().is_some_and(|container| {
            agent_supports_direct_host_shell(&container.mounts, &container.host_config)
        });
        if running_image_id.as_deref() == Some(latest_image_id.as_str()) && spec_ready {
            return Ok(false);
        }

        if self.agent_auth.organization_id(server_id).await?.is_some() {
            self.reinstall_agent(server_id).await?;
        } else {
            // Older servers may have an agent container but no row in the
            // monitoring_agents table yet. The panel-wide metrics token is a
            // valid fallback, so repair the container without requiring a
            // pre-existing registration row.
            let executor = crate::services::compute::compose::remote::remote_executor(
                &self.queries.pool,
                server_id,
            )
            .await
            .map_err(sqlx::Error::Protocol)?;
            let mut config = SetupConfig {
                agent_server_id: Some(server_id),
                agent_token: Some(self.agent_auth_token_fallback()),
                agent_panel_url: Some(
                    std::env::var("OPENOXIDE_SERVER_URL")
                        .or_else(|_| std::env::var("RUSTPLOY_SERVER_URL"))
                        .unwrap_or_else(|_| "http://127.0.0.1:4000".into()),
                ),
                ..Default::default()
            };
            self.apply_agent_mtls(server_id, &mut config).await;
            ServerSetup::new_remote(executor, config)
                .ensure_agent()
                .await
                .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        }
        Ok(true)
    }

    /// Keep the panel's own host on the exact same monitor lifecycle as an
    /// SSH-managed host. Local monitoring gets a real server identity so the
    /// same gRPC agent and lifecycle watcher are used as remote hosts.
    pub async fn ensure_local_registration(&self) -> sqlx::Result<i64> {
        // Normalize databases created before the Agent rename. Those rows
        // still point at the removed `openoxide-monitor` DNS name, which
        // makes the panel call an old/unavailable gRPC endpoint.
        sqlx::query(
            "UPDATE servers SET ip_address='openoxide-agent', app_name='localhost'
             WHERE ip_address IN ('openoxide-monitor', '127.0.0.1', 'localhost')
               AND (app_name='localhost' OR name LIKE 'Localhost%')",
        )
        .execute(&self.queries.pool)
        .await?;

        // Consolidate duplicate local server rows (e.g. one Online + one
        // Offline "Localhost (Main server)"). Keep the lowest id, move any
        // resources that reference the extras onto it, then delete the extras.
        let local_ids: Vec<i64> = sqlx::query_scalar(
            "SELECT id FROM servers WHERE (ip_address IN ('127.0.0.1','localhost','openoxide-agent') OR app_name='localhost') ORDER BY id",
        )
        .fetch_all(&self.queries.pool)
        .await?;
        if local_ids.len() > 1 {
            let keep = local_ids[0];
            let extras = &local_ids[1..];
            for &extra in extras {
                // Move server-scoped resources onto the surviving row.
                for stmt in [
                    "UPDATE services SET server_id=? WHERE server_id=?",
                    "UPDATE service_application SET build_server_id=? WHERE build_server_id=?",
                    "UPDATE certificates SET server_id=? WHERE server_id=?",
                    "UPDATE schedules SET server_id=? WHERE server_id=?",
                    "UPDATE deployments SET server_id=? WHERE server_id=?",
                    "UPDATE database_networks SET server_id=? WHERE server_id=?",
                ] {
                    sqlx::query(stmt)
                        .bind(keep)
                        .bind(extra)
                        .execute(&self.queries.pool)
                        .await?;
                }
                // PK-on-server_id tables can't be re-pointed; drop the extra's
                // rows so the surviving row keeps its own.
                for stmt in [
                    "DELETE FROM server_management WHERE server_id=?",
                    "DELETE FROM monitoring_agents WHERE server_id=?",
                    "DELETE FROM server_private_networks WHERE server_id=?",
                    "DELETE FROM server_cleanup_executions WHERE server_id=?",
                    "DELETE FROM servers WHERE id=?",
                ] {
                    sqlx::query(stmt)
                        .bind(extra)
                        .execute(&self.queries.pool)
                        .await?;
                }
            }
            tracing::info!(
                keep,
                removed = extras.len(),
                "consolidated duplicate local server rows"
            );
        }
        let configured_id = std::env::var("OPENOXIDE_LOCAL_SERVER_ID")
            .or_else(|_| std::env::var("MONITORING_SERVER_ID"))
            .ok()
            .and_then(|value| value.parse::<i64>().ok())
            .filter(|value| *value > 0);
        let configured_server: Option<(i64,)> = if let Some(id) = configured_id {
            sqlx::query_as("SELECT id FROM servers WHERE id=?")
                .bind(id)
                .fetch_optional(&self.queries.pool)
                .await?
        } else {
            None
        };
        let server_id = if let Some((id,)) = configured_server {
            id
        } else {
            let existing: Option<(i64,)> = sqlx::query_as(
                "SELECT id FROM servers WHERE (ip_address IN ('127.0.0.1','localhost','openoxide-agent') OR app_name='localhost') LIMIT 1",
            )
            .fetch_optional(&self.queries.pool)
            .await?;
            if let Some((id,)) = existing {
                id
            } else {
                let result = sqlx::query(
                    "INSERT INTO servers (name, description, ip_address, port, username, app_name, server_status, server_type, enable_docker_cleanup, command, metrics_config) VALUES ('Localhost (Main server)', 'Panel host monitoring agent', '127.0.0.1', 22, 'root', 'localhost', 'ACTIVE', 'DEPLOY', 0, '', '{}')",
                )
                .execute(&self.queries.pool)
                .await?;
                result.last_insert_rowid()
            }
        };

        // Register before background watchers start. Otherwise the watcher
        // cannot obtain its query token, marks the host offline, and leaves
        // resource statuses stuck at their previous database value.
        let organization_id: Option<i64> =
            sqlx::query_scalar("SELECT id FROM organization ORDER BY id LIMIT 1")
                .fetch_optional(&self.queries.pool)
                .await?;
        if let (Some(organization_id), Ok(token)) =
            (organization_id, std::env::var("METRICS_TOKEN"))
            && !token.is_empty()
        {
            self.agent_auth
                .register_token(server_id, organization_id, &token)
                .await?;
            tracing::debug!(server_id, "local monitoring agent credentials registered");
        }

        Ok(server_id)
    }

    pub async fn update_local_agent_if_needed(&self) -> sqlx::Result<bool> {
        let server_id = self.ensure_local_registration().await?;

        let executor = crate::services::compute::compose::remote::agent_executor(server_id)
            .await
            .map_err(sqlx::Error::Protocol)?;
        let docker = DockerCli::from_executor(executor);
        let image = crate::utils::setup::agent_image();
        let inspected = docker.container(AGENT_CONTAINER_NAME).inspect().await.ok();
        // The local Agent is managed by docker-compose.agent.yml in the
        // development/self-hosted setup. Let Compose own its image and
        // lifecycle; the panel must not pull a registry image or recreate it.
        if inspected.as_ref().is_some_and(|container| {
            container.config.labels.iter().any(|(key, value)| {
                key == "com.docker.compose.service" && value == "openoxide-agent"
            })
        }) {
            return Ok(false);
        }
        let configured_server_id = inspected.as_ref().and_then(|container| {
            container
                .config
                .env
                .iter()
                .find_map(|value| value.strip_prefix("SERVER_ID=")?.parse::<i64>().ok())
        });
        let mut running_image_id = inspected.as_ref().map(|container| container.image.clone());
        if configured_server_id != Some(server_id) {
            if inspected.is_some() {
                let _ = docker
                    .container(AGENT_CONTAINER_NAME)
                    .remove()
                    .force()
                    .run()
                    .await;
            }
            running_image_id = None;
        } else if let Some(container) = inspected.as_ref()
            && !container.state.running
        {
            docker
                .container(AGENT_CONTAINER_NAME)
                .start()
                .run()
                .await
                .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
            return Ok(true);
        }
        docker.images().pull(image).pull().await.map_err(|error| {
            sqlx::Error::Protocol(format!("local monitor image pull failed: {error}"))
        })?;
        let latest_image_id = docker
            .images()
            .inspect(image)
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?
            .id;
        let spec_ready = inspected.as_ref().is_some_and(|container| {
            agent_supports_direct_host_shell(&container.mounts, &container.host_config)
        });
        if running_image_id.as_deref() == Some(latest_image_id.as_str()) && spec_ready {
            return Ok(false);
        }

        let config = SetupConfig {
            agent_server_id: Some(server_id),
            agent_token: Some(self.agent_auth_token_fallback()),
            agent_panel_url: Some(
                std::env::var("OPENOXIDE_SERVER_URL")
                    .or_else(|_| std::env::var("RUSTPLOY_SERVER_URL"))
                    .unwrap_or_else(|_| "http://openoxide:4000".into()),
            ),
            ..Default::default()
        };
        ServerSetup::new_local(config)
            .ensure_agent()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(true)
    }

    fn agent_auth_token_fallback(&self) -> String {
        std::env::var("METRICS_TOKEN").unwrap_or_default()
    }

    pub async fn reinstall_agent(&self, server_id: i64) -> sqlx::Result<MonitoringActionDto> {
        // Legacy installations can have a server/resource but no monitoring_agents
        // row yet. Resolve the owning organization from linked resources so an
        // outdated Agent can still bootstrap itself.
        let organization_id = match self.agent_auth.organization_id(server_id).await? {
            Some(id) => id,
            None => {
                // Prefer the server's own org; fall back to a linked resource's
                // project org. A server with neither cannot be attributed.
                let from_server: Option<i64> =
                    sqlx::query_scalar("SELECT organization_id FROM servers WHERE id = ?")
                        .bind(server_id)
                        .fetch_optional(&self.queries.pool)
                        .await?;
                match from_server.filter(|id| *id > 0) {
                    Some(id) => id,
                    None => sqlx::query_scalar(
                        "SELECT p.organization_id
                         FROM services s
                         JOIN environments e ON e.id = s.environment_id
                         JOIN projects p ON p.id = e.project_id
                         WHERE s.server_id = ?
                         LIMIT 1",
                    )
                    .bind(server_id)
                    .fetch_optional(&self.queries.pool)
                    .await?
                    .ok_or(sqlx::Error::RowNotFound)?,
                }
            }
        };
        let policy = self.policy(organization_id).await?;
        let token = if self.agent_auth.organization_id(server_id).await?.is_some() {
            self.agent_auth.rotate(server_id, organization_id).await?
        } else {
            let token = self.agent_auth_token_fallback();
            if token.is_empty() {
                return Err(sqlx::Error::Protocol(
                    "cannot reinstall Agent: METRICS_TOKEN is not configured".into(),
                ));
            }
            self.agent_auth
                .register_token(server_id, organization_id, &token)
                .await?;
            token
        };
        let executor = crate::services::compute::compose::remote::remote_executor(
            &self.queries.pool,
            server_id,
        )
        .await
        .map_err(sqlx::Error::Protocol)?;
        let docker = DockerCli::from_remote_executor(executor.clone());
        let _ = docker
            .containers()
            .rm(AGENT_CONTAINER_NAME)
            .force()
            .run()
            .await;
        let mut config = SetupConfig {
            agent_server_id: Some(server_id),
            agent_token: Some(token),
            agent_retention_days: policy.retention_days,
            agent_panel_url: Some(
                std::env::var("OPENOXIDE_SERVER_URL")
                    .or_else(|_| std::env::var("RUSTPLOY_SERVER_URL"))
                    .unwrap_or_else(|_| "http://127.0.0.1:4000".into()),
            ),
            ..Default::default()
        };
        self.apply_agent_mtls(server_id, &mut config).await;
        ServerSetup::new_remote(executor, config)
            .ensure_agent()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(action(
            "reinstall",
            &format!(
                "monitoring agent reinstalled; configured retention policy is {} days",
                policy.retention_days
            ),
        ))
    }

    pub async fn acknowledge(
        &self,
        event_id: i64,
        organization_id: i64,
        user_id: i64,
    ) -> sqlx::Result<bool> {
        match self
            .queries
            .monitoring_lifecycle_acknowledge_event(&MonitoringLifecycleAcknowledgeEventParams {
                acknowledged_by: user_id,
                id: event_id,
                organization_id,
            })
            .await
        {
            Ok(_) => Ok(true),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub async fn silence(
        &self,
        event_id: i64,
        organization_id: i64,
        until: i64,
    ) -> sqlx::Result<bool> {
        if until <= chrono::Utc::now().timestamp() {
            return Err(sqlx::Error::Protocol(
                "silence expiry must be in the future".into(),
            ));
        }
        match self
            .queries
            .monitoring_lifecycle_silence_event(&MonitoringLifecycleSilenceEventParams {
                silenced_until: until,
                id: event_id,
                organization_id,
            })
            .await
        {
            Ok(_) => Ok(true),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub async fn create_window(
        &self,
        organization_id: i64,
        server_id: Option<i64>,
        starts_at: i64,
        ends_at: i64,
        reason: &str,
    ) -> sqlx::Result<i64> {
        if ends_at <= starts_at {
            return Err(sqlx::Error::Protocol(
                "ends_at must follow starts_at".into(),
            ));
        }
        match server_id {
            Some(server_id) => {
                self.queries
                    .monitoring_lifecycle_create_window(&MonitoringLifecycleCreateWindowParams {
                        organization_id,
                        server_id,
                        starts_at,
                        ends_at,
                        reason: reason.to_owned(),
                    })
                    .await
            }
            None => {
                self.queries
                    .monitoring_lifecycle_create_global_window(
                        &MonitoringLifecycleCreateGlobalWindowParams {
                            organization_id,
                            starts_at,
                            ends_at,
                            reason: reason.to_owned(),
                        },
                    )
                    .await
            }
        }
    }

    pub async fn windows(&self, organization_id: i64) -> sqlx::Result<Vec<MaintenanceWindowDto>> {
        self.queries
            .monitoring_lifecycle_list_windows(organization_id)
            .await
            .map(|rows| rows.into_iter().map(Into::into).collect())
    }

    pub async fn delete_window(&self, id: i64, organization_id: i64) -> sqlx::Result<bool> {
        match self
            .queries
            .monitoring_lifecycle_delete_window(id, organization_id)
            .await
        {
            Ok(_) => Ok(true),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

fn action(action: &str, message: &str) -> MonitoringActionDto {
    MonitoringActionDto {
        action: action.into(),
        success: true,
        message: message.into(),
    }
}

/// Docker access that does not depend on the agent's gRPC channel — used for
/// agent lifecycle recovery when the agent itself is down or being stopped.
enum OutOfBandDocker {
    /// Local host: Docker Engine API over the socket mounted into the panel
    /// container. Works even with no agent running at all.
    Local,
    /// Remote host: docker CLI over the SSH executor.
    Remote(Box<DockerCli>),
}

impl OutOfBandDocker {
    async fn stop_agent_container(&self) -> Result<(), String> {
        match self {
            Self::Local => {
                let api = local_docker_api();
                match api
                    .request(
                        "POST",
                        &format!("/containers/{AGENT_CONTAINER_NAME}/stop"),
                        None,
                    )
                    .await
                {
                    // 204 on success; "already stopped" is idempotent success.
                    Ok(_) => Ok(()),
                    Err(error) if error.to_string().contains("already stopped") => Ok(()),
                    Err(error) => Err(format!("failed to stop agent container: {error}")),
                }
            }
            Self::Remote(docker) => {
                // docker CLI exits non-zero when the container is already
                // stopped, so inspect first for an idempotent result.
                if let Ok(inspected) = docker.container(AGENT_CONTAINER_NAME).inspect().await
                    && !inspected.state.running
                {
                    return Ok(());
                }
                docker
                    .containers()
                    .stop(AGENT_CONTAINER_NAME)
                    .run()
                    .await
                    .map(|_| ())
                    .map_err(|error| format!("failed to stop agent container: {error}"))
            }
        }
    }

    async fn start_agent_container(&self) -> Result<(), String> {
        match self {
            Self::Local => {
                let api = local_docker_api();
                match api
                    .request(
                        "POST",
                        &format!("/containers/{AGENT_CONTAINER_NAME}/start"),
                        None,
                    )
                    .await
                {
                    Ok(_) => Ok(()),
                    // "already started" is idempotent success.
                    Err(error) if error.to_string().contains("already started") => Ok(()),
                    Err(error) => {
                        let error = error.to_string();
                        if error.contains("No such container") || error.contains("404") {
                            Err("agent container not found; use ReinstallAgent".to_string())
                        } else {
                            Err(format!("failed to start agent container: {error}"))
                        }
                    }
                }
            }
            Self::Remote(docker) => {
                if let Ok(inspected) = docker.container(AGENT_CONTAINER_NAME).inspect().await
                    && inspected.state.running
                {
                    return Ok(());
                }
                docker
                    .containers()
                    .start(AGENT_CONTAINER_NAME)
                    .run()
                    .await
                    .map(|_| ())
                    .map_err(|error| {
                        if error.to_string().contains("No such container") {
                            "agent container not found; use ReinstallAgent".to_string()
                        } else {
                            format!("failed to start agent container: {error}")
                        }
                    })
            }
        }
    }
}

fn local_docker_api() -> crate::utils::docker::DockerApi {
    crate::utils::docker::DockerApi::new(
        std::env::var("SOCKET_PATH").unwrap_or_else(|_| "/var/run/docker.sock".into()),
    )
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MonitoringPolicyDto {
    pub organization_id: i64,
    pub desired_agent_version: Option<String>,
    pub retention_days: i64,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateMonitoringPolicyDto {
    pub desired_agent_version: Option<String>,
    pub retention_days: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MonitoringActionDto {
    pub action: String,
    pub success: bool,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaintenanceWindowDto {
    pub id: i64,
    pub organization_id: i64,
    pub server_id: Option<i64>,
    pub starts_at: i64,
    pub ends_at: i64,
    pub reason: String,
    pub created_at: i64,
}

impl From<crate::flash_gen::models::MonitoringPolicy> for MonitoringPolicyDto {
    fn from(value: crate::flash_gen::models::MonitoringPolicy) -> Self {
        Self {
            organization_id: value.organization_id,
            // Flash binds the nullable column as non-Option; "" encodes NULL.
            desired_agent_version: value.desired_agent_version.filter(|v| !v.is_empty()),
            retention_days: value.retention_days,
            updated_at: value.updated_at,
        }
    }
}

impl From<crate::flash_gen::models::MonitoringMaintenanceWindows> for MaintenanceWindowDto {
    fn from(value: crate::flash_gen::models::MonitoringMaintenanceWindows) -> Self {
        Self {
            id: value.id,
            organization_id: value.organization_id,
            server_id: value.server_id,
            starts_at: value.starts_at,
            ends_at: value.ends_at,
            reason: value.reason,
            created_at: value.created_at,
        }
    }
}
