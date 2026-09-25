use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    AlertRuleService, AlertRuleServiceExt, CreateAlertRuleRequest, CreateAlertRuleResponse,
    DeleteAlertRuleRequest, Empty, GetAlertRuleRequest, GetAlertRuleResponse,
    ListAlertRulesRequest, ListAlertRulesResponse, TestAlertRuleRequest, TestAlertRuleResponse,
    UpdateAlertRuleRequest, UpdateAlertRuleResponse,
};
use crate::services::integrations::notification::{NotificationLevel, NotificationMessage};
use crate::services::observability::alert::AlertRule;

use super::helper::*;
use super::init::AlertRuleController;

#[auto_grpc::service(connect = true, auth = true)]
impl AlertRuleService for AlertRuleController {
    #[permission(Alert, CanRead)]
    async fn list_rules<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListAlertRulesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListAlertRulesResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_organization(auth.organization_id, organization_id)?;
        let rules = self
            .service
            .list_rules(organization_id)
            .await
            .map_err(|e| internal(e.to_string()))?;
        let rules = rules
            .into_iter()
            .filter_map(|rule| match rule_to_proto(rule) {
                Ok(proto) => Some(proto),
                Err(error) => {
                    tracing::warn!(error = %error, "skipping unreadable alert rule");
                    None
                }
            })
            .collect();
        Response::ok(ListAlertRulesResponse {
            rules,
            ..Default::default()
        })
    }

    #[permission(Alert, CanRead)]
    async fn get_rule<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetAlertRuleRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GetAlertRuleResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_organization(auth.organization_id, organization_id)?;
        let rule = self
            .service
            .get_rule(request.id, organization_id)
            .await
            .map_err(|e| internal(e.to_string()))?
            .ok_or_else(|| ConnectError::not_found("alert rule not found"))?;
        Response::ok(GetAlertRuleResponse {
            rule: ::buffa::MessageField::some(rule_to_proto(rule)?),
            ..Default::default()
        })
    }

    #[permission(Alert, CanUpdate)]
    async fn create_rule<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateAlertRuleRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<CreateAlertRuleResponse> + Send + use<'a>> {
        verify_organization(auth.organization_id, request.organization_id)?;
        let organization_id = request.organization_id;
        let duration_seconds = seconds_i32(request.duration_seconds)?;
        let model = AlertRule {
            id: None,
            name: request.name.to_owned(),
            target_type: proto_target_kind(request.target_kind)?.to_owned(),
            target_id: request.target_id,
            metric_name: proto_metric_kind(request.metric)?.to_owned(),
            operator: proto_operator(request.operator)?.to_owned(),
            threshold: request.threshold,
            duration_seconds,
            // Rules dispatch through the organization's configured channels, so
            // this column carries no routing decision of its own.
            notification_channel: "SYSTEM".to_owned(),
            enabled: state_from(request.enabled).as_flag(),
            organization_id,
            created_at: 0,
            updated_at: 0,
        };
        let id = self
            .service
            .create_rule(model)
            .await
            .map_err(internal)?;
        let created = self
            .service
            .get_rule(id, organization_id)
            .await
            .map_err(internal)?
            .ok_or_else(|| internal("created rule could not be read back".to_string()))?;
        Response::ok(CreateAlertRuleResponse {
            rule: ::buffa::MessageField::some(rule_to_proto(created)?),
            ..Default::default()
        })
    }

    #[permission(Alert, CanUpdate)]
    async fn update_rule<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateAlertRuleRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<UpdateAlertRuleResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_organization(auth.organization_id, organization_id)?;
        self.service
            .get_rule(request.id, organization_id)
            .await
            .map_err(|e| internal(e.to_string()))?
            .ok_or_else(|| ConnectError::not_found("alert rule not found"))?;
        let duration_seconds = seconds_i32(request.duration_seconds)?;
        // `organization_id` comes from the existing row rather than the request,
        // so an update cannot move a rule between tenants.
        let model = AlertRule {
            id: Some(request.id),
            name: request.name.to_owned(),
            target_type: proto_target_kind(request.target_kind)?.to_owned(),
            target_id: request.target_id,
            metric_name: proto_metric_kind(request.metric)?.to_owned(),
            operator: proto_operator(request.operator)?.to_owned(),
            threshold: request.threshold,
            duration_seconds,
            notification_channel: "SYSTEM".to_owned(),
            enabled: state_from(request.enabled).as_flag(),
            organization_id,
            created_at: 0,
            updated_at: 0,
        };
        self.service
            .update_rule(request.id, model)
            .await
            .map_err(internal)?;
        let updated = self
            .service
            .get_rule(request.id, organization_id)
            .await
            .map_err(internal)?
            .ok_or_else(|| ConnectError::not_found("alert rule not found"))?;
        Response::ok(UpdateAlertRuleResponse {
            rule: ::buffa::MessageField::some(rule_to_proto(updated)?),
            ..Default::default()
        })
    }

    #[permission(Alert, CanUpdate)]
    async fn delete_rule<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteAlertRuleRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_organization(auth.organization_id, organization_id)?;
        let removed = self
            .service
            .delete_rule(request.id, organization_id)
            .await
            .map_err(internal)?;
        if removed {
            Response::ok(Empty::default())
        } else {
            Err(ConnectError::not_found("alert rule not found"))
        }
    }

    #[permission(Alert, CanUpdate)]
    async fn test_rule<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, TestAlertRuleRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TestAlertRuleResponse> + Send + use<'a>> {
        let message = NotificationMessage::new(
            "Test alert",
            "This is a test from OpenOxide. If you can read it, alerts will reach you.",
        )
        .level(NotificationLevel::Info)
        .field("Source", "alert configuration test");

        match self.notifications.send_test(request.notification_id, &message).await {
            Ok(()) => Response::ok(TestAlertRuleResponse {
                delivered: true,
                message: "notification delivered".to_owned(),
                ..Default::default()
            }),
            Err(error) => Response::ok(TestAlertRuleResponse {
                delivered: false,
                message: error,
                ..Default::default()
            }),
        }
    }
}
