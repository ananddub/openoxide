use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct AlertRule {
    pub id: Option<i64>,
    pub name: String,
    pub target_type: String,
    pub target_id: i64,
    pub metric_name: String,
    pub operator: String,
    pub threshold: f64,
    pub duration_seconds: i32,
    pub notification_channel: String,
    pub enabled: i32,
    pub organization_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}
