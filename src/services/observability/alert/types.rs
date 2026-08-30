use std::{collections::VecDeque, fmt, str::FromStr};

use os::string_enum;
use serde::{Deserialize, Serialize};

use crate::services::integrations::notification::{NotificationLevel, NotificationMessage};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlertParseError {
    Target(String),
    Metric(String),
    Operator(String),
    EventState(String),
    MissingId(String),
}

impl fmt::Display for AlertParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Target(value) => write!(f, "unknown alert target {value:?}"),
            Self::Metric(value) => write!(f, "unknown alert metric {value:?}"),
            Self::Operator(value) => write!(f, "unknown alert operator {value:?}"),
            Self::EventState(value) => write!(f, "unknown alert event state {value:?}"),
            Self::MissingId(name) => write!(f, "alert rule {name:?} has no id"),
        }
    }
}

impl std::error::Error for AlertParseError {}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ts_rs::TS,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TargetKind {
    Server,
    Application,
    Database,
    Compose,
}

impl TargetKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Server => "SERVER",
            Self::Application => "APPLICATION",
            Self::Database => "DATABASE",
            Self::Compose => "COMPOSE",
        }
    }

    pub fn is_host_level(&self) -> bool {
        matches!(self, Self::Server)
    }
}

impl FromStr for TargetKind {
    type Err = AlertParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw.trim().to_ascii_uppercase().as_str() {
            "SERVER" => Ok(Self::Server),
            "APPLICATION" | "APP" => Ok(Self::Application),
            "DATABASE" | "DB" => Ok(Self::Database),
            "COMPOSE" => Ok(Self::Compose),
            other => Err(AlertParseError::Target(other.to_owned())),
        }
    }
}

impl fmt::Display for TargetKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ts_rs::TS,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MetricKind {
    Cpu,
    Memory,
    Disk,
}

impl MetricKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Cpu => "CPU",
            Self::Memory => "MEMORY",
            Self::Disk => "DISK",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Cpu => "CPU",
            Self::Memory => "Memory",
            Self::Disk => "Disk",
        }
    }
}

impl FromStr for MetricKind {
    type Err = AlertParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw.trim().to_ascii_uppercase().as_str() {
            "CPU" | "CPU_PERCENT" => Ok(Self::Cpu),
            "MEMORY" | "MEM" | "MEMORY_PERCENT" | "RAM" => Ok(Self::Memory),
            "DISK" | "DISK_PERCENT" | "STORAGE" => Ok(Self::Disk),
            other => Err(AlertParseError::Metric(other.to_owned())),
        }
    }
}

impl fmt::Display for MetricKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ts_rs::TS,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Operator {
    GreaterThan,
    GreaterOrEqual,
    LessThan,
    LessOrEqual,
    Equal,
}

impl Operator {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GreaterThan => "GT",
            Self::GreaterOrEqual => "GTE",
            Self::LessThan => "LT",
            Self::LessOrEqual => "LTE",
            Self::Equal => "EQ",
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Self::GreaterThan => ">",
            Self::GreaterOrEqual => ">=",
            Self::LessThan => "<",
            Self::LessOrEqual => "<=",
            Self::Equal => "==",
        }
    }

    pub fn compare(&self, value: f64, threshold: f64) -> bool {
        match self {
            Self::GreaterThan => value > threshold,
            Self::GreaterOrEqual => value >= threshold,
            Self::LessThan => value < threshold,
            Self::LessOrEqual => value <= threshold,
            Self::Equal => (value - threshold).abs() < f64::EPSILON.max(threshold.abs() * 1e-9),
        }
    }
}

impl FromStr for Operator {
    type Err = AlertParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw.trim().to_ascii_uppercase().as_str() {
            ">" | "GT" | "GREATER_THAN" => Ok(Self::GreaterThan),
            ">=" | "GTE" | "GREATER_OR_EQUAL" => Ok(Self::GreaterOrEqual),
            "<" | "LT" | "LESS_THAN" => Ok(Self::LessThan),
            "<=" | "LTE" | "LESS_OR_EQUAL" => Ok(Self::LessOrEqual),
            "==" | "=" | "EQ" | "EQUAL" => Ok(Self::Equal),
            other => Err(AlertParseError::Operator(other.to_owned())),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleState {
    Enabled,
    Disabled,
}

impl RuleState {
    pub fn from_flag(flag: i32) -> Self {
        if flag == 0 {
            Self::Disabled
        } else {
            Self::Enabled
        }
    }

    pub fn as_flag(&self) -> i32 {
        match self {
            Self::Enabled => 1,
            Self::Disabled => 0,
        }
    }

    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled)
    }
}

string_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ts_rs::TS)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum AlertEventState {
        default = Firing;

        Firing => "FIRING",
        Resolved => "RESOLVED",
        NoData => "NO_DATA",
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MetricSample {
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub disk_percent: f64,
}

impl MetricSample {
    pub fn value_for(&self, metric: MetricKind) -> f64 {
        match metric {
            MetricKind::Cpu => self.cpu_percent,
            MetricKind::Memory => self.memory_percent,
            MetricKind::Disk => self.disk_percent,
        }
    }
}

const FIRE_RATIO: f64 = 0.8;

#[derive(Debug)]
pub struct SampleWindow {
    capacity: usize,
    samples: VecDeque<bool>,
}

impl SampleWindow {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            samples: VecDeque::with_capacity(capacity.max(1)),
        }
    }

    pub fn push(&mut self, breached: bool) -> bool {
        if self.samples.len() == self.capacity {
            self.samples.pop_front();
        }
        self.samples.push_back(breached);

        if self.samples.len() < self.capacity {
            return false;
        }

        let breaches = self.samples.iter().filter(|b| **b).count();
        breaches as f64 / self.samples.len() as f64 >= FIRE_RATIO
    }

    pub fn reset(&mut self) {
        self.samples.clear();
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}

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
    pub target_kind: TargetKind,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_states_round_trip_through_the_database_spelling() {
        for state in [
            AlertEventState::Firing,
            AlertEventState::Resolved,
            AlertEventState::NoData,
        ] {
            assert_eq!(state.as_str().parse::<AlertEventState>().unwrap(), state);
        }
    }

    #[test]
    fn unknown_database_state_is_rejected() {
        assert!("ACKNOWLEDGED".parse::<AlertEventState>().is_err());
    }
}
