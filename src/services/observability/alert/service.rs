use crate::{
    db::queries::FlashQueries,
    flash_gen::alert_rule::{AlertRuleCreateParams, AlertRuleUpdateParams},
    services::{
        integrations::notification::{
            NotificationScope, NotificationService, NotificationTrigger,
        },
        observability::alert::{AlertEngine, MetricSample, ParsedRule, TargetKind, TargetReading},
        observability::monitoring::service::MonitoringService,
    },
};
use super::rule::AlertRule;
use auto_di::singleton;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub const EVALUATION_INTERVAL_SECS: i64 = 10;

pub struct AlertService {
    queries: Arc<FlashQueries>,
    monitoring: Arc<MonitoringService>,
    notifications: Arc<NotificationService>,
    engine: Mutex<AlertEngine>,
    rules_cache: RwLock<Option<Vec<ParsedRule>>>,
}

pub(crate) fn rule_from_flash(row: crate::flash_gen::models::AlertRules) -> AlertRule {
    AlertRule {
        id: Some(row.id),
        name: row.name,
        target_type: row.target_type,
        target_id: row.target_id,
        metric_name: row.metric_name,
        operator: row.operator,
        threshold: f64::from(row.threshold),
        duration_seconds: row.duration_seconds as i32,
        notification_channel: row.notification_channel,
        enabled: row.enabled as i32,
        organization_id: row.organization_id,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}

#[singleton]
impl AlertService {
    pub fn new(
        queries: Arc<FlashQueries>,
        monitoring: Arc<MonitoringService>,
        notifications: Arc<NotificationService>,
    ) -> Self {
        Self {
            queries,
            monitoring,
            notifications,
            engine: Mutex::new(AlertEngine::new()),
            rules_cache: RwLock::new(None),
        }
    }

    pub async fn list_rules(&self, organization_id: i64) -> Result<Vec<AlertRule>, String> {
        self.queries
            .alert_rule_list_by_organization(organization_id)
            .await
            .map(|rows| rows.into_iter().map(rule_from_flash).collect())
            .map_err(|e| e.to_string())
    }

    pub async fn list_events(
        &self,
        organization_id: i64,
        limit: i64,
    ) -> Result<Vec<crate::flash_gen::models::AlertEvents>, String> {
        self.queries
            .alert_rule_list_events(organization_id, limit.clamp(1, 500))
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_rule(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<Option<AlertRule>, String> {
        match self
            .queries
            .alert_rule_get_by_id_for_organization(id, organization_id)
            .await
        {
            Ok(row) => Ok(Some(rule_from_flash(row))),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn create_rule(&self, rule: AlertRule) -> Result<i64, String> {
        let now = chrono::Utc::now().timestamp();
        let id = self
            .queries
            .alert_rule_create(&AlertRuleCreateParams {
                name: rule.name,
                target_type: rule.target_type,
                target_id: rule.target_id,
                metric_name: rule.metric_name,
                operator: rule.operator,
                threshold: rule.threshold as f32,
                duration_seconds: i64::from(rule.duration_seconds),
                notification_channel: rule.notification_channel,
                enabled: i64::from(rule.enabled),
                organization_id: rule.organization_id,
                created_at: now,
                updated_at: now,
            })
            .await
            .map_err(|e| e.to_string())?;
        self.invalidate_rules_cache().await;
        Ok(id)
    }

    pub async fn update_rule(&self, id: i64, rule: AlertRule) -> Result<(), String> {
        self.queries
            .alert_rule_update(&AlertRuleUpdateParams {
                name: rule.name,
                target_type: rule.target_type,
                target_id: rule.target_id,
                metric_name: rule.metric_name,
                operator: rule.operator,
                threshold: rule.threshold as f32,
                duration_seconds: i64::from(rule.duration_seconds),
                notification_channel: rule.notification_channel,
                enabled: i64::from(rule.enabled),
                updated_at: chrono::Utc::now().timestamp(),
                id,
                organization_id: rule.organization_id,
            })
            .await
            .map_err(|e| e.to_string())?;
        self.invalidate_rules_cache().await;
        Ok(())
    }

    /// `false` when no rule with that id belongs to the organization.
    pub async fn delete_rule(&self, id: i64, organization_id: i64) -> Result<bool, String> {
        let removed = match self
            .queries
            .alert_rule_delete(id, organization_id)
            .await
        {
            Ok(_) => true,
            Err(sqlx::Error::RowNotFound) => false,
            Err(e) => return Err(e.to_string()),
        };

        if removed {
            self.invalidate_rules_cache().await;
        }
        Ok(removed)
    }

    async fn invalidate_rules_cache(&self) {
        let mut cache = self.rules_cache.write().await;
        *cache = None;
    }

    pub async fn evaluate_once(&self) -> Result<usize, String> {
        let cached_rules = { self.rules_cache.read().await.clone() };

        let rules = match cached_rules {
            Some(rules) => rules,
            None => {
                // Evaluation spans every tenant; each fired alert carries its
                // own organization so dispatch stays scoped.
                let rows = self.queries.alert_rule_list().await.map_err(|e| e.to_string())?;
                let rows: Vec<AlertRule> = rows.into_iter().map(rule_from_flash).collect();
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
            if alert.target_kind == TargetKind::Server {
                let server_id = alert.target_key.parse::<i64>().unwrap_or_default();
                if self
                    .queries
                    .monitoring_lifecycle_is_in_maintenance(alert.organization_id, server_id)
                    .await
                    .map(|hit| hit != 0)
                    .unwrap_or(false)
                {
                    tracing::debug!(
                        server_id,
                        "alert suppressed by monitoring maintenance window"
                    );
                    continue;
                }
            }
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
        }

        Ok(fired.len())
    }

    async fn collect_readings(&self) -> Result<Vec<TargetReading>, String> {
        let mut readings = self.host_readings().await?;
        readings.extend(self.container_readings().await?);
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
            .filter(|metric| {
                metric
                    .timestamp
                    .is_some_and(|timestamp| now.saturating_sub(timestamp) <= HOST_SAMPLE_TTL_SECS)
            })
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
    async fn container_readings(&self) -> Result<Vec<TargetReading>, String> {
        const SAMPLE_TTL_SECS: i64 = EVALUATION_INTERVAL_SECS * 3;
        let now = chrono::Utc::now().timestamp();
        Ok(self
            .monitoring
            .get_latest_container_metrics()
            .await?
            .into_iter()
            .filter(|metric| now.saturating_sub(metric.timestamp) <= SAMPLE_TTL_SECS)
            .filter_map(|metric| {
                let (kind, target_id) = if let Some(id) = metric.application_id {
                    (TargetKind::Application, id)
                } else if let Some(id) = metric.compose_id {
                    (TargetKind::Compose, id)
                } else {
                    return None;
                };
                let payload: serde_json::Value = serde_json::from_str(&metric.metrics_json).ok()?;
                Some(TargetReading {
                    kind,
                    key: target_id.to_string(),
                    display_name: format!(
                        "{} (server {})",
                        metric.container_name, metric.server_id
                    ),
                    sample: MetricSample {
                        cpu_percent: payload
                            .get("cpu_percent")
                            .and_then(|v| v.as_f64())
                            .unwrap_or_default(),
                        memory_percent: payload
                            .get("memory_percent")
                            .and_then(|v| v.as_f64())
                            .unwrap_or_default(),
                        disk_percent: 0.0,
                    },
                })
            })
            .collect())
    }
}
