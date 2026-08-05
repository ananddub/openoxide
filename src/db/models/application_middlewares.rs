use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ApplicationMiddleware {
    pub id: i64,
    pub application_id: i64,
    pub name: String,
    pub middleware_type: String,
    pub enabled: i64,
    pub config: String,
    pub created_at: i64,
    pub updated_at: i64,
}
