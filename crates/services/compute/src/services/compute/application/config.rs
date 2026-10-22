use super::{ApplicationRecord, ApplicationService};
use crate::api::connect::generated::openoxide::api::v1::{
    UpdateBuildConfigRequest, UpdatePreviewConfigRequest, UpdateResourceConfigRequest,
};
use crate::core::cache::CacheKey;
use crate::flash_gen::service_application::UpdatePreviewConfigParams;

impl ApplicationService {
    pub async fn patch_preview_config(
        &self,
        id: i64,
        input: UpdatePreviewConfigRequest,
    ) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        let preview_certificate_type = match input.certificate_type.as_str() {
            "LETSENCRYPT" => crate::utils::traefik::types::CertificateType::LetsEncrypt,
            "NONE" => crate::utils::traefik::types::CertificateType::None,
            _ => crate::utils::traefik::types::CertificateType::None,
        };
        let preview_port = input.port.unwrap_or(0);
        let preview_wildcard = input.wildcard;
        let preview_custom_cert_resolver = input.custom_cert_resolver;
        if input.active && preview_wildcard.as_deref().is_none_or(str::is_empty) {
            return Err(sqlx::Error::Protocol(
                "preview_wildcard is required when preview deployments are active".into(),
            ));
        }
        if preview_certificate_type == crate::utils::traefik::types::CertificateType::Custom
            && preview_custom_cert_resolver
                .as_deref()
                .is_none_or(str::is_empty)
        {
            return Err(sqlx::Error::Protocol(
                "preview_custom_cert_resolver is required for CUSTOM certificates".into(),
            ));
        }
        self.queries
            .update_preview_config(&UpdatePreviewConfigParams {
                preview_env: input.preview_env.unwrap_or_default(),
                preview_build_args: input.build_args.unwrap_or_default(),
                preview_build_secrets: input.build_secrets.unwrap_or_default(),
                preview_labels: input.labels.unwrap_or_default(),
                preview_wildcard: preview_wildcard.unwrap_or_default(),
                preview_port,
                preview_https: i64::from(input.https),
                preview_path: input.path,
                preview_certificate_type: preview_certificate_type.as_str().to_owned(),
                preview_custom_cert_resolver: preview_custom_cert_resolver.unwrap_or_default(),
                preview_limit: input.limit,
                is_preview_deployments_active: i64::from(input.active),
                preview_require_collaborator_permissions: i64::from(
                    input.require_collaborator_permissions,
                ),
                service_id: id,
            })
            .await?;
        self.cache.invalidate(&CacheKey::Application(id)).await;
        Ok(())
    }

    pub async fn patch_build_config(
        &self,
        id: i64,
        input: UpdateBuildConfigRequest,
    ) -> sqlx::Result<ApplicationRecord> {
        // COALESCE keep-on-None across both split tables, exactly as the old
        // repository did — needs nullable binds, so it stays raw SQL.
        sqlx::query(
            "UPDATE service_application SET \
             build_args = COALESCE(?, build_args), \
             build_secrets = COALESCE(?, build_secrets), \
             dockerfile = COALESCE(?, dockerfile), \
             docker_context_path = COALESCE(?, docker_context_path), \
             docker_build_stage = COALESCE(?, docker_build_stage), \
             publish_directory = COALESCE(?, publish_directory), \
             is_static_spa = COALESCE(?, is_static_spa), \
             railpack_version = COALESCE(?, railpack_version), \
             heroku_version = COALESCE(?, heroku_version), \
             build_path = COALESCE(?, build_path), \
             clean_cache = COALESCE(?, clean_cache) \
             WHERE service_id = ?;",
        )
        .bind(&input.build_args)
        .bind(&input.build_secrets)
        .bind(&input.dockerfile)
        .bind(&input.context_path)
        .bind(&input.build_stage)
        .bind(&input.publish_directory)
        .bind(input.static_spa.map(i64::from))
        .bind(&input.railpack_version)
        .bind(&input.heroku_version)
        .bind(&input.build_path)
        .bind(input.clean_cache.map(i64::from))
        .bind(id)
        .execute(&self.queries.pool)
        .await?;
        if input.watch_paths.is_some()
            || input.enable_submodules.is_some()
            || input.command.is_some()
            || input.args.is_some()
            || input.auto_deploy.is_some()
        {
            sqlx::query(
                "UPDATE services SET \
                 command = COALESCE(?, command), \
                 args = COALESCE(?, args) \
                 WHERE id = ?;",
            )
            .bind(&input.command)
            .bind(&input.args)
            .bind(id)
            .execute(&self.queries.pool)
            .await?;
            sqlx::query(
                "UPDATE service_git SET \
                 watch_paths = COALESCE(?, watch_paths), \
                 enable_submodules = COALESCE(?, enable_submodules), \
                 auto_deploy = COALESCE(?, auto_deploy) \
                 WHERE service_id = ?;",
            )
            .bind(&input.watch_paths)
            .bind(input.enable_submodules.map(i64::from))
            .bind(input.auto_deploy.map(i64::from))
            .bind(id)
            .execute(&self.queries.pool)
            .await?;
        }
        self.cache.invalidate(&CacheKey::Application(id)).await;
        self.get_by_id(id).await
    }

    pub async fn patch_resource_config(
        &self,
        id: i64,
        input: UpdateResourceConfigRequest,
    ) -> sqlx::Result<ApplicationRecord> {
        // Option binds with NULL-overwrite semantics (as the old repository did);
        // the generator cannot emit nullable binds, so this stays raw SQL.
        sqlx::query(
            "UPDATE services SET memory_reservation = ?, memory_limit = ?, cpu_reservation = ?, cpu_limit = ?, replicas = COALESCE(?, replicas) WHERE id = ?;",
        )
        .bind(&input.memory_reservation)
        .bind(&input.memory_limit)
        .bind(&input.cpu_reservation)
        .bind(&input.cpu_limit)
        .bind(input.replicas)
        .bind(id)
        .execute(&self.queries.pool)
        .await?;

        self.cache.invalidate(&CacheKey::Application(id)).await;
        let updated = self.get_by_id(id).await?;
        let app_name = updated.app_name.clone();
        let server_id = updated.server_id;
        let mem_lim = input.memory_limit;
        let cpu_lim = input.cpu_limit;
        let target_replicas = input.replicas;
        let _db_pool = self.db.clone();

        tokio::spawn(async move {
            let Ok(executor) =
                crate::services::compute::compose::remote::agent_executor_for(server_id).await
            else {
                tracing::warn!(application = %app_name, "could not resolve Agent for live resource update");
                return;
            };
            let docker = crate::utils::docker::DockerCli::from_executor(executor);

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
                && let Some(c) = containers.first()
            {
                let target_name = c.names.trim_start_matches('/').to_string();
                let containers_handle = docker.containers();
                let mut update_builder = containers_handle.update(&target_name);
                if let Some(mem) = mem_lim
                    && !mem.trim().is_empty()
                {
                    update_builder = update_builder.memory_str(mem);
                    has_cgroup_update = true;
                }
                if let Some(cpu) = cpu_lim
                    && !cpu.trim().is_empty()
                {
                    update_builder = update_builder.cpus_str(cpu);
                    has_cgroup_update = true;
                }
                if has_cgroup_update {
                    let _ = update_builder.run().await;
                }
            }

            // 2. Live Swarm service scaling if replicas is provided
            if let Some(reps) = target_replicas
                && reps > 0
            {
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
        });

        Ok(updated)
    }
}
