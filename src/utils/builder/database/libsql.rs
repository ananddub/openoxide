use crate::db::models::mounts::Mount;
use crate::repository::LibsqlRepository;
use crate::utils::builder::database::builder::{
    DeployResources, DeploySpec, Limits, RestartPolicy, StackFile, StackMount, StackService,
    UpdateConfig,
};
use crate::utils::exec::{ExecError, ExecResult};
use std::collections::BTreeMap;

pub async fn build_libsql_stack(
    db_id: i64,
    mounts: &[Mount],
) -> ExecResult<(String, String, String)> {
    let repo =
        auto_di::resolve::<LibsqlRepository>()
            .await
            .map_err(|e| ExecError::CommandFailed {
                code: None,
                stderr: format!("Failed to resolve LibsqlRepository: {}", e),
            })?;
    let db = repo
        .get_details(db_id)
        .await
        .map_err(|e| ExecError::CommandFailed {
            code: None,
            stderr: format!("Failed to fetch libsql db: {}", e),
        })?;

    // Parse command and args
    let command: Option<Vec<String>>;
    if let Some(c) = &db.command {
        let mut full = c.split_whitespace().map(String::from).collect::<Vec<_>>();
        if let Some(a_str) = &db.args {
            if let Ok(parsed_args) = serde_json::from_str::<Vec<String>>(a_str) {
                full.extend(parsed_args);
            }
        }
        command = Some(full);
    } else {
        let mut final_command = "sqld --db-path iku.db --http-listen-addr 0.0.0.0:8080 --grpc-listen-addr 0.0.0.0:5001 --admin-listen-addr 0.0.0.0:5000".to_string();
        if db.enable_namespaces != 0 {
            final_command.push_str(" --enable-namespaces");
        }
        command = Some(vec!["/bin/sh".to_string(), "-c".to_string(), final_command]);
    }

    // Parse environment variables
    let mut resolved_env = crate::utils::builder::env::generate_env_db(
        db.environment_id,
        db.env_var.as_deref().unwrap_or(""),
    )
    .await
    .unwrap_or_default();

    let sqld_node = db.sqld_node.to_ascii_lowercase();
    resolved_env.insert("SQLD_NODE".to_string(), sqld_node.clone());
    resolved_env.insert(
        "SQLD_HTTP_AUTH".to_string(),
        format!(
            "basic:{}",
            base64_encode(format!("{}:{}", db.database_user, db.database_password).as_bytes())
        ),
    );
    if sqld_node == "replica" {
        if let Some(url) = &db.sqld_primary_url {
            resolved_env.insert("SQLD_PRIMARY_URL".to_string(), url.clone());
        }
        resolved_env.insert("SQLD_AUTH_TOKEN".to_string(), db.database_password.clone());
    }
    if db.enable_namespaces != 0 {
        resolved_env.insert("SQLD_ENABLE_NAMESPACES".to_string(), "true".to_string());
    }

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
            target: "/var/lib/sqld".to_string(),
            read_only: false,
        });
    }

    let mut ports = Vec::new();
    if let Some(port) = db.external_port {
        ports.push(format!("{}:8080", port));
    }
    if let Some(port) = db.external_grpc_port {
        ports.push(format!("{}:5001", port));
    }
    if let Some(port) = db.external_admin_port {
        ports.push(format!("{}:5000", port));
    }
    let (service_networks, networks) =
        crate::utils::builder::database::builder::resolve_database_networks(
            Some(&db.network_ids),
            db.detach_rustploy_network,
        )
        .await?;

    let service = StackService {
        image: db.docker_image.clone(),
        environment: resolved_env.into_iter().collect(),
        command,
        volumes: stack_mounts.clone(),
        networks: service_networks,
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
        stderr: format!("Failed to generate libsql yaml: {}", e),
    })?;

    Ok((db.app_name, db.docker_image, yaml))
}

fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let n = match chunk.len() {
            3 => ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32),
            2 => ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8),
            1 => (chunk[0] as u32) << 16,
            _ => unreachable!(),
        };
        result.push(ALPHABET[((n >> 18) & 0x3F) as usize] as char);
        result.push(ALPHABET[((n >> 12) & 0x3F) as usize] as char);
        result.push(if chunk.len() > 1 {
            ALPHABET[((n >> 6) & 0x3F) as usize] as char
        } else {
            '='
        });
        result.push(if chunk.len() > 2 {
            ALPHABET[(n & 0x3F) as usize] as char
        } else {
            '='
        });
    }
    result
}
