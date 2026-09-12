use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverviewServiceItemDto {
    pub id: i64,
    pub name: String,
    pub service_type: String,
    pub status: String,
    pub created_at: i64,
    pub project_id: i64,
    pub project_name: String,
    pub environment_id: i64,
    pub environment_name: String,
    pub db_kind: Option<String>,
    pub server_id: Option<i64>,
    pub server_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverviewDomainItemDto {
    pub id: i64,
    pub host: String,
    pub path: Option<String>,
    pub port: Option<i32>,
    pub https: bool,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub service_name: String,
    pub project_name: String,
}

/// Typed linkage mirrors so the compose/database/application detail tabs
/// can filter their own rules (they match on compose_id / *_id / app_name).
/// Flattened into the payload so the JSON stays top-level.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackupLinkage {
    pub service_id: Option<i64>,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub postgres_id: Option<i64>,
    pub mysql_id: Option<i64>,
    pub mariadb_id: Option<i64>,
    pub mongo_id: Option<i64>,
    pub redis_id: Option<i64>,
    pub libsql_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverviewBackupItemDto {
    pub id: i64,
    pub name: String,
    pub backup_type: String,
    pub status: String,
    pub destination: String,
    #[serde(flatten)]
    pub linkage: BackupLinkage,
    pub created_at: i64,
}
