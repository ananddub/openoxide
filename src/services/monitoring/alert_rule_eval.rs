use std::collections::VecDeque;

use crate::db::models::alert_rule::AlertRule;

/// How many samples the window holds when a rule sets no duration.
const DEFAULT_WINDOW_SAMPLES: usize = 15;
/// Upper bound on window size, so a huge `duration_seconds` can't grow the
/// buffer without limit.
const MAX_WINDOW_SAMPLES: usize = 300;
/// Fraction of a full window that must breach before the rule fires. Matches
/// Dozzle's behaviour: a single spike is noise, a sustained majority is real.
const FIRE_RATIO: f64 = 0.8;
/// Longest a rule can stay silent after firing.
const MAX_COOLDOWN_SECONDS: i64 = 3600;
/// Cooldown used when a rule doesn't specify one.
const DEFAULT_COOLDOWN_SECONDS: i64 = 300;

/// Comparison used by a rule's threshold test.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    GreaterThan,
    GreaterOrEqual,
    LessThan,
    LessOrEqual,
    Equal,
}

impl Operator {
    /// Parses the `operator` column. Accepts both the symbolic and the
    /// spelled-out forms so rows written by different clients still work.
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_uppercase().as_str() {
            ">" | "GT" | "GREATER_THAN" => Some(Self::GreaterThan),
            ">=" | "GTE" | "GREATER_OR_EQUAL" => Some(Self::GreaterOrEqual),
            "<" | "LT" | "LESS_THAN" => Some(Self::LessThan),
            "<=" | "LTE" | "LESS_OR_EQUAL" => Some(Self::LessOrEqual),
            "==" | "=" | "EQ" | "EQUAL" => Some(Self::Equal),
            _ => None,
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
            // Floats rarely compare exactly, so treat "equal" as a small band.
            // Without this an `EQ` rule would essentially never fire.
            Self::Equal => (value - threshold).abs() < f64::EPSILON.max(threshold.abs() * 1e-9),
        }
    }
}

/// Which metric a rule watches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetricKind {
    Cpu,
    Memory,
    Disk,
}

impl MetricKind {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_uppercase().as_str() {
            "CPU" | "CPU_PERCENT" => Some(Self::Cpu),
            "MEMORY" | "MEM" | "MEMORY_PERCENT" | "RAM" => Some(Self::Memory),
            "DISK" | "DISK_PERCENT" | "STORAGE" => Some(Self::Disk),
            _ => None,
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

/// A single metric reading for one target, as fed to the engine.
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

/// A rule parsed into the form the evaluator actually needs. Rows whose
/// `metric_name` or `operator` we don't recognise are rejected here rather than
/// silently never firing.
#[derive(Clone, Debug)]
pub struct ParsedRule {
    pub id: i64,
    pub name: String,
    pub target_type: String,
    pub target_id: i64,
    pub metric: MetricKind,
    pub operator: Operator,
    pub threshold: f64,
    pub window_samples: usize,
    pub cooldown_seconds: i64,
}

impl ParsedRule {
    /// Converts a stored row, returning `None` for disabled rules and `Err` for
    /// rows the engine cannot interpret.
    pub fn from_row(rule: &AlertRule, sample_interval_secs: i64) -> Result<Option<Self>, String> {
        if rule.enabled == 0 {
            return Ok(None);
        }

        let id = rule
            .id
            .ok_or_else(|| format!("alert rule {:?} has no id", rule.name))?;

        let metric = MetricKind::parse(&rule.metric_name).ok_or_else(|| {
            format!(
                "alert rule {:?} watches unknown metric {:?}",
                rule.name, rule.metric_name
            )
        })?;

        let operator = Operator::parse(&rule.operator).ok_or_else(|| {
            format!(
                "alert rule {:?} uses unknown operator {:?}",
                rule.name, rule.operator
            )
        })?;

        Ok(Some(Self {
            id,
            name: rule.name.clone(),
            target_type: rule.target_type.to_ascii_uppercase(),
            target_id: rule.target_id,
            metric,
            operator,
            threshold: rule.threshold,
            window_samples: window_samples_for(rule.duration_seconds, sample_interval_secs),
            cooldown_seconds: cooldown_for(rule.duration_seconds),
        }))
    }

    /// Identifies a rule/target pair in the engine's state maps. A rule can
    /// watch many containers, and each needs its own window and cooldown.
    pub fn state_key(&self, target: &str) -> String {
        format!("{}:{}", self.id, target)
    }
}

/// Turns a rule's `duration_seconds` into a sample count. A rule that wants to
/// see 60s of breach at a 10s scrape interval needs 6 samples.
fn window_samples_for(duration_seconds: i32, sample_interval_secs: i64) -> usize {
    if duration_seconds <= 0 {
        return DEFAULT_WINDOW_SAMPLES;
    }

    let interval = sample_interval_secs.max(1);
    let samples = (i64::from(duration_seconds) / interval).max(1);
    (samples as usize).min(MAX_WINDOW_SAMPLES)
}

/// Derives the post-fire silence from the rule's duration, clamped so a rule
/// can neither spam nor go quiet for more than an hour.
fn cooldown_for(duration_seconds: i32) -> i64 {
    if duration_seconds <= 0 {
        return DEFAULT_COOLDOWN_SECONDS;
    }
    i64::from(duration_seconds).clamp(1, MAX_COOLDOWN_SECONDS)
}

/// Rolling window of breach/no-breach results for one rule/target pair.
///
/// Firing needs a *full* window that is mostly breaches. Requiring fullness is
/// what stops an alert firing on the very first scrape after startup.
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

    /// Records one evaluation and reports whether the window now says "fire".
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

    /// Drops recorded samples, used when a rule's shape changes so an old
    /// window can't satisfy a new threshold.
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

#[cfg(test)]
mod tests {
    use super::*;

    fn row(metric: &str, operator: &str, threshold: f64, duration: i32) -> AlertRule {
        AlertRule {
            id: Some(1),
            name: "test rule".into(),
            target_type: "SERVER".into(),
            target_id: 1,
            metric_name: metric.into(),
            operator: operator.into(),
            threshold,
            duration_seconds: duration,
            notification_channel: "SYSTEM".into(),
            enabled: 1,
            created_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn parses_symbolic_and_named_operators() {
        assert_eq!(Operator::parse(">"), Some(Operator::GreaterThan));
        assert_eq!(Operator::parse("GT"), Some(Operator::GreaterThan));
        assert_eq!(Operator::parse("gte"), Some(Operator::GreaterOrEqual));
        assert_eq!(Operator::parse("<="), Some(Operator::LessOrEqual));
        assert_eq!(Operator::parse("nonsense"), None);
    }

    #[test]
    fn operators_compare_as_expected() {
        assert!(Operator::GreaterThan.compare(90.0, 80.0));
        assert!(!Operator::GreaterThan.compare(80.0, 80.0));
        assert!(Operator::GreaterOrEqual.compare(80.0, 80.0));
        assert!(Operator::LessThan.compare(10.0, 20.0));
        assert!(Operator::Equal.compare(50.0, 50.0));
    }

    #[test]
    fn parses_metric_aliases() {
        assert_eq!(MetricKind::parse("cpu"), Some(MetricKind::Cpu));
        assert_eq!(MetricKind::parse("RAM"), Some(MetricKind::Memory));
        assert_eq!(MetricKind::parse("disk_percent"), Some(MetricKind::Disk));
        assert_eq!(MetricKind::parse("gpu"), None);
    }

    #[test]
    fn disabled_rules_are_skipped() {
        let mut r = row("CPU", ">", 80.0, 60);
        r.enabled = 0;
        assert!(ParsedRule::from_row(&r, 10).unwrap().is_none());
    }

    #[test]
    fn unknown_metric_is_an_error_not_a_silent_skip() {
        let r = row("TEMPERATURE", ">", 80.0, 60);
        assert!(ParsedRule::from_row(&r, 10).is_err());
    }

    #[test]
    fn unknown_operator_is_an_error() {
        let r = row("CPU", "~=", 80.0, 60);
        assert!(ParsedRule::from_row(&r, 10).is_err());
    }

    #[test]
    fn duration_converts_to_sample_count() {
        // 60s of breach at a 10s scrape interval is 6 samples.
        let parsed = ParsedRule::from_row(&row("CPU", ">", 80.0, 60), 10)
            .unwrap()
            .unwrap();
        assert_eq!(parsed.window_samples, 6);
    }

    #[test]
    fn zero_duration_uses_defaults() {
        let parsed = ParsedRule::from_row(&row("CPU", ">", 80.0, 0), 10)
            .unwrap()
            .unwrap();
        assert_eq!(parsed.window_samples, DEFAULT_WINDOW_SAMPLES);
        assert_eq!(parsed.cooldown_seconds, DEFAULT_COOLDOWN_SECONDS);
    }

    #[test]
    fn window_size_is_capped() {
        let parsed = ParsedRule::from_row(&row("CPU", ">", 80.0, i32::MAX), 1)
            .unwrap()
            .unwrap();
        assert_eq!(parsed.window_samples, MAX_WINDOW_SAMPLES);
    }

    #[test]
    fn cooldown_is_capped_at_an_hour() {
        assert_eq!(cooldown_for(100_000), MAX_COOLDOWN_SECONDS);
    }

    #[test]
    fn partial_window_never_fires() {
        let mut w = SampleWindow::new(5);
        // Even an unbroken run of breaches stays quiet until the window fills,
        // so a rule can't fire on the first scrape after startup.
        for _ in 0..4 {
            assert!(!w.push(true));
        }
        assert!(w.push(true));
    }

    #[test]
    fn single_spike_does_not_fire() {
        let mut w = SampleWindow::new(5);
        w.push(false);
        w.push(false);
        w.push(true);
        w.push(false);
        assert!(!w.push(false));
    }

    #[test]
    fn sustained_breach_fires() {
        let mut w = SampleWindow::new(5);
        for _ in 0..4 {
            w.push(true);
        }
        assert!(w.push(true));
    }

    #[test]
    fn eighty_percent_is_enough() {
        let mut w = SampleWindow::new(5);
        w.push(true);
        w.push(true);
        w.push(true);
        w.push(false);
        // 4 of 5 breaches == exactly the 0.8 ratio.
        assert!(w.push(true));
    }

    #[test]
    fn below_ratio_stays_quiet() {
        let mut w = SampleWindow::new(5);
        w.push(true);
        w.push(true);
        w.push(true);
        w.push(false);
        assert!(!w.push(false));
    }

    #[test]
    fn window_slides_and_can_recover() {
        let mut w = SampleWindow::new(4);
        for _ in 0..4 {
            w.push(true);
        }
        // Recovery: as clean samples push the breaches out, it stops firing.
        w.push(false);
        assert!(!w.push(false));
    }

    #[test]
    fn reset_clears_progress() {
        let mut w = SampleWindow::new(3);
        w.push(true);
        w.push(true);
        w.reset();
        assert!(w.is_empty());
        assert!(!w.push(true));
    }

    #[test]
    fn state_key_separates_targets() {
        let parsed = ParsedRule::from_row(&row("CPU", ">", 80.0, 60), 10)
            .unwrap()
            .unwrap();
        assert_ne!(parsed.state_key("web-1"), parsed.state_key("web-2"));
    }
}
