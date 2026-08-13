use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::db::models::alert_rule::AlertRule;
use crate::services::alert::{AlertEventState, MetricKind, Operator, RuleState, TargetKind};

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateAlertRuleDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub target_kind: TargetKind,
    pub target_id: i64,
    pub metric: MetricKind,
    pub operator: Operator,
    #[validate(range(min = 0.0, max = 100.0))]
    pub threshold: f64,
    /// Seconds a breach must persist. Also drives the post-fire cooldown.
    #[validate(range(min = 0, max = 86_400))]
    pub duration_seconds: i32,
    pub organization_id: i64,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_enabled() -> bool {
    true
}

impl CreateAlertRuleDto {
    pub fn into_model(self) -> AlertRule {
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

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct UpdateAlertRuleDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub target_kind: TargetKind,
    pub target_id: i64,
    pub metric: MetricKind,
    pub operator: Operator,
    #[validate(range(min = 0.0, max = 100.0))]
    pub threshold: f64,
    #[validate(range(min = 0, max = 86_400))]
    pub duration_seconds: i32,
    pub enabled: bool,
}

impl UpdateAlertRuleDto {
    /// `organization_id` comes from the existing row rather than the request, so
    /// an update cannot move a rule between tenants.
    pub fn into_model(self, id: i64, organization_id: i64) -> AlertRule {
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

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct AlertRuleResponseDto {
    pub id: i64,
    pub name: String,
    pub target_kind: TargetKind,
    pub target_id: i64,
    pub metric: MetricKind,
    pub operator: Operator,
    pub threshold: f64,
    pub duration_seconds: i32,
    pub enabled: bool,
    pub organization_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl AlertRuleResponseDto {
    /// Fails when a stored row holds a value no longer in the enums, rather than
    /// guessing a default that would silently change what the rule watches.
    pub fn from_model(rule: AlertRule) -> Result<Self, String> {
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

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct TestNotificationDto {
    pub notification_id: i64,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct TestNotificationResponseDto {
    pub delivered: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct AlertEventResponseDto {
    pub id: i64,
    pub alert_rule_id: i64,
    pub organization_id: i64,
    pub target_key: String,
    pub state: AlertEventState,
    pub value: Option<f64>,
    pub threshold: Option<f64>,
    pub message: String,
    pub created_at: i64,
    pub acknowledged_at: Option<i64>,
    pub acknowledged_by: Option<i64>,
    pub silenced_until: Option<i64>,
    pub resolved_at: Option<i64>,
    pub notification_correlation_id: Option<String>,
}

impl TryFrom<crate::db::repository::alert_rule::AlertEvent> for AlertEventResponseDto {
    type Error = String;

    fn try_from(value: crate::db::repository::alert_rule::AlertEvent) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            alert_rule_id: value.alert_rule_id,
            organization_id: value.organization_id,
            target_key: value.target_key,
            state: value.state.parse().map_err(|error| format!("{error}"))?,
            value: value.value,
            threshold: value.threshold,
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
