use crate::{
    api::dto::database::{
        DatabaseConnectionDto, DatabaseCredentialRotationDto, DatabaseExportBundleDto,
        DatabaseValidationDto, ImportDatabaseDto, PostgresAdvancedConfigDto,
        PostgresAdvancedConfigResponseDto,
    },
    core::cache::CacheKey,
    services::database::{DatabaseKind, DatabaseService},
    utils::docker::query::{ContainerFilter, filter::ContainerStatus},
};

impl DatabaseService {
    pub async fn export(
        &self,
        kind: DatabaseKind,
        id: i64,
        include_secrets: bool,
    ) -> sqlx::Result<DatabaseExportBundleDto> {
        let record = self.get_by_id(kind, id).await?;
        let credentials = self.repo_management.credentials(kind, id).await?;
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
            network_ids: crate::api::dto::database::parse_json_string_vec(&record.network_ids),
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
                crate::api::dto::database::CreateDatabaseDto {
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
                    crate::api::dto::database::PatchDatabaseDto {
                        name: None,
                        description: None,
                        docker_image: None,
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
        let value = self.repo_postgres.get_details(id).await?;
        Ok(PostgresAdvancedConfigResponseDto {
            settings: serde_json::from_str(&value.postgres_config)
                .map_err(|error| sqlx::Error::Protocol(error.to_string()))?,
            replication_role: crate::api::dto::database::PostgresReplicationRole::try_from(
                value.replication_role.as_str(),
            )
            .map_err(sqlx::Error::Protocol)?,
            primary_host: value.primary_host,
            primary_port: value.primary_port,
            replication_user: value.replication_user,
            replication_password_configured: value.replication_password.is_some(),
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
        if matches!(
            role,
            crate::api::dto::database::PostgresReplicationRole::Replica
        ) && (input
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
        let config = serde_json::to_string(&input.settings)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        if !self
            .repo_postgres
            .update_advanced_config(
                id,
                &config,
                role.as_str(),
                input.primary_host.as_deref(),
                input.primary_port,
                input.replication_user.as_deref(),
                input.replication_password.as_deref(),
            )
            .await?
        {
            return Err(sqlx::Error::RowNotFound);
        }
        self.cache.invalidate(&CacheKey::Database(id)).await;
        self.get_postgres_advanced_config(id).await
    }

    pub async fn connection_details(
        &self,
        kind: DatabaseKind,
        id: i64,
    ) -> sqlx::Result<DatabaseConnectionDto> {
        let value = self.repo_management.credentials(kind, id).await?;
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
        let current = self.repo_management.credentials(kind, id).await?;
        let live_changed = if crate::services::database::types::DatabaseRuntimeStatus::try_from(
            record.app_status.as_str(),
        )
        .map_err(sqlx::Error::Protocol)?
            == crate::services::database::types::DatabaseRuntimeStatus::Running
        {
            self.apply_live_password(kind, &current, &password).await?;
            true
        } else {
            false
        };
        if !self
            .repo_management
            .rotate_password(kind, id, &password)
            .await?
        {
            if live_changed {
                let mut rollback = current.clone();
                rollback.password = password.clone();
                let _ = self
                    .apply_live_password(kind, &rollback, &current.password)
                    .await;
            }
            return Err(sqlx::Error::RowNotFound);
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
        current: &crate::repository::database_management::DatabaseCredentials,
        password: &str,
    ) -> sqlx::Result<()> {
        if kind == DatabaseKind::Libsql {
            return Err(sqlx::Error::Protocol(
                "stop libSQL before rotating its authentication token".into(),
            ));
        }
        let docker = match current.server_id {
            Some(server_id) => {
                crate::services::compose::remote::remote_executor(self.db.as_ref(), server_id)
                    .await
                    .map(crate::utils::docker::DockerCli::from_remote_executor)
                    .map_err(sqlx::Error::Protocol)?
            }
            None => crate::utils::docker::DockerCli::new_local(),
        };
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
                .push("database is detached from rustploy-network without another network".into());
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
