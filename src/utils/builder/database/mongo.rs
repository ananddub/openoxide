use crate::db::models::mounts::Mount;
use crate::repository::MongoRepository;
use crate::utils::builder::database::builder::{
    DeployResources, DeploySpec, Limits, RestartPolicy, StackFile, StackMount, StackService,
    UpdateConfig,
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
    if replica_sets_enabled(db.replica_sets) {
        command = Some(vec![
            "/bin/bash".to_string(),
            "-c".to_string(),
            mongo_replica_startup_script(&db.app_name, &db.database_user, &db.database_password),
        ]);
    } else if let Some(c) = &db.command {
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
    if replica_sets_enabled(db.replica_sets) {
        resolved_env.insert("MONGO_INITDB_DATABASE".to_string(), "admin".to_string());
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
            target: "/data/db".to_string(),
            read_only: false,
        });
    }

    let mut ports = Vec::new();
    if let Some(port) = db.external_port {
        ports.push(format!("{}:27017", port));
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
        stderr: format!("Failed to generate mongo yaml: {}", e),
    })?;

    Ok((db.app_name, db.docker_image, yaml))
}

fn mongo_replica_startup_script(app_name: &str, user: &str, password: &str) -> String {
    format!(
        r#"mongod --port 27017 --replSet rs0 --bind_ip_all &
MONGOD_PID=$!
until mongosh --eval "db.adminCommand('ping')" >/dev/null 2>&1; do
  sleep 2
done
REPLICA_STATUS=$(mongosh --quiet --eval "rs.status().ok || 0")
if [ "$REPLICA_STATUS" != "1" ]; then
  echo "Initializing MongoDB replica set..."
  mongosh --eval {init}
else
  echo "MongoDB replica set already initialized."
fi
wait "$MONGOD_PID"
"#,
        init = shell_single_quote(&format!(
            r#"rs.initiate({{ _id: "rs0", members: [{{ _id: 0, host: "{app_name}:27017", priority: 1 }}] }});
while (!rs.isMaster().ismaster) {{ sleep(1000); }}
db.getSiblingDB("admin").createUser({{ user: "{user}", pwd: "{password}", roles: ["root"] }});"#,
            app_name = js_string_escape(app_name),
            user = js_string_escape(user),
            password = js_string_escape(password),
        )),
    )
}

fn replica_sets_enabled(value: i64) -> bool {
    value != 0
}

fn shell_single_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn js_string_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}
