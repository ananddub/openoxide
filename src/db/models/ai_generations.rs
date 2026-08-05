use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct AiGeneration {
    pub id: i64,
    pub ai_setting_id: i64,
    pub organization_id: i64,
    pub created_by: i64,
    pub prompt: String,
    pub output_json: String,
    pub status: String,
    pub compose_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}
