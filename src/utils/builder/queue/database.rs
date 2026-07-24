use auto_di::resolve;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{
    core::config::Config,
    services::{
        compose::remote::{deployment_pid_file, remote_executor},
        database::DatabaseOperation,
    },
    utils::{
        builder::errors::BuilderError,
        builder::{
            custom_type::IdType, database::builder::DatabaseBuilder, hash_state::ApplicationState,
            queue::queue::BuilderQueue,
        },
        cgroup::{CgroupBuilder, CpuLimit, MemoryLimit},
        exec::{CommandExecutor, LocalExecutor},
    },
};

use super::resource::{get_cpu_cores, get_total_memory_kb, parse_cpu_limit, parse_memory_limit};

impl BuilderQueue {
    pub(crate) async fn execute_operation_db(
        &self,
        database_id: i64,
        database_kind: String,
        deployment_id: i64,
        operation: DatabaseOperation,
    ) -> Result<(), BuilderError> {
        let kind = std::str::FromStr::from_str(&database_kind).map_err(|_| {
            BuilderError::Execution(format!("Invalid database kind: {}", database_kind))
        })?;

        let dep_repo = resolve::<crate::repository::DeploymentRepository>()
            .await
            .map_err(|e| {
                BuilderError::Execution(format!("could not resolve DeploymentRepository: {e}"))
            })?;

        let (server_id, environment_id, project_id, memory_limit_db, cpu_limit_db) = dep_repo
            .get_database_deployment_context(database_id, &database_kind)
            .await
            .map_err(|e| BuilderError::Execution(format!("Database context not found: {}", e)))?;

        let db_key = IdType::DatabaseId(database_id);
        let state = resolve::<ApplicationState>().await.map_err(|e| {
            BuilderError::Execution(format!("could not resolve application state: {e}"))
        })?;
        state.reset_default(db_key.clone(), environment_id, project_id);
        let cancel = state
            .cancellation_token(db_key.clone())
            .unwrap_or_else(CancellationToken::new);

        let config = resolve::<Config>().await.map_err(|e| {
            BuilderError::Execution(format!("could not resolve application config: {e}"))
        })?;

        let executor = match server_id {
            Some(sid) => {
                let pid_file = deployment_pid_file(deployment_id);
                dep_repo
                    .set_pid(deployment_id, &pid_file)
                    .await
                    .map_err(|e| {
                        BuilderError::Execution(format!(
                            "could not persist remote deployment pid file: {e}"
                        ))
                    })?;
                CommandExecutor::Remote(
                    remote_executor(self.db.as_ref(), sid)
                        .await
                        .map_err(|e| BuilderError::Execution(e.to_string()))?
                        .with_job_pid_file(pid_file),
                )
            }
            None => CommandExecutor::Local(LocalExecutor::new()),
        };

        let mut mem_limit_str = memory_limit_db;
        let mut cpu_limit_str = cpu_limit_db;

        if mem_limit_str.is_none() || cpu_limit_str.is_none() {
            if let Some(sid) = server_id {
                let server_repo = resolve::<crate::repository::ServerRepository>()
                    .await
                    .map_err(|e| {
                        BuilderError::Execution(format!("could not resolve ServerRepository: {e}"))
                    })?;
                if let Ok(Some(server)) = server_repo.get_by_id(sid).await {
                    if mem_limit_str.is_none() {
                        mem_limit_str = server.build_memory_limit;
                    }
                    if cpu_limit_str.is_none() {
                        cpu_limit_str = server.build_cpu_limit;
                    }
                }
            }
        }

        let mut mem_limit = mem_limit_str.as_deref().and_then(parse_memory_limit);
        let mut cpu_limit = cpu_limit_str.as_deref().and_then(parse_cpu_limit);

        if mem_limit.is_none() {
            if let Some(total_kb) = get_total_memory_kb(&executor).await {
                mem_limit = Some(MemoryLimit::KB(total_kb / 2));
            }
        }
        if cpu_limit.is_none() {
            if let Some(total_cores) = get_cpu_cores(&executor).await {
                let half_cores = (total_cores / 2.0).max(1.0);
                cpu_limit = Some(CpuLimit::Cores(half_cores));
            }
        }

        if mem_limit.is_none() {
            mem_limit = parse_memory_limit(&config.build_memory_limit);
        }
        if cpu_limit.is_none() {
            cpu_limit = parse_cpu_limit(&config.build_cpu_limit);
        }

        let mut cgroup = None;
        if mem_limit.is_some() || cpu_limit.is_some() {
            let mut builder = CgroupBuilder::new("rustploy-build", executor.clone());
            if let Some(mem) = mem_limit {
                builder = builder.memory(mem);
            }
            if let Some(cpu) = cpu_limit {
                builder = builder.cpu(cpu);
            }
            let cg = builder.build();
            if let Err(e) = cg.apply().await {
                tracing::warn!(error = %e, "Failed to apply/ensure build cgroup, proceeding without limits");
            } else {
                cgroup = Some(cg);
            }
        }

        let (events_tx, events_rx) = mpsc::channel(6);
        tokio::spawn(super::deployment_log::record_builder_events(
            deployment_id,
            events_rx,
            "db",
        ));

        let events_tx_clone = events_tx.clone();
        let cgroup_clone = cgroup.clone();

        let build_future = async move {
            let mut builder = DatabaseBuilder::new(executor)
                .with_state(state, db_key)
                .with_events(events_tx);

            if let Some(cg) = cgroup_clone {
                builder = builder.with_cgroup(cg);
            }

            match operation {
                DatabaseOperation::Deploy
                | DatabaseOperation::Redeploy
                | DatabaseOperation::Reload
                | DatabaseOperation::Start => builder
                    .deploy(kind, database_id, &cancel)
                    .await
                    .map(|_| ())
                    .map_err(|e| BuilderError::Execution(e.to_string())),
                DatabaseOperation::Stop => {
                    let app_name = dep_repo
                        .get_database_app_name(database_id, &database_kind)
                        .await
                        .map_err(|e| {
                            BuilderError::Execution(format!(
                                "Failed to query database app_name: {}",
                                e
                            ))
                        })?;

                    builder
                        .stop(&app_name, &cancel)
                        .await
                        .map(|_| ())
                        .map_err(|e| BuilderError::Execution(e.to_string()))
                }
            }
        };

        super::deployment_log::DEPLOYMENT_SENDER
            .scope(events_tx_clone, build_future)
            .await
    }
}
