use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    sqlx::Type,
    poem_openapi::Enum,
    ts_rs::TS,
)]
#[sqlx(rename_all = "lowercase")]
#[oai(rename_all = "lowercase")]
pub enum DatabaseKind {
    Postgres,
    Mysql,
    Mariadb,
    Mongo,
    Redis,
    Libsql,
}

impl DatabaseKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Postgres => "postgres",
            Self::Mysql => "mysql",
            Self::Mariadb => "mariadb",
            Self::Mongo => "mongo",
            Self::Redis => "redis",
            Self::Libsql => "libsql",
        }
    }

    pub fn default_image(self) -> &'static str {
        match self {
            Self::Postgres => "postgres:18",
            Self::Mysql => "mysql:9",
            Self::Mariadb => "mariadb:13",
            Self::Mongo => "mongo:8",
            Self::Redis => "redis:8",
            Self::Libsql => "ghcr.io/tursodatabase/libsql-server:v0.24.32",
        }
    }
}

impl FromStr for DatabaseKind {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "postgres" | "postgresql" => Ok(Self::Postgres),
            "mysql" => Ok(Self::Mysql),
            "mariadb" => Ok(Self::Mariadb),
            "mongo" | "mongodb" => Ok(Self::Mongo),
            "redis" => Ok(Self::Redis),
            "libsql" => Ok(Self::Libsql),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DatabaseRecord {
    pub kind: DatabaseKind,
    pub id: i64,
    pub name: String,
    pub app_name: String,
    pub description: Option<String>,
    pub docker_image: String,
    pub database_name: Option<String>,
    pub database_user: Option<String>,
    pub database_password: Option<String>,
    pub external_port: Option<i64>,
    pub env_var: Option<String>,
    pub memory_reservation: Option<String>,
    pub memory_limit: Option<String>,
    pub cpu_reservation: Option<String>,
    pub cpu_limit: Option<String>,
    pub replicas: i64,
    pub network_ids: String,
    pub detach_rustploy_network: i64,
    pub app_status: String,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

use os::string_enum;

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum DatabaseRuntimeStatus {
        default = Idle;
        Idle => "IDLE",
        Queued => "QUEUED",
        Building => "BUILDING",
        Deploying => "DEPLOYING",
        Starting => "STARTING",
        Running => "RUNNING",
        Healthy => "HEALTHY",
        Stopping => "STOPPING",
        Stopped => "STOPPED",
        Cancelling => "CANCELLING",
        Cancelled => "CANCELLED",
        Done => "DONE",
        Failed => "FAILED"
    }
}

impl DatabaseRuntimeStatus {
    pub fn is_building(self) -> bool {
        matches!(
            self,
            Self::Queued | Self::Building | Self::Deploying | Self::Starting | Self::Cancelling
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DatabaseOperation {
    Deploy,
    Redeploy,
    Reload,
    Start,
    Stop,
}

impl DatabaseOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Deploy => "deploy",
            Self::Redeploy => "redeploy",
            Self::Reload => "reload",
            Self::Start => "start",
            Self::Stop => "stop",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Deploy => "Deploying Database",
            Self::Redeploy => "Redeploying Database",
            Self::Reload => "Reloading Database",
            Self::Start => "Starting Database",
            Self::Stop => "Stopping Database",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DatabaseOperationResult {
    pub database: DatabaseRecord,
    pub operation: DatabaseOperation,
}
