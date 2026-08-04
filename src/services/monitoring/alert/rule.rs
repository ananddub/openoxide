use super::{
    error::AlertParseError, metric::MetricKind, operator::Operator, rule_state::RuleState,
    target::TargetKind,
};
use crate::db::models::alert_rule::AlertRule;

const DEFAULT_WINDOW_SAMPLES: usize = 15;
const MAX_WINDOW_SAMPLES: usize = 300;
const MAX_COOLDOWN_SECONDS: i64 = 3600;
const DEFAULT_COOLDOWN_SECONDS: i64 = 300;

#[derive(Clone, Debug)]
pub struct ParsedRule {
    pub id: i64,
    pub name: String,
    pub target_kind: TargetKind,
    pub target_id: i64,
    pub metric: MetricKind,
    pub operator: Operator,
    pub threshold: f64,
    pub window_samples: usize,
    pub cooldown_seconds: i64,
    pub organization_id: i64,
}

impl ParsedRule {
    pub fn from_row(
        rule: &AlertRule,
        sample_interval_secs: i64,
    ) -> Result<Option<Self>, AlertParseError> {
        if !RuleState::from_flag(rule.enabled).is_enabled() {
            return Ok(None);
        }

        let id = rule
            .id
            .ok_or_else(|| AlertParseError::MissingId(rule.name.clone()))?;

        Ok(Some(Self {
            id,
            name: rule.name.clone(),
            target_kind: rule.target_type.parse()?,
            target_id: rule.target_id,
            metric: rule.metric_name.parse()?,
            operator: rule.operator.parse()?,
            threshold: rule.threshold,
            window_samples: window_samples_for(rule.duration_seconds, sample_interval_secs),
            cooldown_seconds: cooldown_for(rule.duration_seconds),
            organization_id: rule.organization_id,
        }))
    }

    pub fn state_key(&self, target: &str) -> String {
        format!("{}:{}", self.id, target)
    }
}

fn window_samples_for(duration_seconds: i32, sample_interval_secs: i64) -> usize {
    if duration_seconds <= 0 {
        return DEFAULT_WINDOW_SAMPLES;
    }

    let interval = sample_interval_secs.max(1);
    let samples = (i64::from(duration_seconds) / interval).max(1);
    (samples as usize).min(MAX_WINDOW_SAMPLES)
}

fn cooldown_for(duration_seconds: i32) -> i64 {
    if duration_seconds <= 0 {
        return DEFAULT_COOLDOWN_SECONDS;
    }
    i64::from(duration_seconds).clamp(1, MAX_COOLDOWN_SECONDS)
}
