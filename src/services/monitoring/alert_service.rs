use crate::{
    db::{models::alert_rule::AlertRule, repository::AlertRuleRepository},
    services::{
        monitoring::{
            alert::{AlertEngine, MetricSample, ParsedRule, TargetKind, TargetReading},
            monitoring_service::MonitoringService,
            sse::MonitoringSseBus,
        },
        notification::{NotificationScope, NotificationService, NotificationTrigger},
    },
};
use auto_di::singleton;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub const EVALUATION_INTERVAL_SECS: i64 = 10;

/// Newest metrics seen for one container, keyed by container id.
#[derive(Clone, Debug)]
struct ContainerSample {
    server_id: i64,
    kind: TargetKind,
    target_id: i64,
    name: String,
    cpu_percent: f64,
    memory_percent: f64,
    received_at: i64,
}

pub struct AlertService {
    repo: Arc<AlertRuleRepository>,
    monitoring: Arc<MonitoringService>,
    notifications: Arc<NotificationService>,
    sse_bus: Arc<MonitoringSseBus>,
    engine: Mutex<AlertEngine>,
    rules_cache: RwLock<Option<Vec<ParsedRule>>>,
    container_samples: RwLock<HashMap<String, ContainerSample>>,
}

#[singleton]
impl AlertService {
    pub fn new(
        repo: Arc<AlertRuleRepository>,
        monitoring: Arc<MonitoringService>,
        notifications: Arc<NotificationService>,
        sse_bus: Arc<MonitoringSseBus>,
    ) -> Self {
        Self {
            repo,
            monitoring,
            notifications,
            sse_bus,
            engine: Mutex::new(AlertEngine::new()),
            rules_cache: RwLock::new(None),
            container_samples: RwLock::new(HashMap::new()),
        }
    }

    pub async fn list_rules(&self, organization_id: i64) -> Result<Vec<AlertRule>, String> {
        self.repo
            .list_by_organization(organization_id)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn list_events(&self, organization_id: i64, limit: i64) -> Result<Vec<crate::db::repository::alert_rule::AlertEvent>, String> {
        self.repo.list_events(organization_id, limit).await.map_err(|e| e.to_string())
    }

    pub async fn get_rule(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<Option<AlertRule>, String> {
        self.repo
            .get_by_id_for_organization(id, organization_id)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn create_rule(&self, rule: AlertRule) -> Result<i64, String> {
        let id = self.repo.create(&rule).await.map_err(|e| e.to_string())?;
        self.invalidate_rules_cache().await;
        Ok(id)
    }

    pub async fn update_rule(&self, id: i64, rule: AlertRule) -> Result<(), String> {
        self.repo
            .update(id, &rule)
            .await
            .map_err(|e| e.to_string())?;
        self.invalidate_rules_cache().await;
        Ok(())
    }

    /// `false` when no rule with that id belongs to the organization.
    pub async fn delete_rule(&self, id: i64, organization_id: i64) -> Result<bool, String> {
        let removed = self
            .repo
            .delete(id, organization_id)
            .await
            .map_err(|e| e.to_string())?;

        if removed {
            self.invalidate_rules_cache().await;
        }
        Ok(removed)
    }

    async fn invalidate_rules_cache(&self) {
        let mut cache = self.rules_cache.write().await;
        *cache = None;
    }

    /// Latches the newest metrics per container off the SSE bus.
    ///
    /// Agents push far more often than rules are evaluated, so only the latest
    /// reading per container is kept; the evaluation loop reads whatever landed
    /// since its last pass.
    fn start_container_ingest(self: &Arc<Self>) {
        let service = Arc::clone(self);
        let mut receiver = self.sse_bus.subscribe_container_metrics();

        tokio::spawn(async move {
            loop {
                match receiver.recv().await {
                    Ok(event) => {
                        // Agents that cannot attribute a container to a resource
                        // send zeros; such a container has nothing to alert on.
                        let Some((kind, target_id)) = classify_container(&event) else {
                            continue;
                        };

                        let memory_percent = if event.memory_limit_mb > 0.0 {
                            (event.memory_used_mb / event.memory_limit_mb) * 100.0
                        } else {
                            0.0
                        };

                        let mut samples = service.container_samples.write().await;
                        samples.insert(
                            format!("{}:{}", event.server_id, event.container_id),
                            ContainerSample {
                                server_id: event.server_id,
                                kind,
                                target_id,
                                name: event.container_name.clone(),
                                cpu_percent: event.cpu_percent,
                                memory_percent,
                                received_at: chrono::Utc::now().timestamp(),
                            },
                        );
                    }
                    // A slow pass can fall behind a fast agent. Only the newest
                    // reading matters, so skipped events are not an error.
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(skipped)) => {
                        tracing::debug!(skipped, "dropped stale container metrics");
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => return,
                }
            }
        });
    }

    pub fn start(self: &Arc<Self>) {
        self.start_container_ingest();

        let service = Arc::clone(self);

        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(std::time::Duration::from_secs(
                EVALUATION_INTERVAL_SECS as u64,
            ));
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

    pub async fn evaluate_once(&self) -> Result<usize, String> {
        let cached_rules = { self.rules_cache.read().await.clone() };

        let rules = match cached_rules {
            Some(rules) => rules,
            None => {
                // Evaluation spans every tenant; each fired alert carries its
                // own organization so dispatch stays scoped.
                let rows = self.repo.list().await.map_err(|e| e.to_string())?;
                let mut parsed_rules = Vec::new();
                for row in &rows {
                    match ParsedRule::from_row(row, EVALUATION_INTERVAL_SECS) {
                        Ok(Some(parsed)) => parsed_rules.push(parsed),
                        Ok(None) => {}
                        Err(error) => tracing::warn!(error = %error, "skipping invalid alert rule"),
                    }
                }
                let mut cache = self.rules_cache.write().await;
                *cache = Some(parsed_rules.clone());
                parsed_rules
            }
        };

        if rules.is_empty() {
            let mut engine = self.engine.lock().await;
            engine.retain_rules(&[]);
            return Ok(0);
        }

        let readings = self.collect_readings().await?;
        if readings.is_empty() {
            return Ok(0);
        }

        let mut engine = self.engine.lock().await;
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
                .notify(
                    NotificationScope::Organization(alert.organization_id),
                    NotificationTrigger::ServerThreshold,
                    &alert.to_message(),
                )
                .await;
            let _ = self.repo.record_event(alert.rule_id, alert.organization_id, &alert.target_key, "FIRING", Some(alert.value), Some(alert.threshold), &alert.to_message().body).await;
        }

        Ok(fired.len())
    }

    async fn collect_readings(&self) -> Result<Vec<TargetReading>, String> {
        let mut readings = self.host_readings().await?;
        readings.extend(self.container_readings().await);
        Ok(readings)
    }

    async fn host_readings(&self) -> Result<Vec<TargetReading>, String> {
        const HOST_SAMPLE_TTL_SECS: i64 = EVALUATION_INTERVAL_SECS * 3;
        let now = chrono::Utc::now().timestamp();
        Ok(self
            .monitoring
            .get_latest_metrics_per_server()
            .await?
            .into_iter()
            .filter(|metric| metric.timestamp.is_some_and(|timestamp| now.saturating_sub(timestamp) <= HOST_SAMPLE_TTL_SECS))
            .map(|metric| TargetReading {
                kind: TargetKind::Server,
                key: metric.server_id.to_string(),
                display_name: if metric.distro.is_empty() {
                    format!("server {}", metric.server_id)
                } else {
                    metric.distro.clone()
                },
                sample: MetricSample {
                    cpu_percent: metric.cpu,
                    memory_percent: metric.mem_used,
                    disk_percent: metric.disk_used,
                },
            })
            .collect())
    }

    /// Latest reading per container, from what agents have pushed since the last
    /// pass.
    ///
    /// Containers report no disk percentage — the cgroup path has no such figure
    /// and a container's writable layer is not a percentage of anything — so a
    /// DISK rule on a container target never breaches.
    async fn container_readings(&self) -> Vec<TargetReading> {
        const SAMPLE_TTL_SECS: i64 = EVALUATION_INTERVAL_SECS * 3;
        let now = chrono::Utc::now().timestamp();
        let mut samples = self.container_samples.write().await;
        samples.retain(|_, sample| now.saturating_sub(sample.received_at) <= SAMPLE_TTL_SECS);

        samples
            .values()
            .map(|sample| TargetReading {
                kind: sample.kind,
                key: sample.target_id.to_string(),
                display_name: format!("{} (server {})", sample.name, sample.server_id),
                sample: MetricSample {
                    cpu_percent: sample.cpu_percent,
                    memory_percent: sample.memory_percent,
                    disk_percent: 0.0,
                },
            })
            .collect()
    }
}

/// Which resource a pushed container metric belongs to.
///
/// Agents report ids they can attribute and zero otherwise, so an unattributed
/// container yields `None` rather than being filed under resource 0.
fn classify_container(
    event: &crate::services::monitoring::sse::ContainerMetricSseEvent,
) -> Option<(TargetKind, i64)> {
    if event.application_id > 0 {
        return Some((TargetKind::Application, event.application_id));
    }
    if event.compose_id > 0 {
        return Some((TargetKind::Compose, event.compose_id));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::monitoring::sse::ContainerMetricSseEvent;

    fn event(application_id: i64, compose_id: i64) -> ContainerMetricSseEvent {
        ContainerMetricSseEvent {
            server_id: 1,
            application_id,
            compose_id,
            container_id: "abc123".into(),
            container_name: "web".into(),
            cpu_percent: 10.0,
            memory_used_mb: 256.0,
            memory_limit_mb: 1024.0,
            net_rx_kbps: 0.0,
            net_tx_kbps: 0.0,
            timestamp: 0,
        }
    }

    #[test]
    fn an_application_container_is_classified_as_such() {
        assert_eq!(
            classify_container(&event(7, 0)),
            Some((TargetKind::Application, 7))
        );
    }

    #[test]
    fn a_compose_container_is_classified_as_such() {
        assert_eq!(
            classify_container(&event(0, 9)),
            Some((TargetKind::Compose, 9))
        );
    }

    #[test]
    fn an_application_id_wins_when_both_are_set() {
        assert_eq!(
            classify_container(&event(7, 9)),
            Some((TargetKind::Application, 7))
        );
    }

    /// Agents send zeros for containers they cannot attribute; those must not be
    /// filed under resource 0, which would make one rule match everything.
    #[test]
    fn an_unattributed_container_is_ignored() {
        assert_eq!(classify_container(&event(0, 0)), None);
    }
}
