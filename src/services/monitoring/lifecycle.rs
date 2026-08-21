use std::sync::Arc;

use auto_di::singleton;

use crate::{
    api::dto::monitoring::{
        MaintenanceWindowDto, MonitoringActionDto, MonitoringPolicyDto, UpdateMonitoringPolicyDto,
    },
    repository::MonitoringLifecycleRepository,
    services::monitoring::agent_auth::MonitoringAgentAuth,
    utils::{
        docker::DockerCli,
        setup::{ServerSetup, SetupConfig},
    },
};

pub struct MonitoringLifecycleService {
    db: Arc<sqlx::SqlitePool>,
    repository: Arc<MonitoringLifecycleRepository>,
    agent_auth: Arc<MonitoringAgentAuth>,
}

#[singleton]
impl MonitoringLifecycleService {
    fn new(
        db: Arc<sqlx::SqlitePool>,
        repository: Arc<MonitoringLifecycleRepository>,
        agent_auth: Arc<MonitoringAgentAuth>,
    ) -> Self {
        Self {
            db,
            repository,
            agent_auth,
        }
    }

    pub async fn policy(&self, organization_id: i64) -> sqlx::Result<MonitoringPolicyDto> {
        self.repository
            .policy(organization_id)
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
        self.repository
            .update_policy(
                organization_id,
                input.desired_agent_version.as_deref(),
                input.retention_days,
            )
            .await
            .map(Into::into)
    }

    pub async fn restart_agent(&self, server_id: i64) -> sqlx::Result<MonitoringActionDto> {
        let executor = crate::services::compose::remote::remote_executor(&self.db, server_id)
            .await
            .map_err(sqlx::Error::Protocol)?;
        DockerCli::from_remote_executor(executor)
            .containers()
            .restart("openoxide-monitor")
            .run()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(action("restart", "monitoring agent restarted"))
    }

    /// Pull the published agent image and recreate the monitor only when the
    /// running container is backed by an older image.
    pub async fn update_agent_if_needed(&self, server_id: i64) -> sqlx::Result<bool> {
        let executor = crate::services::compose::remote::remote_executor(&self.db, server_id)
            .await
            .map_err(sqlx::Error::Protocol)?;
        let docker = DockerCli::from_remote_executor(executor);
        let image = crate::utils::setup::monitoring_image();

        let inspected = docker.container("openoxide-monitor").inspect().await.ok();
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
                .container("openoxide-monitor")
                .remove()
                .force()
                .run()
                .await;
            running_image_id = None;
        }
        if let Some(container) = inspected.as_ref() {
            if configured_server_id == Some(server_id) && !container.state.running {
                docker
                    .container("openoxide-monitor")
                    .start()
                    .run()
                    .await
                    .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
                return Ok(true);
            }
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

        if running_image_id.as_deref() == Some(latest_image_id.as_str()) {
            return Ok(false);
        }

        if self.agent_auth.organization_id(server_id).await?.is_some() {
            self.reinstall_agent(server_id).await?;
        } else {
            // Older servers may have an agent container but no row in the
            // monitoring_agents table yet. The panel-wide metrics token is a
            // valid fallback, so repair the container without requiring a
            // pre-existing registration row.
            let executor = crate::services::compose::remote::remote_executor(&self.db, server_id)
                .await
                .map_err(sqlx::Error::Protocol)?;
            let mut config = SetupConfig::default();
            config.monitoring_server_id = Some(server_id);
            config.monitoring_token = Some(self.agent_auth_token_fallback());
            config.monitoring_panel_url = Some(
                std::env::var("OPENOXIDE_SERVER_URL")
                    .or_else(|_| std::env::var("RUSTPLOY_SERVER_URL"))
                    .unwrap_or_else(|_| "http://127.0.0.1:4000".into()),
            );
            ServerSetup::new_remote(executor, config)
                .ensure_monitoring()
                .await
                .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        }
        Ok(true)
    }

    /// Keep the panel's own host on the exact same monitor lifecycle as an
    /// SSH-managed host. Local monitoring gets a real server identity so the
    /// same gRPC agent and lifecycle watcher are used as remote hosts.
    pub async fn ensure_local_registration(&self) -> sqlx::Result<i64> {
        let configured_id = std::env::var("OPENOXIDE_LOCAL_SERVER_ID")
            .or_else(|_| std::env::var("MONITORING_SERVER_ID"))
            .ok()
            .and_then(|value| value.parse::<i64>().ok())
            .filter(|value| *value > 0);
        let configured_server: Option<(i64,)> = if let Some(id) = configured_id {
            sqlx::query_as("SELECT id FROM servers WHERE id=?")
                .bind(id)
                .fetch_optional(self.db.as_ref())
                .await?
        } else {
            None
        };
        let server_id = if let Some((id,)) = configured_server {
            id
        } else {
            let existing: Option<(i64,)> = sqlx::query_as(
                "SELECT id FROM servers WHERE (ip_address IN ('127.0.0.1','localhost','openoxide-monitor') OR app_name='localhost') LIMIT 1",
            )
            .fetch_optional(self.db.as_ref())
            .await?;
            if let Some((id,)) = existing {
                id
            } else {
                let result = sqlx::query(
                    "INSERT INTO servers (name, description, ip_address, port, username, app_name, server_status, server_type, command, metrics_config) VALUES ('Localhost (Main server)', 'Panel host monitoring agent', '127.0.0.1', 22, 'root', 'localhost', 'ACTIVE', 'DEPLOY', '', '{}')",
                )
                .execute(self.db.as_ref())
                .await?;
                result.last_insert_rowid()
            }
        };

        // Register before background watchers start. Otherwise the watcher
        // cannot obtain its query token, marks the host offline, and leaves
        // resource statuses stuck at their previous database value.
        let organization_id: Option<i64> =
            sqlx::query_scalar("SELECT id FROM organization ORDER BY id LIMIT 1")
                .fetch_optional(self.db.as_ref())
                .await?;
        if let (Some(organization_id), Ok(token)) =
            (organization_id, std::env::var("METRICS_TOKEN"))
        {
            if !token.is_empty() {
                self.agent_auth
                    .register_token(server_id, organization_id, &token)
                    .await?;
                tracing::debug!(server_id, "local monitoring agent credentials registered");
            }
        }

        Ok(server_id)
    }

    pub async fn update_local_agent_if_needed(&self) -> sqlx::Result<bool> {
        let server_id = self.ensure_local_registration().await?;

        let docker = DockerCli::new_local();
        let image = crate::utils::setup::monitoring_image();
        let legacy = "openoxide_monitor";
        if docker
            .container("openoxide-monitor")
            .inspect()
            .await
            .is_err()
            && docker.container(legacy).inspect().await.is_ok()
        {
            let _ = docker.container(legacy).remove().force().run().await;
        }
        let inspected = docker.container("openoxide-monitor").inspect().await.ok();
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
                    .container("openoxide-monitor")
                    .remove()
                    .force()
                    .run()
                    .await;
            }
            running_image_id = None;
        } else if let Some(container) = inspected.as_ref() {
            if !container.state.running {
                docker
                    .container("openoxide-monitor")
                    .start()
                    .run()
                    .await
                    .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
                return Ok(true);
            }
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
        if running_image_id.as_deref() == Some(latest_image_id.as_str()) {
            return Ok(false);
        }

        let mut config = SetupConfig::default();
        config.monitoring_server_id = Some(server_id);
        config.monitoring_token = Some(self.agent_auth_token_fallback());
        config.monitoring_panel_url = Some(
            std::env::var("OPENOXIDE_SERVER_URL")
                .or_else(|_| std::env::var("RUSTPLOY_SERVER_URL"))
                .unwrap_or_else(|_| "http://openoxide:4000".into()),
        );
        ServerSetup::new_local(config)
            .ensure_monitoring()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(true)
    }

    fn agent_auth_token_fallback(&self) -> String {
        std::env::var("METRICS_TOKEN").unwrap_or_default()
    }

    pub async fn reinstall_agent(&self, server_id: i64) -> sqlx::Result<MonitoringActionDto> {
        let organization_id = self
            .agent_auth
            .organization_id(server_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let policy = self.repository.policy(organization_id).await?;
        let token = self.agent_auth.rotate(server_id, organization_id).await?;
        let executor = crate::services::compose::remote::remote_executor(&self.db, server_id)
            .await
            .map_err(sqlx::Error::Protocol)?;
        let docker = DockerCli::from_remote_executor(executor.clone());
        let _ = docker
            .containers()
            .rm("openoxide-monitor")
            .force()
            .run()
            .await;
        let mut config = SetupConfig::default();
        config.monitoring_server_id = Some(server_id);
        config.monitoring_token = Some(token);
        config.monitoring_retention_days = policy.retention_days;
        config.monitoring_panel_url = Some(
            std::env::var("OPENOXIDE_SERVER_URL")
                .or_else(|_| std::env::var("RUSTPLOY_SERVER_URL"))
                .unwrap_or_else(|_| "http://127.0.0.1:4000".into()),
        );
        ServerSetup::new_remote(executor, config)
            .ensure_monitoring()
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
        self.repository
            .acknowledge_event(event_id, organization_id, user_id)
            .await
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
        self.repository
            .silence_event(event_id, organization_id, until)
            .await
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
        self.repository
            .create_window(organization_id, server_id, starts_at, ends_at, reason)
            .await
    }

    pub async fn windows(&self, organization_id: i64) -> sqlx::Result<Vec<MaintenanceWindowDto>> {
        self.repository
            .list_windows(organization_id)
            .await
            .map(|rows| rows.into_iter().map(Into::into).collect())
    }

    pub async fn delete_window(&self, id: i64, organization_id: i64) -> sqlx::Result<bool> {
        self.repository.delete_window(id, organization_id).await
    }
}

fn action(action: &str, message: &str) -> MonitoringActionDto {
    MonitoringActionDto {
        action: action.into(),
        success: true,
        message: message.into(),
    }
}

impl From<crate::db::repository::monitoring_lifecycle::MonitoringPolicy> for MonitoringPolicyDto {
    fn from(value: crate::db::repository::monitoring_lifecycle::MonitoringPolicy) -> Self {
        Self {
            organization_id: value.organization_id,
            desired_agent_version: value.desired_agent_version,
            retention_days: value.retention_days,
            updated_at: value.updated_at,
        }
    }
}

impl From<crate::db::repository::monitoring_lifecycle::MaintenanceWindow> for MaintenanceWindowDto {
    fn from(value: crate::db::repository::monitoring_lifecycle::MaintenanceWindow) -> Self {
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
