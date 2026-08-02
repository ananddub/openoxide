use std::collections::HashMap;

use super::alert_rule_eval::{MetricSample, ParsedRule, SampleWindow};
use crate::services::notification::{NotificationLevel, NotificationMessage};

/// One target's readings for a single evaluation pass.
///
/// `key` identifies the thing being watched (a server id, a container name) and
/// is what windows and cooldowns are tracked against.
#[derive(Clone, Debug)]
pub struct TargetReading {
    pub key: String,
    pub display_name: String,
    pub sample: MetricSample,
}

/// A rule that has fired, ready to be turned into a notification.
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
}

impl FiredAlert {
    /// Renders the alert into the panel's provider-agnostic message shape, so
    /// every configured notification provider can send it unchanged.
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

/// Per rule/target state: the rolling window and when it last fired.
#[derive(Debug)]
struct RuleState {
    window: SampleWindow,
    last_fired_at: Option<i64>,
}

/// Evaluates alert rules against metric readings.
///
/// Deliberately holds no database or network handle: it takes readings, returns
/// the alerts that should be sent, and keeps only the window/cooldown state in
/// between. That keeps the firing logic fully unit-testable, with `now_secs`
/// passed in rather than read from the clock.
#[derive(Debug, Default)]
pub struct AlertEngine {
    state: HashMap<String, RuleState>,
}

impl AlertEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Runs every rule against every reading whose target it matches.
    ///
    /// A rule matches a reading when its `target_id` is 0 (meaning "all targets
    /// of this type") or the reading's key equals the rule's `target_id`. That
    /// lets one rule cover a whole fleet without enumerating it.
    pub fn evaluate(
        &mut self,
        rules: &[ParsedRule],
        readings: &[TargetReading],
        now_secs: i64,
    ) -> Vec<FiredAlert> {
        let mut fired = Vec::new();

        for rule in rules {
            for reading in readings {
                if !Self::rule_covers(rule, reading) {
                    continue;
                }

                let value = reading.sample.value_for(rule.metric);
                let breached = rule.operator.compare(value, rule.threshold);
                let key = rule.state_key(&reading.key);

                let entry = self.state.entry(key).or_insert_with(|| RuleState {
                    window: SampleWindow::new(rule.window_samples),
                    last_fired_at: None,
                });

                // An edited rule must not inherit a window sized for the old
                // duration, or it would fire on stale samples.
                if entry.window.capacity() != rule.window_samples {
                    entry.window = SampleWindow::new(rule.window_samples);
                }

                if !entry.window.push(breached) {
                    continue;
                }

                if let Some(last) = entry.last_fired_at {
                    if now_secs.saturating_sub(last) < rule.cooldown_seconds {
                        continue;
                    }
                }

                entry.last_fired_at = Some(now_secs);
                // Start the next window clean so a firing rule waits out a full
                // window again rather than re-firing the moment cooldown ends.
                entry.window.reset();

                fired.push(FiredAlert {
                    rule_id: rule.id,
                    rule_name: rule.name.clone(),
                    target_key: reading.key.clone(),
                    target_display: reading.display_name.clone(),
                    metric_label: rule.metric.label(),
                    operator_symbol: rule.operator.symbol(),
                    value,
                    threshold: rule.threshold,
                    window_samples: rule.window_samples,
                });
            }
        }

        fired
    }

    /// Drops state for rules that no longer exist, so deleting a rule doesn't
    /// leak its window and cooldown entries for the life of the process.
    pub fn retain_rules(&mut self, rules: &[ParsedRule]) {
        let live: Vec<String> = rules.iter().map(|r| format!("{}:", r.id)).collect();
        self.state
            .retain(|key, _| live.iter().any(|prefix| key.starts_with(prefix)));
    }

    pub fn tracked_targets(&self) -> usize {
        self.state.len()
    }

    fn rule_covers(rule: &ParsedRule, reading: &TargetReading) -> bool {
        // 0 is the wildcard: apply to every target of this type.
        if rule.target_id == 0 {
            return true;
        }
        reading.key == rule.target_id.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::models::alert_rule::AlertRule;

    fn rule(threshold: f64, duration: i32, target_id: i64) -> ParsedRule {
        let row = AlertRule {
            id: Some(7),
            name: "cpu high".into(),
            target_type: "SERVER".into(),
            target_id,
            metric_name: "CPU".into(),
            operator: ">".into(),
            threshold,
            duration_seconds: duration,
            notification_channel: "SYSTEM".into(),
            enabled: 1,
            created_at: 0,
            updated_at: 0,
        };
        // 10s interval, so duration 30 => a 3-sample window.
        ParsedRule::from_row(&row, 10).unwrap().unwrap()
    }

    fn reading(key: &str, cpu: f64) -> TargetReading {
        TargetReading {
            key: key.into(),
            display_name: format!("server {key}"),
            sample: MetricSample {
                cpu_percent: cpu,
                memory_percent: 0.0,
                disk_percent: 0.0,
            },
        }
    }

    #[test]
    fn fires_only_after_the_window_fills() {
        let mut engine = AlertEngine::new();
        let rules = vec![rule(80.0, 30, 1)];
        let readings = vec![reading("1", 95.0)];

        assert!(engine.evaluate(&rules, &readings, 0).is_empty());
        assert!(engine.evaluate(&rules, &readings, 10).is_empty());
        assert_eq!(engine.evaluate(&rules, &readings, 20).len(), 1);
    }

    #[test]
    fn healthy_metrics_never_fire() {
        let mut engine = AlertEngine::new();
        let rules = vec![rule(80.0, 30, 1)];
        let readings = vec![reading("1", 12.0)];

        for t in 0..10 {
            assert!(engine.evaluate(&rules, &readings, t * 10).is_empty());
        }
    }

    #[test]
    fn cooldown_suppresses_repeats() {
        let mut engine = AlertEngine::new();
        // 3-sample window, 30s cooldown.
        let rules = vec![rule(80.0, 30, 1)];
        let readings = vec![reading("1", 95.0)];

        let mut fired = 0;
        for t in 0..3 {
            fired += engine.evaluate(&rules, &readings, t).len();
        }
        assert_eq!(fired, 1);

        // Samples keep arriving and the window refills, but every pass here
        // lands inside the 30s cooldown, so nothing is sent.
        for t in 3..12 {
            assert!(engine.evaluate(&rules, &readings, t).is_empty());
        }
    }

    #[test]
    fn fires_again_after_cooldown_expires() {
        let mut engine = AlertEngine::new();
        let rules = vec![rule(80.0, 30, 1)];
        let readings = vec![reading("1", 95.0)];

        for t in 0..3 {
            engine.evaluate(&rules, &readings, t * 10);
        }

        // Well past the cooldown, and enough passes to refill the window.
        let mut fired = 0;
        for t in 10..14 {
            fired += engine.evaluate(&rules, &readings, t * 100).len();
        }
        assert_eq!(fired, 1);
    }

    #[test]
    fn each_target_tracks_independently() {
        let mut engine = AlertEngine::new();
        // Wildcard rule covering every server.
        let rules = vec![rule(80.0, 30, 0)];
        let readings = vec![reading("1", 95.0), reading("2", 10.0)];

        for t in 0..2 {
            engine.evaluate(&rules, &readings, t * 10);
        }
        let fired = engine.evaluate(&rules, &readings, 20);

        // Only the hot server alerts; the quiet one stays silent.
        assert_eq!(fired.len(), 1);
        assert_eq!(fired[0].target_key, "1");
    }

    #[test]
    fn rule_ignores_targets_it_does_not_cover() {
        let mut engine = AlertEngine::new();
        let rules = vec![rule(80.0, 30, 1)];
        let readings = vec![reading("2", 99.0)];

        for t in 0..5 {
            assert!(engine.evaluate(&rules, &readings, t * 10).is_empty());
        }
    }

    #[test]
    fn editing_a_rule_resizes_its_window() {
        let mut engine = AlertEngine::new();
        let readings = vec![reading("1", 95.0)];

        engine.evaluate(&[rule(80.0, 30, 1)], &readings, 0);
        // Widening the duration must not fire off the old, shorter window.
        let widened = vec![rule(80.0, 100, 1)];
        assert!(engine.evaluate(&widened, &readings, 10).is_empty());
    }

    #[test]
    fn deleted_rules_stop_consuming_state() {
        let mut engine = AlertEngine::new();
        let rules = vec![rule(80.0, 30, 1)];
        engine.evaluate(&rules, &[reading("1", 95.0)], 0);
        assert_eq!(engine.tracked_targets(), 1);

        engine.retain_rules(&[]);
        assert_eq!(engine.tracked_targets(), 0);
    }

    #[test]
    fn message_includes_value_and_threshold() {
        let mut engine = AlertEngine::new();
        let rules = vec![rule(80.0, 30, 1)];
        let readings = vec![reading("1", 95.0)];
        for t in 0..2 {
            engine.evaluate(&rules, &readings, t * 10);
        }
        let fired = engine.evaluate(&rules, &readings, 20);

        let msg = fired[0].to_message();
        assert!(msg.body.contains("95.0"));
        assert!(msg.body.contains("80.0"));
        assert_eq!(msg.level, NotificationLevel::Warning);
    }
}
