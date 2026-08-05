use crate::db::models::alert_rule::AlertRule;
use crate::services::alert::AlertEventState;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct AlertRuleRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl AlertRuleRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    /// Every rule across all tenants. Only the evaluation loop should use this;
    /// anything serving a request must scope by organization.
    pub async fn list(&self) -> Result<Vec<AlertRule>, sqlx::Error> {
        sqlx::query_as!(
            AlertRule,
            r#"SELECT id AS "id?: i64", name, target_type, target_id, metric_name, operator, threshold AS "threshold: f64", duration_seconds AS "duration_seconds: i32", notification_channel, enabled AS "enabled: i32", organization_id AS "organization_id: i64", created_at, updated_at FROM alert_rules ORDER BY created_at DESC"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn list_by_organization(
        &self,
        organization_id: i64,
    ) -> Result<Vec<AlertRule>, sqlx::Error> {
        sqlx::query_as!(
            AlertRule,
            r#"SELECT id AS "id?: i64", name, target_type, target_id, metric_name, operator, threshold AS "threshold: f64", duration_seconds AS "duration_seconds: i32", notification_channel, enabled AS "enabled: i32", organization_id AS "organization_id: i64", created_at, updated_at FROM alert_rules WHERE organization_id = ? ORDER BY created_at DESC"#,
            organization_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<AlertRule>, sqlx::Error> {
        sqlx::query_as!(
            AlertRule,
            r#"SELECT id AS "id?: i64", name, target_type, target_id, metric_name, operator, threshold AS "threshold: f64", duration_seconds AS "duration_seconds: i32", notification_channel, enabled AS "enabled: i32", organization_id AS "organization_id: i64", created_at, updated_at FROM alert_rules WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    /// Scoped lookup, so one tenant cannot read another's rule by guessing ids.
    pub async fn get_by_id_for_organization(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<Option<AlertRule>, sqlx::Error> {
        sqlx::query_as!(
            AlertRule,
            r#"SELECT id AS "id?: i64", name, target_type, target_id, metric_name, operator, threshold AS "threshold: f64", duration_seconds AS "duration_seconds: i32", notification_channel, enabled AS "enabled: i32", organization_id AS "organization_id: i64", created_at, updated_at FROM alert_rules WHERE id = ? AND organization_id = ?"#,
            id,
            organization_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, rule: &AlertRule) -> Result<i64, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        let res = sqlx::query!(
            r#"INSERT INTO alert_rules (name, target_type, target_id, metric_name, operator, threshold, duration_seconds, notification_channel, enabled, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            rule.name,
            rule.target_type,
            rule.target_id,
            rule.metric_name,
            rule.operator,
            rule.threshold,
            rule.duration_seconds,
            rule.notification_channel,
            rule.enabled,
            rule.organization_id,
            now,
            now
        )
        .execute(self.pool.as_ref())
        .await?;

        Ok(res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, rule: &AlertRule) -> Result<(), sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        sqlx::query!(
            r#"UPDATE alert_rules SET name = ?, target_type = ?, target_id = ?, metric_name = ?, operator = ?, threshold = ?, duration_seconds = ?, notification_channel = ?, enabled = ?, updated_at = ? WHERE id = ? AND organization_id = ?"#,
            rule.name,
            rule.target_type,
            rule.target_id,
            rule.metric_name,
            rule.operator,
            rule.threshold,
            rule.duration_seconds,
            rule.notification_channel,
            rule.enabled,
            now,
            id,
            rule.organization_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    /// Returns whether a row was actually removed, so a caller can tell a
    /// missing rule from one belonging to another tenant.
    pub async fn delete(&self, id: i64, organization_id: i64) -> Result<bool, sqlx::Error> {
        let res = sqlx::query!(
            r#"DELETE FROM alert_rules WHERE id = ? AND organization_id = ?"#,
            id,
            organization_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(res.rows_affected() > 0)
    }

    pub async fn record_event(
        &self,
        rule_id: i64,
        organization_id: i64,
        target_key: &str,
        state: AlertEventState,
        value: Option<f64>,
        threshold: Option<f64>,
        message: &str,
    ) -> Result<(), sqlx::Error> {
        let state = state.as_str();
        sqlx::query!(
            "INSERT INTO alert_events (alert_rule_id, organization_id, target_key, state, value, threshold, message, resolved_at) VALUES (?, ?, ?, ?, ?, ?, ?, CASE WHEN ? = 'RESOLVED' THEN unixepoch() END)",
            rule_id,
            organization_id,
            target_key,
            state,
            value,
            threshold,
            message,
            state
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn list_events(
        &self,
        organization_id: i64,
        limit: i64,
    ) -> Result<Vec<AlertEvent>, sqlx::Error> {
        sqlx::query_as!(
            AlertEvent,
            r#"SELECT id AS "id!: i64", alert_rule_id AS "alert_rule_id!: i64", organization_id AS "organization_id!: i64", target_key AS "target_key!: String", state AS "state!: String", value, threshold, message AS "message!: String", created_at AS "created_at!: i64", acknowledged_at, acknowledged_by, silenced_until, resolved_at, notification_correlation_id FROM alert_events WHERE organization_id=? ORDER BY created_at DESC LIMIT ?"#,
            organization_id,
            limit.clamp(1, 500)
        )
        .fetch_all(self.pool.as_ref())
        .await
    }
}

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct AlertEvent {
    pub id: i64,
    pub alert_rule_id: i64,
    pub organization_id: i64,
    pub target_key: String,
    pub state: String,
    pub value: Option<f64>,
    pub threshold: Option<f64>,
    pub message: String,
    pub created_at: i64,
    pub acknowledged_at: Option<i64>,
    pub acknowledged_by: Option<i64>,
    pub silenced_until: Option<i64>,
    pub resolved_at: Option<i64>,
    pub notification_correlation_id: Option<String>,
}
