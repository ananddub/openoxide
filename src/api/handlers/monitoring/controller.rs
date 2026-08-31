use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, MonitoringOperationsService, MonitoringOperationsServiceExt, OperationResult,
    ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
};

use super::helper::*;
use super::init::MonitoringController;

#[auto_grpc::service(connect = true, auth = true)]
impl MonitoringOperationsService for MonitoringController {
    async fn get_local_metrics<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, Empty>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let payload = serde_json::to_string(&MonitoringStatusResponseDto {
            status: "ok".into(),
            service: "openoxide monitoring service".into(),
            endpoints: vec![
                "/monitoring/server/{id}".into(),
                "/monitoring/containers/{id}".into(),
            ],
        }).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "monitoring_status".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Monitoring, CanMonitor)]
    async fn get_server_metrics<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        verify_server_organization(&self.agent_auth, request.id, auth.organization_id).await?;
        let metrics = self.service
            .server_history(request.id, 50)
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Unavailable, e))?;
        let payload = serde_json::to_string(&metrics)
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        Response::ok(ResourceResponse {
            id: request.id,
            kind: "server_metrics".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Monitoring, CanMonitor)]
    async fn get_container_metrics<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        verify_server_organization(&self.agent_auth, request.id, auth.organization_id).await?;
        let query: ContainerQueryParam = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        verify_claim_organization(auth.organization_id, query.organization_id)?;
        let metrics = self.service
            .container_history(
                request.id,
                query.app_name.as_deref().unwrap_or_default(),
                query.limit.unwrap_or(100),
            )
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e))?;
        let payload = serde_json::to_string(&metrics)
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        Response::ok(ResourceResponse {
            id: request.id,
            kind: "container_metrics".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Monitoring, CanMonitor)]
    async fn get_container_states<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        verify_server_organization(&self.agent_auth, request.id, auth.organization_id).await?;
        let states = self.service
            .container_states(request.id)
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Unavailable, e))?;
        let payload = serde_json::to_string(&states)
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        let item = ResourceResponse {
            id: request.id,
            kind: "container_states".into(),
            payload,
            ..Default::default()
        };
        Response::ok(ResourceListResponse { items: vec![item], ..Default::default() })
    }

    #[permission(Alert, CanWrite)]
    async fn create_agent_token<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: RotateMonitoringTokenDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        if dto.organization_id != auth.organization_id {
            return Err(ConnectError::new(ErrorCode::PermissionDenied, "organization does not match authenticated scope"));
        }
        if !self.agent_auth
            .server_belongs_to_organization(request.id, dto.organization_id)
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?
        {
            return Err(ConnectError::new(ErrorCode::PermissionDenied, "server is not linked to this organization"));
        }
        let token = self.agent_auth
            .rotate(request.id, dto.organization_id)
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        let resp = MonitoringTokenResponseDto {
            server_id: request.id,
            organization_id: dto.organization_id,
            token,
        };
        let payload = serde_json::to_string(&resp).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: request.id,
            kind: "monitoring_token".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Monitoring, CanMonitor)]
    async fn get_agent_status<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        verify_server_organization(&self.agent_auth, request.id, auth.organization_id).await?;
        let status = self.agent_auth
            .status(request.id)
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?
            .ok_or_else(|| ConnectError::not_found("monitoring agent is not registered"))?;
        let state = match status.last_seen_at {
            Some(ts) if chrono::Utc::now().timestamp() - ts <= 30 => "ONLINE",
            Some(_) => "OFFLINE",
            None => "NEVER_SEEN",
        };
        let dto = MonitoringAgentStatusDto {
            server_id: status.server_id,
            organization_id: status.organization_id,
            last_seen_at: status.last_seen_at,
            agent_version: status.agent_version,
            state: state.into(),
        };
        let payload = serde_json::to_string(&dto).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "get_agent_status".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Alert, CanWrite)]
    async fn start_agent<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        verify_server_organization(&self.agent_auth, request.id, auth.organization_id).await?;
        let action = self.lifecycle.start_agent(request.id).await.map_err(internal)?;
        let payload = serde_json::to_string(&action).unwrap_or_default();
        Response::ok(OperationResult {
            success: action.success,
            operation: "start_agent".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Alert, CanWrite)]
    async fn stop_agent<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        verify_server_organization(&self.agent_auth, request.id, auth.organization_id).await?;
        let action = self.lifecycle.stop_agent(request.id).await.map_err(internal)?;
        let payload = serde_json::to_string(&action).unwrap_or_default();
        Response::ok(OperationResult {
            success: action.success,
            operation: "stop_agent".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Alert, CanWrite)]
    async fn restart_agent<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        verify_server_organization(&self.agent_auth, request.id, auth.organization_id).await?;
        let action = self.lifecycle.restart_agent(request.id).await.map_err(internal)?;
        let payload = serde_json::to_string(&action).unwrap_or_default();
        Response::ok(OperationResult {
            success: action.success,
            operation: "restart_agent".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Alert, CanWrite)]
    async fn reinstall_agent<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        verify_server_organization(&self.agent_auth, request.id, auth.organization_id).await?;
        let action = self.lifecycle.reinstall_agent(request.id).await.map_err(internal)?;
        let payload = serde_json::to_string(&action).unwrap_or_default();
        Response::ok(OperationResult {
            success: action.success,
            operation: "reinstall_agent".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Monitoring, CanMonitor)]
    async fn export_server_metrics<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        verify_server_organization(&self.agent_auth, request.id, auth.organization_id).await?;
        let metrics = self.service
            .server_history(request.id, 1000)
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
        Response::ok(ResourceResponse {
            id: request.id,
            kind: "server_metrics_csv".into(),
            payload: csv,
            ..Default::default()
        })
    }

    #[permission(Monitoring, CanMonitor)]
    async fn get_prometheus_metrics<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        verify_server_organization(&self.agent_auth, request.id, auth.organization_id).await?;
        let metric = self.service
            .server_history(request.id, 1)
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Unavailable, e))?
            .into_iter()
            .next()
            .ok_or_else(|| ConnectError::not_found("no metrics available"))?;
        let payload = format!(
            "# TYPE openoxide_server_cpu_percent gauge\nopenoxide_server_cpu_percent{{server_id=\"{}\"}} {}\n# TYPE openoxide_server_memory_used_bytes gauge\nopenoxide_server_memory_used_bytes{{server_id=\"{}\"}} {}\n# TYPE openoxide_server_disk_used_bytes gauge\nopenoxide_server_disk_used_bytes{{server_id=\"{}\"}} {}\n",
            request.id, metric.cpu, request.id, metric.mem_used, request.id, metric.disk_used
        );
        Response::ok(ResourceResponse {
            id: request.id,
            kind: "prometheus_metrics".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Alert, CanWrite)]
    async fn create_maintenance_window<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateMaintenanceWindowDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_claim_organization(auth.organization_id, organization_id)?;
        if let Some(server_id) = dto.server_id {
            verify_server_organization(&self.agent_auth, server_id, organization_id).await?;
        }
        let id = self.lifecycle
            .create_window(
                organization_id,
                dto.server_id,
                dto.starts_at,
                dto.ends_at,
                &dto.reason,
            )
            .await
            .map_err(internal)?;
        let payload = serde_json::to_string(&serde_json::json!({ "id": id })).unwrap_or_default();
        Response::ok(ResourceResponse {
            id,
            kind: "maintenance_window".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Monitoring, CanMonitor)]
    async fn list_maintenance_windows<'a>(
        &'a self,
        ctx: RequestContext,
        query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let organization_id = query.organization_id.unwrap_or(auth.organization_id);
        verify_claim_organization(auth.organization_id, organization_id)?;
        let windows = self.lifecycle.windows(organization_id).await.map_err(internal)?;
        let items = windows.into_iter().map(|w| {
            let payload = serde_json::to_string(&w).unwrap_or_default();
            ResourceResponse {
                id: w.id,
                kind: "maintenance_window".into(),
                payload,
                ..Default::default()
            }
        }).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Alert, CanWrite)]
    async fn delete_maintenance_window<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_claim_organization(auth.organization_id, organization_id)?;
        if self.lifecycle.delete_window(request.id, organization_id).await.map_err(internal)? {
            Response::ok(Empty::default())
        } else {
            Err(ConnectError::not_found("maintenance window not found"))
        }
    }
}
