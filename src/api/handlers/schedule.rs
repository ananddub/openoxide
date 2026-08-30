use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, OperationResult, ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
    ScheduleOperationsService, ScheduleOperationsServiceExt, StreamQuery,
};
use crate::core::cache::{AppStateCache, CacheKey};
use crate::services::schedule::ScheduleService;
use crate::services::schedule::schedule::{CreateScheduleDto, PatchScheduleDto};

async fn refresh_schedule_runner() {
    if let Ok(runner) = auto_di::resolve::<crate::services::schedule::ScheduleRunner>().await {
        if let Err(error) = runner.refresh_jobs().await {
            tracing::error!(%error, "could not refresh scheduler after schedule mutation");
        }
    }
}

pub struct ScheduleController {
    service: Arc<ScheduleService>,
    cache: Arc<AppStateCache>,
}

#[singleton]
impl ScheduleController {
    pub fn new(service: Arc<ScheduleService>, cache: Arc<AppStateCache>) -> Self {
        Self { service, cache }
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ConnectError {
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

fn schedule_to_resource(dto: ScheduleResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "schedule".into(),
        payload,
        ..Default::default()
    }
}

fn execution_to_resource(dto: ScheduleExecutionDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "schedule_execution".into(),
        payload,
        ..Default::default()
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl ScheduleOperationsService for ScheduleController {
    #[permission(Schedule, CanRead)]
    async fn get_schedule<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let schedule = self.service.get_by_id(request.id).await.map_err(map_sqlx_error)?;
        Response::ok(schedule_to_resource(ScheduleResponseDto::from(schedule)))
    }

    #[permission(Schedule, CanRead)]
    async fn list_application_schedules<'a>(
        &'a self,
        ctx: RequestContext,
        query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let application_id = query.organization_id.unwrap_or(0);
        let items = self.service.list_by_application(application_id).await.map_err(map_sqlx_error)?;
        let items = items.into_iter().map(ScheduleResponseDto::from).map(schedule_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Schedule, CanRead)]
    async fn list_compose_schedules<'a>(
        &'a self,
        ctx: RequestContext,
        query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let compose_id = query.organization_id.unwrap_or(0);
        let items = self.service.list_by_compose(compose_id).await.map_err(map_sqlx_error)?;
        let items = items.into_iter().map(ScheduleResponseDto::from).map(schedule_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Schedule, CanRead)]
    async fn list_database_schedules<'a>(
        &'a self,
        _ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        Response::ok(ResourceListResponse { items: vec![], ..Default::default() })
    }

    #[permission(Schedule, CanRead)]
    async fn list_server_schedules<'a>(
        &'a self,
        ctx: RequestContext,
        query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let server_id = query.organization_id.unwrap_or(0);
        let items = self.service.list_by_server(server_id).await.map_err(map_sqlx_error)?;
        let items = items.into_iter().map(ScheduleResponseDto::from).map(schedule_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Schedule, CanRead)]
    async fn list_organization_schedules<'a>(
        &'a self,
        ctx: RequestContext,
        query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let organization_id = query.organization_id.unwrap_or(0);
        let items = self.service.list_by_organization(organization_id).await.map_err(map_sqlx_error)?;
        let items = items.into_iter().map(ScheduleResponseDto::from).map(schedule_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Schedule, CanCreate)]
    async fn create_schedule<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateScheduleDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let app_id = dto.application_id;
        let comp_id = dto.compose_id;
        let created = self.service.create(dto).await.map_err(map_sqlx_error)?;
        if let Some(id) = app_id {
            self.cache.invalidate(&CacheKey::SchedulesApp(id)).await;
        }
        if let Some(id) = comp_id {
            self.cache.invalidate(&CacheKey::SchedulesCompose(id)).await;
        }
        refresh_schedule_runner().await;
        Response::ok(schedule_to_resource(ScheduleResponseDto::from(created)))
    }

    #[permission(Schedule, CanCreate)]
    async fn update_schedule<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: PatchScheduleDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let updated = self.service.patch(request.id, dto).await.map_err(map_sqlx_error)?;
        self.cache.invalidate_all().await;
        refresh_schedule_runner().await;
        Response::ok(schedule_to_resource(ScheduleResponseDto::from(updated)))
    }

    #[permission(Schedule, CanDelete)]
    async fn delete_schedule<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.delete(request.id).await.map_err(map_sqlx_error)?;
        self.cache.invalidate_all().await;
        refresh_schedule_runner().await;
        Response::ok(Empty::default())
    }

    #[permission(Schedule, CanDeploy)]
    async fn trigger_schedule<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let run = self.service.run_now(request.id).await.map_err(map_sqlx_error)?;
        let dto = ScheduleRunResponseDto::from(run);
        let payload = serde_json::to_string(&dto).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "trigger_schedule".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Schedule, CanRead)]
    async fn get_runtime_policy<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        self.service.get_by_id(request.id).await.map_err(map_sqlx_error)?;
        let policy = self.service.runtime_policy(request.id).await.map_err(map_sqlx_error)?;
        let dto = ScheduleRuntimePolicyDto::from(policy);
        let payload = serde_json::to_string(&dto).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: request.id,
            kind: "schedule_runtime_policy".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Schedule, CanDeploy)]
    async fn update_runtime_policy<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: UpdateScheduleRuntimePolicyDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        if !(0..=10).contains(&dto.retry_count)
            || !(1..=86400).contains(&dto.retry_delay_seconds)
            || !(30..=86400).contains(&dto.lease_seconds)
        {
            return Err(ConnectError::invalid_argument("invalid schedule runtime policy"));
        }
        self.service.get_by_id(request.id).await.map_err(map_sqlx_error)?;
        let policy = self.service.update_runtime_policy(
            request.id,
            dto.retry_count,
            dto.retry_delay_seconds,
            dto.missed_run_policy.as_str(),
            dto.concurrency_policy.as_str(),
            dto.lease_seconds,
            dto.notify_on_success,
            dto.notify_on_failure,
        ).await.map_err(map_sqlx_error)?;
        let resp = ScheduleRuntimePolicyDto::from(policy);
        let payload = serde_json::to_string(&resp).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: request.id,
            kind: "schedule_runtime_policy".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Schedule, CanRead)]
    async fn list_executions<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let schedule_id = request.organization_id.unwrap_or(0);
        self.service.get_by_id(schedule_id).await.map_err(map_sqlx_error)?;
        let rows = self.service.list_executions(schedule_id, 200).await.map_err(map_sqlx_error)?;
        let items = rows.into_iter().map(ScheduleExecutionDto::from).map(execution_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Schedule, CanRead)]
    async fn get_logs<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        self.service.get_by_id(request.id).await.map_err(map_sqlx_error)?;
        let content = crate::services::schedule::file_log::read(&format!("schedule-{}", request.id))
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        let dto = ScheduleLogDto { content };
        let payload = serde_json::to_string(&dto).unwrap_or_default();
        let item = ResourceResponse {
            id: request.id,
            kind: "schedule_log".into(),
            payload,
            ..Default::default()
        };
        Response::ok(ResourceListResponse { items: vec![item], ..Default::default() })
    }

    #[permission(Schedule, CanRead)]
    async fn stream_logs(
        &self,
        _ctx: RequestContext,
        _query: ServiceRequest<'_, StreamQuery>,
    ) -> ServiceResult<connectrpc::ServiceStream<impl connectrpc::Encodable<ResourceResponse> + Send + use<>>> {
        Response::stream_ok(futures::stream::empty::<
            Result<ResourceResponse, ConnectError>,
        >())
    }

    #[permission(Schedule, CanRead)]
    async fn get_execution_logs<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let execution_id = request.organization_id.unwrap_or(0);
        self.service.get_by_id(request.id).await.map_err(map_sqlx_error)?;
        let belongs = self.service.execution_exists(request.id, execution_id)
            .await.map_err(map_sqlx_error)?;
        if !belongs {
            return Err(ConnectError::not_found("schedule execution not found"));
        }
        let content = crate::services::schedule::file_log::read(&format!(
            "schedule-{}-execution-{}", request.id, execution_id
        )).await.map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        let dto = ScheduleLogDto { content };
        let payload = serde_json::to_string(&dto).unwrap_or_default();
        let item = ResourceResponse {
            id: request.id,
            kind: "schedule_execution_log".into(),
            payload,
            ..Default::default()
        };
        Response::ok(ResourceListResponse { items: vec![item], ..Default::default() })
    }

    #[permission(Schedule, CanRead)]
    async fn stream_execution_logs(
        &self,
        _ctx: RequestContext,
        _query: ServiceRequest<'_, StreamQuery>,
    ) -> ServiceResult<connectrpc::ServiceStream<impl connectrpc::Encodable<ResourceResponse> + Send + use<>>> {
        Response::stream_ok(futures::stream::empty::<
            Result<ResourceResponse, ConnectError>,
        >())
    }
}

// ── Helper DTOs for ResourceRequest/ResourceResponse payloads ────────────

use serde::{Deserialize, Serialize};

use crate::flash_gen::models::{ScheduleExecutions, ScheduleRuntimePolicies, Schedules};
use crate::services::schedule::ScheduleRunResult;
use crate::services::schedule::types::{
    ConcurrencyPolicy, MissedRunPolicy, ScheduleExecutionStatus, ScheduleTriggerKind,
};

#[derive(Debug, Clone, Serialize)]
struct ScheduleLogDto {
    content: String,
}

#[derive(Debug, Clone, Serialize)]
struct ScheduleResponseDto {
    id: i64,
    name: String,
    description: Option<String>,
    cron_expression: String,
    app_name: String,
    service_name: Option<String>,
    shell_type: String,
    schedule_type: String,
    schedule_action: String,
    command: String,
    script: Option<String>,
    timezone: Option<String>,
    enabled: i64,
    service_id: Option<i64>,
    server_id: Option<i64>,
    organization_id: Option<i64>,
    created_at: i64,
    updated_at: i64,
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
            schedule_type: value.schedule_type,
            schedule_action: value.schedule_action,
            command: value.command,
            script: value.script,
            timezone: value.timezone,
            enabled: value.enabled,
            service_id: value.service_id,
            server_id: value.server_id,
            organization_id: value.organization_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct ScheduleRunResponseDto {
    schedule: ScheduleResponseDto,
    action: String,
    deployment_id: Option<i64>,
    message: String,
    stdout: Option<String>,
    stderr: Option<String>,
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
struct UpdateScheduleRuntimePolicyDto {
    retry_count: i64,
    retry_delay_seconds: i64,
    missed_run_policy: MissedRunPolicy,
    concurrency_policy: ConcurrencyPolicy,
    lease_seconds: i64,
    notify_on_success: bool,
    notify_on_failure: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ScheduleRuntimePolicyDto {
    schedule_id: i64,
    retry_count: i64,
    retry_delay_seconds: i64,
    missed_run_policy: MissedRunPolicy,
    concurrency_policy: ConcurrencyPolicy,
    lease_seconds: i64,
    notify_on_success: bool,
    notify_on_failure: bool,
    last_scheduled_at: Option<i64>,
    updated_at: i64,
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
struct ScheduleExecutionDto {
    id: i64,
    schedule_id: i64,
    organization_id: Option<i64>,
    trigger_kind: ScheduleTriggerKind,
    status: ScheduleExecutionStatus,
    attempt: i64,
    scheduled_at: i64,
    started_at: i64,
    finished_at: Option<i64>,
    message: Option<String>,
    stdout: Option<String>,
    stderr: Option<String>,
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
