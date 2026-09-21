use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, OperationResult, ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
    ScheduleOperationsService, ScheduleOperationsServiceExt,
};
use crate::core::cache::CacheKey;
use crate::services::automation::schedule::schedule::{CreateScheduleDto, PatchScheduleDto};

use super::helper::*;
use super::init::ScheduleController;

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
        let content = crate::services::automation::schedule::file_log::read(&format!("schedule-{}", request.id))
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
    async fn get_execution_logs<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let execution_id = request.organization_id.unwrap_or(0);
        self.service.get_by_id(request.id).await.map_err(map_sqlx_error)?;
        self.service
            .execution_exists(request.id, execution_id)
            .await
            .map_err(map_sqlx_error)?
            .ok_or_else(|| ConnectError::not_found("schedule execution not found"))?;
        let content = crate::services::automation::schedule::file_log::read(&format!(
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
}
