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
            .restart("rustploy-monitor")
            .run()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(action("restart", "monitoring agent restarted"))
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
            .rm("rustploy-monitor")
            .force()
            .run()
            .await;
        let mut config = SetupConfig::default();
        config.monitoring_server_id = Some(server_id);
        config.monitoring_token = Some(token);
        config.monitoring_retention_days = policy.retention_days;
        config.monitoring_panel_url = Some(
            std::env::var("RUSTPLOY_SERVER_URL").unwrap_or_else(|_| "http://127.0.0.1:4000".into()),
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
