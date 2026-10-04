use crate::db::queries::FlashQueries;
use crate::services::compute::database::DatabaseKind;
use crate::utils::builder::shared::BuilderContext;
use crate::utils::builder::spec::BuilderEvent;
use crate::utils::docker::query::filter::{TaskDesiredState, TaskFilter};
use crate::utils::exec::{CommandExecutor, ExecError, ExecResult};
use os::pipeline;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use tokio::time::{Duration, Instant};
use tokio_util::sync::CancellationToken;

#[derive(Serialize, Clone, Default)]
pub struct TopLevelVolume {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
}

#[derive(Serialize)]
pub struct StackFile {
    pub version: &'static str,
    pub services: BTreeMap<String, StackService>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub networks: BTreeMap<String, ExternalNetwork>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub volumes: BTreeMap<String, TopLevelVolume>,
}

#[derive(Serialize)]
pub struct StackService {
    pub image: String,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub environment: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<StackMount>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub networks: BTreeMap<String, ServiceNetworkAttachment>,
    pub deploy: DeploySpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<HealthSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_grace_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HealthSpec {
    #[serde(alias = "Test")]
    pub test: Vec<String>,
    #[serde(alias = "Interval")]
    #[serde(deserialize_with = "deserialize_duration")]
    pub interval: String,
    #[serde(alias = "Timeout")]
    #[serde(deserialize_with = "deserialize_duration")]
    pub timeout: String,
    #[serde(alias = "Retries")]
    pub retries: u32,
    #[serde(alias = "StartPeriod")]
    #[serde(deserialize_with = "deserialize_duration")]
    pub start_period: String,
}

#[derive(Serialize)]
pub struct DeploySpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    pub resources: DeployResources,
    pub restart_policy: RestartPolicy,
    pub update_config: UpdateConfig,
    pub rollback_config: UpdateConfig,
    #[serde(skip_serializing_if = "Placement::is_empty")]
    pub placement: Placement,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_mode: Option<String>,
}

#[derive(Serialize)]
pub struct DeployResources {
    #[serde(skip_serializing_if = "Limits::is_empty")]
    pub limits: Limits,
    #[serde(skip_serializing_if = "Limits::is_empty")]
    pub reservations: Limits,
}

#[derive(Serialize, Default)]
pub struct Limits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}

impl Limits {
    pub fn is_empty(&self) -> bool {
        self.cpus.is_none() && self.memory.is_none()
    }
}

#[derive(Serialize)]
pub struct RestartPolicy {
    pub condition: String,
    pub delay: String,
    pub max_attempts: u64,
    pub window: String,
}

#[derive(Serialize)]
pub struct UpdateConfig {
    pub parallelism: u64,
    pub delay: String,
    pub order: String,
    pub failure_action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_failure_ratio: Option<f64>,
}

#[derive(Serialize, Default)]
pub struct Placement {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub preferences: Vec<PlacementPreference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_replicas_per_node: Option<u32>,
}

#[derive(Serialize, Default)]
pub struct PlacementPreference {
    pub spread: String,
}

impl Placement {
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
            && self.preferences.is_empty()
            && self.max_replicas_per_node.is_none()
    }
}

pub async fn apply_database_swarm_settings(
    kind: DatabaseKind,
    id: i64,
    service: &mut StackService,
    networks: &mut BTreeMap<String, ExternalNetwork>,
) -> ExecResult<()> {
    let settings = auto_di::resolve::<
        crate::services::compute::database::swarm_settings::DatabaseSwarmSettingsService,
    >()
    .await
    .map_err(|error| ExecError::CommandFailed {
        code: None,
        stderr: error.to_string(),
    })?
    .get(kind, id)
    .await
    .map_err(|error| ExecError::CommandFailed {
        code: None,
        stderr: error.to_string(),
    })?;

    if let Some(raw) = settings.health_check_swarm.filter(|v| !v.trim().is_empty()) {
        service.healthcheck = Some(parse_setting(&raw, "health check")?);
    }
    if let Some(raw) = settings
        .restart_policy_swarm
        .filter(|v| !v.trim().is_empty())
    {
        let value: serde_json::Value = parse_setting(&raw, "restart policy")?;
        service.deploy.restart_policy = RestartPolicy {
            condition: string_field(&value, "Condition", "condition", "on-failure"),
            delay: duration_field(&value, "Delay", "delay", "5s"),
            max_attempts: number_field(&value, "MaxAttempts", "max_attempts", 3),
            window: duration_field(&value, "Window", "window", "120s"),
        };
    }
    if let Some(raw) = settings.placement_swarm.filter(|v| !v.trim().is_empty()) {
        let value: serde_json::Value = parse_setting(&raw, "placement")?;
        service.deploy.placement.constraints = array_field(&value, "Constraints", "constraints");
        service.deploy.placement.preferences = field(&value, "Preferences", "preferences")
            .and_then(|v| v.as_array())
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| {
                        string_field_opt(item, "SpreadDescriptor", "spread")
                            .map(|spread| PlacementPreference { spread })
                    })
                    .collect()
            })
            .unwrap_or_default();
        service.deploy.placement.max_replicas_per_node =
            field(&value, "MaxReplicas", "max_replicas_per_node")
                .and_then(|v| v.as_u64())
                .map(|v| v as u32);
        apply_platform_constraints(&value, &mut service.deploy.placement.constraints)?;
    }
    if let Some(raw) = settings.mode_swarm.filter(|v| !v.trim().is_empty()) {
        let value: serde_json::Value = parse_setting(&raw, "mode")?;
        if let Some(replicated) = value.get("Replicated").or_else(|| value.get("replicated")) {
            service.deploy.mode = None;
            service.deploy.replicas = replicated
                .get("Replicas")
                .or_else(|| replicated.get("replicas"))
                .and_then(|v| v.as_u64())
                .map(|v| v as u32);
        } else if value
            .get("Global")
            .or_else(|| value.get("global"))
            .is_some()
        {
            service.deploy.mode = Some("global".to_string());
            service.deploy.replicas = None;
        } else if let Some(mode) = value.as_str() {
            service.deploy.mode = Some(mode.to_string());
        }
    }
    apply_update_setting(
        settings.update_config_swarm,
        &mut service.deploy.update_config,
        "update config",
    )?;
    apply_update_setting(
        settings.rollback_config_swarm,
        &mut service.deploy.rollback_config,
        "rollback config",
    )?;
    if let Some(raw) = settings.labels_swarm.filter(|v| !v.trim().is_empty()) {
        let value: BTreeMap<String, String> = parse_setting(&raw, "labels")?;
        service.deploy.labels = value
            .into_iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect();
    }
    if let Some(raw) = settings
        .endpoint_spec_swarm
        .filter(|v| !v.trim().is_empty())
    {
        let value: serde_json::Value = parse_setting(&raw, "endpoint spec")?;
        service.deploy.endpoint_mode = Some(string_field(&value, "Mode", "mode", "vip"));
        if let Some(ports) = field(&value, "Ports", "ports").and_then(|v| v.as_array()) {
            for port in ports {
                let target = field(port, "TargetPort", "target_port").and_then(|v| v.as_u64());
                let published =
                    field(port, "PublishedPort", "published_port").and_then(|v| v.as_u64());
                if let (Some(target), Some(published)) = (target, published) {
                    let protocol = string_field(port, "Protocol", "protocol", "tcp");
                    let mode = string_field(port, "PublishMode", "publish_mode", "ingress");
                    if mode == "ingress" {
                        service.ports.push(serde_json::Value::String(format!(
                            "{published}:{target}/{protocol}"
                        )));
                    } else {
                        // Long syntax for host (or other) publish modes.
                        service.ports.push(serde_json::json!({
                            "target": target,
                            "published": published,
                            "protocol": protocol,
                            "mode": mode,
                        }));
                    }
                }
            }
        }
    }
    if let Some(raw) = settings.network_swarm.filter(|v| !v.trim().is_empty()) {
        apply_network_setting(&raw, service, networks)?;
    }
    service.ulimits = settings
        .ulimits_swarm
        .filter(|v| !v.trim().is_empty())
        .map(|raw| parse_setting(&raw, "ulimits").map(|value| normalize_ulimits(&value)))
        .transpose()?;
    service.stop_grace_period = settings
        .stop_grace_period_swarm
        .map(|value| format!("{value}ns"));
    Ok(())
}

fn parse_setting<T: serde::de::DeserializeOwned>(raw: &str, name: &str) -> ExecResult<T> {
    serde_json::from_str(raw).map_err(|error| ExecError::CommandFailed {
        code: None,
        stderr: format!("Invalid database {name} JSON: {error}"),
    })
}

fn field<'a>(
    value: &'a serde_json::Value,
    first: &str,
    second: &str,
) -> Option<&'a serde_json::Value> {
    value.get(first).or_else(|| value.get(second))
}
fn string_field(value: &serde_json::Value, first: &str, second: &str, default: &str) -> String {
    field(value, first, second)
        .and_then(|v| v.as_str())
        .unwrap_or(default)
        .to_string()
}
fn string_field_opt(value: &serde_json::Value, first: &str, second: &str) -> Option<String> {
    field(value, first, second)
        .and_then(|v| v.as_str())
        .map(str::to_owned)
}
fn number_field(value: &serde_json::Value, first: &str, second: &str, default: u64) -> u64 {
    field(value, first, second)
        .and_then(|v| v.as_u64())
        .unwrap_or(default)
}
fn duration_field(value: &serde_json::Value, first: &str, second: &str, default: &str) -> String {
    field(value, first, second)
        .map(|v| {
            v.as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| format!("{}ns", v.as_u64().unwrap_or_default()))
        })
        .unwrap_or_else(|| default.to_string())
}
fn array_field(value: &serde_json::Value, first: &str, second: &str) -> Vec<String> {
    field(value, first, second)
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default()
}

fn deserialize_duration<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    Ok(match value {
        serde_json::Value::String(value) => value,
        serde_json::Value::Number(value) => format!("{}ns", value),
        _ => "0s".to_string(),
    })
}

fn normalize_ulimits(value: &serde_json::Value) -> serde_json::Value {
    if let Some(items) = value.as_array() {
        let mut result = serde_json::Map::new();
        for item in items {
            let Some(name) = string_field_opt(item, "Name", "name") else {
                continue;
            };
            let mut limits = serde_json::Map::new();
            if let Some(soft) = field(item, "Soft", "soft") {
                limits.insert("soft".to_string(), soft.clone());
            }
            if let Some(hard) = field(item, "Hard", "hard") {
                limits.insert("hard".to_string(), hard.clone());
            }
            result.insert(name, serde_json::Value::Object(limits));
        }
        return serde_json::Value::Object(result);
    }
    let Some(object) = value.as_object() else {
        return value.clone();
    };
    if object.contains_key("nofile_soft") || object.contains_key("nofile_hard") {
        let mut nofile = serde_json::Map::new();
        if let Some(value) = object.get("nofile_soft") {
            nofile.insert("soft".to_string(), value.clone());
        }
        if let Some(value) = object.get("nofile_hard") {
            nofile.insert("hard".to_string(), value.clone());
        }
        let mut result = serde_json::Map::new();
        result.insert("nofile".to_string(), serde_json::Value::Object(nofile));
        result.into()
    } else {
        value.clone()
    }
}

fn apply_platform_constraints(
    value: &serde_json::Value,
    constraints: &mut Vec<String>,
) -> ExecResult<()> {
    let platforms = field(value, "Platforms", "platforms")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    if platforms.len() > 1 {
        return Err(ExecError::CommandFailed {
            code: None,
            stderr: "Database placement supports one platform per Swarm service".to_string(),
        });
    }
    if let Some(platform) = platforms.first() {
        if let Some(architecture) = string_field_opt(platform, "Architecture", "architecture") {
            constraints.push(format!("node.platform.arch == {architecture}"));
        }
        if let Some(os) = string_field_opt(platform, "OS", "os") {
            constraints.push(format!("node.platform.os == {os}"));
        }
    }
    Ok(())
}

fn apply_network_setting(
    raw: &str,
    service: &mut StackService,
    networks: &mut BTreeMap<String, ExternalNetwork>,
) -> ExecResult<()> {
    let value: serde_json::Value = parse_setting(raw, "network")?;
    let attachments = value.as_array().cloned().unwrap_or_else(|| vec![value]);
    for attachment in attachments {
        let Some(target) = string_field_opt(&attachment, "Target", "target") else {
            return Err(ExecError::CommandFailed {
                code: None,
                stderr: "Database Swarm network target is required".to_string(),
            });
        };
        let aliases = array_field(&attachment, "Aliases", "aliases");
        if field(&attachment, "DriverOpts", "driver_opts")
            .and_then(|value| value.as_object())
            .is_some_and(|value| !value.is_empty())
        {
            return Err(ExecError::CommandFailed {
                code: None,
                stderr: "Driver options belong to top-level network definitions and cannot be applied to an external Swarm attachment".to_string(),
            });
        }
        service.networks.insert(
            target.clone(),
            ServiceNetworkAttachment {
                aliases,
                driver_opts: BTreeMap::new(),
            },
        );
        networks.entry(target.clone()).or_insert(ExternalNetwork {
            external: true,
            name: target,
        });
    }
    Ok(())
}
fn apply_update_setting(
    raw: Option<String>,
    target: &mut UpdateConfig,
    name: &str,
) -> ExecResult<()> {
    let Some(raw) = raw.filter(|v| !v.trim().is_empty()) else {
        return Ok(());
    };
    let value: serde_json::Value = parse_setting(&raw, name)?;
    target.parallelism = number_field(&value, "Parallelism", "parallelism", target.parallelism);
    target.delay = duration_field(&value, "Delay", "delay", &target.delay);
    target.order = string_field(&value, "Order", "order", &target.order);
    target.failure_action = string_field(
        &value,
        "FailureAction",
        "failure_action",
        &target.failure_action,
    );
    target.monitor = field(&value, "Monitor", "monitor").map(|v| {
        v.as_str()
            .map(str::to_owned)
            .unwrap_or_else(|| format!("{}ns", v.as_u64().unwrap_or_default()))
    });
    target.max_failure_ratio =
        field(&value, "MaxFailureRatio", "max_failure_ratio").and_then(|v| v.as_f64());
    Ok(())
}

#[derive(Serialize)]
pub struct ExternalNetwork {
    pub external: bool,
    pub name: String,
}

#[derive(Serialize, Default)]
pub struct ServiceNetworkAttachment {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub driver_opts: BTreeMap<String, String>,
}

/// Parsed view over `service_database.engine_config` — engine-specific fields
/// that used to live in per-engine tables.
#[derive(Debug, Default)]
pub struct EngineConfig(serde_json::Value);

impl EngineConfig {
    pub fn parse(raw: &str) -> Self {
        Self(serde_json::from_str(raw).unwrap_or_default())
    }

    pub fn string(&self, key: &str) -> String {
        self.0
            .get(key)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string()
    }

    pub fn opt_string(&self, key: &str) -> Option<String> {
        let value = self.string(key);
        (!value.is_empty()).then_some(value)
    }

    pub fn flag(&self, key: &str) -> i64 {
        self.0
            .get(key)
            .and_then(|v| v.as_bool().map(|b| b as i64).or_else(|| v.as_i64()))
            .unwrap_or(0)
    }

    pub fn opt_int(&self, key: &str) -> Option<i64> {
        self.0.get(key).and_then(|v| v.as_i64())
    }
}

pub async fn resolve_database_networks(
    network_ids: Option<&str>,
    detach_rustploy_network: i64,
) -> ExecResult<(Vec<String>, BTreeMap<String, ExternalNetwork>)> {
    let mut names = BTreeSet::new();
    if detach_rustploy_network == 0 {
        names.insert(super::super::swarm::OPENOXIDE_NETWORK.to_string());
    }

    if let Some(raw) = network_ids
        && let Ok(parsed) = serde_json::from_str::<Vec<String>>(raw)
    {
        let q = auto_di::resolve::<FlashQueries>()
            .await
            .map_err(|e| ExecError::CommandFailed {
                code: None,
                stderr: format!("Failed to resolve FlashQueries: {}", e),
            })?;
        for value in parsed {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                continue;
            }
            let resolved = match trimmed.parse::<i64>() {
                Ok(id) => q
                    .database_networks_get_by_id(id)
                    .await
                    .map(|network| network.docker_network_name)
                    .unwrap_or_else(|_| trimmed.to_string()),
                Err(_) => trimmed.to_string(),
            };
            let resolved = resolved.trim();
            if !resolved.is_empty() {
                names.insert(resolved.to_string());
            }
        }
    }

    let service_networks = names.iter().cloned().collect();
    let networks = names
        .into_iter()
        .map(|name| {
            (
                name.clone(),
                ExternalNetwork {
                    external: true,
                    name,
                },
            )
        })
        .collect();

    Ok((service_networks, networks))
}

#[derive(Serialize, Clone)]
pub struct StackMount {
    #[serde(rename = "type")]
    pub kind: &'static str,
    pub source: String,
    pub target: String,
    #[serde(skip_serializing_if = "is_false")]
    pub read_only: bool,
}

fn is_false(value: &bool) -> bool {
    !*value
}

#[derive(Clone, Debug)]
pub struct DatabaseBuilder {
    pub(super) ctx: BuilderContext,
}

impl DatabaseBuilder {
    pub fn new(executor: CommandExecutor) -> Self {
        Self {
            ctx: BuilderContext::new(executor),
        }
    }

    pub fn with_events(mut self, events: tokio::sync::mpsc::Sender<BuilderEvent>) -> Self {
        self.ctx = self.ctx.with_events(events);
        self
    }

    pub fn with_state(
        mut self,
        state: std::sync::Arc<crate::utils::builder::hash_state::ApplicationState>,
        id: crate::utils::builder::custom_type::IdType,
    ) -> Self {
        self.ctx = self.ctx.with_state(state, id);
        self
    }

    pub async fn deploy(
        &self,
        db_kind: DatabaseKind,
        db_id: i64,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        self.ctx.emit(BuilderEvent::Preparing).await;
        self.ctx.cancelled(cancel)?;

        // Fetch mounts for this database
        let q = auto_di::resolve::<FlashQueries>()
            .await
            .map_err(|e| ExecError::CommandFailed {
                code: None,
                stderr: format!("Failed to resolve FlashQueries: {}", e),
            })?;
        let mounts =
            q.mount_fetch_for_database(db_id)
                .await
                .map_err(|e| ExecError::CommandFailed {
                    code: None,
                    stderr: format!("Failed to fetch mounts: {}", e),
                })?;

        // 1. Build stack file based on database kind
        let (app_name, docker_image, stack_file_content) = match db_kind {
            DatabaseKind::Postgres => super::postgres::build_postgres_stack(db_id, &mounts).await?,
            DatabaseKind::Mysql => super::mysql::build_mysql_stack(db_id, &mounts).await?,
            DatabaseKind::Mariadb => super::mariadb::build_mariadb_stack(db_id, &mounts).await?,
            DatabaseKind::Mongo => super::mongo::build_mongo_stack(db_id, &mounts).await?,
            DatabaseKind::Redis => super::redis::build_redis_stack(db_id, &mounts).await?,
            DatabaseKind::Libsql => super::libsql::build_libsql_stack(db_id, &mounts).await?,
        };

        self.ctx
            .emit(BuilderEvent::Message(format!(
                "Stack configuration generated for {app_name} (image: {docker_image})"
            )))
            .await;

        self.ctx.emit(BuilderEvent::Building).await;
        self.ctx.cancelled(cancel)?;

        // Pull database docker image on host
        self.ctx
            .emit(BuilderEvent::Message(format!(
                "Pulling image {docker_image}..."
            )))
            .await;
        let pull_output = self
            .ctx
            .docker
            .images()
            .pull(&docker_image)
            .cancel_with(cancel.clone())
            .pull()
            .await?;
        if !pull_output.stdout.trim().is_empty() {
            self.ctx
                .emit(BuilderEvent::Message(pull_output.stdout.clone()))
                .await;
        }
        if !pull_output.stderr.trim().is_empty() {
            self.ctx
                .emit(BuilderEvent::Message(pull_output.stderr.clone()))
                .await;
        }

        self.ctx.emit(BuilderEvent::ImageReady).await;
        self.ctx.cancelled(cancel)?;

        // 2. Prepare workspace & write stack compose file
        let db_dir = format!("/tmp/openoxide-db-{}", app_name);
        let stack_file_path = format!("{}/stack.yml", db_dir);

        // 3. Ensure Swarm and Network
        self.ctx
            .emit(BuilderEvent::Message(
                "Ensuring swarm manager and overlay network...".to_string(),
            ))
            .await;
        super::super::swarm::ensure_swarm_manager(&self.ctx.executor, &self.ctx.docker, cancel)
            .await?;
        super::super::swarm::ensure_overlay_network(
            &self.ctx.docker,
            super::super::swarm::OPENOXIDE_NETWORK,
            cancel,
        )
        .await?;

        self.ctx.emit(BuilderEvent::Deploying).await;
        self.ctx.cancelled(cancel)?;
        self.ctx
            .emit(BuilderEvent::Message(format!(
                "> docker stack deploy -c {stack_file_path} {app_name}"
            )))
            .await;

        self.ctx
            .executor
            .run_cancelled("mkdir", ["-p", db_dir.as_str()], cancel)
            .await?;
        self.ctx
            .write_file_cancelled(&stack_file_path, stack_file_content.as_bytes(), cancel)
            .await?;

        let stacks = self.ctx.docker.stacks();
        let deploy_cmd = stacks
            .deploy(app_name.clone())
            .compose_file(&stack_file_path)
            .cancel_with(cancel.clone());

        let pipeline = pipeline! {
            deploy_cmd;
        };

        let result = pipeline.execute_cancelled(&self.ctx.executor, cancel).await;

        match result {
            Ok(output) => {
                if !output.stdout.is_empty() {
                    self.ctx
                        .emit(BuilderEvent::Message(output.stdout.clone()))
                        .await;
                }
                if !output.stderr.is_empty() {
                    self.ctx
                        .emit(BuilderEvent::Message(output.stderr.clone()))
                        .await;
                }
            }
            Err(error) => {
                self.ctx.emit(BuilderEvent::Failed(error.to_string())).await;
                return Err(error);
            }
        }

        // Wait healthy
        self.ctx.emit(BuilderEvent::HealthCheck).await;
        let service_name = format!("{}_db", app_name);
        self.ctx
            .emit(BuilderEvent::Message(format!(
                "Waiting for service {service_name} to become healthy..."
            )))
            .await;
        self.wait_healthy(&service_name, cancel).await?;
        self.ctx
            .emit(BuilderEvent::Message(format!(
                "Service {service_name} is healthy"
            )))
            .await;

        // Post-deploy password sync: ensure container engine uses the exact password configured in database
        self.ctx
            .emit(BuilderEvent::Message(format!(
                "Synchronising configured credentials into {service_name}..."
            )))
            .await;
        if let Ok(q) = auto_di::resolve::<FlashQueries>().await
            && let Ok(creds) = q.database_management_credentials(db_id).await
        {
            let password = creds.db_password.clone().unwrap_or_default();
            if !password.is_empty()
                && let Ok(containers) = self.ctx.docker.containers().ps().all().list().await
                && let Some(matched) = containers.iter().find(|c| {
                    let name = c.names.trim_start_matches('/').to_lowercase();
                    name.starts_with(&format!("{}_db", app_name).to_lowercase())
                        || name.contains(&app_name.to_lowercase())
                })
            {
                let container_id = &matched.id;
                let user_name = creds
                    .db_user
                    .clone()
                    .unwrap_or_else(|| "openoxide".to_string());
                let escaped_pwd = password.replace('\'', "''");
                match db_kind {
                    DatabaseKind::Postgres => {
                        let sql = format!(
                            "ALTER USER \"{}\" WITH PASSWORD '{}';",
                            user_name, escaped_pwd
                        );
                        let _ = self
                            .ctx
                            .docker
                            .containers()
                            .exec(container_id)
                            .run(["psql", "-U", &user_name, "-d", "postgres", "-c", &sql])
                            .await;
                        let _ = self
                            .ctx
                            .docker
                            .containers()
                            .exec(container_id)
                            .run(["psql", "-U", "postgres", "-c", &sql])
                            .await;
                    }
                    DatabaseKind::Mysql | DatabaseKind::Mariadb => {
                        let sql = format!(
                            "ALTER USER '{}'@'%' IDENTIFIED BY '{}'; FLUSH PRIVILEGES;",
                            user_name, escaped_pwd
                        );
                        let _ = self
                            .ctx
                            .docker
                            .containers()
                            .exec(container_id)
                            .run(["mysql", "-u", "root", "-e", &sql])
                            .await;
                    }
                    DatabaseKind::Redis => {
                        let _ = self
                            .ctx
                            .docker
                            .containers()
                            .exec(container_id)
                            .run(["redis-cli", "CONFIG", "SET", "requirepass", &password])
                            .await;
                        let _ = self
                            .ctx
                            .docker
                            .containers()
                            .exec(container_id)
                            .run(["redis-cli", "-a", &password, "CONFIG", "REWRITE"])
                            .await;
                    }
                    _ => {}
                }
            }
        }

        self.ctx.emit(BuilderEvent::Deployed).await;
        Ok(())
    }

    pub async fn stop(&self, app_name: &str, cancel: &CancellationToken) -> ExecResult<()> {
        let services = self
            .ctx
            .docker
            .services()
            .list()
            .filter(crate::utils::docker::query::ServiceFilter::name(format!(
                "{}_",
                app_name
            )))
            .run_json()
            .await;
        if let Ok(services) = services {
            for s in services {
                let _ = self
                    .ctx
                    .docker
                    .services()
                    .remove(&s.name)
                    .cancel_with(cancel.clone())
                    .run()
                    .await;
            }
        } else {
            let service_name = format!("{}_db", app_name);
            let _ = self
                .ctx
                .docker
                .services()
                .remove(&service_name)
                .cancel_with(cancel.clone())
                .run()
                .await;
        }
        Ok(())
    }

    /// Recreate the database's swarm tasks from the existing image (no
    /// redeploy) `reload`.
    pub async fn reload(&self, app_name: &str, cancel: &CancellationToken) -> ExecResult<()> {
        self.ctx
            .emit(BuilderEvent::Message(format!("reloading {app_name}")))
            .await;
        if let Ok(services) = self
            .ctx
            .docker
            .services()
            .list()
            .filter(crate::utils::docker::query::ServiceFilter::name(format!(
                "{}_",
                app_name
            )))
            .run_json()
            .await
        {
            for s in services {
                let _ = self
                    .ctx
                    .docker
                    .services()
                    .update(&s.name)
                    .force()
                    .cancel_with(cancel.clone())
                    .run()
                    .await;
            }
        }
        self.ctx.emit(BuilderEvent::Deployed).await;
        Ok(())
    }

    async fn wait_healthy(&self, service_name: &str, cancel: &CancellationToken) -> ExecResult<()> {
        let deadline = Instant::now() + self.ctx.health_timeout;
        loop {
            self.ctx.cancelled(cancel)?;
            let health_result = self
                .ctx
                .docker
                .services()
                .ps(service_name)
                .filter(TaskFilter::DesiredState(TaskDesiredState::Running))
                .run_json()
                .await;
            let rows = match health_result {
                Ok(rows) => rows,
                Err(error)
                    if Instant::now() < deadline
                        && crate::utils::docker::error::is_transient_docker_error(
                            &error.to_string(),
                        ) =>
                {
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    continue;
                }
                Err(error) => return Err(error),
            };
            if rows.iter().any(|row| {
                row.desired_state.eq_ignore_ascii_case("running")
                    && row.current_state.starts_with("Running")
            }) {
                return Ok(());
            }
            if let Some(failed_task) = rows.iter().find(|row| {
                !row.error.is_empty() && row.desired_state.eq_ignore_ascii_case("running")
            }) {
                return Err(ExecError::CommandFailed {
                    code: None,
                    stderr: failed_task.error.clone(),
                });
            }
            if Instant::now() >= deadline {
                return Err(ExecError::Timeout {
                    seconds: self.ctx.health_timeout.as_secs(),
                });
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }
}
