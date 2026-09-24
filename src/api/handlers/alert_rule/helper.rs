use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    AlertMetricKind as ProtoMetricKind, AlertOperator as ProtoOperator, AlertRule as ProtoAlertRule,
    AlertTargetKind as ProtoTargetKind,
};
use crate::services::observability::alert::{AlertRule, MetricKind, Operator, RuleState, TargetKind};

pub fn internal(error: String) -> ConnectError {
    tracing::error!(error = %error, "alert rule operation failed");
    ConnectError::new(ErrorCode::Internal, error)
}

pub fn verify_organization(authenticated: i64, requested: i64) -> Result<(), ConnectError> {
    if authenticated == requested {
        Ok(())
    } else {
        Err(ConnectError::new(
            ErrorCode::PermissionDenied,
            "organization does not match authenticated scope",
        ))
    }
}

pub fn state_from(enabled: bool) -> RuleState {
    if enabled {
        RuleState::Enabled
    } else {
        RuleState::Disabled
    }
}

/// The wire carries seconds as int64, but the rules engine stores an int32; a
/// value that does not fit is a client error, not something to truncate.
pub fn seconds_i32(seconds: i64) -> Result<i32, ConnectError> {
    i32::try_from(seconds).map_err(|_| {
        ConnectError::new(
            ErrorCode::InvalidArgument,
            "duration_seconds is out of range",
        )
    })
}

pub fn proto_target_kind(
    value: ::buffa::EnumValue<ProtoTargetKind>,
) -> Result<&'static str, ConnectError> {
    match value {
        ::buffa::EnumValue::Known(ProtoTargetKind::Server) => Ok(TargetKind::Server.as_str()),
        ::buffa::EnumValue::Known(ProtoTargetKind::Application) => {
            Ok(TargetKind::Application.as_str())
        }
        ::buffa::EnumValue::Known(ProtoTargetKind::Database) => Ok(TargetKind::Database.as_str()),
        ::buffa::EnumValue::Known(ProtoTargetKind::Compose) => Ok(TargetKind::Compose.as_str()),
        _ => Err(ConnectError::new(
            ErrorCode::InvalidArgument,
            "unknown alert target kind",
        )),
    }
}

pub fn proto_metric_kind(
    value: ::buffa::EnumValue<ProtoMetricKind>,
) -> Result<&'static str, ConnectError> {
    match value {
        ::buffa::EnumValue::Known(ProtoMetricKind::Cpu) => Ok(MetricKind::Cpu.as_str()),
        ::buffa::EnumValue::Known(ProtoMetricKind::Memory) => Ok(MetricKind::Memory.as_str()),
        ::buffa::EnumValue::Known(ProtoMetricKind::Disk) => Ok(MetricKind::Disk.as_str()),
        _ => Err(ConnectError::new(
            ErrorCode::InvalidArgument,
            "unknown alert metric kind",
        )),
    }
}

pub fn proto_operator(
    value: ::buffa::EnumValue<ProtoOperator>,
) -> Result<&'static str, ConnectError> {
    match value {
        ::buffa::EnumValue::Known(ProtoOperator::GreaterThan) => Ok(Operator::GreaterThan.as_str()),
        ::buffa::EnumValue::Known(ProtoOperator::GreaterOrEqual) => {
            Ok(Operator::GreaterOrEqual.as_str())
        }
        ::buffa::EnumValue::Known(ProtoOperator::LessThan) => Ok(Operator::LessThan.as_str()),
        ::buffa::EnumValue::Known(ProtoOperator::LessOrEqual) => Ok(Operator::LessOrEqual.as_str()),
        ::buffa::EnumValue::Known(ProtoOperator::Equal) => Ok(Operator::Equal.as_str()),
        _ => Err(ConnectError::new(
            ErrorCode::InvalidArgument,
            "unknown alert operator",
        )),
    }
}

/// Fails when a stored row holds a value no longer in the enums, rather than
/// guessing a default that would silently change what the rule watches.
pub fn rule_to_proto(rule: AlertRule) -> Result<ProtoAlertRule, ConnectError> {
    let target_kind = match rule
        .target_type
        .parse::<TargetKind>()
        .map_err(|_| internal(format!("unknown stored target type {}", rule.target_type)))?
    {
        TargetKind::Server => ProtoTargetKind::Server,
        TargetKind::Application => ProtoTargetKind::Application,
        TargetKind::Database => ProtoTargetKind::Database,
        TargetKind::Compose => ProtoTargetKind::Compose,
    };
    let metric = match rule
        .metric_name
        .parse::<MetricKind>()
        .map_err(|_| internal(format!("unknown stored metric {}", rule.metric_name)))?
    {
        MetricKind::Cpu => ProtoMetricKind::Cpu,
        MetricKind::Memory => ProtoMetricKind::Memory,
        MetricKind::Disk => ProtoMetricKind::Disk,
    };
    let operator = match rule
        .operator
        .parse::<Operator>()
        .map_err(|_| internal(format!("unknown stored operator {}", rule.operator)))?
    {
        Operator::GreaterThan => ProtoOperator::GreaterThan,
        Operator::GreaterOrEqual => ProtoOperator::GreaterOrEqual,
        Operator::LessThan => ProtoOperator::LessThan,
        Operator::LessOrEqual => ProtoOperator::LessOrEqual,
        Operator::Equal => ProtoOperator::Equal,
    };
    Ok(ProtoAlertRule {
        id: rule
            .id
            .ok_or("persisted alert rule must have an id")
            .map_err(|e| internal(e.to_string()))?,
        name: rule.name,
        target_kind: ::buffa::EnumValue::Known(target_kind),
        target_id: rule.target_id,
        metric: ::buffa::EnumValue::Known(metric),
        operator: ::buffa::EnumValue::Known(operator),
        threshold: rule.threshold,
        duration_seconds: i64::from(rule.duration_seconds),
        enabled: RuleState::from_flag(rule.enabled).is_enabled(),
        organization_id: rule.organization_id,
        created_at: rule.created_at,
        updated_at: rule.updated_at,
        ..Default::default()
    })
}
