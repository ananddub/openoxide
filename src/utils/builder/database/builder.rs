use crate::repository::MountRepository;
use crate::services::database::DatabaseKind;
use crate::utils::builder::shared::BuilderContext;
use crate::utils::builder::spec::BuilderEvent;
use crate::utils::docker::query::filter::{TaskDesiredState, TaskFilter};
use crate::utils::exec::{CommandExecutor, ExecError, ExecResult};
use os::pipeline;
use serde::Serialize;
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub networks: Vec<String>,
    pub deploy: DeploySpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<HealthSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_grace_period: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<String>,
}

#[derive(Serialize, Clone)]
pub struct HealthSpec {
    pub test: Vec<String>,
    pub interval: String,
    pub timeout: String,
    pub retries: u32,
    pub start_period: String,
}

#[derive(Serialize)]
pub struct DeploySpec {
    pub replicas: u32,
    pub resources: DeployResources,
    pub restart_policy: RestartPolicy,
    pub update_config: UpdateConfig,
    pub rollback_config: UpdateConfig,
    #[serde(skip_serializing_if = "Placement::is_empty")]
    pub placement: Placement,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
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
    pub condition: &'static str,
    pub delay: &'static str,
    pub max_attempts: u8,
    pub window: &'static str,
}

#[derive(Serialize)]
pub struct UpdateConfig {
    pub parallelism: u8,
    pub delay: &'static str,
    pub order: &'static str,
    pub failure_action: &'static str,
}

#[derive(Serialize, Default)]
pub struct Placement {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<String>,
}

impl Placement {
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }
}

#[derive(Serialize)]
pub struct ExternalNetwork {
    pub external: bool,
    pub name: String,
}

pub async fn resolve_database_networks(
    network_ids: Option<&str>,
    detach_rustploy_network: i64,
) -> ExecResult<(Vec<String>, BTreeMap<String, ExternalNetwork>)> {
    let mut names = BTreeSet::new();
    if detach_rustploy_network == 0 {
        names.insert(super::super::swarm::OPENOXIDE_NETWORK.to_string());
    }

    if let Some(raw) = network_ids {
        if let Ok(parsed) = serde_json::from_str::<Vec<String>>(raw) {
            let repo = auto_di::resolve::<crate::repository::DatabaseNetworkRepository>()
                .await
                .map_err(|e| ExecError::CommandFailed {
                    code: None,
                    stderr: format!("Failed to resolve DatabaseNetworkRepository: {}", e),
                })?;
            let resolved =
                repo.resolve_names(&parsed)
                    .await
                    .map_err(|e| ExecError::CommandFailed {
                        code: None,
                        stderr: format!("Failed to resolve database networks: {}", e),
                    })?;
            for name in resolved {
                let name = name.trim();
                if !name.is_empty() {
                    names.insert(name.to_string());
                }
            }
        }
    }

    let service_networks = names.iter().cloned().collect::<Vec<_>>();
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

    pub fn with_cgroup(mut self, cg: crate::utils::cgroup::Cgroup) -> Self {
        self.ctx = self.ctx.with_cgroup(cg);
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
        let mount_repo =
            auto_di::resolve::<MountRepository>()
                .await
                .map_err(|e| ExecError::CommandFailed {
                    code: None,
                    stderr: format!("Failed to resolve MountRepository: {}", e),
                })?;
        let mounts =
            mount_repo
                .fetch_for_database(db_id)
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

        self.ctx.emit(BuilderEvent::Building).await;
        self.ctx.cancelled(cancel)?;

        // Pull database docker image on host
        self.ctx
            .docker
            .images()
            .pull(&docker_image)
            .cancel_with(cancel.clone())
            .pull()
            .await?;

        self.ctx.emit(BuilderEvent::ImageReady).await;
        self.ctx.cancelled(cancel)?;

        // 2. Prepare workspace & write stack compose file
        let db_dir = format!("/tmp/openoxide-db-{}", app_name);
        let stack_file_path = format!("{}/stack.yml", db_dir);

        // 3. Ensure Swarm and Network
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

        let result = self
            .ctx
            .apply_cgroup(pipeline)?
            .execute_cancelled(&self.ctx.executor, cancel)
            .await;

        match result {
            Ok(output) => {
                if !output.stdout.is_empty() {
                    self.ctx.emit(BuilderEvent::Message(output.stdout.clone())).await;
                }
                if !output.stderr.is_empty() {
                    self.ctx.emit(BuilderEvent::Message(output.stderr.clone())).await;
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
        self.wait_healthy(&service_name, cancel).await?;

        // Post-deploy password sync: ensure container engine uses the exact password configured in database
        if let Ok(mgmt) = auto_di::resolve::<crate::repository::DatabaseManagementRepository>().await {
            if let Ok(creds) = mgmt.credentials(db_kind, db_id).await {
                if !creds.password.is_empty() {
                    if let Ok(containers) = self.ctx.docker.containers().ps().all().list().await {
                        if let Some(matched) = containers.iter().find(|c| {
                            let name = c.names.trim_start_matches('/').to_lowercase();
                            name.starts_with(&format!("{}_db", app_name).to_lowercase())
                                || name.contains(&app_name.to_lowercase())
                        }) {
                            let container_id = &matched.id;
                            let user_name = creds.username.unwrap_or_else(|| "openoxide".to_string());
                            let escaped_pwd = creds.password.replace('\'', "''");
                            match db_kind {
                                DatabaseKind::Postgres => {
                                    let sql = format!("ALTER USER \"{}\" WITH PASSWORD '{}';", user_name, escaped_pwd);
                                    let _ = self.ctx.docker.containers().exec(container_id).run(["psql", "-U", &user_name, "-d", "postgres", "-c", &sql]).await;
                                    let _ = self.ctx.docker.containers().exec(container_id).run(["psql", "-U", "postgres", "-c", &sql]).await;
                                }
                                DatabaseKind::Mysql | DatabaseKind::Mariadb => {
                                    let sql = format!("ALTER USER '{}'@'%' IDENTIFIED BY '{}'; FLUSH PRIVILEGES;", user_name, escaped_pwd);
                                    let _ = self.ctx.docker.containers().exec(container_id).run(["mysql", "-u", "root", "-e", &sql]).await;
                                }
                                DatabaseKind::Redis => {
                                    let _ = self.ctx.docker.containers().exec(container_id).run(["redis-cli", "CONFIG", "SET", "requirepass", &creds.password]).await;
                                    let _ = self.ctx.docker.containers().exec(container_id).run(["redis-cli", "-a", &creds.password, "CONFIG", "REWRITE"]).await;
                                }
                                _ => {}
                            }
                        }
                    }
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
                if &s.replicas != "0/0" {
                    let _ = self
                        .ctx
                        .docker
                        .services()
                        .scale()
                        .service(&s.name, 0)
                        .cancel_with(cancel.clone())
                        .run()
                        .await;
                }
            }
        } else {
            let service_name = format!("{}_db", app_name);
            let _ = self
                .ctx
                .docker
                .services()
                .scale()
                .service(&service_name, 0)
                .cancel_with(cancel.clone())
                .run()
                .await;
        }
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
