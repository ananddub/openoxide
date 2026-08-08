use crate::core::middleware::permission::{Alert, CanMonitor, CanWrite, Server};
use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::alert_rule::{
        AlertEventResponseDto, AlertRuleResponseDto, CreateAlertRuleDto, TestNotificationDto,
        TestNotificationResponseDto, UpdateAlertRuleDto,
    },
    core::middleware::{permission::RequirePermission, validator::ValidatedJson},
    services::{
        alert::AlertService,
        monitoring::lifecycle::MonitoringLifecycleService,
        notification::{NotificationLevel, NotificationMessage, NotificationService},
    },
};

type ApiError = (StatusCode, String);

pub struct AlertRuleController {
    service: Arc<AlertService>,
    notifications: Arc<NotificationService>,
    lifecycle: Arc<MonitoringLifecycleService>,
}

#[controller("/api/alerts")]
impl AlertRuleController {
    fn new(
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

    #[get("/organization/{organization_id}")]
    async fn list(
        &self,
        RequirePermission(claims, _): RequirePermission<Server, CanMonitor>,
        Path(organization_id): Path<i64>,
    ) -> Result<Json<Vec<AlertRuleResponseDto>>, ApiError> {
        verify_organization(claims.user.group_id, organization_id)?;
        let rules = self
            .service
            .list_rules(organization_id)
            .await
            .map_err(internal)?;

        // A row the enums can no longer represent is logged and dropped rather
        // than failing the whole list.
        let response = rules
            .into_iter()
            .filter_map(|rule| match AlertRuleResponseDto::from_model(rule) {
                Ok(dto) => Some(dto),
                Err(error) => {
                    tracing::warn!(error = %error, "skipping unreadable alert rule");
                    None
                }
            })
            .collect();

        Ok(Json(response))
    }

    #[get("/events/organization/{organization_id}")]
    async fn events(
        &self,
        RequirePermission(claims, _): RequirePermission<Server, CanMonitor>,
        Path(organization_id): Path<i64>,
    ) -> Result<Json<Vec<AlertEventResponseDto>>, ApiError> {
        if claims.user.group_id != organization_id {
            return Err((
                StatusCode::FORBIDDEN,
                "organization does not match authenticated scope".into(),
            ));
        }
        let events = self
            .service
            .list_events(organization_id, 200)
            .await
            .map_err(|error| internal(error.to_string()))?
            .into_iter()
            .map(AlertEventResponseDto::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| (StatusCode::UNPROCESSABLE_ENTITY, error))?;
        Ok(Json(events))
    }

    #[get("/{id}/organization/{organization_id}")]
    async fn get(
        &self,
        RequirePermission(claims, _): RequirePermission<Server, CanMonitor>,
        Path((id, organization_id)): Path<(i64, i64)>,
    ) -> Result<Json<AlertRuleResponseDto>, ApiError> {
        verify_organization(claims.user.group_id, organization_id)?;
        let rule = self
            .service
            .get_rule(id, organization_id)
            .await
            .map_err(|error| internal(error.to_string()))?
            .ok_or_else(not_found)?;

        AlertRuleResponseDto::from_model(rule)
            .map(Json)
            .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e))
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(claims, _): RequirePermission<Alert, CanWrite>,
        ValidatedJson(body): ValidatedJson<CreateAlertRuleDto>,
    ) -> Result<(StatusCode, Json<AlertRuleResponseDto>), ApiError> {
        verify_organization(claims.user.group_id, body.organization_id)?;
        let organization_id = body.organization_id;
        let id = self
            .service
            .create_rule(body.into_model())
            .await
            .map_err(internal)?;

        let created = self
            .service
            .get_rule(id, organization_id)
            .await
            .map_err(internal)?
            .ok_or_else(|| internal("created rule could not be read back".to_string()))?;

        AlertRuleResponseDto::from_model(created)
            .map(|dto| (StatusCode::CREATED, Json(dto)))
            .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e))
    }

    #[put("/{id}/organization/{organization_id}")]
    async fn update(
        &self,
        RequirePermission(claims, _): RequirePermission<Alert, CanWrite>,
        Path((id, organization_id)): Path<(i64, i64)>,
        ValidatedJson(body): ValidatedJson<UpdateAlertRuleDto>,
    ) -> Result<Json<AlertRuleResponseDto>, ApiError> {
        verify_organization(claims.user.group_id, organization_id)?;
        // Confirms the rule belongs to this organization before writing.
        self.service
            .get_rule(id, organization_id)
            .await
            .map_err(internal)?
            .ok_or_else(not_found)?;

        self.service
            .update_rule(id, body.into_model(id, organization_id))
            .await
            .map_err(internal)?;

        let updated = self
            .service
            .get_rule(id, organization_id)
            .await
            .map_err(internal)?
            .ok_or_else(not_found)?;

        AlertRuleResponseDto::from_model(updated)
            .map(Json)
            .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e))
    }

    #[delete("/{id}/organization/{organization_id}")]
    async fn delete(
        &self,
        RequirePermission(claims, _): RequirePermission<Alert, CanWrite>,
        Path((id, organization_id)): Path<(i64, i64)>,
    ) -> Result<StatusCode, ApiError> {
        verify_organization(claims.user.group_id, organization_id)?;
        let removed = self
            .service
            .delete_rule(id, organization_id)
            .await
            .map_err(internal)?;

        if removed {
            Ok(StatusCode::NO_CONTENT)
        } else {
            Err(not_found())
        }
    }

    /// Sends a sample alert through one configured channel, so an operator can
    /// confirm delivery without waiting for a real breach.
    #[post("/test")]
    async fn test_notification(
        &self,
        RequirePermission(_claims, _): RequirePermission<Alert, CanWrite>,
        Json(body): Json<TestNotificationDto>,
    ) -> Result<Json<TestNotificationResponseDto>, ApiError> {
        let message = NotificationMessage::new(
            "Test alert",
            "This is a test from Rustploy. If you can read it, alerts will reach you.",
        )
        .level(NotificationLevel::Info)
        .field("Source", "alert configuration test");

        match self
            .notifications
            .send_test(body.notification_id, &message)
            .await
        {
            Ok(()) => Ok(Json(TestNotificationResponseDto {
                delivered: true,
                message: "notification delivered".to_string(),
            })),
            // The provider's own error is the useful part here, so it is
            // returned rather than flattened into a generic failure.
            Err(error) => Ok(Json(TestNotificationResponseDto {
                delivered: false,
                message: error,
            })),
        }
    }

    #[post("/events/{event_id}/acknowledge")]
    async fn acknowledge_event(
        &self,
        RequirePermission(claims, _): RequirePermission<Alert, CanWrite>,
        Path(event_id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        if self
            .lifecycle
            .acknowledge(event_id, claims.user.group_id, claims.user.user_id)
            .await
            .map_err(|error| internal(error.to_string()))?
        {
            Ok(StatusCode::NO_CONTENT)
        } else {
            Err((StatusCode::NOT_FOUND, "alert event not found".into()))
        }
    }

    #[post("/events/{event_id}/silence")]
    async fn silence_event(
        &self,
        RequirePermission(claims, _): RequirePermission<Alert, CanWrite>,
        Path(event_id): Path<i64>,
        Json(body): Json<crate::api::dto::monitoring::SilenceAlertDto>,
    ) -> Result<StatusCode, ApiError> {
        if self
            .lifecycle
            .silence(event_id, claims.user.group_id, body.until)
            .await
            .map_err(|error| internal(error.to_string()))?
        {
            Ok(StatusCode::NO_CONTENT)
        } else {
            Err((StatusCode::NOT_FOUND, "alert event not found".into()))
        }
    }
}

fn internal(error: String) -> ApiError {
    tracing::error!(error = %error, "alert rule operation failed");
    (StatusCode::INTERNAL_SERVER_ERROR, error)
}

fn not_found() -> ApiError {
    (StatusCode::NOT_FOUND, "alert rule not found".to_string())
}

fn verify_organization(authenticated: i64, requested: i64) -> Result<(), ApiError> {
    if authenticated == requested {
        Ok(())
    } else {
        Err((
            StatusCode::FORBIDDEN,
            "organization does not match authenticated scope".into(),
        ))
    }
}
