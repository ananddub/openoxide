use crate::db::queries::FlashQueries;
use crate::flash_gen::models::Mounts;
use crate::utils::builder::database::builder::{
    DeployResources, DeploySpec, EngineConfig, Limits, RestartPolicy, StackFile, StackMount,
    StackService, UpdateConfig,
};
use crate::utils::exec::{ExecError, ExecResult};
use std::collections::BTreeMap;

pub async fn build_postgres_stack(
    db_id: i64,
    mounts: &[Mounts],
) -> ExecResult<(String, String, String)> {
    let q = auto_di::resolve::<FlashQueries>()
        .await
        .map_err(|e| ExecError::CommandFailed {
            code: None,
            stderr: format!("Failed to resolve FlashQueries: {}", e),
        })?;
    let db = q
        .database_get_by_id(db_id)
        .await
        .map_err(|e| ExecError::CommandFailed {
            code: None,
            stderr: format!("Failed to fetch postgres db: {}", e),
        })?;
    let engine = EngineConfig::parse(db.engine_config.as_deref().unwrap_or(""));
    let command_row = (!db.command.trim().is_empty()).then(|| db.command.clone());
    let database_name = db.db_name.clone().unwrap_or_default();
    let database_user = db.db_user.clone().unwrap_or_default();
    let database_password = db.db_password.clone().unwrap_or_default();
    let docker_image = db.docker_image.clone().unwrap_or_default();
    let postgres_config = engine.string("postgres_config");
    let replication_role = {
        let role = engine.string("replication_role");
        if role.is_empty() {
            "STANDALONE".to_string()
        } else {
            role
        }
    };

    // Parse command and args
    let mut command: Option<Vec<String>> = None;
    if let Some(c) = &command_row {
        let mut full = c.split_whitespace().map(String::from).collect::<Vec<_>>();
        if let Some(a_str) = &db.args {
            if let Ok(parsed_args) = serde_json::from_str::<Vec<String>>(a_str) {
                full.extend(parsed_args);
            }
        }
        command = Some(full);
    } else if let Some(a_str) = &db.args {
        if let Ok(parsed_args) = serde_json::from_str::<Vec<String>>(a_str) {
            if !parsed_args.is_empty() {
                let mut full = vec!["postgres".to_string()];
                full.extend(parsed_args);
                command = Some(full);
            }
        }
    }
    let advanced: BTreeMap<String, String> =
        serde_json::from_str(&postgres_config).map_err(|error| ExecError::CommandFailed {
            code: None,
            stderr: format!("Invalid PostgreSQL advanced config: {error}"),
        })?;
    if !advanced.is_empty() || replication_role != "STANDALONE" {
        let full = command.get_or_insert_with(|| vec!["postgres".into()]);
        for (key, value) in advanced {
            full.extend(["-c".into(), format!("{key}={value}")]);
        }
        if replication_role == "PRIMARY" {
            full.extend([
                "-c".into(),
                "wal_level=replica".into(),
                "-c".into(),
                "max_wal_senders=10".into(),
                "-c".into(),
                "max_replication_slots=10".into(),
            ]);
        }
    }

    // Parse environment variables
    let mut resolved_env = crate::utils::builder::env::generate_env_db(
        db.environment_id,
        db.env_var.as_deref().unwrap_or(""),
    )
    .await
    .unwrap_or_default();

    resolved_env.insert("POSTGRES_DB".to_string(), database_name.clone());
    resolved_env.insert("POSTGRES_USER".to_string(), database_user.clone());
    resolved_env.insert("POSTGRES_PASSWORD".to_string(), database_password.clone());

    // Generate stack mounts
    let mut stack_mounts = Vec::new();
    for m in mounts {
        stack_mounts.push(StackMount {
            kind: match m.mount_type.as_str() {
                "VOLUME" => "volume",
                _ => "bind",
            },
            source: match m.mount_type.as_str() {
                "VOLUME" => m
                    .volume_name
                    .clone()
                    .unwrap_or_else(|| format!("{}-data", db.app_name)),
                _ => m.host_path.clone().unwrap_or_default(),
            },
            target: m.mount_path.clone(),
            read_only: false,
        });
    }
    if stack_mounts.is_empty() {
        stack_mounts.push(StackMount {
            kind: "volume",
            source: format!("{}-data", db.app_name),
            target: get_postgres_mount_path(&docker_image),
            read_only: false,
        });
    }

    let mut ports = Vec::new();
    if let Some(port) = db.external_port {
        ports.push(format!("{}:5432", port));
    }
    let (service_networks, mut networks) =
        crate::utils::builder::database::builder::resolve_database_networks(
            Some(&db.network_ids),
            db.detach_rustploy_network,
        )
        .await?;

    let mut service = StackService {
        image: docker_image.clone(),
        environment: resolved_env.into_iter().collect(),
        command,
        volumes: stack_mounts.clone(),
        networks: service_networks
            .into_iter()
            .map(|name| (name, Default::default()))
            .collect(),
        deploy: DeploySpec {
            replicas: Some(db.replicas as u32),
            mode: None,
            resources: DeployResources {
                limits: Limits {
                    cpus: db.cpu_limit.clone(),
                    memory: db.memory_limit.clone(),
                },
                reservations: Limits {
                    cpus: db.cpu_reservation.clone(),
                    memory: db.memory_reservation.clone(),
                },
            },
            restart_policy: RestartPolicy {
                condition: "on-failure".into(),
                delay: "5s".into(),
                max_attempts: 3,
                window: "120s".into(),
            },
            update_config: UpdateConfig {
                parallelism: 1,
                delay: "5s".into(),
                order: "stop-first".into(),
                failure_action: "rollback".into(),
                monitor: None,
                max_failure_ratio: None,
            },
            rollback_config: UpdateConfig {
                parallelism: 1,
                delay: "5s".into(),
                order: "stop-first".into(),
                failure_action: "pause".into(),
                monitor: None,
                max_failure_ratio: None,
            },
            endpoint_mode: None,
            placement: Default::default(),
            labels: Vec::new(),
        },
        healthcheck: None,
        stop_grace_period: None,
        ulimits: None,
        ports,
    };
    crate::utils::builder::database::builder::apply_database_swarm_settings(
        crate::services::database::DatabaseKind::Postgres,
        db_id,
        &mut service,
        &mut networks,
    )
    .await?;

    let mut services = BTreeMap::new();
    services.insert("db".to_string(), service);

    let mut top_level_volumes = BTreeMap::new();
    for m in &stack_mounts {
        if m.kind == "volume" {
            top_level_volumes.insert(
                m.source.clone(),
                crate::utils::builder::database::builder::TopLevelVolume {
                    name: Some(m.source.clone()),
                    external: None,
                },
            );
        }
    }

    let file = StackFile {
        version: "3.8",
        services,
        networks,
        volumes: top_level_volumes,
    };

    let yaml = serde_yaml::to_string(&file).map_err(|e| ExecError::CommandFailed {
        code: None,
        stderr: format!("Failed to generate postgres yaml: {}", e),
    })?;

    Ok((db.app_name, docker_image, yaml))
}

fn get_postgres_mount_path(image: &str) -> String {
    if let Some(pos) = image.find("postgres:") {
        let version_part = &image[pos + 9..];
        let end = version_part
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(version_part.len());
        if let Ok(version) = version_part[..end].parse::<i32>() {
            if version >= 18 {
                return format!("/var/lib/postgresql/{}/docker", version);
            }
        }
    }
    "/var/lib/postgresql/data".to_string()
}
