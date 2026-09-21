use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    AlertOperationsService, AlertOperationsServiceExt, Empty, OperationResult,
    ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
};
use crate::services::integrations::notification::{NotificationLevel, NotificationMessage};

use super::helper::*;
use super::init::AlertRuleController;

#[auto_grpc::service(connect = true, auth = true)]
impl AlertOperationsService for AlertRuleController {
    #[permission(Alert, CanMonitor)]
    async fn list_rules<'a>(
        &'a self,
        ctx: RequestContext,
        query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let organization_id = query.organization_id.unwrap_or(0);
        verify_organization(auth.organization_id, organization_id)?;
        let rules = self.service.list_rules(organization_id).await.map_err(|e| internal(e.to_string()))?;
        let items = rules
            .into_iter()
            .filter_map(|rule| match AlertRuleResponseDto::from_model(rule) {
                Ok(dto) => {
                    let payload = serde_json::to_string(&dto).unwrap_or_default();
                    Some(ResourceResponse {
                        id: dto.id,
                        kind: "alert_rule".into(),
                        payload,
                        ..Default::default()
                    })
                }
                Err(error) => {
                    tracing::warn!(error = %error, "skipping unreadable alert rule");
                    None
                }
            })
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Alert, CanMonitor)]
    async fn list_events<'a>(
        &'a self,
        ctx: RequestContext,
        query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let organization_id = query.organization_id.unwrap_or(0);
        verify_organization(auth.organization_id, organization_id)?;
        let events = self.service
            .list_events(organization_id, 200)
            .await
            .map_err(|e| internal(e.to_string()))?
            .into_iter()
            .map(AlertEventResponseDto::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e))?;
        let items = events.into_iter().map(|dto| {
            let payload = serde_json::to_string(&dto).unwrap_or_default();
            ResourceResponse {
                id: dto.id,
                kind: "alert_event".into(),
                payload,
                ..Default::default()
            }
        }).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Alert, CanMonitor)]
    async fn get_rule<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        verify_organization(auth.organization_id, organization_id)?;
        let rule = self.service
            .get_rule(request.id, organization_id)
            .await
            .map_err(|e| internal(e.to_string()))?
            .ok_or_else(|| ConnectError::not_found("alert rule not found"))?;
        let dto = AlertRuleResponseDto::from_model(rule)
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e))?;
        let payload = serde_json::to_string(&dto).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: dto.id,
            kind: "alert_rule".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Alert, CanWrite)]
    async fn create_rule<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateAlertRuleDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        verify_organization(auth.organization_id, dto.organization_id)?;
        let organization_id = dto.organization_id;
        let id = self.service.create_rule(dto.into_model()).await.map_err(internal)?;
        let created = self.service
            .get_rule(id, organization_id)
            .await
            .map_err(internal)?
            .ok_or_else(|| internal("created rule could not be read back".to_string()))?;
        let resp = AlertRuleResponseDto::from_model(created)
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e))?;
        let payload = serde_json::to_string(&resp).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: resp.id,
            kind: "alert_rule".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Alert, CanWrite)]
    async fn update_rule<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        verify_organization(auth.organization_id, organization_id)?;
        self.service
            .get_rule(request.id, organization_id)
            .await
            .map_err(|e| internal(e.to_string()))?
            .ok_or_else(|| ConnectError::not_found("alert rule not found"))?;
        let dto: UpdateAlertRuleDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service
            .update_rule(request.id, dto.into_model(request.id, organization_id))
            .await
            .map_err(internal)?;
        let updated = self.service
            .get_rule(request.id, organization_id)
            .await
            .map_err(internal)?
            .ok_or_else(|| ConnectError::not_found("alert rule not found"))?;
        let resp = AlertRuleResponseDto::from_model(updated)
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e))?;
        let payload = serde_json::to_string(&resp).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: resp.id,
            kind: "alert_rule".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Alert, CanWrite)]
    async fn delete_rule<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        verify_organization(auth.organization_id, organization_id)?;
        let removed = self.service
            .delete_rule(request.id, organization_id)
            .await
            .map_err(internal)?;
        if removed {
            Response::ok(Empty::default())
        } else {
            Err(ConnectError::not_found("alert rule not found"))
        }
    }

    #[permission(Alert, CanWrite)]
    async fn test_rule<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: TestNotificationDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let message = NotificationMessage::new(
            "Test alert",
            "This is a test from OpenOxide. If you can read it, alerts will reach you.",
        )
        .level(NotificationLevel::Info)
        .field("Source", "alert configuration test");

        let resp = match self.notifications.send_test(dto.notification_id, &message).await {
            Ok(()) => TestNotificationResponseDto {
                delivered: true,
                message: "notification delivered".to_string(),
            },
            Err(error) => TestNotificationResponseDto {
                delivered: false,
                message: error,
            },
        };
        let payload = serde_json::to_string(&resp).unwrap_or_default();
        Response::ok(OperationResult {
            success: resp.delivered,
            operation: "test_rule".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Alert, CanWrite)]
    async fn acknowledge_event<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        if self.lifecycle
            .acknowledge(request.id, organization_id, auth.user_id())
            .await
            .map_err(|e| internal(e.to_string()))?
        {
            Response::ok(OperationResult {
                success: true,
                operation: "acknowledge_event".into(),
                ..Default::default()
            })
        } else {
            Err(ConnectError::not_found("alert event not found"))
        }
    }

    #[permission(Alert, CanWrite)]
    async fn silence_event<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: SilenceAlertDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let organization_id = request.organization_id.unwrap_or(0);
        if self.lifecycle
            .silence(request.id, organization_id, dto.until)
            .await
            .map_err(|e| internal(e.to_string()))?
        {
            Response::ok(OperationResult {
                success: true,
                operation: "silence_event".into(),
                ..Default::default()
            })
        } else {
            Err(ConnectError::not_found("alert event not found"))
        }
    }
}
