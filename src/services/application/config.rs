use super::{ApplicationRecord, ApplicationService};
use crate::api::dto::application::{
    PatchBuildConfigDto, PatchPreviewConfigDto, PatchResourceConfigDto,
};
use crate::core::cache::CacheKey;

impl ApplicationService {
    pub async fn patch_preview_config(
        &self,
        id: i64,
        input: PatchPreviewConfigDto,
    ) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        if input.active && input.preview_wildcard.as_deref().is_none_or(str::is_empty) {
            return Err(sqlx::Error::Protocol(
                "preview_wildcard is required when preview deployments are active".into(),
            ));
        }
        if input.preview_certificate_type == crate::utils::traefik::types::CertificateType::Custom
            && input
                .preview_custom_cert_resolver
                .as_deref()
                .is_none_or(str::is_empty)
        {
            return Err(sqlx::Error::Protocol(
                "preview_custom_cert_resolver is required for CUSTOM certificates".into(),
            ));
        }
        self.repo_app
            .update_preview_config(
                id,
                input.preview_env.as_deref(),
                input.preview_build_args.as_deref(),
                input.preview_build_secrets.as_deref(),
                input.preview_labels.as_deref(),
                input.preview_wildcard.as_deref(),
                input.preview_port,
                input.preview_https,
                &input.preview_path,
                input.preview_certificate_type.as_str(),
                input.preview_custom_cert_resolver.as_deref(),
                input.preview_limit,
                input.active,
                input.require_collaborator_permissions,
            )
            .await?;
        self.cache.invalidate(&CacheKey::Application(id)).await;
        Ok(())
    }
    pub async fn patch_build_config(
        &self,
        id: i64,
        input: PatchBuildConfigDto,
    ) -> sqlx::Result<ApplicationRecord> {
        self.repo_app
            .patch_build_config(
                id,
                input.build_args,
                input.build_secrets,
                input.dockerfile,
                input.docker_context_path,
                input.docker_build_stage,
                input.publish_directory,
                input.is_static_spa,
                input.create_env_file,
                input.railpack_version,
                input.heroku_version,
                input.command,
                input.args,
                input.build_path,
                input.clean_cache,
                input.enable_submodules,
                input.watch_paths,
            )
            .await?;
        self.cache.invalidate(&CacheKey::Application(id)).await;
        self.get_by_id(id).await
    }

    pub async fn patch_resource_config(
        &self,
        id: i64,
        input: PatchResourceConfigDto,
    ) -> sqlx::Result<ApplicationRecord> {
        self.repo_app
            .patch_resource_config(
                id,
                input.memory_reservation.clone(),
                input.memory_limit.clone(),
                input.cpu_reservation.clone(),
                input.cpu_limit.clone(),
                input.replicas,
            )
            .await?;

        self.cache.invalidate(&CacheKey::Application(id)).await;
        let updated = self.get_by_id(id).await?;
        let app_name = updated.app_name.clone();
        let server_id = updated.server_id;
        let mem_lim = input.memory_limit;
        let cpu_lim = input.cpu_limit;
        let target_replicas = input.replicas;
        let db_pool = self.db.clone();

        tokio::spawn(async move {
            let docker = match server_id {
                Some(sid) => {
                    if let Ok(exec) =
                        crate::services::compose::remote::remote_executor(db_pool.as_ref(), sid)
                            .await
                    {
                        crate::utils::docker::DockerCli::from_remote_executor(exec)
                    } else {
                        crate::utils::docker::DockerCli::new_local()
                    }
                }
                None => crate::utils::docker::DockerCli::new_local(),
            };

            // 1. Live cgroup update for container resources (memory/cpu)
            let mut has_cgroup_update = false;
            if let Ok(containers) = docker
                .containers()
                .ps()
                .filter(crate::utils::docker::query::ContainerFilter::Name(
                    app_name.clone(),
                ))
                .list()
                .await
            {
                if let Some(c) = containers.first() {
                    let target_name = c.names.trim_start_matches('/').to_string();
                    let containers_handle = docker.containers();
                    let mut update_builder = containers_handle.update(&target_name);
                    if let Some(mem) = mem_lim {
                        if !mem.trim().is_empty() {
                            update_builder = update_builder.memory_str(mem);
                            has_cgroup_update = true;
                        }
                    }
                    if let Some(cpu) = cpu_lim {
                        if !cpu.trim().is_empty() {
                            update_builder = update_builder.cpus_str(cpu);
                            has_cgroup_update = true;
                        }
                    }
                    if has_cgroup_update {
                        let _ = update_builder.run().await;
                    }
                }
            }

            // 2. Live Swarm service scaling if replicas is provided
            if let Some(reps) = target_replicas {
                if reps > 0 {
                    let swarm_service_name = format!("{}_{}", app_name, app_name);
                    let _ = docker
                        .services()
                        .scale()
                        .service(&swarm_service_name, reps as u32)
                        .run()
                        .await;
                    let _ = docker
                        .services()
                        .scale()
                        .service(&app_name, reps as u32)
                        .run()
                        .await;
                }
            }
        });

        Ok(updated)
    }
}
