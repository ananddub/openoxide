use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::services::schedule::types::{
    ConcurrencyPolicy, MissedRunPolicy, ScheduleExecutionStatus, ScheduleTriggerKind,
};
use crate::{db::models::schedules::Schedule, services::schedule::ScheduleRunResult};

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct ScheduleLogDto {
    pub content: String,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateScheduleDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub cron_expression: String,
    #[validate(length(min = 1, max = 255))]
    pub app_name: Option<String>,
    #[validate(length(max = 255))]
    pub service_name: Option<String>,
    pub shell_type: Option<String>,
    pub schedule_type: Option<String>,
    pub schedule_action: Option<String>,
    #[validate(length(min = 1))]
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

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchScheduleDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub cron_expression: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub app_name: Option<String>,
    #[validate(length(max = 255))]
    pub service_name: Option<String>,
    pub shell_type: Option<String>,
    pub schedule_type: Option<String>,
    pub schedule_action: Option<String>,
    #[validate(length(min = 1))]
    pub command: Option<String>,
    pub script: Option<String>,
    pub timezone: Option<String>,
    pub enabled: Option<i64>,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub server_id: Option<i64>,
    pub organization_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct ScheduleResponseDto {
    pub id: Option<i64>,
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
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub server_id: Option<i64>,
    pub organization_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Schedule> for ScheduleResponseDto {
    fn from(value: Schedule) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            cron_expression: value.cron_expression,
            app_name: value.app_name,
            service_name: value.service_name,
            shell_type: value.shell_type,
            schedule_type: value.schedule_type,
            schedule_action: value.schedule_action,
            command: value.command,
            script: value.script,
            timezone: value.timezone,
            enabled: value.enabled,
            application_id: value.application_id,
            compose_id: value.compose_id,
            server_id: value.server_id,
            organization_id: value.organization_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
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

#[derive(Debug, Clone, Deserialize, poem_openapi::Object)]
pub struct UpdateScheduleRuntimePolicyDto {
    pub retry_count: i64,
    pub retry_delay_seconds: i64,
    pub missed_run_policy: MissedRunPolicy,
    pub concurrency_policy: ConcurrencyPolicy,
    pub lease_seconds: i64,
    pub notify_on_success: bool,
    pub notify_on_failure: bool,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
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

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
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

impl From<crate::db::repository::schedule_runtime::ScheduleRuntimePolicy>
    for ScheduleRuntimePolicyDto
{
    fn from(value: crate::db::repository::schedule_runtime::ScheduleRuntimePolicy) -> Self {
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

impl From<crate::db::repository::schedule_runtime::ScheduleExecution> for ScheduleExecutionDto {
    fn from(value: crate::db::repository::schedule_runtime::ScheduleExecution) -> Self {
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
