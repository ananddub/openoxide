use crate::{
    core::cache::CacheKey,
    services::compute::database::{DatabaseKind, DatabaseService},
    utils::docker::query::{ContainerFilter, filter::ContainerStatus},
};
use os::string_enum;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::crud::{CreateDatabaseDto, PatchDatabaseDto, parse_json_string_vec};

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum PostgresReplicationRole {
        default = Standalone;

        Standalone => "STANDALONE",
        Primary => "PRIMARY",
        Replica => "REPLICA",
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DatabaseConnectionDto {
    pub kind: crate::services::compute::database::DatabaseKind,
    pub host: String,
    pub port: i64,
    pub database: Option<String>,
    pub username: Option<String>,
    pub password: String,
    pub internal_url: String,
    pub external_url: Option<String>,
    pub server_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DatabaseCredentialRotationDto {
    pub password: String,
    pub redeploy_required: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DatabaseValidationDto {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImportDatabaseDto {
    pub archive: String,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PostgresAdvancedConfigDto {
    pub settings: BTreeMap<String, String>,
    pub replication_role: PostgresReplicationRole,
    pub primary_host: Option<String>,
    pub primary_port: Option<i64>,
    pub replication_user: Option<String>,
    pub replication_password: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostgresAdvancedConfigResponseDto {
    pub settings: BTreeMap<String, String>,
    pub replication_role: PostgresReplicationRole,
    pub primary_host: Option<String>,
    pub primary_port: Option<i64>,
    pub replication_user: Option<String>,
    pub replication_password_configured: bool,
    pub redeploy_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseExportBundleDto {
    pub schema_version: u32,
    pub exported_at: i64,
    pub secrets_included: bool,
    pub kind: crate::services::compute::database::DatabaseKind,
    pub name: String,
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
    pub network_ids: Vec<String>,
    pub detach_rustploy_network: i64,
}

#[derive(Debug, Clone)]
struct EngineCredentials {
    app_name: String,
    username: Option<String>,
    password: String,
    database_name: Option<String>,
    external_port: Option<i64>,
    server_id: Option<i64>,
}

impl DatabaseService {
    async fn engine_credentials(
        &self,
        kind: DatabaseKind,
        id: i64,
    ) -> sqlx::Result<EngineCredentials> {
        let _ = kind;
        let row = self.queries.database_management_credentials(id).await?;
        Ok(EngineCredentials {
            app_name: row.app_name,
            username: row.db_user,
            password: row.db_password.unwrap_or_default(),
            database_name: row.db_name,
            external_port: row.external_port,
            server_id: row.server_id,
        })
    }

    pub async fn export(
        &self,
        kind: DatabaseKind,
        id: i64,
        include_secrets: bool,
    ) -> sqlx::Result<DatabaseExportBundleDto> {
        let record = self.get_by_id(kind, id).await?;
        let credentials = self.engine_credentials(kind, id).await?;
        Ok(DatabaseExportBundleDto {
            schema_version: 1,
            exported_at: chrono::Utc::now().timestamp(),
            secrets_included: include_secrets,
            kind,
            name: record.name,
            description: record.description,
            docker_image: record.docker_image,
            database_name: record.database_name,
            database_user: record.database_user,
            database_password: include_secrets.then_some(credentials.password),
            external_port: record.external_port,
            env_var: include_secrets.then_some(record.env_var).flatten(),
            memory_reservation: record.memory_reservation,
            memory_limit: record.memory_limit,
            cpu_reservation: record.cpu_reservation,
            cpu_limit: record.cpu_limit,
            replicas: record.replicas,
            network_ids: parse_json_string_vec(&record.network_ids),
            detach_rustploy_network: record.detach_rustploy_network,
        })
    }

    pub async fn import(&self, input: ImportDatabaseDto) -> sqlx::Result<super::DatabaseRecord> {
        let bundle: DatabaseExportBundleDto = serde_json::from_str(&input.archive)
            .map_err(|error| sqlx::Error::Protocol(format!("invalid database archive: {error}")))?;
        if bundle.schema_version != 1 {
            return Err(sqlx::Error::Protocol(format!(
                "unsupported database archive schema version: {}",
                bundle.schema_version
            )));
        }
        let created = self
            .create(
                bundle.kind,
                CreateDatabaseDto {
                    name: input.name.unwrap_or(bundle.name),
                    description: bundle.description,
                    environment_id: input.environment_id,
                    server_id: input.server_id,
                    docker_image: Some(bundle.docker_image),
                    database_name: bundle.database_name,
                    database_user: bundle.database_user,
                    database_password: bundle.database_password,
                    database_root_password: None,
                    external_port: bundle.external_port,
                    external_grpc_port: None,
                    external_admin_port: None,
                    command: None,
                    args: None,
                    env_var: bundle.env_var,
                    replica_sets: None,
                    sqld_node: None,
                    sqld_primary_url: None,
                    enable_namespaces: None,
                    network_ids: Some(bundle.network_ids),
                    detach_rustploy_network: Some(bundle.detach_rustploy_network),
                },
            )
            .await?;
        if bundle.memory_reservation.is_some()
            || bundle.memory_limit.is_some()
            || bundle.cpu_reservation.is_some()
            || bundle.cpu_limit.is_some()
            || bundle.replicas != 1
        {
            return self
                .patch(
                    bundle.kind,
                    created.id,
                    PatchDatabaseDto {
                        name: None,
                        description: None,
                        docker_image: None,
                        database_name: None,
                        database_user: None,
                        database_password: None,
                        database_root_password: None,
                        external_port: None,
                        external_grpc_port: None,
                        external_admin_port: None,
                        command: None,
                        args: None,
                        env_var: None,
                        memory_reservation: bundle.memory_reservation,
                        memory_limit: bundle.memory_limit,
                        cpu_reservation: bundle.cpu_reservation,
                        cpu_limit: bundle.cpu_limit,
                        replicas: Some(bundle.replicas),
                        server_id: None,
                        replica_sets: None,
                        sqld_node: None,
                        sqld_primary_url: None,
                        enable_namespaces: None,
                        network_ids: None,
                        detach_rustploy_network: None,
                    },
                )
                .await;
        }
        Ok(created)
    }

    pub async fn get_postgres_advanced_config(
        &self,
        id: i64,
    ) -> sqlx::Result<PostgresAdvancedConfigResponseDto> {
        let row = self.queries.database_get_by_id(id).await?;
        let config: serde_json::Value = row
            .engine_config
            .as_deref()
            .and_then(|value| serde_json::from_str(value).ok())
            .unwrap_or(serde_json::Value::Null);
        let settings_value = config.get("postgres_config").cloned().unwrap_or_default();
        let settings_value = match settings_value {
            serde_json::Value::String(inner) => serde_json::from_str::<serde_json::Value>(&inner)
                .map_err(|error| sqlx::Error::Protocol(error.to_string()))?,
            other => other,
        };
        Ok(PostgresAdvancedConfigResponseDto {
            settings: serde_json::from_value(settings_value)
                .map_err(|error| sqlx::Error::Protocol(error.to_string()))?,
            replication_role: PostgresReplicationRole::from_str(
                config
                    .get("replication_role")
                    .and_then(|value| value.as_str())
                    .unwrap_or("STANDALONE"),
            )
            .ok_or_else(|| sqlx::Error::Protocol("invalid replication role".into()))?,
            primary_host: config
                .get("primary_host")
                .and_then(|value| value.as_str())
                .map(str::to_string),
            primary_port: config.get("primary_port").and_then(|value| value.as_i64()),
            replication_user: config
                .get("replication_user")
                .and_then(|value| value.as_str())
                .map(str::to_string),
            replication_password_configured: config
                .get("replication_password")
                .and_then(|value| value.as_str())
                .is_some(),
            redeploy_required: true,
        })
    }

    pub async fn update_postgres_advanced_config(
        &self,
        id: i64,
        input: PostgresAdvancedConfigDto,
    ) -> sqlx::Result<PostgresAdvancedConfigResponseDto> {
        self.get_by_id(DatabaseKind::Postgres, id).await?;
        validate_postgres_settings(&input.settings)?;
        let role = input.replication_role;
        if matches!(role, PostgresReplicationRole::Replica) && (input
            .primary_host
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
            || input
                .replication_user
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty()
            || input.replication_password.as_deref().unwrap_or("").len() < 16)
        {
            return Err(sqlx::Error::Protocol(
                "replica requires primary_host, replication_user and a 16+ character replication_password".into(),
            ));
        }
        let config = serde_json::to_value(&input.settings)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let row = self.queries.database_get_by_id(id).await?;
        let mut engine: serde_json::Map<String, serde_json::Value> = row
            .engine_config
            .as_deref()
            .and_then(|value| serde_json::from_str(value).ok())
            .unwrap_or_default();
        engine.insert("postgres_config".into(), config);
        engine.insert("replication_role".into(), serde_json::json!(role.as_str()));
        if let Some(value) = &input.primary_host {
            engine.insert("primary_host".into(), serde_json::json!(value));
        }
        if let Some(value) = input.primary_port {
            engine.insert("primary_port".into(), serde_json::json!(value));
        }
        if let Some(value) = &input.replication_user {
            engine.insert("replication_user".into(), serde_json::json!(value));
        }
        if let Some(value) = &input.replication_password {
            engine.insert("replication_password".into(), serde_json::json!(value));
        }
        let serialized = serde_json::Value::Object(engine).to_string();
        self.queries
            .update_database_engine_config(&serialized, id)
            .await?;
        self.cache.invalidate(&CacheKey::Database(id)).await;
        self.get_postgres_advanced_config(id).await
    }

    pub async fn connection_details(
        &self,
        kind: DatabaseKind,
        id: i64,
    ) -> sqlx::Result<DatabaseConnectionDto> {
        let value = self.engine_credentials(kind, id).await?;
        let port = default_port(kind);
        let host = format!("{}_db", value.app_name);
        let internal_url = connection_url(
            kind,
            &host,
            port,
            value.username.as_deref(),
            &value.password,
            value.database_name.as_deref(),
        );
        let external_url = value.external_port.map(|external_port| {
            connection_url(
                kind,
                "SERVER_HOST",
                external_port,
                value.username.as_deref(),
                &value.password,
                value.database_name.as_deref(),
            )
        });
        Ok(DatabaseConnectionDto {
            kind,
            host,
            port,
            database: value.database_name,
            username: value.username,
            password: value.password,
            internal_url,
            external_url,
            server_id: value.server_id,
        })
    }

    pub async fn rotate_credentials(
        &self,
        kind: DatabaseKind,
        id: i64,
        password: Option<String>,
    ) -> sqlx::Result<DatabaseCredentialRotationDto> {
        let record = self.get_by_id(kind, id).await?;
        let password = password.unwrap_or_else(super::queries::random_secret);
        if password.len() < 16 {
            return Err(sqlx::Error::Protocol(
                "database password must contain at least 16 characters".into(),
            ));
        }
        let current = self.engine_credentials(kind, id).await?;
        let live_changed = if crate::services::compute::database::types::DatabaseRuntimeStatus::from_str(
            record.app_status.as_str(),
        )
        .unwrap_or(crate::services::compute::database::types::DatabaseRuntimeStatus::Idle)
            == crate::services::compute::database::types::DatabaseRuntimeStatus::Running
        {
            self.apply_live_password(kind, &current, &password).await?;
            true
        } else {
            false
        };
        if let Err(error) = self
            .queries
            .database_management_rotate_password(&password, id)
            .await
        {
            if live_changed {
                let mut rollback = current.clone();
                rollback.password = password.clone();
                let _ = self
                    .apply_live_password(kind, &rollback, &current.password)
                    .await;
            }
            return Err(error);
        }
        self.cache.invalidate(&CacheKey::Database(id)).await;
        Ok(DatabaseCredentialRotationDto {
            password,
            redeploy_required: !live_changed,
        })
    }

    async fn apply_live_password(
        &self,
        kind: DatabaseKind,
        current: &EngineCredentials,
        password: &str,
    ) -> sqlx::Result<()> {
        if kind == DatabaseKind::Libsql {
            return Err(sqlx::Error::Protocol(
                "stop libSQL before rotating its authentication token".into(),
            ));
        }
        let docker = crate::services::compute::compose::remote::agent_executor_for(current.server_id)
            .await
            .map(crate::utils::docker::DockerCli::from_executor)
            .map_err(sqlx::Error::Protocol)?;
        let containers = docker
            .containers()
            .ps()
            .filter(ContainerFilter::Status(ContainerStatus::Running))
            .filter(ContainerFilter::Name(format!("{}_db", current.app_name)))
            .list()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let container = containers
            .first()
            .ok_or_else(|| sqlx::Error::Protocol("running database container not found".into()))?;
        let username = current.username.as_deref().unwrap_or_default();
        let database = current.database_name.as_deref().unwrap_or("admin");
        let sql_password = password.replace('\'', "''");
        let output = match kind {
            DatabaseKind::Postgres => {
                let sql_user = username.replace('"', "\"\"");
                docker
                    .containers()
                    .exec(&container.id)
                    .env("PGPASSWORD", &current.password)
                    .run([
                        "psql",
                        "-v",
                        "ON_ERROR_STOP=1",
                        "-U",
                        username,
                        "-d",
                        database,
                        "-c",
                        &format!("ALTER USER \"{sql_user}\" WITH PASSWORD '{sql_password}'"),
                    ])
                    .await
            }
            DatabaseKind::Mysql | DatabaseKind::Mariadb => {
                docker
                    .containers()
                    .exec(&container.id)
                    .env("MYSQL_PWD", &current.password)
                    .run([
                        "mysql",
                        "-u",
                        username,
                        "-e",
                        &format!(
                            "ALTER USER '{}'@'%' IDENTIFIED BY '{sql_password}'",
                            username.replace('\'', "''")
                        ),
                    ])
                    .await
            }
            DatabaseKind::Mongo => {
                docker
                    .containers()
                    .exec(&container.id)
                    .run([
                        "mongosh",
                        "--quiet",
                        "--username",
                        username,
                        "--password",
                        &current.password,
                        "--authenticationDatabase",
                        "admin",
                        "--eval",
                        &format!(
                            "db.getSiblingDB('admin').changeUserPassword('{}', '{}')",
                            username.replace('\'', "\\'"),
                            password.replace('\'', "\\'")
                        ),
                    ])
                    .await
            }
            DatabaseKind::Redis => {
                docker
                    .containers()
                    .exec(&container.id)
                    .run([
                        "redis-cli",
                        "-a",
                        &current.password,
                        "CONFIG",
                        "SET",
                        "requirepass",
                        password,
                    ])
                    .await
            }
            DatabaseKind::Libsql => unreachable!(),
        }
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        if !output.success() {
            return Err(sqlx::Error::Protocol(format!(
                "database rejected credential rotation: {}",
                output.stderr
            )));
        }
        Ok(())
    }

    pub async fn validate_configuration(
        &self,
        kind: DatabaseKind,
        id: i64,
    ) -> sqlx::Result<DatabaseValidationDto> {
        let record = self.get_by_id(kind, id).await?;
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        validate_port(record.external_port, "external_port", &mut errors);
        validate_cpu(record.cpu_limit.as_deref(), "cpu_limit", &mut errors);
        validate_cpu(
            record.cpu_reservation.as_deref(),
            "cpu_reservation",
            &mut errors,
        );
        validate_memory(record.memory_limit.as_deref(), "memory_limit", &mut errors);
        validate_memory(
            record.memory_reservation.as_deref(),
            "memory_reservation",
            &mut errors,
        );
        if record.replicas < 1 {
            errors.push("replicas must be at least 1".into());
        }
        if record.detach_rustploy_network != 0 && record.network_ids == "[]" {
            warnings
                .push("database is detached from openoxide-network without another network".into());
        }
        if record.external_port.is_some() {
            warnings.push("database is exposed on a host port".into());
        }
        Ok(DatabaseValidationDto {
            valid: errors.is_empty(),
            errors,
            warnings,
        })
    }
}

fn validate_postgres_settings(
    settings: &std::collections::BTreeMap<String, String>,
) -> sqlx::Result<()> {
    const ALLOWED: &[&str] = &[
        "max_connections",
        "shared_buffers",
        "effective_cache_size",
        "maintenance_work_mem",
        "work_mem",
        "wal_level",
        "max_wal_senders",
        "max_replication_slots",
        "wal_keep_size",
        "log_min_duration_statement",
        "statement_timeout",
        "idle_in_transaction_session_timeout",
    ];
    for (key, value) in settings {
        if !ALLOWED.contains(&key.as_str()) {
            return Err(sqlx::Error::Protocol(format!(
                "unsupported PostgreSQL setting: {key}"
            )));
        }
        if value.trim().is_empty() || value.contains(['\n', '\r', '\0']) {
            return Err(sqlx::Error::Protocol(format!(
                "invalid PostgreSQL setting value for {key}"
            )));
        }
    }
    Ok(())
}

fn default_port(kind: DatabaseKind) -> i64 {
    match kind {
        DatabaseKind::Postgres => 5432,
        DatabaseKind::Mysql | DatabaseKind::Mariadb => 3306,
        DatabaseKind::Mongo => 27017,
        DatabaseKind::Redis => 6379,
        DatabaseKind::Libsql => 8080,
    }
}

fn connection_url(
    kind: DatabaseKind,
    host: &str,
    port: i64,
    username: Option<&str>,
    password: &str,
    database: Option<&str>,
) -> String {
    let scheme = match kind {
        DatabaseKind::Postgres => "postgresql",
        DatabaseKind::Mysql | DatabaseKind::Mariadb => "mysql",
        DatabaseKind::Mongo => "mongodb",
        DatabaseKind::Redis => "redis",
        DatabaseKind::Libsql => "http",
    };
    let credentials = match username {
        Some(username) => format!("{}:{}@", encode(username), encode(password)),
        None if kind == DatabaseKind::Redis => format!(":{}@", encode(password)),
        None => String::new(),
    };
    let path = database
        .map(|value| format!("/{}", encode(value)))
        .unwrap_or_default();
    format!("{scheme}://{credentials}{host}:{port}{path}")
}

fn encode(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                (byte as char).to_string()
            }
            _ => format!("%{byte:02X}"),
        })
        .collect()
}

fn validate_port(value: Option<i64>, field: &str, errors: &mut Vec<String>) {
    if let Some(value) = value
        && !(1..=65_535).contains(&value)
    {
        errors.push(format!("{field} must be between 1 and 65535"));
    }
}

fn validate_cpu(value: Option<&str>, field: &str, errors: &mut Vec<String>) {
    if let Some(value) = value
        && value
            .parse::<f64>()
            .ok()
            .filter(|value| *value > 0.0)
            .is_none()
    {
        errors.push(format!("{field} must be a positive CPU value"));
    }
}

fn validate_memory(value: Option<&str>, field: &str, errors: &mut Vec<String>) {
    if let Some(value) = value {
        let split = value
            .find(|ch: char| !ch.is_ascii_digit())
            .unwrap_or(value.len());
        let (amount, unit) = value.split_at(split);
        if amount
            .parse::<u64>()
            .ok()
            .filter(|value| *value > 0)
            .is_none()
            || !matches!(
                unit.to_ascii_uppercase().as_str(),
                "B" | "K" | "KB" | "M" | "MB" | "G" | "GB"
            )
        {
            errors.push(format!("{field} must use a positive Docker memory value"));
        }
    }
}
