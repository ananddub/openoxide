use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};
use serde::{Deserialize, Serialize};

use crate::api::connect::generated::openoxide::api::v1::{
    AlertOperationsService, AlertOperationsServiceExt, Empty, OperationResult,
    ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
};
use crate::services::alert::AlertRule;
use crate::services::{
    alert::{AlertEventState, AlertService, MetricKind, Operator, RuleState, TargetKind},
    monitoring::lifecycle::MonitoringLifecycleService,
    notification::{NotificationLevel, NotificationMessage, NotificationService},
};

pub struct AlertRuleController {
    service: Arc<AlertService>,
    notifications: Arc<NotificationService>,
    lifecycle: Arc<MonitoringLifecycleService>,
}

#[singleton]
impl AlertRuleController {
    pub fn new(
        service: Arc<AlertService>,
        notifications: Arc<NotificationService>,
        lifecycle: Arc<MonitoringLifecycleService>,
    ) -> Self {
        Self {
            service,
            notifications,
            lifecycle,
        }
    }
}

fn internal(error: String) -> ConnectError {
    tracing::error!(error = %error, "alert rule operation failed");
    ConnectError::new(ErrorCode::Internal, error)
}

fn verify_organization(authenticated: i64, requested: i64) -> Result<(), ConnectError> {
    if authenticated == requested {
        Ok(())
    } else {
        Err(ConnectError::new(
            ErrorCode::PermissionDenied,
            "organization does not match authenticated scope",
        ))
    }
}

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

// ── Alert rule payload DTOs ───────────────────────────────────────

#[derive(Debug, Deserialize)]
struct CreateAlertRuleDto {
    name: String,
    target_kind: TargetKind,
    target_id: i64,
    metric: MetricKind,
    operator: Operator,
    threshold: f64,
    /// Seconds a breach must persist. Also drives the post-fire cooldown.
    duration_seconds: i32,
    organization_id: i64,
    #[serde(default = "default_enabled")]
    enabled: bool,
}

fn default_enabled() -> bool {
    true
}

impl CreateAlertRuleDto {
    fn into_model(self) -> AlertRule {
        AlertRule {
            id: None,
            name: self.name,
            target_type: self.target_kind.as_str().to_owned(),
            target_id: self.target_id,
            metric_name: self.metric.as_str().to_owned(),
            operator: self.operator.as_str().to_owned(),
            threshold: self.threshold,
            duration_seconds: self.duration_seconds,
            // Rules dispatch through the organization's configured channels, so
            // this column carries no routing decision of its own.
            notification_channel: "SYSTEM".to_owned(),
            enabled: state_from(self.enabled).as_flag(),
            organization_id: self.organization_id,
            created_at: 0,
            updated_at: 0,
        }
    }
}

#[derive(Debug, Deserialize)]
struct UpdateAlertRuleDto {
    name: String,
    target_kind: TargetKind,
    target_id: i64,
    metric: MetricKind,
    operator: Operator,
    threshold: f64,
    duration_seconds: i32,
    enabled: bool,
}

impl UpdateAlertRuleDto {
    /// `organization_id` comes from the existing row rather than the request, so
    /// an update cannot move a rule between tenants.
    fn into_model(self, id: i64, organization_id: i64) -> AlertRule {
        AlertRule {
            id: Some(id),
            name: self.name,
            target_type: self.target_kind.as_str().to_owned(),
            target_id: self.target_id,
            metric_name: self.metric.as_str().to_owned(),
            operator: self.operator.as_str().to_owned(),
            threshold: self.threshold,
            duration_seconds: self.duration_seconds,
            notification_channel: "SYSTEM".to_owned(),
            enabled: state_from(self.enabled).as_flag(),
            organization_id,
            created_at: 0,
            updated_at: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct AlertRuleResponseDto {
    id: i64,
    name: String,
    target_kind: TargetKind,
    target_id: i64,
    metric: MetricKind,
    operator: Operator,
    threshold: f64,
    duration_seconds: i32,
    enabled: bool,
    organization_id: i64,
    created_at: i64,
    updated_at: i64,
}

impl AlertRuleResponseDto {
    /// Fails when a stored row holds a value no longer in the enums, rather than
    /// guessing a default that would silently change what the rule watches.
    fn from_model(rule: AlertRule) -> Result<Self, String> {
        Ok(Self {
            id: rule.id.ok_or("persisted alert rule must have an id")?,
            name: rule.name,
            target_kind: rule.target_type.parse().map_err(|e| format!("{e}"))?,
            target_id: rule.target_id,
            metric: rule.metric_name.parse().map_err(|e| format!("{e}"))?,
            operator: rule.operator.parse().map_err(|e| format!("{e}"))?,
            threshold: rule.threshold,
            duration_seconds: rule.duration_seconds,
            enabled: RuleState::from_flag(rule.enabled).is_enabled(),
            organization_id: rule.organization_id,
            created_at: rule.created_at,
            updated_at: rule.updated_at,
        })
    }
}

#[derive(Debug, Deserialize)]
struct TestNotificationDto {
    notification_id: i64,
}

#[derive(Debug, Serialize)]
struct TestNotificationResponseDto {
    delivered: bool,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct AlertEventResponseDto {
    id: i64,
    alert_rule_id: i64,
    organization_id: i64,
    target_key: String,
    state: AlertEventState,
    value: Option<f64>,
    threshold: Option<f64>,
    message: String,
    created_at: i64,
    acknowledged_at: Option<i64>,
    acknowledged_by: Option<i64>,
    silenced_until: Option<i64>,
    resolved_at: Option<i64>,
    notification_correlation_id: Option<String>,
}

impl TryFrom<crate::flash_gen::models::AlertEvents> for AlertEventResponseDto {
    type Error = String;

    fn try_from(value: crate::flash_gen::models::AlertEvents) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            alert_rule_id: value.alert_rule_id,
            organization_id: value.organization_id,
            target_key: value.target_key,
            state: value.state.parse().map_err(|error| format!("{error}"))?,
            value: value.value.map(f64::from),
            threshold: value.threshold.map(f64::from),
            message: value.message,
            created_at: value.created_at,
            acknowledged_at: value.acknowledged_at,
            acknowledged_by: value.acknowledged_by,
            silenced_until: value.silenced_until,
            resolved_at: value.resolved_at,
            notification_correlation_id: value.notification_correlation_id,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SilenceAlertDto {
    until: i64,
}

fn state_from(enabled: bool) -> RuleState {
    if enabled {
        RuleState::Enabled
    } else {
        RuleState::Disabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_dto() -> CreateAlertRuleDto {
        CreateAlertRuleDto {
            name: "High CPU".into(),
            target_kind: TargetKind::Server,
            target_id: 1,
            metric: MetricKind::Cpu,
            operator: Operator::GreaterThan,
            threshold: 80.0,
            duration_seconds: 300,
            organization_id: 42,
            enabled: true,
        }
    }

    #[test]
    fn a_created_rule_stores_its_enums_as_the_schema_spells_them() {
        let model = create_dto().into_model();

        assert_eq!(model.target_type, "SERVER");
        assert_eq!(model.metric_name, "CPU");
        assert_eq!(model.operator, "GT");
        assert_eq!(model.enabled, 1);
        assert_eq!(model.organization_id, 42);
    }

    #[test]
    fn a_disabled_rule_stores_a_zero_flag() {
        let mut dto = create_dto();
        dto.enabled = false;
        assert_eq!(dto.into_model().enabled, 0);
    }

    #[test]
    fn a_rule_round_trips_through_its_response_shape() {
        let model = create_dto().into_model();
        let stored = AlertRule {
            id: Some(7),
            ..model
        };

        let response = AlertRuleResponseDto::from_model(stored).unwrap();
        assert_eq!(response.id, 7);
        assert_eq!(response.target_kind, TargetKind::Server);
        assert_eq!(response.metric, MetricKind::Cpu);
        assert_eq!(response.operator, Operator::GreaterThan);
        assert!(response.enabled);
    }

    #[test]
    fn a_row_with_an_unknown_metric_is_rejected() {
        let model = AlertRule {
            id: Some(1),
            metric_name: "TEMPERATURE".into(),
            ..create_dto().into_model()
        };

        let error = AlertRuleResponseDto::from_model(model).unwrap_err();
        assert!(error.contains("TEMPERATURE"), "got {error}");
    }

    #[test]
    fn a_row_without_an_id_is_rejected() {
        assert!(AlertRuleResponseDto::from_model(create_dto().into_model()).is_err());
    }

    #[test]
    fn an_update_cannot_move_a_rule_between_tenants() {
        let dto = UpdateAlertRuleDto {
            name: "Renamed".into(),
            target_kind: TargetKind::Application,
            target_id: 9,
            metric: MetricKind::Memory,
            operator: Operator::GreaterOrEqual,
            threshold: 90.0,
            duration_seconds: 60,
            enabled: false,
        };

        let model = dto.into_model(7, 42);
        assert_eq!(model.id, Some(7));
        assert_eq!(model.organization_id, 42);
        assert_eq!(model.enabled, 0);
    }
}
