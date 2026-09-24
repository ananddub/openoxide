use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, ScheduleCreateScheduleRequest, ScheduleCreateScheduleResponse,
    ScheduleDeleteScheduleRequest, ScheduleGetExecutionLogsRequest, ScheduleGetExecutionLogsResponse,
    ScheduleGetLogsRequest, ScheduleGetLogsResponse, ScheduleGetRuntimePolicyRequest,
    ScheduleGetRuntimePolicyResponse, ScheduleGetScheduleRequest, ScheduleGetScheduleResponse,
    ScheduleListApplicationSchedulesRequest, ScheduleListApplicationSchedulesResponse,
    ScheduleListComposeSchedulesRequest, ScheduleListComposeSchedulesResponse,
    ScheduleListDatabaseSchedulesRequest, ScheduleListDatabaseSchedulesResponse,
    ScheduleListExecutionsRequest, ScheduleListExecutionsResponse,
    ScheduleListOrganizationSchedulesRequest, ScheduleListOrganizationSchedulesResponse,
    ScheduleListSchedulesRequest, ScheduleListSchedulesResponse, ScheduleListServerSchedulesRequest,
    ScheduleListServerSchedulesResponse, ScheduleLog, SchedulePauseScheduleRequest,
    SchedulePauseScheduleResponse, ScheduleResumeScheduleRequest, ScheduleResumeScheduleResponse,
    ScheduleRunScheduleRequest, ScheduleRunScheduleResponse, ScheduleService, ScheduleServiceExt,
    ScheduleTriggerScheduleRequest, ScheduleTriggerScheduleResponse,
    ScheduleUpdateRuntimePolicyRequest, ScheduleUpdateRuntimePolicyResponse,
    ScheduleUpdateScheduleRequest, ScheduleUpdateScheduleResponse,
};
use crate::core::cache::CacheKey;
use crate::services::automation::schedule::schedule::{CreateScheduleDto, PatchScheduleDto};

use super::helper::*;
use super::init::ScheduleController;

#[auto_grpc::service(connect = true, auth = true)]
impl ScheduleService for ScheduleController {
    #[permission(Schedule, CanRead)]
    async fn list_schedules<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, ScheduleListSchedulesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ScheduleListSchedulesResponse> + Send + use<'a>>
    {
        let items = self.service.list_enabled().await.map_err(map_sqlx_error)?;
        let schedules = items.into_iter().map(schedule_to_proto).collect();
        Response::ok(ScheduleListSchedulesResponse {
            schedules,
            ..Default::default()
        })
    }

    #[permission(Schedule, CanRead)]
    async fn get_schedule<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ScheduleGetScheduleRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ScheduleGetScheduleResponse> + Send + use<'a>>
    {
        let schedule = self.service.get_by_id(request.id).await.map_err(map_sqlx_error)?;
        Response::ok(ScheduleGetScheduleResponse {
            schedule: ::buffa::MessageField::some(schedule_to_proto(schedule)),
            ..Default::default()
        })
    }

    #[permission(Schedule, CanCreate)]
    async fn create_schedule<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ScheduleCreateScheduleRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ScheduleCreateScheduleResponse> + Send + use<'a>>
    {
        let dto = CreateScheduleDto {
            name: request.name.to_owned(),
            description: request.description.map(str::to_owned),
            cron_expression: request.cron_expression.to_owned(),
            app_name: request.app_name.map(str::to_owned),
            service_name: request.service_name.map(str::to_owned),
            shell_type: request.shell_type.map(|v| shell_type_from_proto(v).to_owned()),
            schedule_type: request.schedule_type.map(|v| schedule_type_from_proto(v).to_owned()),
            schedule_action: request
                .schedule_action
                .map(|v| schedule_action_from_proto(v).to_owned()),
            command: request.command.to_owned(),
            script: request.script.map(str::to_owned),
            timezone: request.timezone.map(str::to_owned),
            enabled: request.enabled.map(i64::from),
            application_id: request.application_id,
            compose_id: request.compose_id,
            server_id: request.server_id,
            organization_id: request.organization_id,
        };
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
        Response::ok(ScheduleCreateScheduleResponse {
            schedule: ::buffa::MessageField::some(schedule_to_proto(created)),
            ..Default::default()
        })
    }

    #[permission(Schedule, CanCreate)]
    async fn update_schedule<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ScheduleUpdateScheduleRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ScheduleUpdateScheduleResponse> + Send + use<'a>>
    {
        let dto = PatchScheduleDto {
            name: request.name.map(str::to_owned),
            description: request.description.map(str::to_owned),
            cron_expression: request.cron_expression.map(str::to_owned),
            app_name: request.app_name.map(str::to_owned),
            service_name: request.service_name.map(str::to_owned),
            shell_type: request.shell_type.map(|v| shell_type_from_proto(v).to_owned()),
            schedule_type: request.schedule_type.map(|v| schedule_type_from_proto(v).to_owned()),
            schedule_action: request
                .schedule_action
                .map(|v| schedule_action_from_proto(v).to_owned()),
            command: request.command.map(str::to_owned),
            script: request.script.map(str::to_owned),
            timezone: request.timezone.map(str::to_owned),
            enabled: request.enabled.map(i64::from),
            application_id: request.application_id,
            compose_id: request.compose_id,
            server_id: request.server_id,
            organization_id: request.organization_id,
        };
        let updated = self
            .service
            .patch(request.id, dto)
            .await
            .map_err(map_sqlx_error)?;
        self.cache.invalidate_all().await;
        refresh_schedule_runner().await;
        Response::ok(ScheduleUpdateScheduleResponse {
            schedule: ::buffa::MessageField::some(schedule_to_proto(updated)),
            ..Default::default()
        })
    }

    #[permission(Schedule, CanDelete)]
    async fn delete_schedule<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ScheduleDeleteScheduleRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.delete(request.id).await.map_err(map_sqlx_error)?;
        self.cache.invalidate_all().await;
        refresh_schedule_runner().await;
        Response::ok(Empty::default())
    }

    #[permission(Schedule, CanDeploy)]
    async fn run_schedule<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ScheduleRunScheduleRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ScheduleRunScheduleResponse> + Send + use<'a>>
    {
        let run = self.service.run_now(request.id).await.map_err(map_sqlx_error)?;
        Response::ok(ScheduleRunScheduleResponse {
            result: ::buffa::MessageField::some(run_result_to_proto(run)),
            ..Default::default()
        })
    }

    #[permission(Schedule, CanDeploy)]
    async fn pause_schedule<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SchedulePauseScheduleRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SchedulePauseScheduleResponse> + Send + use<'a>>
    {
        let schedule = self
            .service
            .set_enabled(request.id, false)
            .await
            .map_err(map_sqlx_error)?;
        self.cache.invalidate_all().await;
        refresh_schedule_runner().await;
        Response::ok(SchedulePauseScheduleResponse {
            schedule: ::buffa::MessageField::some(schedule_to_proto(schedule)),
            ..Default::default()
        })
    }

    #[permission(Schedule, CanDeploy)]
    async fn resume_schedule<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ScheduleResumeScheduleRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ScheduleResumeScheduleResponse> + Send + use<'a>>
    {
        let schedule = self
            .service
            .set_enabled(request.id, true)
            .await
            .map_err(map_sqlx_error)?;
        self.cache.invalidate_all().await;
        refresh_schedule_runner().await;
        Response::ok(ScheduleResumeScheduleResponse {
            schedule: ::buffa::MessageField::some(schedule_to_proto(schedule)),
            ..Default::default()
        })
    }

    #[permission(Schedule, CanRead)]
    async fn list_application_schedules<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ScheduleListApplicationSchedulesRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ScheduleListApplicationSchedulesResponse> + Send + use<'a>,
    > {
        let items = self
            .service
            .list_by_application(request.application_id)
            .await
            .map_err(map_sqlx_error)?;
        let schedules = items.into_iter().map(schedule_to_proto).collect();
        Response::ok(ScheduleListApplicationSchedulesResponse {
            schedules,
            ..Default::default()
        })
    }

    #[permission(Schedule, CanRead)]
    async fn list_compose_schedules<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ScheduleListComposeSchedulesRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ScheduleListComposeSchedulesResponse> + Send + use<'a>,
    > {
        let items = self
            .service
            .list_by_compose(request.compose_id)
            .await
            .map_err(map_sqlx_error)?;
        let schedules = items.into_iter().map(schedule_to_proto).collect();
        Response::ok(ScheduleListComposeSchedulesResponse {
            schedules,
            ..Default::default()
        })
    }

    #[permission(Schedule, CanRead)]
    async fn list_database_schedules<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, ScheduleListDatabaseSchedulesRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ScheduleListDatabaseSchedulesResponse> + Send + use<'a>,
    > {
        Response::ok(ScheduleListDatabaseSchedulesResponse {
            schedules: vec![],
            ..Default::default()
        })
    }

    #[permission(Schedule, CanRead)]
    async fn list_server_schedules<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ScheduleListServerSchedulesRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ScheduleListServerSchedulesResponse> + Send + use<'a>,
    > {
        let items = self
            .service
            .list_by_server(request.server_id)
            .await
            .map_err(map_sqlx_error)?;
        let schedules = items.into_iter().map(schedule_to_proto).collect();
        Response::ok(ScheduleListServerSchedulesResponse {
            schedules,
            ..Default::default()
        })
    }

    #[permission(Schedule, CanRead)]
    async fn list_organization_schedules<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ScheduleListOrganizationSchedulesRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ScheduleListOrganizationSchedulesResponse> + Send + use<'a>,
    > {
        let items = self
            .service
            .list_by_organization(request.organization_id)
            .await
            .map_err(map_sqlx_error)?;
        let schedules = items.into_iter().map(schedule_to_proto).collect();
        Response::ok(ScheduleListOrganizationSchedulesResponse {
            schedules,
            ..Default::default()
        })
    }

    #[permission(Schedule, CanDeploy)]
    async fn trigger_schedule<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ScheduleTriggerScheduleRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ScheduleTriggerScheduleResponse> + Send + use<'a>>
    {
        let run = self.service.run_now(request.id).await.map_err(map_sqlx_error)?;
        Response::ok(ScheduleTriggerScheduleResponse {
            result: ::buffa::MessageField::some(run_result_to_proto(run)),
            ..Default::default()
        })
    }

    #[permission(Schedule, CanRead)]
    async fn get_runtime_policy<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ScheduleGetRuntimePolicyRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ScheduleGetRuntimePolicyResponse> + Send + use<'a>,
    > {
        self.service.get_by_id(request.id).await.map_err(map_sqlx_error)?;
        let policy = self.service.runtime_policy(request.id).await.map_err(map_sqlx_error)?;
        Response::ok(ScheduleGetRuntimePolicyResponse {
            policy: ::buffa::MessageField::some(runtime_policy_to_proto(policy)),
            ..Default::default()
        })
    }

    #[permission(Schedule, CanDeploy)]
    async fn update_runtime_policy<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ScheduleUpdateRuntimePolicyRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ScheduleUpdateRuntimePolicyResponse> + Send + use<'a>,
    > {
        if !(0..=10).contains(&request.retry_count)
            || !(1..=86400).contains(&request.retry_delay_seconds)
            || !(30..=86400).contains(&request.lease_seconds)
        {
            return Err(ConnectError::invalid_argument("invalid schedule runtime policy"));
        }
        self.service.get_by_id(request.id).await.map_err(map_sqlx_error)?;
        let missed_run_policy = missed_run_policy_from_proto(request.missed_run_policy);
        let concurrency_policy = concurrency_policy_from_proto(request.concurrency_policy);
        let policy = self
            .service
            .update_runtime_policy(
                request.id,
                request.retry_count,
                request.retry_delay_seconds,
                missed_run_policy,
                concurrency_policy,
                request.lease_seconds,
                request.notify_on_success,
                request.notify_on_failure,
            )
            .await
            .map_err(map_sqlx_error)?;
        Response::ok(ScheduleUpdateRuntimePolicyResponse {
            policy: ::buffa::MessageField::some(runtime_policy_to_proto(policy)),
            ..Default::default()
        })
    }

    #[permission(Schedule, CanRead)]
    async fn list_executions<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ScheduleListExecutionsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ScheduleListExecutionsResponse> + Send + use<'a>>
    {
        let schedule_id = request.schedule_id;
        self.service.get_by_id(schedule_id).await.map_err(map_sqlx_error)?;
        let rows = self
            .service
            .list_executions(schedule_id, 200)
            .await
            .map_err(map_sqlx_error)?;
        let executions = rows.into_iter().map(execution_to_proto).collect();
        Response::ok(ScheduleListExecutionsResponse {
            executions,
            ..Default::default()
        })
    }

    #[permission(Schedule, CanRead)]
    async fn get_logs<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ScheduleGetLogsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ScheduleGetLogsResponse> + Send + use<'a>> {
        self.service.get_by_id(request.id).await.map_err(map_sqlx_error)?;
        let content =
            crate::services::automation::schedule::file_log::read(&format!("schedule-{}", request.id))
                .await
                .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        Response::ok(ScheduleGetLogsResponse {
            log: ::buffa::MessageField::some(ScheduleLog {
                content,
                ..Default::default()
            }),
            ..Default::default()
        })
    }

    #[permission(Schedule, CanRead)]
    async fn get_execution_logs<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ScheduleGetExecutionLogsRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ScheduleGetExecutionLogsResponse> + Send + use<'a>,
    > {
        let schedule_id = request.schedule_id;
        let execution_id = request.execution_id;
        self.service.get_by_id(schedule_id).await.map_err(map_sqlx_error)?;
        self.service
            .execution_exists(schedule_id, execution_id)
            .await
            .map_err(map_sqlx_error)?
            .then_some(())
            .ok_or_else(|| ConnectError::not_found("schedule execution not found"))?;
        let content = crate::services::automation::schedule::file_log::read(&format!(
            "schedule-{schedule_id}-execution-{execution_id}"
        ))
        .await
        .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        Response::ok(ScheduleGetExecutionLogsResponse {
            log: ::buffa::MessageField::some(ScheduleLog {
                content,
                ..Default::default()
            }),
            ..Default::default()
        })
    }
}
