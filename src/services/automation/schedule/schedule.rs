use std::sync::Arc;

use auto_di::singleton;
use os::string_enum;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::queries::FlashQueries;
use crate::flash_gen::backup_executions::BackupExecutionsStartParams;
use crate::flash_gen::models::{
    Backups, ScheduleExecutions, ScheduleRuntimePolicies, Schedules, VolumeBackups,
};
use crate::flash_gen::schedule_runtime::{
    ScheduleRuntimeAcquireParams, ScheduleRuntimeBeginParams, ScheduleRuntimeFinishParams,
    ScheduleRuntimeUpdatePolicyParams,
};
use crate::flash_gen::schedules::{
    SchedulesCreateAndReturnParams, SchedulesUpdateAndReturnParams,
};
use crate::services::compute::compose::ComposeType;
use crate::services::automation::schedule::types::{
    ConcurrencyPolicy, MissedRunPolicy, ScheduleAction, ScheduleExecutionStatus,
    ScheduleTriggerKind, ScheduleType, ShellType,
};
use crate::utils::docker::query::ContainerFilter;
use crate::{
    services::{
        compute::application::{ApplicationOperation, ApplicationService},
        compute::compose::{ComposeOperation, ComposeService},
        integrations::notification::{
            NotificationLevel, NotificationMessage, NotificationScope, NotificationService,
            NotificationTrigger,
        },
    },
    utils::docker::DockerCli,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BackupsDatabaseTypeEnum {
    Postgres,
    Mariadb,
    Mysql,
    Mongo,
    Redis,
    Libsql,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VolumeBackupsServiceTypeEnum {
    Application,
    Compose,
    Postgres,
    Mysql,
    Mariadb,
    Mongo,
    Redis,
    Libsql,
}

#[derive(Debug, Deserialize)]
pub struct CreateScheduleDto {
    pub name: String,
    pub description: Option<String>,
    pub cron_expression: String,
    pub app_name: Option<String>,
    pub service_name: Option<String>,
    pub shell_type: Option<String>,
    pub schedule_type: Option<String>,
    pub schedule_action: Option<String>,
    pub command: String,
    pub script: Option<String>,
    pub timezone: Option<String>,
    #[serde(default)]
    pub enabled: Option<i64>,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub server_id: Option<i64>,
    pub organization_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PatchScheduleDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub cron_expression: Option<String>,
    pub app_name: Option<String>,
    pub service_name: Option<String>,
    pub shell_type: Option<String>,
    pub schedule_type: Option<String>,
    pub schedule_action: Option<String>,
    pub command: Option<String>,
    pub script: Option<String>,
    pub timezone: Option<String>,
    pub enabled: Option<i64>,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub server_id: Option<i64>,
    pub organization_id: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct ScheduleRunResult {
    pub schedule: Schedules,
    pub action: String,
    pub deployment_id: Option<i64>,
    pub message: String,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

pub struct ScheduleService {
    queries: Arc<FlashQueries>,
    pub applications: Arc<ApplicationService>,
    pub compose: Arc<ComposeService>,
    pub notifications: Arc<NotificationService>,
}

#[singleton]
impl ScheduleService {
    fn new(
        queries: Arc<FlashQueries>,
        applications: Arc<ApplicationService>,
        compose: Arc<ComposeService>,
        notifications: Arc<NotificationService>,
    ) -> Self {
        Self {
            queries,
            applications,
            compose,
            notifications,
        }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Schedules> {
        self.queries.schedules_get_by_id(id).await
    }

    pub async fn list_by_application(&self, application_id: i64) -> sqlx::Result<Vec<Schedules>> {
        let rows = self.queries.schedules_list_by_application(application_id).await?;
        Ok(rows)
    }

    pub async fn list_by_compose(&self, compose_id: i64) -> sqlx::Result<Vec<Schedules>> {
        let rows = self.queries.schedules_list_by_compose(compose_id).await?;
        Ok(rows)
    }

    pub async fn list_by_server(&self, server_id: i64) -> sqlx::Result<Vec<Schedules>> {
        let rows = self.queries.schedules_list_by_server(server_id).await?;
        Ok(rows)
    }

    pub async fn list_by_organization(&self, organization_id: i64) -> sqlx::Result<Vec<Schedules>> {
        let rows = self
            .queries
            .schedules_list_by_organization(organization_id)
            .await?;
        Ok(rows)
    }

    pub async fn list_enabled(&self) -> sqlx::Result<Vec<Schedules>> {
        let rows = self.queries.schedules_list_enabled().await?;
        Ok(rows)
    }

    pub async fn create(&self, input: CreateScheduleDto) -> sqlx::Result<Schedules> {
        validate_cron_expression(&input.cron_expression)?;
        let shell_type = normalize_shell_type(input.shell_type.as_deref())?;
        let schedule_type = if let Some(st) = input
            .schedule_type
            .as_deref()
            .filter(|s| !s.trim().is_empty())
        {
            normalize_schedule_type(Some(st))?
        } else if input.compose_id.is_some() {
            "COMPOSE".to_string()
        } else if input.server_id.is_some() {
            "SERVER".to_string()
        } else {
            "APPLICATION".to_string()
        };
        let schedule_action = normalize_schedule_action(
            input.schedule_action.as_deref(),
            &schedule_type,
            &input.command,
        )?;
        let enabled = normalize_enabled(input.enabled.unwrap_or(1))?;
        let service_id = input.application_id.or(input.compose_id);
        validate_target(
            &schedule_type,
            &schedule_action,
            service_id,
            input.server_id,
            input.service_name.as_deref(),
        )?;
        let app_name = if let Some(an) = input.app_name.as_deref().filter(|s| !s.trim().is_empty())
        {
            format!("{an}-sched-{}", uuid::Uuid::new_v4().simple())
        } else {
            generate_schedule_app_name(&input.name)
        };

        let row = self
            .queries
            .schedules_create_and_return(&SchedulesCreateAndReturnParams {
                name: input.name,
                description: input.description.unwrap_or_default(),
                cron_expression: input.cron_expression,
                app_name,
                service_name: input.service_name.unwrap_or_default(),
                shell_type,
                schedule_type,
                schedule_action,
                command: input.command,
                script: input.script.unwrap_or_default(),
                timezone: input.timezone.unwrap_or_default(),
                enabled,
                service_id: service_id.unwrap_or(0),
                server_id: input.server_id.unwrap_or(0),
                organization_id: input.organization_id.unwrap_or(0),
            })
            .await?;
        Ok(row)
    }

    pub async fn patch(&self, id: i64, input: PatchScheduleDto) -> sqlx::Result<Schedules> {
        let current = self.get_by_id(id).await?;
        let name = input.name.unwrap_or(current.name);
        let description = input.description.or(current.description);
        let cron_expression = input.cron_expression.unwrap_or(current.cron_expression);
        validate_cron_expression(&cron_expression)?;
        let app_name = input.app_name.unwrap_or(current.app_name);
        let service_name = input.service_name.or(current.service_name);
        let shell_type = match input.shell_type {
            Some(value) => normalize_shell_type(Some(&value))?,
            None => current.shell_type,
        };
        let schedule_type = match input.schedule_type {
            Some(value) => normalize_schedule_type(Some(&value))?,
            None => current.schedule_type,
        };
        let command = input.command.unwrap_or(current.command);
        let schedule_action = match input.schedule_action {
            Some(value) => normalize_schedule_action(Some(&value), &schedule_type, &command)?,
            None => current.schedule_action,
        };
        let script = input.script.or(current.script);
        let timezone = input.timezone.or(current.timezone);
        let enabled = normalize_enabled(input.enabled.unwrap_or(current.enabled))?;
        let service_id = input
            .application_id
            .or(input.compose_id)
            .or(current.service_id);
        let server_id = input.server_id.or(current.server_id);
        let organization_id = input.organization_id.or(current.organization_id);
        validate_target(
            &schedule_type,
            &schedule_action,
            service_id,
            server_id,
            service_name.as_deref(),
        )?;

        let row = self
            .queries
            .schedules_update_and_return(&SchedulesUpdateAndReturnParams {
                name,
                description: description.unwrap_or_default(),
                cron_expression,
                app_name,
                service_name: service_name.unwrap_or_default(),
                shell_type,
                schedule_type,
                schedule_action,
                command,
                script: script.unwrap_or_default(),
                timezone: timezone.unwrap_or_default(),
                enabled,
                service_id: service_id.unwrap_or(0),
                server_id: server_id.unwrap_or(0),
                organization_id: organization_id.unwrap_or(0),
                id,
            })
            .await?;
        Ok(row)
    }

    pub async fn set_enabled(&self, id: i64, enabled: bool) -> sqlx::Result<Schedules> {
        let row = self
            .queries
            .schedules_set_enabled(i64::from(enabled), id)
            .await?;
        Ok(row)
    }

    pub async fn restore_database_backup(&self, id: i64, backup_file: &str) -> sqlx::Result<()> {
        let backup = self.queries.get_backup_by_i_d(id).await?;

        let server_id = self.resolve_database_backup_server_id(&backup).await?;
        let executor = crate::services::compute::compose::remote::agent_executor_for(server_id)
            .await
            .map_err(sqlx::Error::Protocol)?;

        let dumper = self.resolve_database_dumper(&backup).await?;
        let dest_model = self.queries.get_destination_by_i_d(backup.destination_id).await?;

        let destination = crate::utils::backup::database::S3Destination {
            access_key: dest_model.access_key,
            secret_key: dest_model.secret_access_key,
            bucket: dest_model.bucket,
            region: dest_model.region,
            endpoint: dest_model.endpoint,
            provider: Some(dest_model.provider),
        };

        let runner =
            crate::utils::backup::database::BackupRunner::new(&executor, &dumper, &destination);

        let cancel = tokio_util::sync::CancellationToken::new();
        let execution_id = self
            .backup_execution_start(
                crate::services::automation::backup::types::BackupKind::Database,
                crate::services::automation::backup::types::BackupOperation::Restore,
                Some(id),
                Some(backup_file),
            )
            .await?;
        match runner.run_restore(backup_file, &cancel).await {
            Ok(_) => {
                self.backup_execution_succeed(execution_id).await?;
            }
            Err(error) => {
                let message = error.to_string();
                self.queries
                    .backup_executions_fail(&message, execution_id)
                    .await?;
                return Err(sqlx::Error::Protocol(message));
            }
        }

        Ok(())
    }

    pub async fn restore_volume_backup(&self, id: i64, backup_file: &str) -> sqlx::Result<()> {
        let backup = self.queries.get_volume_backup_by_i_d(id).await?;

        let server_id = self.resolve_volume_backup_server_id(&backup).await?;
        let executor = crate::services::compute::compose::remote::agent_executor_for(server_id)
            .await
            .map_err(sqlx::Error::Protocol)?;

        let target = self.resolve_volume_service_target(&backup).await?;
        let dest_model = self
            .queries
            .get_destination_by_i_d(backup.destination_id)
            .await?;

        let destination = crate::utils::backup::database::S3Destination {
            access_key: dest_model.access_key,
            secret_key: dest_model.secret_access_key,
            bucket: dest_model.bucket,
            region: dest_model.region,
            endpoint: dest_model.endpoint,
            provider: Some(dest_model.provider),
        };

        let volume_backup = crate::utils::backup::volume::VolumeBackup {
            volume_name: backup.volume_name.clone(),
            service: target,
            turn_off: backup.turn_off == 1,
        };

        let runner = crate::utils::backup::volume::VolumeBackupRunner::new(
            &executor,
            &volume_backup,
            &destination,
        );

        let cancel = tokio_util::sync::CancellationToken::new();
        let execution_id = self
            .backup_execution_start(
                crate::services::automation::backup::types::BackupKind::Volume,
                crate::services::automation::backup::types::BackupOperation::Restore,
                Some(id),
                Some(backup_file),
            )
            .await?;
        match runner.run_restore(backup_file, &cancel).await {
            Ok(_) => {
                self.backup_execution_succeed(execution_id).await?;
            }
            Err(error) => {
                let message = error.to_string();
                self.queries
                    .backup_executions_fail(&message, execution_id)
                    .await?;
                return Err(sqlx::Error::Protocol(message));
            }
        }

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        self.queries.schedules_delete(id).await
    }

    pub async fn run_now(&self, id: i64) -> sqlx::Result<ScheduleRunResult> {
        self.run_with_policy(
            id,
            ScheduleTriggerKind::Manual,
            chrono::Utc::now().timestamp(),
        )
        .await
    }

    pub async fn run_scheduled(
        &self,
        id: i64,
        trigger_kind: ScheduleTriggerKind,
        scheduled_at: i64,
    ) -> sqlx::Result<ScheduleRunResult> {
        self.run_with_policy(id, trigger_kind, scheduled_at).await
    }

    pub async fn recover_missed(&self, id: i64, cron_expression: &str) -> sqlx::Result<bool> {
        let policy = self.runtime_policy(id).await?;
        let now = chrono::Utc::now();
        let Some(last_timestamp) = policy.last_scheduled_at else {
            self.runtime_mark_scheduled(id, now.timestamp()).await?;
            return Ok(false);
        };
        if MissedRunPolicy::try_from(policy.missed_run_policy.as_str())
            .map_err(sqlx::Error::Protocol)?
            != MissedRunPolicy::RunOnce
        {
            return Ok(false);
        }
        let last = chrono::DateTime::from_timestamp(last_timestamp, 0)
            .ok_or_else(|| sqlx::Error::Protocol("invalid last schedule timestamp".into()))?;
        let cron = croner::Cron::new(cron_expression)
            .parse()
            .map_err(|error| sqlx::Error::Protocol(format!("invalid cron expression: {error}")))?;
        let next = cron.find_next_occurrence(&last, false).map_err(|error| {
            sqlx::Error::Protocol(format!("could not calculate missed run: {error}"))
        })?;
        if next <= now {
            self.run_scheduled(id, ScheduleTriggerKind::Missed, next.timestamp())
                .await?;
            return Ok(true);
        }
        Ok(false)
    }

    async fn run_with_policy(
        &self,
        id: i64,
        trigger_kind: ScheduleTriggerKind,
        scheduled_at: i64,
    ) -> sqlx::Result<ScheduleRunResult> {
        let schedule = self.get_by_id(id).await?;
        if schedule.enabled == 0 {
            return Err(sqlx::Error::Protocol("schedule is disabled".into()));
        }
        let policy = self.runtime_policy(id).await?;
        let owner = Uuid::new_v4().to_string();
        let concurrency = ConcurrencyPolicy::try_from(policy.concurrency_policy.as_str())
            .map_err(sqlx::Error::Protocol)?;
        let locked = match concurrency {
            ConcurrencyPolicy::Allow => true,
            ConcurrencyPolicy::Queue => {
                let deadline = chrono::Utc::now().timestamp() + policy.lease_seconds;
                loop {
                    if self
                        .runtime_acquire(id, &owner, policy.lease_seconds)
                        .await?
                    {
                        break true;
                    }
                    if chrono::Utc::now().timestamp() >= deadline {
                        break false;
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
            ConcurrencyPolicy::Skip => {
                self.runtime_acquire(id, &owner, policy.lease_seconds)
                    .await?
            }
        };
        if !locked {
            let execution_id = self
                .runtime_begin(id, schedule.organization_id, trigger_kind, 1, scheduled_at)
                .await?;
            self.runtime_finish(
                execution_id,
                ScheduleExecutionStatus::Skipped,
                Some("previous execution is still active"),
                None,
                None,
            )
            .await?;
            return Err(sqlx::Error::Protocol(
                "schedule skipped by concurrency policy".into(),
            ));
        }
        // A cron callback can already be queued when a schedule is deleted or
        // disabled. Re-read after acquiring the execution lock so that callback
        // cannot execute a stale schedule snapshot.
        let schedule = match self.get_by_id(id).await {
            Ok(schedule) if schedule.enabled != 0 => schedule,
            Ok(_) | Err(sqlx::Error::RowNotFound) => {
                if concurrency != ConcurrencyPolicy::Allow {
                    let _ = self.runtime_release(id, &owner).await;
                }
                return Err(sqlx::Error::Protocol(
                    "schedule was deleted or disabled".into(),
                ));
            }
            Err(error) => {
                if concurrency != ConcurrencyPolicy::Allow {
                    let _ = self.runtime_release(id, &owner).await;
                }
                return Err(error);
            }
        };
        self.runtime_mark_scheduled(id, scheduled_at).await?;

        let mut final_result = None;
        let mut final_error = None;
        for attempt in 1..=(policy.retry_count + 1) {
            let attempt_trigger = if attempt == 1 {
                trigger_kind
            } else {
                ScheduleTriggerKind::Retry
            };
            let execution_id = self
                .runtime_begin(
                    id,
                    schedule.organization_id,
                    attempt_trigger,
                    attempt,
                    scheduled_at,
                )
                .await?;
            match self.run_once(schedule.clone()).await {
                Ok(result) => {
                    self.runtime_finish(
                        execution_id,
                        ScheduleExecutionStatus::Succeeded,
                        Some(&result.message),
                        result.stdout.as_deref(),
                        result.stderr.as_deref(),
                    )
                    .await?;
                    final_result = Some(result);
                    break;
                }
                Err(error) => {
                    let message = error.to_string();
                    self.runtime_finish(
                        execution_id,
                        ScheduleExecutionStatus::Failed,
                        Some(&message),
                        None,
                        Some(&message),
                    )
                    .await?;
                    final_error = Some(error);
                    if attempt <= policy.retry_count {
                        tokio::time::sleep(std::time::Duration::from_secs(
                            policy.retry_delay_seconds as u64,
                        ))
                        .await;
                    }
                }
            }
        }
        if concurrency != ConcurrencyPolicy::Allow {
            let _ = self.runtime_release(id, &owner).await;
        }
        match final_result {
            Some(result) => {
                if policy.notify_on_success != 0 {
                    self.notify_schedule(
                        &schedule,
                        NotificationTrigger::ScheduleSuccess,
                        NotificationLevel::Info,
                        &result.message,
                    )
                    .await;
                }
                Ok(result)
            }
            None => {
                let error = final_error
                    .unwrap_or_else(|| sqlx::Error::Protocol("schedule execution failed".into()));
                if policy.notify_on_failure != 0 {
                    self.notify_schedule(
                        &schedule,
                        NotificationTrigger::ScheduleFailure,
                        NotificationLevel::Critical,
                        &error.to_string(),
                    )
                    .await;
                }
                Err(error)
            }
        }
    }

    async fn notify_schedule(
        &self,
        schedule: &Schedules,
        trigger: NotificationTrigger,
        level: NotificationLevel,
        detail: &str,
    ) {
        let Some(organization_id) = schedule.organization_id else {
            return;
        };
        let message = NotificationMessage::new(format!("Schedule: {}", schedule.name), detail)
            .level(level)
            .field("Schedule", schedule.name.clone())
            .field("Action", schedule.schedule_action.clone());
        self.notifications
            .notify(
                NotificationScope::Organization(organization_id),
                trigger,
                &message,
            )
            .await;
    }

    async fn run_once(&self, schedule: Schedules) -> sqlx::Result<ScheduleRunResult> {
        let v = ScheduleType::try_from(schedule.schedule_type.as_str()).map_err(|_| {
            sqlx::Error::Protocol(format!("invalid schedule type: {}", schedule.schedule_type))
        })?;
        match v {
            ScheduleType::Application => self.run_application_schedule(schedule).await,
            ScheduleType::Compose => self.run_compose_schedule(schedule).await,
            ScheduleType::Server | ScheduleType::DokpanelServer => {
                self.run_shell_schedule(schedule).await
            }
        }
    }

    async fn run_application_schedule(
        &self,
        schedule: Schedules,
    ) -> sqlx::Result<ScheduleRunResult> {
        let application_id = schedule.service_id.ok_or_else(|| {
            sqlx::Error::Protocol("application schedule requires a linked service".into())
        })?;
        if schedule.schedule_action == ScheduleAction::Exec.as_str() {
            return self
                .run_application_container_command(schedule, application_id)
                .await;
        }

        let operation = parse_application_operation(&schedule.schedule_action)?;
        let result = self
            .applications
            .run_operation(application_id, operation)
            .await?;
        Ok(ScheduleRunResult {
            schedule,
            action: operation.as_str().into(),
            deployment_id: result.deployment_id,
            message: "application operation queued".into(),
            stdout: None,
            stderr: None,
        })
    }

    async fn run_compose_schedule(&self, schedule: Schedules) -> sqlx::Result<ScheduleRunResult> {
        let compose_id = schedule.service_id.ok_or_else(|| {
            sqlx::Error::Protocol("compose schedule requires a linked service".into())
        })?;
        if schedule.schedule_action == ScheduleAction::Exec.as_str() {
            return self
                .run_compose_container_command(schedule, compose_id)
                .await;
        }

        let operation = parse_compose_operation(&schedule.schedule_action)?;
        let result = self.compose.run_operation(compose_id, operation).await?;
        Ok(ScheduleRunResult {
            schedule,
            action: operation.as_str().into(),
            deployment_id: result.deployment_id,
            message: "compose operation queued".into(),
            stdout: None,
            stderr: None,
        })
    }

    async fn run_shell_schedule(&self, schedule: Schedules) -> sqlx::Result<ScheduleRunResult> {
        let command = schedule
            .script
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or(schedule.command.trim());
        if command.is_empty() {
            return Err(sqlx::Error::Protocol("schedule command is empty".into()));
        }

        let executor = crate::services::compute::compose::remote::agent_executor_for(schedule.server_id)
            .await
            .map_err(sqlx::Error::Protocol)?;
        let v = ShellType::try_from(schedule.shell_type.as_str()).map_err(|_| {
            sqlx::Error::Protocol(format!("invalid shell type: {}", schedule.shell_type))
        })?;
        let shell = v.executable();
        let output = executor
            .run(shell, ["-lc", command])
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(ScheduleRunResult {
            schedule,
            action: "shell".into(),
            deployment_id: None,
            message: "schedule command executed".into(),
            stdout: Some(output.stdout),
            stderr: Some(output.stderr),
        })
    }

    async fn run_application_container_command(
        &self,
        schedule: Schedules,
        application_id: i64,
    ) -> sqlx::Result<ScheduleRunResult> {
        let app = self.queries.get_service_by_i_d(application_id).await?;
        let app_name = app.app_name;
        let server_id = app.server_id;
        // Applications are always deployed as Swarm services.
        let service_name = format!("{app_name}_{app_name}");
        self.run_swarm_container_command(schedule, server_id, &service_name)
            .await
    }

    async fn run_compose_container_command(
        &self,
        schedule: Schedules,
        compose_id: i64,
    ) -> sqlx::Result<ScheduleRunResult> {
        let compose = self.queries.get_service_by_i_d(compose_id).await?;
        let app_name = compose.app_name;
        let server_id = compose.server_id;
        let compose_type = self
            .queries
            .get_service_compose(compose_id)
            .await?
            .compose_type;

        let compose_service = schedule.service_name.clone().ok_or_else(|| {
            sqlx::Error::Protocol("compose EXEC schedule requires service_name".into())
        })?;
        let v = ComposeType::try_from(compose_type.as_str()).map_err(|_| {
            sqlx::Error::Protocol(format!("invalid compose type: {}", compose_type))
        })?;
        match v {
            ComposeType::Stack => {
                let service_name = if compose_service.starts_with(&format!("{app_name}_")) {
                    compose_service
                } else {
                    format!("{app_name}_{compose_service}")
                };
                self.run_swarm_container_command(schedule, server_id, &service_name)
                    .await
            }
            ComposeType::DockerCompose => {
                self.run_compose_project_container_command(
                    schedule,
                    server_id,
                    &app_name,
                    &compose_service,
                )
                .await
            }
        }
    }

    /// Execute command in a Swarm service container (Application or Stack).
    async fn run_swarm_container_command(
        &self,
        schedule: Schedules,
        server_id: Option<i64>,
        service_name: &str,
    ) -> sqlx::Result<ScheduleRunResult> {
        let command = schedule_command(&schedule)?;
        let docker = self.docker_for_server(server_id).await?;
        let container = docker
            .containers()
            .ps()
            .filter(ContainerFilter::label(
                "com.docker.swarm.service.name",
                service_name,
            ))
            .list()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .into_iter()
            .find(|c| !c.id.trim().is_empty())
            .ok_or_else(|| {
                sqlx::Error::Protocol(format!(
                    "running container not found for swarm service {service_name}"
                ))
            })?;
        let v = ShellType::try_from(schedule.shell_type.as_str()).map_err(|_| {
            sqlx::Error::Protocol(format!("invalid shell type: {}", schedule.shell_type))
        })?;
        let shell = v.executable();
        let output = docker
            .containers()
            .exec(&container.id)
            .run([shell, "-lc", command])
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        Ok(ScheduleRunResult {
            action: "exec".into(),
            deployment_id: None,
            message: format!("container command executed on swarm service {service_name}"),
            stdout: Some(output.stdout),
            stderr: Some(output.stderr),
            schedule,
        })
    }

    /// Execute command in a docker compose container.
    async fn run_compose_project_container_command(
        &self,
        schedule: Schedules,
        server_id: Option<i64>,
        project_name: &str,
        service_name: &str,
    ) -> sqlx::Result<ScheduleRunResult> {
        let command = schedule_command(&schedule)?;
        let docker = self.docker_for_server(server_id).await?;
        let container = docker
            .containers()
            .ps()
            .filter(ContainerFilter::label("com.docker.compose.project", project_name))
            .filter(ContainerFilter::label("com.docker.compose.service", service_name))
            .list()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .into_iter()
            .find(|c| !c.id.trim().is_empty())
            .ok_or_else(|| {
                sqlx::Error::Protocol(format!(
                    "running container not found for compose project={project_name} service={service_name}"
                ))
            })?;
        let v = ShellType::try_from(schedule.shell_type.as_str()).map_err(|_| {
            sqlx::Error::Protocol(format!("invalid shell type: {}", schedule.shell_type))
        })?;
        let shell = v.executable();
        let output = docker
            .containers()
            .exec(&container.id)
            .run([shell, "-lc", command])
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        Ok(ScheduleRunResult {
            action: "exec".into(),
            deployment_id: None,
            message: format!("container command executed on compose {project_name}/{service_name}"),
            stdout: Some(output.stdout),
            stderr: Some(output.stderr),
            schedule,
        })
    }

    async fn docker_for_server(&self, server_id: Option<i64>) -> sqlx::Result<DockerCli> {
        crate::services::compute::compose::remote::agent_executor_for(server_id)
            .await
            .map(DockerCli::from_executor)
            .map_err(sqlx::Error::Protocol)
    }

    async fn resolve_volume_backup_server_id(
        &self,
        backup: &VolumeBackups,
    ) -> sqlx::Result<Option<i64>> {
        if let Some(service_id) = backup.service_id {
            return Ok(self.queries.get_service_by_i_d(service_id).await?.server_id);
        }
        Ok(None)
    }

    async fn resolve_volume_service_target(
        &self,
        backup: &VolumeBackups,
    ) -> sqlx::Result<crate::utils::backup::volume::VolumeServiceTarget> {
        let service_type = parse_volume_service_type(&backup.service_type)
            .unwrap_or(VolumeBackupsServiceTypeEnum::Application);

        match service_type {
            VolumeBackupsServiceTypeEnum::Compose => {
                let service = backup.service_name.clone().unwrap_or_default();
                Ok(
                    crate::utils::backup::volume::VolumeServiceTarget::ComposeService {
                        project: backup.app_name.clone(),
                        service,
                    },
                )
            }
            _ => {
                let service_name = if let Some(service_id) = backup.service_id {
                    self.queries.get_service_by_i_d(service_id).await?.app_name
                } else {
                    backup
                        .service_name
                        .clone()
                        .unwrap_or_else(|| format!("{}_{}", backup.app_name, backup.app_name))
                };
                Ok(
                    crate::utils::backup::volume::VolumeServiceTarget::SwarmService { service_name },
                )
            }
        }
    }

    async fn resolve_database_backup_server_id(
        &self,
        backup: &Backups,
    ) -> sqlx::Result<Option<i64>> {
        if let Some(service_id) = backup.service_id {
            return Ok(self.queries.get_service_by_i_d(service_id).await?.server_id);
        }
        Ok(None)
    }

    async fn resolve_database_dumper(
        &self,
        backup: &Backups,
    ) -> sqlx::Result<crate::utils::backup::database::DatabaseDumper> {
        if let Some(service_id) = backup.service_id {
            let creds = self
                .queries
                .database_management_credentials(service_id)
                .await?;
            let db_type = self.queries.get_service_database(service_id).await?.db_type;
            let kind = parse_database_type(&db_type).ok_or_else(|| {
                sqlx::Error::Protocol(format!("unsupported database type: {db_type}"))
            })?;
            let user = creds.db_user.unwrap_or_default();
            let password = creds.db_password.unwrap_or_default();
            let service_name = creds.app_name;
            return Ok(database_dumper_for(
                kind,
                user,
                password,
                backup.database_name.clone(),
                creds.db_name.unwrap_or_default(),
                service_name,
            ));
        }

        let (user, password) = if let Some(meta_str) = &backup.metadata {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(meta_str) {
                let u = json
                    .get("databaseUser")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let p = json
                    .get("databasePassword")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                (u, p)
            } else {
                ("".to_string(), "".to_string())
            }
        } else {
            ("".to_string(), "".to_string())
        };

        let service_name = backup
            .service_name
            .clone()
            .unwrap_or_else(|| backup.app_name.clone());

        let db_type = backup.database_type.clone().unwrap_or_default();
        let kind = parse_database_type(&db_type).ok_or_else(|| {
            sqlx::Error::Protocol(format!("unsupported database type: {db_type}"))
        })?;
        Ok(database_dumper_for(
            kind,
            user,
            password,
            backup.database_name.clone(),
            backup.database_name.clone(),
            service_name,
        ))
    }

    pub async fn run_database_backup(&self, id: i64) -> sqlx::Result<()> {
        let backup = self.queries.get_backup_by_i_d(id).await?;
        if backup.enabled == 0 {
            return Err(sqlx::Error::Protocol("backup is disabled".into()));
        }

        let server_id = self.resolve_database_backup_server_id(&backup).await?;
        let executor = crate::services::compute::compose::remote::agent_executor_for(server_id)
            .await
            .map_err(sqlx::Error::Protocol)?;

        let dumper = self.resolve_database_dumper(&backup).await?;
        let dest_model = self
            .queries
            .get_destination_by_i_d(backup.destination_id)
            .await?;

        let destination = crate::utils::backup::database::S3Destination {
            access_key: dest_model.access_key,
            secret_key: dest_model.secret_access_key,
            bucket: dest_model.bucket,
            region: dest_model.region,
            endpoint: dest_model.endpoint,
            provider: Some(dest_model.provider),
        };

        let runner =
            crate::utils::backup::database::BackupRunner::new(&executor, &dumper, &destination);
        // Re-read immediately before starting the external backup command. A
        // deleted/disabled job may have had a callback already queued.
        let current = self.queries.get_backup_by_i_d(id).await.ok();
        if current.as_ref().is_none_or(|item| item.enabled == 0) {
            return Err(sqlx::Error::Protocol(
                "backup was deleted or disabled".into(),
            ));
        }

        let ext = dumper.file_extension();
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let object_key = format!(
            "{}/{}_{}.{}",
            backup.prefix.trim_matches('/'),
            backup.database_name,
            timestamp,
            ext
        );

        let cancel = tokio_util::sync::CancellationToken::new();
        let execution_id = self
            .backup_execution_start(
                crate::services::automation::backup::types::BackupKind::Database,
                crate::services::automation::backup::types::BackupOperation::Backup,
                Some(id),
                Some(&object_key),
            )
            .await?;
        match runner.run(&object_key, &cancel).await {
            Ok(_) => {
                self.backup_execution_succeed(execution_id).await?;
            }
            Err(error) => {
                let message = error.to_string();
                self.queries
                    .backup_executions_fail(&message, execution_id)
                    .await?;
                return Err(sqlx::Error::Protocol(message));
            }
        }

        Ok(())
    }

    pub async fn run_volume_backup(&self, id: i64) -> sqlx::Result<()> {
        let backup = self.queries.get_volume_backup_by_i_d(id).await?;
        if backup.enabled == 0 {
            return Err(sqlx::Error::Protocol("volume backup is disabled".into()));
        }

        let server_id = self.resolve_volume_backup_server_id(&backup).await?;
        let executor = crate::services::compute::compose::remote::agent_executor_for(server_id)
            .await
            .map_err(sqlx::Error::Protocol)?;

        let target = self.resolve_volume_service_target(&backup).await?;
        let dest_model = self
            .queries
            .get_destination_by_i_d(backup.destination_id)
            .await?;

        let destination = crate::utils::backup::database::S3Destination {
            access_key: dest_model.access_key,
            secret_key: dest_model.secret_access_key,
            bucket: dest_model.bucket,
            region: dest_model.region,
            endpoint: dest_model.endpoint,
            provider: Some(dest_model.provider),
        };

        let volume_backup = crate::utils::backup::volume::VolumeBackup {
            volume_name: backup.volume_name.clone(),
            service: target,
            turn_off: backup.turn_off == 1,
        };

        let runner = crate::utils::backup::volume::VolumeBackupRunner::new(
            &executor,
            &volume_backup,
            &destination,
        );
        let current = self.queries.get_volume_backup_by_i_d(id).await.ok();
        if current
            .as_ref()
            .is_none_or(|item| item.enabled == 0 || !volume_backup_has_valid_resource(item))
        {
            return Err(sqlx::Error::Protocol(
                "volume backup was deleted, disabled, or has no valid resource".into(),
            ));
        }

        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let object_key = format!(
            "{}/{}_{}.tar.gz",
            backup.prefix.trim_matches('/'),
            backup.name,
            timestamp
        );

        let cancel = tokio_util::sync::CancellationToken::new();
        let execution_id = self
            .backup_execution_start(
                crate::services::automation::backup::types::BackupKind::Volume,
                crate::services::automation::backup::types::BackupOperation::Backup,
                Some(id),
                Some(&object_key),
            )
            .await?;
        match runner.run(&object_key, &cancel).await {
            Ok(_) => {
                self.backup_execution_succeed(execution_id).await?;
            }
            Err(error) => {
                let message = error.to_string();
                self.queries
                    .backup_executions_fail(&message, execution_id)
                    .await?;
                return Err(sqlx::Error::Protocol(message));
            }
        }

        Ok(())
    }

    pub async fn runtime_policy(&self, schedule_id: i64) -> sqlx::Result<ScheduleRuntimePolicies> {
        self.queries.schedule_runtime_ensure_policy(schedule_id).await?;
        self.queries.schedule_runtime_get_policy(schedule_id).await
    }

    pub async fn update_runtime_policy(
        &self,
        schedule_id: i64,
        retry_count: i64,
        retry_delay_seconds: i64,
        missed_run_policy: &str,
        concurrency_policy: &str,
        lease_seconds: i64,
        notify_on_success: bool,
        notify_on_failure: bool,
    ) -> sqlx::Result<ScheduleRuntimePolicies> {
        self.queries
            .schedule_runtime_update_policy(&ScheduleRuntimeUpdatePolicyParams {
                schedule_id,
                retry_count,
                retry_delay_seconds,
                missed_run_policy: missed_run_policy.to_string(),
                concurrency_policy: concurrency_policy.to_string(),
                lease_seconds,
                notify_on_success: i64::from(notify_on_success),
                notify_on_failure: i64::from(notify_on_failure),
            })
            .await?;
        self.runtime_policy(schedule_id).await
    }

    pub async fn list_executions(
        &self,
        schedule_id: i64,
        limit: i64,
    ) -> sqlx::Result<Vec<ScheduleExecutions>> {
        self.queries
            .schedule_runtime_list(schedule_id, limit.clamp(1, 500))
            .await
    }

    pub async fn execution_exists(
        &self,
        schedule_id: i64,
        execution_id: i64,
    ) -> sqlx::Result<bool> {
        let count = self
            .queries
            .schedule_runtime_execution_exists(execution_id, schedule_id)
            .await?;
        Ok(count != 0)
    }

    async fn runtime_acquire(
        &self,
        schedule_id: i64,
        owner_id: &str,
        lease_seconds: i64,
    ) -> sqlx::Result<bool> {
        let now = chrono::Utc::now().timestamp();
        // The upsert's WHERE clause rejects expired-lease takeovers; an empty
        // RETURNING means another owner still holds the lock.
        let acquired = self
            .queries
            .schedule_runtime_acquire(&ScheduleRuntimeAcquireParams {
                schedule_id,
                owner_id: owner_id.to_string(),
                acquired_at: now,
                expires_at: now + lease_seconds,
                expires_at2: now,
            })
            .await;
        Ok(acquired.is_ok())
    }

    async fn runtime_release(&self, schedule_id: i64, owner_id: &str) -> sqlx::Result<()> {
        self.queries
            .schedule_runtime_release(schedule_id, owner_id)
            .await
    }

    async fn runtime_mark_scheduled(
        &self,
        schedule_id: i64,
        timestamp: i64,
    ) -> sqlx::Result<()> {
        self.queries
            .schedule_runtime_mark_scheduled(timestamp, schedule_id)
            .await
    }

    async fn runtime_begin(
        &self,
        schedule_id: i64,
        organization_id: Option<i64>,
        trigger_kind: ScheduleTriggerKind,
        attempt: i64,
        scheduled_at: i64,
    ) -> sqlx::Result<i64> {
        self.queries
            .schedule_runtime_begin(&ScheduleRuntimeBeginParams {
                schedule_id,
                organization_id: organization_id.unwrap_or(0),
                trigger_kind: trigger_kind.as_str().to_string(),
                attempt,
                scheduled_at,
            })
            .await
    }

    async fn runtime_finish(
        &self,
        id: i64,
        status: ScheduleExecutionStatus,
        message: Option<&str>,
        stdout: Option<&str>,
        stderr: Option<&str>,
    ) -> sqlx::Result<()> {
        let schedule_id = self
            .queries
            .schedule_runtime_get_execution_schedule(id)
            .await?;
        self.queries
            .schedule_runtime_finish(&ScheduleRuntimeFinishParams {
                status: status.as_str().to_string(),
                message: message.unwrap_or("").to_string(),
                stdout: stdout.unwrap_or("").to_string(),
                stderr: stderr.unwrap_or("").to_string(),
                id,
            })
            .await?;
        if let Err(error) = crate::services::automation::schedule::file_log::append(
            &format!("schedule-{schedule_id}"),
            Some(id),
            status.as_str(),
            message,
            stdout,
            stderr,
        )
        .await
        {
            tracing::warn!(execution_id = id, %error, "could not write schedule execution log file");
        }
        if let Err(error) = crate::services::automation::schedule::file_log::append(
            &format!("schedule-{schedule_id}-execution-{id}"),
            Some(id),
            status.as_str(),
            message,
            stdout,
            stderr,
        )
        .await
        {
            tracing::warn!(execution_id = id, %error, "could not write per-execution schedule log file");
        }
        Ok(())
    }

    async fn backup_execution_start(
        &self,
        backup_kind: crate::services::automation::backup::types::BackupKind,
        operation: crate::services::automation::backup::types::BackupOperation,
        backup_id: Option<i64>,
        object_key: Option<&str>,
    ) -> sqlx::Result<i64> {
        let backup_id = backup_id.unwrap_or(0);
        let backup_kind = backup_kind.as_str().to_string();
        let operation = operation.as_str().to_string();
        let attempt = self
            .queries
            .backup_executions_next_attempt(
                &crate::flash_gen::backup_executions::BackupExecutionsNextAttemptParams {
                    backup_kind: backup_kind.clone(),
                    operation: operation.clone(),
                    backup_id,
                },
            )
            .await?
            .unwrap_or(1);
        self.queries
            .backup_executions_start(&BackupExecutionsStartParams {
                backup_kind,
                operation,
                backup_id,
                object_key: object_key.unwrap_or("").to_string(),
                attempt,
            })
            .await
    }

    async fn backup_execution_succeed(&self, execution_id: i64) -> sqlx::Result<()> {
        self.queries
            .backup_executions_succeed(
                &crate::flash_gen::backup_executions::BackupExecutionsSucceedParams {
                    sha256_checksum: String::new(),
                    size_bytes: 0,
                    id: execution_id,
                },
            )
            .await
    }

    pub async fn list_backups(&self) -> sqlx::Result<Vec<Backups>> {
        self.queries.backups_get_all().await.map(|rows| {
            rows.into_iter()
                .map(|row| Backups {
                    id: row.id,
                    app_name: row.app_name,
                    schedule: row.schedule,
                    enabled: row.enabled,
                    database_name: row.database_name,
                    prefix: row.prefix,
                    service_name: row.service_name,
                    keep_latest_count: row.keep_latest_count,
                    backup_type: row.backup_type,
                    database_type: row.database_type,
                    metadata: row.metadata,
                    service_id: row.service_id,
                    destination_id: row.destination_id,
                    organization_id: row.organization_id,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                })
                .collect()
        })
    }

    pub async fn get_backup(&self, id: i64) -> sqlx::Result<Option<Backups>> {
        self.queries.get_backup_by_i_d(id).await.map(Some)
    }

    pub async fn list_volume_backups(&self) -> sqlx::Result<Vec<VolumeBackups>> {
        let rows = self.queries.volume_backups_get_all().await?;
        Ok(rows
            .into_iter()
            .map(|row| VolumeBackups {
                id: row.id,
                name: row.name,
                volume_name: row.volume_name,
                prefix: row.prefix,
                service_type: row.service_type,
                app_name: row.app_name,
                service_name: row.service_name,
                turn_off: row.turn_off,
                cron_expression: row.cron_expression,
                keep_latest_count: row.keep_latest_count,
                enabled: row.enabled,
                service_id: row.service_id,
                destination_id: row.destination_id,
                organization_id: row.organization_id,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .collect())
    }

    pub async fn get_volume_backup(&self, id: i64) -> sqlx::Result<Option<VolumeBackups>> {
        self.queries.get_volume_backup_by_i_d(id).await.map(Some)
    }
}

fn database_dumper_for(
    kind: BackupsDatabaseTypeEnum,
    user: String,
    password: String,
    database: String,
    managed_database: String,
    service_name: String,
) -> crate::utils::backup::database::DatabaseDumper {
    use crate::utils::backup::database::{
        ContainerTarget, DatabaseDumper, DbCredentials,
    };
    let target = ContainerTarget { service_name };
    match kind {
        BackupsDatabaseTypeEnum::Postgres => DatabaseDumper::Postgres {
            creds: DbCredentials {
                user,
                password,
                database: managed_database,
            },
            target,
        },
        BackupsDatabaseTypeEnum::Mysql => DatabaseDumper::Mysql {
            creds: DbCredentials {
                user,
                password,
                database: managed_database,
            },
            target,
        },
        BackupsDatabaseTypeEnum::Mariadb => DatabaseDumper::MariaDb {
            creds: DbCredentials {
                user,
                password,
                database: managed_database,
            },
            target,
        },
        BackupsDatabaseTypeEnum::Mongo => DatabaseDumper::Mongo {
            creds: DbCredentials {
                user,
                password,
                database,
            },
            target,
        },
        BackupsDatabaseTypeEnum::Redis => DatabaseDumper::Redis { target },
        BackupsDatabaseTypeEnum::Libsql => DatabaseDumper::LibSql { database, target },
    }
}


pub(crate) fn validate_backup_cron_expression(value: &str) -> sqlx::Result<()> {
    let expression = value.trim();
    if expression.is_empty() {
        return Err(sqlx::Error::Protocol(
            "cron expression cannot be empty".into(),
        ));
    }

    let fields_count = expression.split_whitespace().count();
    if fields_count != 5 && fields_count != 6 {
        return Err(sqlx::Error::Protocol(
            "cron expression must contain 5 fields (e.g., '0 * * * *') or 6 fields".into(),
        ));
    }

    if croner::Cron::new(expression).parse().is_ok() {
        return Ok(());
    }

    if fields_count == 5 {
        let with_seconds = format!("0 {expression}");
        if croner::Cron::new(&with_seconds).parse().is_ok() {
            return Ok(());
        }
    }

    croner::Cron::new(expression)
        .parse()
        .map(|_| ())
        .map_err(|error| sqlx::Error::Protocol(format!("invalid cron expression: {error}")))
}

fn validate_cron_expression(value: &str) -> sqlx::Result<()> {
    validate_backup_cron_expression(value)
}

fn normalize_shell_type(value: Option<&str>) -> sqlx::Result<String> {
    let value = value.unwrap_or("BASH").trim().to_ascii_uppercase();
    let v = ShellType::try_from(value.as_str())
        .map_err(|_| sqlx::Error::Protocol(format!("invalid shell type: {}", value)))?;
    match v {
        ShellType::Bash => Ok(value),
        ShellType::Sh => Ok(value),
    }
}

fn normalize_schedule_type(value: Option<&str>) -> sqlx::Result<String> {
    let value = value.unwrap_or("APPLICATION").trim().to_ascii_uppercase();
    let v = ScheduleType::try_from(value.as_str())
        .map_err(|_| sqlx::Error::Protocol(format!("invalid schedule type: {}", value)))?;
    match v {
        ScheduleType::Application => Ok(value),
        ScheduleType::Compose => Ok(value),
        ScheduleType::Server => Ok(value),
        ScheduleType::DokpanelServer => Ok(value),
    }
}

fn normalize_schedule_action(
    value: Option<&str>,
    schedule_type: &str,
    command: &str,
) -> sqlx::Result<String> {
    let value = value
        .map(|value| value.trim().to_ascii_uppercase())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| infer_schedule_action(schedule_type, command));
    let v = ScheduleAction::try_from(value.as_str())
        .map_err(|_| sqlx::Error::Protocol(format!("invalid schedule action: {}", value)))?;
    match v {
        ScheduleAction::Exec => Ok(value),
        ScheduleAction::Deploy => Ok(value),
        ScheduleAction::Redeploy => Ok(value),
        ScheduleAction::Rebuild => Ok(value),
        ScheduleAction::Reload => Ok(value),
        ScheduleAction::Start => Ok(value),
        ScheduleAction::Stop => Ok(value),
    }
}

fn infer_schedule_action(schedule_type: &str, command: &str) -> String {
    let command = command.trim().to_ascii_lowercase();
    let schedule = ScheduleType::try_from(schedule_type)
        .map_err(|_| sqlx::Error::Protocol(format!("invalid schedule type: {}", schedule_type)))
        .unwrap();
    let is_operation = match schedule {
        ScheduleType::Application => matches!(
            command.as_str(),
            "deploy" | "redeploy" | "rebuild" | "reload" | "start"
        ),
        ScheduleType::Compose => matches!(
            command.as_str(),
            "deploy" | "redeploy" | "reload" | "start" | "stop"
        ),
        _ => false,
    };
    if is_operation {
        command.to_ascii_uppercase()
    } else {
        "EXEC".into()
    }
}

fn normalize_enabled(value: i64) -> sqlx::Result<i64> {
    match value {
        0 | 1 => Ok(value),
        _ => Err(sqlx::Error::Protocol("enabled must be 0 or 1".into())),
    }
}

fn validate_target(
    schedule_type: &str,
    schedule_action: &str,
    service_id: Option<i64>,
    server_id: Option<i64>,
    service_name: Option<&str>,
) -> sqlx::Result<()> {
    let v = ScheduleType::try_from(schedule_type)
        .map_err(|_| sqlx::Error::Protocol(format!("invalid schedule type: {}", schedule_type)))
        .unwrap();
    match v {
        ScheduleType::Application if service_id.is_none() => Err(sqlx::Error::Protocol(
            "APPLICATION schedule requires application_id".into(),
        )),
        ScheduleType::Compose if service_id.is_none() => Err(sqlx::Error::Protocol(
            "COMPOSE schedule requires compose_id".into(),
        )),
        ScheduleType::Compose
            if schedule_action == "EXEC" && service_name.is_none_or(|v| v.trim().is_empty()) =>
        {
            Err(sqlx::Error::Protocol(
                "COMPOSE EXEC schedule requires service_name".into(),
            ))
        }
        ScheduleType::Server if server_id.is_none() => Err(sqlx::Error::Protocol(
            "SERVER schedule requires server_id".into(),
        )),
        ScheduleType::Server if schedule_action != "EXEC" => Err(sqlx::Error::Protocol(
            "SERVER schedule only supports EXEC action".into(),
        )),
        ScheduleType::DokpanelServer if schedule_action != "EXEC" => Err(sqlx::Error::Protocol(
            "DOKPANEL-SERVER schedule only supports EXEC action".into(),
        )),
        _ => Ok(()),
    }
}

fn parse_application_operation(value: &str) -> sqlx::Result<ApplicationOperation> {
    let v = ScheduleAction::try_from(value.trim().to_ascii_uppercase().as_str())
        .map_err(|_| sqlx::Error::Protocol(format!("invalid schedule action: {}", value)))
        .unwrap();
    match v {
        ScheduleAction::Deploy => Ok(ApplicationOperation::Deploy),
        ScheduleAction::Redeploy => Ok(ApplicationOperation::Redeploy),
        ScheduleAction::Rebuild => Ok(ApplicationOperation::Rebuild),
        ScheduleAction::Reload => Ok(ApplicationOperation::Reload),
        ScheduleAction::Start => Ok(ApplicationOperation::Start),
        _ => Err(sqlx::Error::Protocol(
            "application schedule command must be deploy, redeploy, rebuild, reload or start"
                .into(),
        )),
    }
}

fn parse_compose_operation(value: &str) -> sqlx::Result<ComposeOperation> {
    let v = ScheduleAction::try_from(value.trim().to_ascii_uppercase().as_str())
        .map_err(|_| sqlx::Error::Protocol(format!("invalid schedule action: {}", value)))
        .unwrap();
    match v {
        ScheduleAction::Deploy => Ok(ComposeOperation::Deploy),
        ScheduleAction::Redeploy => Ok(ComposeOperation::Redeploy),
        ScheduleAction::Reload => Ok(ComposeOperation::Reload),
        ScheduleAction::Start => Ok(ComposeOperation::Start),
        ScheduleAction::Stop => Ok(ComposeOperation::Stop),
        _ => Err(sqlx::Error::Protocol(
            "compose schedule command must be deploy, redeploy, reload, start or stop".into(),
        )),
    }
}

fn schedule_command(schedule: &Schedules) -> sqlx::Result<&str> {
    let command = schedule
        .script
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(schedule.command.trim());
    if command.is_empty() {
        return Err(sqlx::Error::Protocol("schedule command is empty".into()));
    }
    Ok(command)
}

fn generate_schedule_app_name(name: &str) -> String {
    let slug = name
        .trim()
        .to_ascii_lowercase()
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    let slug = if slug.is_empty() {
        "schedule".into()
    } else {
        slug
    };
    format!("{slug}-{}", Uuid::new_v4().simple())
}

fn parse_database_type(value: &str) -> Option<BackupsDatabaseTypeEnum> {
    match DatabaseBackupTypeInput::from_str(value)? {
        DatabaseBackupTypeInput::Postgres | DatabaseBackupTypeInput::Postgresql => {
            Some(BackupsDatabaseTypeEnum::Postgres)
        }
        DatabaseBackupTypeInput::Mysql => Some(BackupsDatabaseTypeEnum::Mysql),
        DatabaseBackupTypeInput::Mariadb => Some(BackupsDatabaseTypeEnum::Mariadb),
        DatabaseBackupTypeInput::Mongo | DatabaseBackupTypeInput::Mongodb => {
            Some(BackupsDatabaseTypeEnum::Mongo)
        }
        DatabaseBackupTypeInput::Redis => Some(BackupsDatabaseTypeEnum::Redis),
        DatabaseBackupTypeInput::Libsql => Some(BackupsDatabaseTypeEnum::Libsql),
    }
}

fn parse_volume_service_type(value: &str) -> Option<VolumeBackupsServiceTypeEnum> {
    match VolumeBackupServiceTypeInput::from_str(value)? {
        VolumeBackupServiceTypeInput::Application => {
            Some(VolumeBackupsServiceTypeEnum::Application)
        }
        VolumeBackupServiceTypeInput::Compose => Some(VolumeBackupsServiceTypeEnum::Compose),
        VolumeBackupServiceTypeInput::Postgres => Some(VolumeBackupsServiceTypeEnum::Postgres),
        VolumeBackupServiceTypeInput::Mysql => Some(VolumeBackupsServiceTypeEnum::Mysql),
        VolumeBackupServiceTypeInput::Mariadb => Some(VolumeBackupsServiceTypeEnum::Mariadb),
        VolumeBackupServiceTypeInput::Mongo | VolumeBackupServiceTypeInput::Mongodb => {
            Some(VolumeBackupsServiceTypeEnum::Mongo)
        }
        VolumeBackupServiceTypeInput::Redis => Some(VolumeBackupsServiceTypeEnum::Redis),
        VolumeBackupServiceTypeInput::Libsql => Some(VolumeBackupsServiceTypeEnum::Libsql),
    }
}

pub(crate) fn volume_backup_has_valid_resource(backup: &VolumeBackups) -> bool {
    backup.service_id.is_some()
}

#[cfg(test)]
mod volume_backup_resource_tests {
    use super::volume_backup_has_valid_resource;
    use crate::flash_gen::models::VolumeBackups;

    fn backup(service_id: Option<i64>) -> VolumeBackups {
        VolumeBackups {
            id: 1,
            name: "test".into(),
            volume_name: "data".into(),
            prefix: "backups".into(),
            service_type: "COMPOSE".into(),
            app_name: "test-backup".into(),
            service_name: None,
            turn_off: 0,
            cron_expression: "* * * * *".into(),
            keep_latest_count: None,
            enabled: 1,
            service_id,
            destination_id: 1,
            organization_id: 1,
            created_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn orphan_volume_backup_is_invalid() {
        assert!(!volume_backup_has_valid_resource(&backup(None)));
    }

    #[test]
    fn resource_linked_volume_backup_is_valid() {
        assert!(volume_backup_has_valid_resource(&backup(Some(7))));
    }
}

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum DatabaseBackupTypeInput {
        default = Postgres;

        Postgres => "POSTGRES",
        Postgresql => "POSTGRESQL",
        Mysql => "MYSQL",
        Mariadb => "MARIADB",
        Mongo => "MONGO",
        Mongodb => "MONGODB",
        Redis => "REDIS",
        Libsql => "LIBSQL",
    }
}

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum VolumeBackupServiceTypeInput {
        default = Application;

        Application => "APPLICATION",
        Compose => "COMPOSE",
        Postgres => "POSTGRES",
        Mysql => "MYSQL",
        Mariadb => "MARIADB",
        Mongo => "MONGO",
        Mongodb => "MONGODB",
        Redis => "REDIS",
        Libsql => "LIBSQL",
    }
}
