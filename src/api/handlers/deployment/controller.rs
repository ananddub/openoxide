use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CancelDeploymentRequest, ContainerList, ComposeLogsRequest, DeploymentEvent,
    DeploymentService, DeploymentServiceExt, Empty, GetDeploymentRequest, ListDeploymentsRequest,
    ListDeploymentsResponse, LogExport, LogTargetRequest, ResourceTargetRequest,
    SearchLogsRequest, StreamTargetRequest,
};
use crate::services::deployment::{
    CancelDeploymentResult, ComposeLogOptions, DeploymentListFilter, DockerLogOptions,
    LogSearchOptions,
};

use super::helper::*;
use super::init::DeploymentController;

#[auto_grpc::service(connect = true, auth = true)]
impl DeploymentService for DeploymentController {
    #[permission(Deployment, CanRead)]
    async fn list_deployments<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ListDeploymentsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListDeploymentsResponse> + Send + use<'a>> {
        let filter = DeploymentListFilter {
            status: request.resource_type.map(|s| s.to_owned()),
            state: None,
            application_id: None,
            compose_id: None,
            database_id: None,
            server_id: None,
            limit: request.limit.unwrap_or(50),
            offset: request.offset.unwrap_or(0),
        };
        let items = self.service.list(filter).await.map_err(map_error)?;
        let deployments = items.into_iter().map(|d| deployment_to_proto(&d)).collect();
        Response::ok(ListDeploymentsResponse { deployments, ..Default::default() })
    }

    #[permission(Logs, CanRead)]
    async fn search_logs<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SearchLogsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListDeploymentsResponse> + Send + use<'a>> {
        let items = self.service
            .search_logs(LogSearchOptions {
                query: Some(request.query.to_owned()),
                limit: 200,
            })
            .await
            .map_err(map_error)?;
        // Map log search results to the proto Deployment envelope; the
        // matched line and location ride in `status`/`resource_type`.
        let deployments = items
            .into_iter()
            .map(|r| crate::api::connect::generated::openoxide::api::v1::Deployment {
                id: r.deployment_id,
                resource_type: r.title,
                resource_id: r.deployment_id,
                status: r.line,
                error: None,
                logs: None,
                created_at: r.line_number as i64,
                updated_at: r.line_number as i64,
                ..Default::default()
            })
            .collect();
        Response::ok(ListDeploymentsResponse { deployments, ..Default::default() })
    }

    #[permission(Deployment, CanRead)]
    async fn list_active_deployments<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, Empty>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListDeploymentsResponse> + Send + use<'a>> {
        let items = self.service.list_active_components().await.map_err(map_error)?;
        let deployments = items.into_iter().map(|d| active_to_deployment(&d)).collect();
        Response::ok(ListDeploymentsResponse { deployments, ..Default::default() })
    }

    #[permission(Deployment, CanRead)]
    async fn list_running_deployments<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, Empty>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListDeploymentsResponse> + Send + use<'a>> {
        let items = self.service.list_running(50, 0).await.map_err(map_error)?;
        let deployments = items.into_iter().map(|d| deployment_to_proto(&d)).collect();
        Response::ok(ListDeploymentsResponse { deployments, ..Default::default() })
    }

    #[permission(Deployment, CanRead)]
    async fn get_deployment<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, GetDeploymentRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<crate::api::connect::generated::openoxide::api::v1::Deployment> + Send + use<'a>> {
        let item = self.service.get_by_id(request.id).await.map_err(map_error)?;
        Response::ok(deployment_to_proto(&item))
    }

    #[permission(Deployment, CanRead)]
    async fn stream_application_events(
        &self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, StreamTargetRequest>,
    ) -> ServiceResult<connectrpc::ServiceStream<impl connectrpc::Encodable<DeploymentEvent> + Send + use<>>> {
        let _ = request.id;
        Response::stream_ok(empty_event_stream())
    }

    #[permission(Deployment, CanRead)]
    async fn stream_compose_events(
        &self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, StreamTargetRequest>,
    ) -> ServiceResult<connectrpc::ServiceStream<impl connectrpc::Encodable<DeploymentEvent> + Send + use<>>> {
        let _ = request.id;
        Response::stream_ok(empty_event_stream())
    }

    #[permission(Deployment, CanRead)]
    async fn stream_database_events(
        &self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, StreamTargetRequest>,
    ) -> ServiceResult<connectrpc::ServiceStream<impl connectrpc::Encodable<DeploymentEvent> + Send + use<>>> {
        let _ = request.id;
        Response::stream_ok(empty_event_stream())
    }

    #[permission(Deployment, CanRead)]
    async fn stream_application_stats(
        &self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, StreamTargetRequest>,
    ) -> ServiceResult<connectrpc::ServiceStream<impl connectrpc::Encodable<DeploymentEvent> + Send + use<>>> {
        let _ = request.id;
        Response::stream_ok(empty_event_stream())
    }

    #[permission(Deployment, CanRead)]
    async fn stream_compose_stats(
        &self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, StreamTargetRequest>,
    ) -> ServiceResult<connectrpc::ServiceStream<impl connectrpc::Encodable<DeploymentEvent> + Send + use<>>> {
        let _ = request.id;
        Response::stream_ok(empty_event_stream())
    }

    #[permission(Deployment, CanRead)]
    async fn stream_database_stats(
        &self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, StreamTargetRequest>,
    ) -> ServiceResult<connectrpc::ServiceStream<impl connectrpc::Encodable<DeploymentEvent> + Send + use<>>> {
        let _ = request.id;
        Response::stream_ok(empty_event_stream())
    }

    #[permission(Logs, CanRead)]
    async fn stream_container_logs(
        &self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, LogTargetRequest>,
    ) -> ServiceResult<connectrpc::ServiceStream<impl connectrpc::Encodable<DeploymentEvent> + Send + use<>>> {
        let _ = &request.target;
        Response::stream_ok(empty_event_stream())
    }

    #[permission(Deployment, CanRead)]
    async fn export_container_logs<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, LogTargetRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<LogExport> + Send + use<'a>> {
        let target = request.target.to_owned();
        let server_id = request.server_id;
        let options = docker_log_options(request.tail, None, Some(false), None, None);
        let output = self.service
            .docker_container_logs(server_id, target.clone(), options)
            .await
            .map_err(map_error)?;
        let content = output.stdout + &output.stderr;
        Response::ok(LogExport {
            content,
            file_name: format!("{}.log", sanitize_filename(&target)),
            ..Default::default()
        })
    }

    #[permission(Deployment, CanRead)]
    async fn stream_docker_stats(
        &self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, Empty>,
    ) -> ServiceResult<connectrpc::ServiceStream<impl connectrpc::Encodable<DeploymentEvent> + Send + use<>>> {
        Response::stream_ok(empty_event_stream())
    }

    #[permission(Deployment, CanRead)]
    async fn list_containers<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, Empty>,
    ) -> ServiceResult<impl connectrpc::Encodable<ContainerList> + Send + use<'a>> {
        let containers = self.service.list_docker_containers(None, true).await.map_err(map_error)?;
        let containers = containers.into_iter().map(|c| crate::api::connect::generated::openoxide::api::v1::Container {
            id: c.id,
            name: c.names,
            image: c.image,
            state: c.state,
            status: Some(c.status),
            ..Default::default()
        }).collect();
        Response::ok(ContainerList { containers, ..Default::default() })
    }

    #[permission(Deployment, CanRead)]
    async fn stream_container_stats(
        &self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, LogTargetRequest>,
    ) -> ServiceResult<connectrpc::ServiceStream<impl connectrpc::Encodable<DeploymentEvent> + Send + use<>>> {
        let _ = &request.target;
        Response::stream_ok(empty_event_stream())
    }

    #[permission(Deployment, CanRead)]
    async fn stream_service_logs(
        &self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, LogTargetRequest>,
    ) -> ServiceResult<connectrpc::ServiceStream<impl connectrpc::Encodable<DeploymentEvent> + Send + use<>>> {
        let _ = &request.target;
        Response::stream_ok(empty_event_stream())
    }

    #[permission(Deployment, CanRead)]
    async fn export_service_logs<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, LogTargetRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<LogExport> + Send + use<'a>> {
        let target = request.target.to_owned();
        let server_id = request.server_id;
        let options = docker_log_options(request.tail, None, Some(false), None, None);
        let output = self.service
            .export_container_logs(server_id, &target, options)
            .await
            .map_err(map_error)?;
        let selector = DockerLogStream::All;
        let bytes = select_log_output(output, selector);
        Response::ok(LogExport {
            content: String::from_utf8_lossy(&bytes).into_owned(),
            file_name: format!("{}.log", sanitize_filename(&target)),
            ..Default::default()
        })
    }

    #[permission(Deployment, CanRead)]
    async fn stream_compose_logs(
        &self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeLogsRequest>,
    ) -> ServiceResult<connectrpc::ServiceStream<impl connectrpc::Encodable<DeploymentEvent> + Send + use<>>> {
        let _ = &request.service;
        Response::stream_ok(empty_event_stream())
    }

    #[permission(Deployment, CanRead)]
    async fn export_compose_logs<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeLogsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<LogExport> + Send + use<'a>> {
        let service_name = request.service.map(str::to_owned).unwrap_or_else(|| "compose".into());
        let server_id = request.server_id;
        let options = ComposeLogOptions {
            file: None,
            project_directory: None,
            project_name: None,
            service: request.service.map(str::to_owned),
            logs: DockerLogOptions {
                tail: 200,
                timestamps: false,
                follow: false,
                since: None,
                until: None,
            },
        };
        let output = self.service
            .export_compose_logs(server_id, options)
            .await
            .map_err(map_error)?;
        let selector = DockerLogStream::All;
        let bytes = select_log_output(output, selector);
        Response::ok(LogExport {
            content: String::from_utf8_lossy(&bytes).into_owned(),
            file_name: format!("{}.log", sanitize_filename(&service_name)),
            ..Default::default()
        })
    }

    #[permission(Deployment, CanRead)]
    async fn stream_deployment_logs(
        &self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, GetDeploymentRequest>,
    ) -> ServiceResult<connectrpc::ServiceStream<impl connectrpc::Encodable<DeploymentEvent> + Send + use<>>> {
        let _ = request.id;
        Response::stream_ok(empty_event_stream())
    }

    #[permission(Deployment, CanDeploy)]
    async fn cancel_deployment<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, CancelDeploymentRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        match self.service.cancel(request.id).await {
            Ok(CancelDeploymentResult::CancelRequested) => Response::ok(Empty::default()),
            Ok(CancelDeploymentResult::NotRunning) => Err(ConnectError::new(
                ErrorCode::FailedPrecondition,
                "deployment is not running, so it cannot be cancelled",
            )),
            Ok(CancelDeploymentResult::NotCancellable) => Err(ConnectError::invalid_argument(
                "deployment is not attached to an application or compose project",
            )),
            Ok(CancelDeploymentResult::NotActiveInThisProcess) => Err(ConnectError::new(
                ErrorCode::FailedPrecondition,
                "deployment is not active in this process; it may already be finished or recovered after restart",
            )),
            Err(error) => Err(map_error(error)),
        }
    }

    #[permission(Deployment, CanDelete)]
    async fn delete_deployment<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, GetDeploymentRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let deleted = self.service.delete(request.id).await.map_err(map_error)?;
        if deleted {
            Response::ok(Empty::default())
        } else {
            Err(ConnectError::not_found("deployment not found"))
        }
    }

    #[permission(Deployment, CanDelete)]
    async fn delete_application_deployments<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceTargetRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.clear_all(Some(request.id), None, None, None).await.map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Deployment, CanDelete)]
    async fn delete_compose_deployments<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceTargetRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.clear_all(None, Some(request.id), None, None).await.map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Deployment, CanDelete)]
    async fn delete_database_deployments<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceTargetRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.clear_all(None, None, Some(request.id), None).await.map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Deployment, CanDelete)]
    async fn clear_deployments<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, Empty>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.clear_all(None, None, None, None).await.map_err(map_error)?;
        Response::ok(Empty::default())
    }
}
