use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, MonitoringContainerState, MonitoringCreateAgentTokenRequest,
    MonitoringCreateAgentTokenResponse, MonitoringCreateMaintenanceWindowRequest, MonitoringCreateMaintenanceWindowResponse,
    MonitoringDeleteMaintenanceWindowRequest, MonitoringExportServerMetricsRequest,
    MonitoringExportServerMetricsResponse, MonitoringGetAgentStatusRequest,
    MonitoringGetAgentStatusResponse, MonitoringGetContainerMetricsRequest,
    MonitoringGetContainerMetricsResponse, MonitoringGetContainerStateRequest,
    MonitoringGetContainerStatesRequest, MonitoringGetContainerStatesResponse,
    MonitoringGetLocalMetricsRequest, MonitoringGetLocalMetricsResponse, MonitoringGetMetricsRequest,
    MonitoringGetOverviewRequest, MonitoringGetPrometheusMetricsRequest,
    MonitoringGetPrometheusMetricsResponse, MonitoringGetServerMetricsRequest,
    MonitoringGetServerMetricsResponse, MonitoringListMaintenanceWindowsRequest,
    MonitoringListMaintenanceWindowsResponse, MonitoringReinstallAgentRequest,
    MonitoringReinstallAgentResponse, MonitoringRestartAgentRequest,
    MonitoringRestartAgentResponse, MonitoringService, MonitoringServiceExt, MonitoringStartAgentRequest,
    MonitoringStartAgentResponse, MonitoringStopAgentRequest, MonitoringStopAgentResponse,
};

use super::helper::*;
use super::init::MonitoringController;

#[auto_grpc::service(connect = true, auth = true)]
impl MonitoringService for MonitoringController {
    async fn get_overview<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, MonitoringGetOverviewRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        Response::ok(Empty::default())
    }

    async fn get_metrics<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, MonitoringGetMetricsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        Response::ok(Empty::default())
    }

    async fn get_container_state<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, MonitoringGetContainerStateRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        Response::ok(Empty::default())
    }

    async fn get_local_metrics<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, MonitoringGetLocalMetricsRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<MonitoringGetLocalMetricsResponse> + Send + use<'a>,
    > {
        Response::ok(MonitoringGetLocalMetricsResponse {
            status: "ok".into(),
            service: "openoxide monitoring service".into(),
            endpoints: vec![
                "/monitoring/server/{id}".into(),
                "/monitoring/containers/{id}".into(),
            ],
            ..Default::default()
        })
    }

    #[permission(Monitoring, CanMonitor)]
    async fn get_server_metrics<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, MonitoringGetServerMetricsRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<MonitoringGetServerMetricsResponse> + Send + use<'a>,
    > {
        let server_id = request.server_id;
        verify_server_organization(&self.agent_auth, server_id, auth.organization_id).await?;
        let metrics = self
            .service
            .server_history(server_id, 50)
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Unavailable, e))?;
        let metrics = metrics.into_iter().map(server_metric_to_proto).collect();
        Response::ok(MonitoringGetServerMetricsResponse {
            metrics,
            ..Default::default()
        })
    }

    #[permission(Monitoring, CanMonitor)]
    async fn get_container_metrics<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, MonitoringGetContainerMetricsRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<MonitoringGetContainerMetricsResponse> + Send + use<'a>,
    > {
        let server_id = request.server_id;
        verify_server_organization(&self.agent_auth, server_id, auth.organization_id).await?;
        verify_claim_organization(auth.organization_id, request.organization_id)?;
        let app_name = request.app_name.unwrap_or_default();
        let metrics = self
            .service
            .container_history(server_id, app_name, request.limit.unwrap_or(100))
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e))?;
        let metrics = metrics.into_iter().map(container_metric_to_proto).collect();
        Response::ok(MonitoringGetContainerMetricsResponse {
            metrics,
            ..Default::default()
        })
    }

    #[permission(Monitoring, CanMonitor)]
    async fn get_container_states<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, MonitoringGetContainerStatesRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<MonitoringGetContainerStatesResponse> + Send + use<'a>,
    > {
        let server_id = request.server_id;
        verify_server_organization(&self.agent_auth, server_id, auth.organization_id).await?;
        let states = self
            .service
            .container_states(server_id)
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Unavailable, e))?;
        let states = states
            .into_iter()
            .map(container_state_to_proto)
            .collect::<Vec<MonitoringContainerState>>();
        Response::ok(MonitoringGetContainerStatesResponse {
            states,
            ..Default::default()
        })
    }

    #[permission(Alert, CanWrite)]
    async fn create_agent_token<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, MonitoringCreateAgentTokenRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<MonitoringCreateAgentTokenResponse> + Send + use<'a>,
    > {
        let server_id = request.server_id;
        let organization_id = request.organization_id;
        if organization_id != auth.organization_id {
            return Err(ConnectError::new(
                ErrorCode::PermissionDenied,
                "organization does not match authenticated scope",
            ));
        }
        if !self
            .agent_auth
            .server_belongs_to_organization(server_id, organization_id)
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?
        {
            return Err(ConnectError::new(
                ErrorCode::PermissionDenied,
                "server is not linked to this organization",
            ));
        }
        let token = self
            .agent_auth
            .rotate(server_id, organization_id)
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        Response::ok(MonitoringCreateAgentTokenResponse {
            server_id,
            organization_id,
            token,
            ..Default::default()
        })
    }

    #[permission(Monitoring, CanMonitor)]
    async fn get_agent_status<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, MonitoringGetAgentStatusRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<MonitoringGetAgentStatusResponse> + Send + use<'a>>
    {
        let server_id = request.server_id;
        verify_server_organization(&self.agent_auth, server_id, auth.organization_id).await?;
        let status = self
            .agent_auth
            .status(server_id)
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?
            .ok_or_else(|| ConnectError::not_found("monitoring agent is not registered"))?;
        let status = agent_status_to_proto(
            status.server_id,
            status.organization_id,
            status.last_seen_at,
            status.agent_version,
        );
        Response::ok(MonitoringGetAgentStatusResponse {
            status: ::buffa::MessageField::some(status),
            ..Default::default()
        })
    }

    #[permission(Alert, CanWrite)]
    async fn start_agent<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, MonitoringStartAgentRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<MonitoringStartAgentResponse> + Send + use<'a>>
    {
        let server_id = request.server_id;
        verify_server_organization(&self.agent_auth, server_id, auth.organization_id).await?;
        let action = self.lifecycle.start_agent(server_id).await.map_err(internal)?;
        Response::ok(MonitoringStartAgentResponse {
            action: ::buffa::MessageField::some(agent_action_to_proto(action)),
            ..Default::default()
        })
    }

    #[permission(Alert, CanWrite)]
    async fn stop_agent<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, MonitoringStopAgentRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<MonitoringStopAgentResponse> + Send + use<'a>>
    {
        let server_id = request.server_id;
        verify_server_organization(&self.agent_auth, server_id, auth.organization_id).await?;
        let action = self.lifecycle.stop_agent(server_id).await.map_err(internal)?;
        Response::ok(MonitoringStopAgentResponse {
            action: ::buffa::MessageField::some(agent_action_to_proto(action)),
            ..Default::default()
        })
    }

    #[permission(Alert, CanWrite)]
    async fn restart_agent<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, MonitoringRestartAgentRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<MonitoringRestartAgentResponse> + Send + use<'a>>
    {
        let server_id = request.server_id;
        verify_server_organization(&self.agent_auth, server_id, auth.organization_id).await?;
        let action = self.lifecycle.restart_agent(server_id).await.map_err(internal)?;
        Response::ok(MonitoringRestartAgentResponse {
            action: ::buffa::MessageField::some(agent_action_to_proto(action)),
            ..Default::default()
        })
    }

    #[permission(Alert, CanWrite)]
    async fn reinstall_agent<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, MonitoringReinstallAgentRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<MonitoringReinstallAgentResponse> + Send + use<'a>,
    > {
        let server_id = request.server_id;
        verify_server_organization(&self.agent_auth, server_id, auth.organization_id).await?;
        let action = self.lifecycle.reinstall_agent(server_id).await.map_err(internal)?;
        Response::ok(MonitoringReinstallAgentResponse {
            action: ::buffa::MessageField::some(agent_action_to_proto(action)),
            ..Default::default()
        })
    }

    #[permission(Monitoring, CanMonitor)]
    async fn export_server_metrics<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, MonitoringExportServerMetricsRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<MonitoringExportServerMetricsResponse> + Send + use<'a>,
    > {
        let server_id = request.server_id;
        verify_server_organization(&self.agent_auth, server_id, auth.organization_id).await?;
        let metrics = self
            .service
            .server_history(server_id, 1000)
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Unavailable, e))?;
        let mut csv = String::from(
            "timestamp,server_id,cpu,mem_used,mem_total,disk_used,total_disk,network_in,network_out\n",
        );
        for metric in metrics {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                metric.timestamp.unwrap_or_default(),
                metric.server_id,
                metric.cpu,
                metric.mem_used,
                metric.mem_total,
                metric.disk_used,
                metric.total_disk,
                metric.network_in,
                metric.network_out
            ));
        }
        Response::ok(MonitoringExportServerMetricsResponse {
            csv,
            ..Default::default()
        })
    }

    #[permission(Monitoring, CanMonitor)]
    async fn get_prometheus_metrics<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, MonitoringGetPrometheusMetricsRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<MonitoringGetPrometheusMetricsResponse> + Send + use<'a>,
    > {
        let server_id = request.server_id;
        verify_server_organization(&self.agent_auth, server_id, auth.organization_id).await?;
        let metric = self
            .service
            .server_history(server_id, 1)
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Unavailable, e))?
            .into_iter()
            .next()
            .ok_or_else(|| ConnectError::not_found("no metrics available"))?;
        let prometheus_text = format!(
            "# TYPE openoxide_server_cpu_percent gauge\nopenoxide_server_cpu_percent{{server_id=\"{}\"}} {}\n# TYPE openoxide_server_memory_used_bytes gauge\nopenoxide_server_memory_used_bytes{{server_id=\"{}\"}} {}\n# TYPE openoxide_server_disk_used_bytes gauge\nopenoxide_server_disk_used_bytes{{server_id=\"{}\"}} {}\n",
            server_id, metric.cpu, server_id, metric.mem_used, server_id, metric.disk_used
        );
        Response::ok(MonitoringGetPrometheusMetricsResponse {
            prometheus_text,
            ..Default::default()
        })
    }

    #[permission(Alert, CanWrite)]
    async fn create_maintenance_window<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, MonitoringCreateMaintenanceWindowRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<MonitoringCreateMaintenanceWindowResponse> + Send + use<'a>,
    > {
        let reason = request.reason.to_owned();
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_claim_organization(auth.organization_id, organization_id)?;
        if let Some(server_id) = request.server_id {
            verify_server_organization(&self.agent_auth, server_id, organization_id).await?;
        }
        let id = self
            .lifecycle
            .create_window(
                organization_id,
                request.server_id,
                request.starts_at,
                request.ends_at,
                &reason,
            )
            .await
            .map_err(internal)?;
        Response::ok(MonitoringCreateMaintenanceWindowResponse {
            id,
            ..Default::default()
        })
    }

    #[permission(Monitoring, CanMonitor)]
    async fn list_maintenance_windows<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, MonitoringListMaintenanceWindowsRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<MonitoringListMaintenanceWindowsResponse> + Send + use<'a>,
    > {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_claim_organization(auth.organization_id, organization_id)?;
        let windows = self
            .lifecycle
            .windows(organization_id)
            .await
            .map_err(internal)?;
        let windows = windows.into_iter().map(maintenance_window_to_proto).collect();
        Response::ok(MonitoringListMaintenanceWindowsResponse {
            windows,
            ..Default::default()
        })
    }

    #[permission(Alert, CanWrite)]
    async fn delete_maintenance_window<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, MonitoringDeleteMaintenanceWindowRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_claim_organization(auth.organization_id, organization_id)?;
        if self
            .lifecycle
            .delete_window(request.id, organization_id)
            .await
            .map_err(internal)?
        {
            Response::ok(Empty::default())
        } else {
            Err(ConnectError::not_found("maintenance window not found"))
        }
    }
}
