use connectrpc::{ConnectError, ErrorCode};
use serde::{Deserialize, Serialize};

use crate::api::connect::generated::openoxide::api::v1::ResourceResponse;
use crate::flash_gen::models::{ScheduleExecutions, ScheduleRuntimePolicies, Schedules};
use crate::services::automation::schedule::ScheduleRunResult;
use crate::services::automation::schedule::types::{
    ConcurrencyPolicy, MissedRunPolicy, ScheduleExecutionStatus, ScheduleTriggerKind,
};

pub async fn refresh_schedule_runner() {
    if let Ok(runner) = auto_di::resolve::<crate::services::automation::schedule::ScheduleRunner>().await {
        if let Err(error) = runner.refresh_jobs().await {
            tracing::error!(%error, "could not refresh scheduler after schedule mutation");
        }
    }
}

pub fn map_sqlx_error(error: sqlx::Error) -> ConnectError {
    match &error {
        sqlx::Error::RowNotFound => ConnectError::not_found("schedule not found"),
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Protocol(msg) => ConnectError::invalid_argument(msg.clone()),
        other => {
            tracing::error!(error = %other, "schedule database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

pub fn schedule_to_resource(dto: ScheduleResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "schedule".into(),
        payload,
        ..Default::default()
    }
}

pub fn execution_to_resource(dto: ScheduleExecutionDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "schedule_execution".into(),
        payload,
        ..Default::default()
    }
}

// ── Helper DTOs for ResourceRequest/ResourceResponse payloads ────────────

#[derive(Debug, Clone, Serialize)]
pub struct ScheduleLogDto {
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScheduleResponseDto {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub cron_expression: String,
    pub app_name: String,
    pub service_name: Option<String>,
    pub shell_type: String,
    pub schedule_type: String,
    pub schedule_action: String,
    pub command: String,
    pub script: Option<String>,
    pub timezone: Option<String>,
    pub enabled: i64,
    pub service_id: Option<i64>,
    // Typed linkage mirrors for the detail-page filters: exactly one is set
    // from (schedule_type, service_id), matching the overview-domain pattern.
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub database_id: Option<i64>,
    pub server_id: Option<i64>,
    pub organization_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Schedules> for ScheduleResponseDto {
    fn from(value: Schedules) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            cron_expression: value.cron_expression,
            app_name: value.app_name,
            service_name: value.service_name,
            shell_type: value.shell_type,
            schedule_action: value.schedule_action,
            command: value.command,
            script: value.script,
            timezone: value.timezone,
            enabled: value.enabled,
            service_id: value.service_id,
            application_id: (value.schedule_type == "APPLICATION")
                .then_some(value.service_id)
                .flatten(),
            compose_id: (value.schedule_type == "COMPOSE")
                .then_some(value.service_id)
                .flatten(),
            database_id: (value.schedule_type == "DATABASE")
                .then_some(value.service_id)
                .flatten(),
            schedule_type: value.schedule_type,
            server_id: value.server_id,
            organization_id: value.organization_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ScheduleRunResponseDto {
    pub schedule: ScheduleResponseDto,
    pub action: String,
    pub deployment_id: Option<i64>,
    pub message: String,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl From<ScheduleRunResult> for ScheduleRunResponseDto {
    fn from(value: ScheduleRunResult) -> Self {
        Self {
            schedule: ScheduleResponseDto::from(value.schedule),
            action: value.action,
            deployment_id: value.deployment_id,
            message: value.message,
            stdout: value.stdout,
            stderr: value.stderr,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateScheduleRuntimePolicyDto {
    pub retry_count: i64,
    pub retry_delay_seconds: i64,
    pub missed_run_policy: MissedRunPolicy,
    pub concurrency_policy: ConcurrencyPolicy,
    pub lease_seconds: i64,
    pub notify_on_success: bool,
    pub notify_on_failure: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScheduleRuntimePolicyDto {
    pub schedule_id: i64,
    pub retry_count: i64,
    pub retry_delay_seconds: i64,
    pub missed_run_policy: MissedRunPolicy,
    pub concurrency_policy: ConcurrencyPolicy,
    pub lease_seconds: i64,
    pub notify_on_success: bool,
    pub notify_on_failure: bool,
    pub last_scheduled_at: Option<i64>,
    pub updated_at: i64,
}

impl From<ScheduleRuntimePolicies> for ScheduleRuntimePolicyDto {
    fn from(value: ScheduleRuntimePolicies) -> Self {
        Self {
            schedule_id: value.schedule_id,
            retry_count: value.retry_count,
            retry_delay_seconds: value.retry_delay_seconds,
            missed_run_policy: MissedRunPolicy::try_from(value.missed_run_policy.as_str())
                .expect("database enforces missed run policy"),
            concurrency_policy: ConcurrencyPolicy::try_from(value.concurrency_policy.as_str())
                .expect("database enforces concurrency policy"),
            lease_seconds: value.lease_seconds,
            notify_on_success: value.notify_on_success != 0,
            notify_on_failure: value.notify_on_failure != 0,
            last_scheduled_at: value.last_scheduled_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ScheduleExecutionDto {
    pub id: i64,
    pub schedule_id: i64,
    pub organization_id: Option<i64>,
    pub trigger_kind: ScheduleTriggerKind,
    pub status: ScheduleExecutionStatus,
    pub attempt: i64,
    pub scheduled_at: i64,
    pub started_at: i64,
    pub finished_at: Option<i64>,
    pub message: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl From<ScheduleExecutions> for ScheduleExecutionDto {
    fn from(value: ScheduleExecutions) -> Self {
        Self {
            id: value.id,
            schedule_id: value.schedule_id,
            organization_id: value.organization_id,
            trigger_kind: ScheduleTriggerKind::try_from(value.trigger_kind.as_str())
                .expect("database enforces schedule trigger"),
            status: ScheduleExecutionStatus::try_from(value.status.as_str())
                .expect("database enforces execution status"),
            attempt: value.attempt,
            scheduled_at: value.scheduled_at,
            started_at: value.started_at,
            finished_at: value.finished_at,
            message: value.message,
            stdout: value.stdout,
            stderr: value.stderr,
        }
    }
}
