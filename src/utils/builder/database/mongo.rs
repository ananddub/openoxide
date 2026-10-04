use crate::db::queries::FlashQueries;
use crate::flash_gen::models::Mounts;
use crate::utils::builder::database::builder::{
    DeployResources, DeploySpec, EngineConfig, Limits, RestartPolicy, StackFile, StackMount,
    StackService, UpdateConfig,
};
use crate::utils::exec::{ExecError, ExecResult};
use std::collections::BTreeMap;

pub async fn build_mongo_stack(
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
            stderr: format!("Failed to fetch mongo db: {}", e),
        })?;
    let engine = EngineConfig::parse(db.engine_config.as_deref().unwrap_or(""));
    let database_user = db.db_user.clone().unwrap_or_default();
    let database_password = db.db_password.clone().unwrap_or_default();
    let docker_image = db.docker_image.clone().unwrap_or_default();
    let command_row = (!db.command.trim().is_empty()).then(|| db.command.clone());

    // Parse command and args
    let mut command: Option<Vec<String>> = None;
    if replica_sets_enabled(engine.flag("replica_sets")) {
        command = Some(vec![
            "/bin/bash".to_string(),
            "-c".to_string(),
            mongo_replica_startup_script(&db.app_name, &database_user, &database_password),
        ]);
    } else if let Some(c) = &command_row {
        let mut full = c.split_whitespace().map(String::from).collect::<Vec<_>>();
        if let Some(a_str) = &db.args
            && let Ok(parsed_args) = serde_json::from_str::<Vec<String>>(a_str)
        {
            full.extend(parsed_args);
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
        database_user.clone(),
    );
    resolved_env.insert(
        "MONGO_INITDB_ROOT_PASSWORD".to_string(),
        database_password.clone(),
    );
    if replica_sets_enabled(engine.flag("replica_sets")) {
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
        ports.push(serde_json::Value::String(format!("{}:27017", port)));
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
        crate::services::compute::database::DatabaseKind::Mongo,
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
        stderr: format!("Failed to generate mongo yaml: {}", e),
    })?;

    Ok((db.app_name, docker_image, yaml))
}

fn mongo_replica_startup_script(app_name: &str, user: &str, password: &str) -> String {
    use crate::utils::os::database::MongoshBuilder;
    use crate::utils::os::exec::script::dsl::{
        CaptureSource, IntoShellIRGeneric, ShellIR, ShellIRWrapper, Statement,
    };

    let init_js = format!(
        r#"rs.initiate({{ _id: "rs0", members: [{{ _id: 0, host: "{app_name}:27017", priority: 1 }}] }});
while (!rs.isMaster().ismaster) {{ sleep(1000); }}
db.getSiblingDB("admin").createUser({{ user: "{user}", pwd: "{password}", roles: ["root"] }});"#,
        app_name = js_string_escape(app_name),
        user = js_string_escape(user),
        password = js_string_escape(password),
    );

    // `until mongosh ping; do sleep 2; done` as a while loop over the negated
    // (`failure`) typed mongosh command.
    let ping = ShellIRWrapper(MongoshBuilder::new().eval_script("db.adminCommand('ping')"))
        .into_shell_ir()
        .stdout_null()
        .stderr_null()
        .failure();

    let status = ShellIRWrapper(
        MongoshBuilder::new()
            .quiet()
            .eval_script("rs.status().ok || 0"),
    )
    .into_shell_ir();

    ShellIR::Sequence(vec![
        ShellIR::Raw("mongod --port 27017 --replSet rs0 --bind_ip_all &".into()),
        ShellIR::Raw("MONGOD_PID=$!".into()),
        ShellIR::While {
            cond: Box::new(ping),
            body: vec![ShellIR::Raw("sleep 2".into())],
        },
        ShellIR::Statement(Statement::VarAssign {
            name: "REPLICA_STATUS".into(),
            val: Box::new(ShellIR::Capture {
                cmd: Box::new(status),
                source: CaptureSource::Stdout,
            }),
            default: None,
        }),
        ShellIR::If {
            cond: Box::new(ShellIR::Raw("[ \"$REPLICA_STATUS\" != \"1\" ]".into())),
            then_branch: vec![
                ShellIR::Raw("echo 'Initializing MongoDB replica set...'".into()),
                ShellIRWrapper(MongoshBuilder::new().eval_script(init_js)).into_shell_ir(),
            ],
            else_branch: Some(vec![ShellIR::Raw(
                "echo 'MongoDB replica set already initialized.'".into(),
            )]),
        },
        ShellIR::Raw("wait \"$MONGOD_PID\"".into()),
    ])
    .to_bash()
}

fn replica_sets_enabled(value: i64) -> bool {
    value != 0
}

fn js_string_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}
