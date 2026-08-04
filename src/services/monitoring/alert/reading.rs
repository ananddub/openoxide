use super::sample::MetricSample;
use super::target::TargetKind;
use crate::services::notification::{NotificationLevel, NotificationMessage};

#[derive(Clone, Debug)]
pub struct TargetReading {
    /// What this reading describes, so a rule watching servers never matches a
    /// container that happens to share its id.
    pub kind: TargetKind,
    pub key: String,
    pub display_name: String,
    pub sample: MetricSample,
}

#[derive(Clone, Debug)]
pub struct FiredAlert {
    pub rule_id: i64,
    pub rule_name: String,
    pub target_key: String,
    pub target_display: String,
    pub metric_label: &'static str,
    pub operator_symbol: &'static str,
    pub value: f64,
    pub threshold: f64,
    pub window_samples: usize,
    /// Carried from the rule so dispatch only reaches this tenant's channels.
    pub organization_id: i64,
}

impl FiredAlert {
    pub fn to_message(&self) -> NotificationMessage {
        NotificationMessage::new(
            format!("{} alert: {}", self.metric_label, self.target_display),
            format!(
                "{} on {} is {:.1}% ({} {:.1}%), sustained over {} samples.",
                self.metric_label,
                self.target_display,
                self.value,
                self.operator_symbol,
                self.threshold,
                self.window_samples
            ),
        )
        .level(NotificationLevel::Warning)
        .field("Rule", self.rule_name.clone())
        .field("Target", self.target_display.clone())
        .field("Metric", self.metric_label)
        .field("Value", format!("{:.1}%", self.value))
        .field(
            "Threshold",
            format!("{} {:.1}%", self.operator_symbol, self.threshold),
        )
    }
}
