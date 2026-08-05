use super::{reading::FiredAlert, reading::TargetReading, rule::ParsedRule, sample::SampleWindow};
use std::collections::HashMap;

#[derive(Debug)]
struct RuleState {
    window: SampleWindow,
    last_fired_at: Option<i64>,
}

#[derive(Debug, Default)]
pub struct AlertEngine {
    state: HashMap<String, RuleState>,
}

impl AlertEngine {
    pub fn new() -> Self {
        Self::default()
    }

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
                entry.window.reset();

                fired.push(FiredAlert {
                    rule_id: rule.id,
                    rule_name: rule.name.clone(),
                    target_key: reading.key.clone(),
                    target_kind: reading.kind,
                    target_display: reading.display_name.clone(),
                    metric_label: rule.metric.label(),
                    operator_symbol: rule.operator.symbol(),
                    value,
                    threshold: rule.threshold,
                    window_samples: rule.window_samples,
                    organization_id: rule.organization_id,
                });
            }
        }

        fired
    }

    pub fn retain_rules(&mut self, rules: &[ParsedRule]) {
        let live: Vec<String> = rules.iter().map(|r| format!("{}:", r.id)).collect();
        self.state
            .retain(|key, _| live.iter().any(|prefix| key.starts_with(prefix)));
    }

    pub fn tracked_targets(&self) -> usize {
        self.state.len()
    }

    fn rule_covers(rule: &ParsedRule, reading: &TargetReading) -> bool {
        if rule.target_kind != reading.kind {
            return false;
        }
        // target_id 0 means every target of this kind, so one rule can cover a
        // whole fleet without enumerating it.
        if rule.target_id == 0 {
            return true;
        }
        reading.key == rule.target_id.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::alert::{MetricKind, MetricSample, Operator, TargetKind};

    fn rule(target_kind: TargetKind, target_id: i64, window_samples: usize) -> ParsedRule {
        ParsedRule {
            id: 1,
            name: "high cpu".into(),
            target_kind,
            target_id,
            metric: MetricKind::Cpu,
            operator: Operator::GreaterThan,
            threshold: 80.0,
            window_samples,
            cooldown_seconds: 300,
            organization_id: 42,
        }
    }

    fn reading(kind: TargetKind, key: &str, cpu: f64) -> TargetReading {
        TargetReading {
            kind,
            key: key.into(),
            display_name: format!("target {key}"),
            sample: MetricSample {
                cpu_percent: cpu,
                memory_percent: 0.0,
                disk_percent: 0.0,
            },
        }
    }

    #[test]
    fn fires_only_once_the_window_is_full() {
        let mut engine = AlertEngine::new();
        let rules = vec![rule(TargetKind::Server, 1, 3)];
        let readings = vec![reading(TargetKind::Server, "1", 95.0)];

        assert!(engine.evaluate(&rules, &readings, 0).is_empty());
        assert!(engine.evaluate(&rules, &readings, 1).is_empty());
        assert_eq!(engine.evaluate(&rules, &readings, 2).len(), 1);
    }

    #[test]
    fn healthy_metrics_never_fire() {
        let mut engine = AlertEngine::new();
        let rules = vec![rule(TargetKind::Server, 1, 2)];
        let readings = vec![reading(TargetKind::Server, "1", 10.0)];

        for tick in 0..10 {
            assert!(engine.evaluate(&rules, &readings, tick).is_empty());
        }
    }

    #[test]
    fn a_cooldown_suppresses_repeats() {
        let mut engine = AlertEngine::new();
        let rules = vec![rule(TargetKind::Server, 1, 2)];
        let readings = vec![reading(TargetKind::Server, "1", 95.0)];

        engine.evaluate(&rules, &readings, 0);
        assert_eq!(engine.evaluate(&rules, &readings, 1).len(), 1);

        // Still inside the 300s cooldown.
        for tick in 2..200 {
            assert!(engine.evaluate(&rules, &readings, tick).is_empty());
        }
    }

    #[test]
    fn it_fires_again_once_the_cooldown_expires() {
        let mut engine = AlertEngine::new();
        let rules = vec![rule(TargetKind::Server, 1, 2)];
        let readings = vec![reading(TargetKind::Server, "1", 95.0)];

        engine.evaluate(&rules, &readings, 0);
        assert_eq!(engine.evaluate(&rules, &readings, 1).len(), 1);

        engine.evaluate(&rules, &readings, 400);
        assert_eq!(engine.evaluate(&rules, &readings, 401).len(), 1);
    }

    /// The bug this guards: matching on id alone let a rule watching server 1
    /// fire on application 1's metrics.
    #[test]
    fn a_rule_does_not_match_a_different_target_kind() {
        let mut engine = AlertEngine::new();
        let rules = vec![rule(TargetKind::Server, 1, 1)];
        let readings = vec![reading(TargetKind::Application, "1", 95.0)];

        assert!(engine.evaluate(&rules, &readings, 0).is_empty());
    }

    #[test]
    fn a_wildcard_rule_covers_every_target_of_its_kind() {
        let mut engine = AlertEngine::new();
        let rules = vec![rule(TargetKind::Application, 0, 1)];
        let readings = vec![
            reading(TargetKind::Application, "7", 95.0),
            reading(TargetKind::Application, "9", 95.0),
            reading(TargetKind::Server, "1", 95.0),
        ];

        let fired = engine.evaluate(&rules, &readings, 0);
        assert_eq!(fired.len(), 2, "only the application readings should fire");
    }

    #[test]
    fn a_rule_ignores_targets_it_does_not_cover() {
        let mut engine = AlertEngine::new();
        let rules = vec![rule(TargetKind::Application, 7, 1)];
        let readings = vec![reading(TargetKind::Application, "9", 95.0)];

        assert!(engine.evaluate(&rules, &readings, 0).is_empty());
    }

    #[test]
    fn each_target_tracks_its_own_window() {
        let mut engine = AlertEngine::new();
        let rules = vec![rule(TargetKind::Application, 0, 2)];

        // Only target 7 breaches; target 9 stays healthy.
        let readings = vec![
            reading(TargetKind::Application, "7", 95.0),
            reading(TargetKind::Application, "9", 5.0),
        ];

        engine.evaluate(&rules, &readings, 0);
        let fired = engine.evaluate(&rules, &readings, 1);

        assert_eq!(fired.len(), 1);
        assert_eq!(fired[0].target_key, "7");
    }

    #[test]
    fn a_fired_alert_carries_its_organization() {
        let mut engine = AlertEngine::new();
        let rules = vec![rule(TargetKind::Server, 1, 1)];
        let readings = vec![reading(TargetKind::Server, "1", 95.0)];

        let fired = engine.evaluate(&rules, &readings, 0);
        assert_eq!(fired[0].organization_id, 42);
    }

    #[test]
    fn editing_a_rule_resizes_its_window() {
        let mut engine = AlertEngine::new();
        let readings = vec![reading(TargetKind::Server, "1", 95.0)];

        let wide = vec![rule(TargetKind::Server, 1, 10)];
        for tick in 0..5 {
            assert!(engine.evaluate(&wide, &readings, tick).is_empty());
        }

        // Shrinking the window must not inherit progress sized for the old one.
        let narrow = vec![rule(TargetKind::Server, 1, 2)];
        assert!(engine.evaluate(&narrow, &readings, 5).is_empty());
        assert_eq!(engine.evaluate(&narrow, &readings, 6).len(), 1);
    }

    #[test]
    fn deleted_rules_stop_consuming_state() {
        let mut engine = AlertEngine::new();
        let rules = vec![rule(TargetKind::Server, 1, 2)];
        let readings = vec![reading(TargetKind::Server, "1", 95.0)];

        engine.evaluate(&rules, &readings, 0);
        assert_eq!(engine.tracked_targets(), 1);

        engine.retain_rules(&[]);
        assert_eq!(engine.tracked_targets(), 0);
    }
}
