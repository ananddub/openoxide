use auto_di::resolve;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{
    core::config::Config,
    db::models::rollbacks::Rollback,
    services::{
        application::ApplicationOperation,
        compose::remote::{deployment_pid_file, remote_executor},
    },
    utils::{
        builder::errors::BuilderError,
        builder::{
            adapter::ApplicationSpecAdapter, application::ApplicationBuilder, custom_type::IdType,
            hash_state::ApplicationState, queue::queue::BuilderQueue,
        },
        cgroup::{CgroupBuilder, CpuLimit, MemoryLimit},
        exec::{CommandExecutor, LocalExecutor},
    },
};

use super::resource::{get_cpu_cores, get_total_memory_kb, parse_cpu_limit, parse_memory_limit};

impl BuilderQueue {
    pub(crate) async fn execute_operation_app(
        &self,
        application_id: i64,
        deployment_id: i64,
        _operation: ApplicationOperation,
    ) -> Result<(), BuilderError> {
        let spec_adapter = resolve::<ApplicationSpecAdapter>().await.map_err(|e| {
            BuilderError::Execution(format!("could not resolve ApplicationSpecAdapter: {e}"))
        })?;
        let spec = spec_adapter.load(application_id).await.map_err(|e| {
            BuilderError::Execution(format!("could not load deployment config: {e}"))
        })?;

        let app_repo = resolve::<crate::repository::ApplicationRepository>()
            .await
            .map_err(|e| {
                BuilderError::Execution(format!("could not resolve ApplicationRepository: {e}"))
            })?;
        let (environment_id, project_id, server_id) = app_repo
            .get_deployment_context(application_id)
            .await
            .map_err(|e| {
                BuilderError::Execution(format!("could not resolve deployment context: {e}"))
            })?;

        let app_key = IdType::AppId(application_id);
        let state = resolve::<ApplicationState>().await.map_err(|e| {
            BuilderError::Execution(format!("could not resolve application state: {e}"))
        })?;
        state.reset_default(app_key.clone(), environment_id, project_id);
        let cancel = state
            .cancellation_token(app_key.clone())
            .unwrap_or_else(CancellationToken::new);

        let config = resolve::<Config>().await.map_err(|e| {
            BuilderError::Execution(format!("could not resolve application config: {e}"))
        })?;

        let executor = match server_id {
            Some(sid) => {
                let pid_file = deployment_pid_file(deployment_id);
                let dep_repo = resolve::<crate::repository::DeploymentRepository>()
                    .await
                    .map_err(|e| {
                        BuilderError::Execution(format!(
                            "could not resolve DeploymentRepository: {e}"
                        ))
                    })?;
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

        // Determine the build cgroup limits (SQLite -> dynamic resource fallbacks -> config values)
        let mut mem_limit_str = None;
        let mut cpu_limit_str = None;

        if let Some(sid) = server_id {
            let server_repo = resolve::<crate::repository::ServerRepository>()
                .await
                .map_err(|e| {
                    BuilderError::Execution(format!("could not resolve ServerRepository: {e}"))
                })?;
            if let Ok(Some(server)) = server_repo.get_by_id(sid).await {
                if let Some(val) = server.build_memory_limit {
                    if !val.trim().is_empty() {
                        mem_limit_str = Some(val);
                    }
                }
                if let Some(val) = server.build_cpu_limit {
                    if !val.trim().is_empty() {
                        cpu_limit_str = Some(val);
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

        // Apply/ensure the shared build cgroup is present with resolved limits
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
            "app",
        ));

        let spec_snapshot = spec.clone();

        let events_tx_clone = events_tx.clone();
        let cgroup_clone = cgroup.clone();
        let executor_clone = executor.clone();
        let build_future = async move {
            let mut builder = ApplicationBuilder::new(executor_clone)
                .with_state(state, app_key)
                .with_events(events_tx);

            if let Some(cg) = cgroup_clone {
                builder = builder.with_cgroup(cg);
            }

            builder
                .deploy(&spec, &cancel)
                .await
                .map_err(|e| BuilderError::Execution(e.to_string()))
        };

        let deploy_result = super::deployment_log::DEPLOYMENT_SENDER
            .scope(events_tx_clone, build_future)
            .await?;

        self.create_rollback_snapshot(
            application_id,
            deployment_id,
            &deploy_result.image,
            &spec_snapshot,
            &executor,
        )
        .await;

        Ok(())
    }

    async fn create_rollback_snapshot(
        &self,
        application_id: i64,
        deployment_id: i64,
        image: &str,
        spec: &crate::utils::builder::spec::ApplicationSpec,
        executor: &CommandExecutor,
    ) {
        let rollback_repo = match resolve::<crate::repository::RollbackRepository>().await {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!(error = %e, "rollback: could not resolve RollbackRepository, skipping snapshot");
                return;
            }
        };

        let version = match rollback_repo
            .get_next_version_for_application(application_id)
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!(error = %e, "rollback: could not determine next version, skipping snapshot");
                return;
            }
        };

        // Tag image: myapp:latest → myapp:v{version}
        let versioned_image = format_versioned_image(image, version);
        let docker = crate::utils::docker::DockerCli::from_executor(executor.clone());
        if let Err(e) = docker.images().tag(image, &versioned_image).run().await {
            tracing::warn!(
                error = %e,
                source_image = image,
                target_image = %versioned_image,
                "rollback: failed to tag image, skipping snapshot"
            );
            return;
        }

        // Keep rollback images available to other swarm nodes when a registry is configured.
        if let Some(auth) = &spec.image_registry {
            let remote_image = if auth.registry.trim().is_empty() {
                versioned_image.clone()
            } else {
                format!(
                    "{}/{}",
                    auth.registry.trim_end_matches('/'),
                    versioned_image
                )
            };
            let distribution = async {
                docker
                    .system()
                    .login()
                    .registry(&auth.registry)
                    .username(&auth.username)
                    .password(&auth.password)
                    .run()
                    .await?;
                if remote_image != versioned_image {
                    docker
                        .images()
                        .tag(&versioned_image, &remote_image)
                        .run()
                        .await?;
                }
                docker.images().push(&remote_image).run().await
            }
            .await;
            let _ = docker
                .system()
                .logout()
                .registry(&auth.registry)
                .run()
                .await;
            if let Err(error) = distribution {
                tracing::warn!(%error, image = %remote_image, "rollback image push failed; local rollback retained");
            }
        }

        tracing::info!(
            source_image = image,
            versioned_image = %versioned_image,
            version,
            deployment_id,
            "rollback: tagged image for rollback"
        );

        // Serialize the full ApplicationSpec as the rollback context
        let full_context = match serde_json::to_string(spec) {
            Ok(json) => Some(json),
            Err(e) => {
                tracing::warn!(error = %e, "rollback: could not serialize spec, saving without context");
                None
            }
        };

        let rollback = Rollback {
            id: None,
            deployment_id,
            version,
            image: Some(versioned_image),
            full_context,
            created_at: chrono::Utc::now().timestamp(),
        };

        match rollback_repo.create(&rollback).await {
            Ok(id) => {
                tracing::info!(
                    rollback_id = id,
                    version,
                    deployment_id,
                    "rollback: snapshot saved"
                );
            }
            Err(e) => {
                tracing::warn!(error = %e, deployment_id, "rollback: failed to save snapshot");
            }
        }
    }
}

fn format_versioned_image(image: &str, version: i64) -> String {
    let base = if let Some(colon_pos) = image.rfind(':') {
        if image[colon_pos..].contains('/') {
            image
        } else {
            &image[..colon_pos]
        }
    } else {
        image
    };
    format!("{}:v{}", base, version)
}
