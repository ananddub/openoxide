use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, Object, TS)]
#[ts(export)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Object, TS)]
#[ts(export)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Object, TS)]
#[ts(export)]
pub struct OverviewBackupItemDto {
    pub id: i64,
    pub name: String,
    pub backup_type: String,
    pub status: String,
    pub destination: String,
    pub created_at: i64,
}
