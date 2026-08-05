use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BackupExecution {
    pub id: i64,
    pub backup_kind: String,
    pub operation: String,
    pub backup_id: Option<i64>,
    pub status: String,
    pub object_key: Option<String>,
    pub checksum_sha256: Option<String>,
    pub size_bytes: Option<i64>,
    pub attempt: i64,
    pub error: Option<String>,
    pub started_at: i64,
    pub finished_at: Option<i64>,
}
