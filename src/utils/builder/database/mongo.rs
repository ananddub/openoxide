use crate::db::models::mounts::Mount;
use crate::repository::MongoRepository;
use crate::utils::builder::database::builder::{
    DeployResources, DeploySpec, ExternalNetwork, Limits, RestartPolicy, StackFile, StackMount,
    StackService, UpdateConfig,
};
use crate::utils::exec::{ExecError, ExecResult};
use std::collections::BTreeMap;

pub async fn build_mongo_stack(
    db_id: i64,
    mounts: &[Mount],
) -> ExecResult<(String, String, String)> {
    let repo =
        auto_di::resolve::<MongoRepository>()
            .await
            .map_err(|e| ExecError::CommandFailed {
                code: None,
                stderr: format!("Failed to resolve MongoRepository: {}", e),
            })?;
    let db = repo
        .get_details(db_id)
        .await
        .map_err(|e| ExecError::CommandFailed {
            code: None,
            stderr: format!("Failed to fetch mongo db: {}", e),
        })?;

    // Parse command and args
    let mut command: Option<Vec<String>> = None;
    if let Some(c) = &db.command {
        let mut full = c.split_whitespace().map(String::from).collect::<Vec<_>>();
        if let Some(a_str) = &db.args {
            if let Ok(parsed_args) = serde_json::from_str::<Vec<String>>(a_str) {
                full.extend(parsed_args);
            }
        }
        command = Some(full);
    }

    // Parse environment variables
    let mut resolved_env = crate::utils::builder::env::generate_env_db(
        db.environment_id,
        db.env_var.as_deref().unwrap_or(""),
    )
    .await
    .unwrap_or_default();

    resolved_env.insert(
        "MONGO_INITDB_ROOT_USERNAME".to_string(),
        db.database_user.clone(),
    );
    resolved_env.insert(
        "MONGO_INITDB_ROOT_PASSWORD".to_string(),
        db.database_password.clone(),
    );

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
            target: "/data/db".to_string(),
            read_only: false,
        });
    }

    let mut ports = Vec::new();
    if let Some(port) = db.external_port {
        ports.push(format!("{}:27017", port));
    }

    let service = StackService {
        image: db.docker_image.clone(),
        environment: resolved_env.into_iter().collect(),
        command,
        volumes: stack_mounts.clone(),
        networks: vec![crate::utils::builder::swarm::RUSTPLOY_NETWORK.to_string()],
        deploy: DeploySpec {
            replicas: db.replicas as u32,
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
                condition: "on-failure",
                delay: "5s",
                max_attempts: 3,
                window: "120s",
            },
            update_config: UpdateConfig {
                parallelism: 1,
                delay: "5s",
                order: "stop-first",
                failure_action: "rollback",
            },
            rollback_config: UpdateConfig {
                parallelism: 1,
                delay: "5s",
                order: "stop-first",
                failure_action: "pause",
            },
            placement: Default::default(),
            labels: Vec::new(),
        },
        healthcheck: None,
        stop_grace_period: None,
        ports,
    };

    let mut services = BTreeMap::new();
    services.insert("db".to_string(), service);

    let mut networks = BTreeMap::new();
    networks.insert(
        crate::utils::builder::swarm::RUSTPLOY_NETWORK.to_string(),
        ExternalNetwork {
            external: true,
            name: crate::utils::builder::swarm::RUSTPLOY_NETWORK.to_string(),
        },
    );

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
        stderr: format!("Failed to generate mongo yaml: {}", e),
    })?;

    Ok((db.app_name, db.docker_image, yaml))
}
