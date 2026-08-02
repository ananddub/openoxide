use crate::{
    db::{models::alert_rule::AlertRule, repository::AlertRuleRepository},
    services::{
        monitoring::{
            alert_engine::{AlertEngine, TargetReading},
            alert_rule_eval::{MetricSample, ParsedRule},
            monitoring_service::MonitoringService,
        },
        notification::{NotificationService, NotificationTrigger},
    },
};
use auto_di::singleton;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Seconds between evaluation passes. Also the assumed spacing between samples,
/// so a rule's `duration_seconds` converts to a window size.
pub const EVALUATION_INTERVAL_SECS: i64 = 10;

/// Owns alert rule CRUD and the periodic evaluation that turns breaches into
/// notifications.
///
/// The engine itself is pure and lives behind a mutex; this service supplies it
/// with rules from the database and metrics from `MonitoringService`, then
/// hands anything that fires to `NotificationService`.
pub struct AlertService {
    repo: Arc<AlertRuleRepository>,
    monitoring: Arc<MonitoringService>,
    notifications: Arc<NotificationService>,
    engine: Mutex<AlertEngine>,
}

#[singleton]
impl AlertService {
    pub fn new(
        repo: Arc<AlertRuleRepository>,
        monitoring: Arc<MonitoringService>,
        notifications: Arc<NotificationService>,
    ) -> Self {
        Self {
            repo,
            monitoring,
            notifications,
            engine: Mutex::new(AlertEngine::new()),
        }
    }

    pub async fn list_rules(&self) -> Result<Vec<AlertRule>, String> {
        self.repo.list().await.map_err(|e| e.to_string())
    }

    pub async fn create_rule(&self, rule: AlertRule) -> Result<i64, String> {
        self.repo.create(&rule).await.map_err(|e| e.to_string())
    }

    pub async fn delete_rule(&self, id: i64) -> Result<(), String> {
        self.repo.delete(id).await.map_err(|e| e.to_string())
    }

    /// Spawns the periodic evaluation loop.
    ///
    /// Runs for the life of the process. A failed pass is logged and the loop
    /// continues — a transient database or metric error should not permanently
    /// stop alerting.
    pub fn start(self: &Arc<Self>) {
        let service = Arc::clone(self);

        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(std::time::Duration::from_secs(
                EVALUATION_INTERVAL_SECS as u64,
            ));
            // If a pass overruns the interval, skip the missed ticks instead of
            // queuing them; catching up would just re-evaluate stale metrics.
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            tracing::info!(
                interval_secs = EVALUATION_INTERVAL_SECS,
                "alert evaluation loop started"
            );

            loop {
                ticker.tick().await;

                match service.evaluate_once().await {
                    Ok(0) => {}
                    Ok(count) => tracing::debug!(count, "alerts dispatched"),
                    Err(error) => tracing::error!(error = %error, "alert evaluation failed"),
                }
            }
        });
    }

    /// Runs one evaluation pass and dispatches whatever fires.
    ///
    /// Returns how many alerts were sent, mostly so the caller can log it.
    /// Errors are returned rather than logged here so the caller decides
    /// whether a failed pass is worth reporting every time.
    pub async fn evaluate_once(&self) -> Result<usize, String> {
        let rows = self.list_rules().await?;

        // Unparseable rules are surfaced once per pass rather than dropped
        // silently, since a typo'd operator otherwise looks like a rule that
        // simply never triggers.
        let mut rules = Vec::new();
        for row in &rows {
            match ParsedRule::from_row(row, EVALUATION_INTERVAL_SECS) {
                Ok(Some(parsed)) => rules.push(parsed),
                Ok(None) => {}
                Err(error) => tracing::warn!(error = %error, "skipping invalid alert rule"),
            }
        }

        let mut engine = self.engine.lock().await;

        if rules.is_empty() {
            // Nothing to watch: drop any state left behind by deleted rules.
            engine.retain_rules(&rules);
            return Ok(0);
        }

        let readings = self.collect_readings().await?;
        if readings.is_empty() {
            return Ok(0);
        }

        engine.retain_rules(&rules);
        let now = chrono::Utc::now().timestamp();
        let fired = engine.evaluate(&rules, &readings, now);
        drop(engine);

        for alert in &fired {
            tracing::info!(
                rule = %alert.rule_name,
                target = %alert.target_display,
                metric = alert.metric_label,
                value = alert.value,
                threshold = alert.threshold,
                "alert rule fired"
            );

            self.notifications
                .notify(NotificationTrigger::ServerThreshold, &alert.to_message())
                .await;
        }

        Ok(fired.len())
    }

    /// Gathers the current metric reading for every target rules can watch.
    ///
    /// Only server-level metrics are collected today. Container-level readings
    /// need per-container history the panel does not persist yet, so rules with
    /// a container target type simply never match rather than misfiring on
    /// host-wide numbers.
    async fn collect_readings(&self) -> Result<Vec<TargetReading>, String> {
        let latest = self.monitoring.get_latest_metrics(1).await?;

        let Some(metric) = latest.into_iter().next() else {
            return Ok(Vec::new());
        };

        Ok(vec![TargetReading {
            // The panel's own host is server 1; agents report under their own id
            // once multi-server metric ingestion lands.
            key: "1".to_string(),
            display_name: if metric.distro.is_empty() {
                "this server".to_string()
            } else {
                metric.distro.clone()
            },
            sample: MetricSample {
                cpu_percent: metric.cpu,
                memory_percent: metric.mem_used,
                disk_percent: metric.disk_used,
            },
        }])
    }
}
