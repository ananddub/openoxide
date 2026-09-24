use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    Schedule as ScheduleProto, ScheduleAction as ProtoScheduleAction,
    ScheduleConcurrencyPolicy as ProtoScheduleConcurrencyPolicy,
    ScheduleExecution as ScheduleExecutionProto,
    ScheduleExecutionStatus as ProtoScheduleExecutionStatus,
    ScheduleMissedRunPolicy as ProtoScheduleMissedRunPolicy,
    ScheduleRun as ScheduleRunProto, ScheduleRuntimePolicy as ScheduleRuntimePolicyProto,
    ScheduleShellType as ProtoScheduleShellType, ScheduleTriggerKind as ProtoScheduleTriggerKind,
    ScheduleType as ProtoScheduleType,
};
use crate::flash_gen::models::{ScheduleExecutions, ScheduleRuntimePolicies, Schedules};
use crate::services::automation::schedule::ScheduleRunResult;

pub async fn refresh_schedule_runner() {
    if let Ok(runner) =
        auto_di::resolve::<crate::services::automation::schedule::ScheduleRunner>().await
    {
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

fn shell_type_to_proto(value: &str) -> ProtoScheduleShellType {
    match value {
        "BASH" => ProtoScheduleShellType::Bash,
        "SH" => ProtoScheduleShellType::Sh,
        _ => ProtoScheduleShellType::Unspecified,
    }
}

fn schedule_type_to_proto(value: &str) -> ProtoScheduleType {
    match value {
        "APPLICATION" => ProtoScheduleType::Application,
        "COMPOSE" => ProtoScheduleType::Compose,
        "DATABASE" => ProtoScheduleType::Database,
        "SERVER" => ProtoScheduleType::Server,
        "DOKPANEL-SERVER" => ProtoScheduleType::DokpanelServer,
        _ => ProtoScheduleType::Unspecified,
    }
}

fn schedule_action_to_proto(value: &str) -> ProtoScheduleAction {
    match value {
        "EXEC" => ProtoScheduleAction::Exec,
        "DEPLOY" => ProtoScheduleAction::Deploy,
        "REDEPLOY" => ProtoScheduleAction::Redeploy,
        "REBUILD" => ProtoScheduleAction::Rebuild,
        "RELOAD" => ProtoScheduleAction::Reload,
        "START" => ProtoScheduleAction::Start,
        "STOP" => ProtoScheduleAction::Stop,
        _ => ProtoScheduleAction::Unspecified,
    }
}

fn trigger_kind_to_proto(value: &str) -> ProtoScheduleTriggerKind {
    match value {
        "CRON" => ProtoScheduleTriggerKind::Cron,
        "MANUAL" => ProtoScheduleTriggerKind::Manual,
        "MISSED" => ProtoScheduleTriggerKind::Missed,
        "RETRY" => ProtoScheduleTriggerKind::Retry,
        _ => ProtoScheduleTriggerKind::Unspecified,
    }
}

fn execution_status_to_proto(value: &str) -> ProtoScheduleExecutionStatus {
    match value {
        "RUNNING" => ProtoScheduleExecutionStatus::Running,
        "SUCCEEDED" => ProtoScheduleExecutionStatus::Succeeded,
        "FAILED" => ProtoScheduleExecutionStatus::Failed,
        "SKIPPED" => ProtoScheduleExecutionStatus::Skipped,
        _ => ProtoScheduleExecutionStatus::Unspecified,
    }
}

fn missed_run_policy_to_proto(value: &str) -> ProtoScheduleMissedRunPolicy {
    match value {
        "SKIP" => ProtoScheduleMissedRunPolicy::Skip,
        "RUN_ONCE" => ProtoScheduleMissedRunPolicy::RunOnce,
        _ => ProtoScheduleMissedRunPolicy::Unspecified,
    }
}

fn concurrency_policy_to_proto(value: &str) -> ProtoScheduleConcurrencyPolicy {
    match value {
        "SKIP" => ProtoScheduleConcurrencyPolicy::Skip,
        "QUEUE" => ProtoScheduleConcurrencyPolicy::Queue,
        "ALLOW" => ProtoScheduleConcurrencyPolicy::Allow,
        _ => ProtoScheduleConcurrencyPolicy::Unspecified,
    }
}

// ── proto → stored-string conversions for write inputs ────────────────

pub fn shell_type_from_proto(value: ::buffa::EnumValue<ProtoScheduleShellType>) -> &'static str {
    match value {
        ::buffa::EnumValue::Known(ProtoScheduleShellType::Bash) => "BASH",
        ::buffa::EnumValue::Known(ProtoScheduleShellType::Sh) => "SH",
        _ => "BASH",
    }
}

pub fn schedule_type_from_proto(value: ::buffa::EnumValue<ProtoScheduleType>) -> &'static str {
    match value {
        ::buffa::EnumValue::Known(ProtoScheduleType::Application) => "APPLICATION",
        ::buffa::EnumValue::Known(ProtoScheduleType::Compose) => "COMPOSE",
        ::buffa::EnumValue::Known(ProtoScheduleType::Database) => "DATABASE",
        ::buffa::EnumValue::Known(ProtoScheduleType::Server) => "SERVER",
        ::buffa::EnumValue::Known(ProtoScheduleType::DokpanelServer) => "DOKPANEL-SERVER",
        _ => "APPLICATION",
    }
}

pub fn schedule_action_from_proto(value: ::buffa::EnumValue<ProtoScheduleAction>) -> &'static str {
    match value {
        ::buffa::EnumValue::Known(ProtoScheduleAction::Exec) => "EXEC",
        ::buffa::EnumValue::Known(ProtoScheduleAction::Deploy) => "DEPLOY",
        ::buffa::EnumValue::Known(ProtoScheduleAction::Redeploy) => "REDEPLOY",
        ::buffa::EnumValue::Known(ProtoScheduleAction::Rebuild) => "REBUILD",
        ::buffa::EnumValue::Known(ProtoScheduleAction::Reload) => "RELOAD",
        ::buffa::EnumValue::Known(ProtoScheduleAction::Start) => "START",
        ::buffa::EnumValue::Known(ProtoScheduleAction::Stop) => "STOP",
        _ => "EXEC",
    }
}

pub fn missed_run_policy_from_proto(
    value: ::buffa::EnumValue<ProtoScheduleMissedRunPolicy>,
) -> &'static str {
    match value {
        ::buffa::EnumValue::Known(ProtoScheduleMissedRunPolicy::Skip) => "SKIP",
        ::buffa::EnumValue::Known(ProtoScheduleMissedRunPolicy::RunOnce) => "RUN_ONCE",
        _ => "SKIP",
    }
}

pub fn concurrency_policy_from_proto(
    value: ::buffa::EnumValue<ProtoScheduleConcurrencyPolicy>,
) -> &'static str {
    match value {
        ::buffa::EnumValue::Known(ProtoScheduleConcurrencyPolicy::Skip) => "SKIP",
        ::buffa::EnumValue::Known(ProtoScheduleConcurrencyPolicy::Queue) => "QUEUE",
        ::buffa::EnumValue::Known(ProtoScheduleConcurrencyPolicy::Allow) => "ALLOW",
        _ => "SKIP",
    }
}

// ── row → proto mappers ───────────────────────────────────────────────

pub fn schedule_to_proto(value: Schedules) -> ScheduleProto {
    ScheduleProto {
        id: value.id,
        name: value.name,
        description: value.description,
        cron_expression: value.cron_expression,
        app_name: value.app_name,
        service_name: value.service_name,
        shell_type: ::buffa::EnumValue::Known(shell_type_to_proto(&value.shell_type)),
        schedule_type: ::buffa::EnumValue::Known(schedule_type_to_proto(&value.schedule_type)),
        schedule_action: ::buffa::EnumValue::Known(schedule_action_to_proto(
            &value.schedule_action,
        )),
        command: value.command,
        script: value.script,
        timezone: value.timezone,
        enabled: value.enabled != 0,
        service_id: value.service_id,
        // Typed linkage mirrors derived from (schedule_type, service_id);
        // exactly one is set, matching the previous payload behaviour.
        application_id: (value.schedule_type == "APPLICATION")
            .then_some(value.service_id)
            .flatten(),
        compose_id: (value.schedule_type == "COMPOSE")
            .then_some(value.service_id)
            .flatten(),
        database_id: (value.schedule_type == "DATABASE")
            .then_some(value.service_id)
            .flatten(),
        server_id: value.server_id,
        organization_id: value.organization_id,
        created_at: value.created_at,
        updated_at: value.updated_at,
        ..Default::default()
    }
}

pub fn run_result_to_proto(value: ScheduleRunResult) -> ScheduleRunProto {
    ScheduleRunProto {
        schedule: ::buffa::MessageField::some(schedule_to_proto(value.schedule)),
        action: value.action,
        deployment_id: value.deployment_id,
        message: value.message,
        stdout: value.stdout,
        stderr: value.stderr,
        ..Default::default()
    }
}

pub fn runtime_policy_to_proto(value: ScheduleRuntimePolicies) -> ScheduleRuntimePolicyProto {
    ScheduleRuntimePolicyProto {
        schedule_id: value.schedule_id,
        retry_count: value.retry_count,
        retry_delay_seconds: value.retry_delay_seconds,
        missed_run_policy: ::buffa::EnumValue::Known(missed_run_policy_to_proto(
            &value.missed_run_policy,
        )),
        concurrency_policy: ::buffa::EnumValue::Known(concurrency_policy_to_proto(
            &value.concurrency_policy,
        )),
        lease_seconds: value.lease_seconds,
        notify_on_success: value.notify_on_success != 0,
        notify_on_failure: value.notify_on_failure != 0,
        last_scheduled_at: value.last_scheduled_at,
        updated_at: value.updated_at,
        ..Default::default()
    }
}

pub fn execution_to_proto(value: ScheduleExecutions) -> ScheduleExecutionProto {
    ScheduleExecutionProto {
        id: value.id,
        schedule_id: value.schedule_id,
        organization_id: value.organization_id,
        trigger_kind: ::buffa::EnumValue::Known(trigger_kind_to_proto(&value.trigger_kind)),
        status: ::buffa::EnumValue::Known(execution_status_to_proto(&value.status)),
        attempt: value.attempt,
        scheduled_at: value.scheduled_at,
        started_at: value.started_at,
        finished_at: value.finished_at,
        message: value.message,
        stdout: value.stdout,
        stderr: value.stderr,
        ..Default::default()
    }
}
