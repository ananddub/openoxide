use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ServerManagement {
    pub server_id: i64,
    pub maintenance_mode: i64,
    pub maintenance_message: Option<String>,
    pub labels: String,
    pub cleanup_policy: String,
    pub gpu_enabled: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ServerCleanupExecution {
    pub id: i64,
    pub server_id: i64,
    pub status: String,
    pub policy: String,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub started_at: i64,
    pub finished_at: Option<i64>,
}
