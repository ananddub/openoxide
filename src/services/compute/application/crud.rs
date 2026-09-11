use uuid::Uuid;

use crate::api::connect::generated::openoxide::api::v1::{
    ApplicationBuildType, ApplicationSourceType, CreateApplicationRequest,
    UpdateApplicationRequest,
};
use crate::core::cache::{CacheEnum, CacheKey};
use crate::flash_gen::resource_dependencies::{
    ServiceDependencyCountsParams, ServiceDependencyCountsRow,
};
use crate::flash_gen::services::{
    AppCoreCreateApplicationNoServerParams, AppCoreCreateApplicationWithServerParams,
    AppCorePatchApplicationParams,
};
use crate::flash_gen::service_git::AppCoreCreateServiceGitSourceParams;
use crate::services::compute::database::crud::serialize_json_string_vec;

use super::{ApplicationRecord, ApplicationService};

fn proto_to_source_type(value: &buffa::EnumValue<ApplicationSourceType>) -> String {
    match value.as_known() {
        Some(ApplicationSourceType::Github) => "GITHUB".into(),
        Some(ApplicationSourceType::Gitlab) => "GITLAB".into(),
        Some(ApplicationSourceType::Bitbucket) => "BITBUCKET".into(),
        Some(ApplicationSourceType::Gitea) => "GITEA".into(),
        Some(ApplicationSourceType::CustomGit) => "CUSTOM".into(),
        Some(ApplicationSourceType::Docker) => "DOCKER".into(),
        Some(ApplicationSourceType::Drop) => "DROP".into(),
        _ => "GITHUB".into(),
    }
}

fn proto_to_build_type(value: &buffa::EnumValue<ApplicationBuildType>) -> String {
    match value.as_known() {
        Some(ApplicationBuildType::Buildpacks) => "BUILDPACKS".into(),
        Some(ApplicationBuildType::Dockerfile) => "DOCKERFILE".into(),
        Some(ApplicationBuildType::Nixpacks) => "NIXPACKS".into(),
        Some(ApplicationBuildType::Heroku) => "HEROKU".into(),
        // The builder's DB spellings (utils/builder/spec.rs) — the wire enum
        // shortens PAKETO/RAILPACK/STATIC; the mapping keeps both aligned.
        Some(ApplicationBuildType::Paketo) => "PAKETO_BUILDPACKS".into(),
        Some(ApplicationBuildType::Railpack) => "RAILPACK".into(),
        Some(ApplicationBuildType::Static) => "STATIC".into(),
        _ => "NIXPACKS".into(),
    }
}

fn dependency_counts_params(id: i64) -> ServiceDependencyCountsParams {
    let service_id = id.to_string();
    ServiceDependencyCountsParams {
        service_id: service_id.clone(),
        service_id2: service_id.clone(),
        service_id3: service_id.clone(),
        service_id4: service_id.clone(),
        service_id5: service_id.clone(),
        service_id6: service_id.clone(),
        service_id7: service_id.clone(),
        service_id8: service_id.clone(),
        base_service_id: service_id.clone(),
        preview_service_id: service_id.clone(),
        service_id9: service_id.clone(),
        service_id10: service_id.clone(),
        service_id11: service_id.clone(),
        service_id12: service_id,
    }
}

impl ApplicationService {
    pub async fn ensure_organization_access(
        &self,
        id: i64,
        organization_id: i64,
    ) -> sqlx::Result<()> {
        if self.queries.services_belongs_to_organization(id, organization_id).await? > 0 {
            Ok(())
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }

    /// Assembles the unified-schema rows (services + service_application +
    /// optional service_git) back into an [ApplicationRecord]. Returns `None`
    /// when the id does not reference a complete APPLICATION service.
    pub async fn load_record(&self, id: i64) -> sqlx::Result<Option<ApplicationRecord>> {
        let service = match self.queries.get_service_by_i_d(id).await {
            Ok(row) => row,
            Err(sqlx::Error::RowNotFound) => return Ok(None),
            Err(error) => return Err(error),
        };
        if service.service_type != "APPLICATION" {
            return Ok(None);
        }
        let ext = match self.queries.get_service_application(id).await {
            Ok(row) => row,
            Err(sqlx::Error::RowNotFound) => return Ok(None),
            Err(error) => return Err(error),
        };
        let git = self.queries.get_service_git_by_i_d(id).await.ok();
        Ok(Some(ApplicationRecord::from((service, ext, git))))
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<ApplicationRecord> {
        let key = CacheKey::Application(id);
        let res = self
            .cache
            .try_get_with(key, async {
                let app = self
                    .load_record(id)
                    .await?
                    .ok_or(sqlx::Error::RowNotFound)?;
                Ok::<_, sqlx::Error>(CacheEnum::Application(app))
            })
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        if let CacheEnum::Application(record) = res {
            Ok(record)
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }

    pub async fn list_by_environment(
        &self,
        environment_id: i64,
    ) -> sqlx::Result<Vec<ApplicationRecord>> {
        let services = self
            .queries
            .list_services_by_type_and_environment("APPLICATION", environment_id)
            .await?;
        let mut list = Vec::with_capacity(services.len());
        for service in services {
            let ext = match self.queries.get_service_application(service.id).await {
                Ok(row) => row,
                Err(sqlx::Error::RowNotFound) => continue,
                Err(error) => return Err(error),
            };
            let git = self.queries.get_service_git_by_i_d(service.id).await.ok();
            list.push(ApplicationRecord::from((service, ext, git)));
        }
        Ok(list)
    }

    pub async fn create(&self, input: CreateApplicationRequest) -> sqlx::Result<ApplicationRecord> {
        let app_name = generate_app_name(&input.name);
        let source_type = proto_to_source_type(&input.source_type);
        let build_type = proto_to_build_type(&input.build_type);
        // Bind params for nullable columns are non-Option; "" is normalized
        // back to None when the row is read.
        let description = input.description.unwrap_or_default();

        let service = match input.server_id {
            Some(server_id) => {
                self.queries
                    .app_core_create_application_with_server(
                        &AppCoreCreateApplicationWithServerParams {
                            name: input.name,
                            app_name,
                            description,
                            environment_id: input.environment_id,
                            server_id,
                        },
                    )
                    .await?
            }
            None => {
                self.queries
                    .app_core_create_application_no_server(
                        &AppCoreCreateApplicationNoServerParams {
                            name: input.name,
                            app_name,
                            description,
                            environment_id: input.environment_id,
                        },
                    )
                    .await?
            }
        };

        // Old create_simple relied on the `applications` table defaults for
        // build and preview configuration; the explicit insert preserves them.
        self.queries
            .app_core_create_application_build_config(service.id, &build_type)
            .await?;
        // Old create_simple stored the requested source type verbatim
        // (auto_deploy default 1, trigger PUSH, submodules off).
        self.queries
            .app_core_create_service_git_source(&AppCoreCreateServiceGitSourceParams {
                service_id: service.id,
                source_type,
                trigger_type: "PUSH".into(),
            })
            .await?;

        self.load_record(service.id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn patch(
        &self,
        id: i64,
        input: UpdateApplicationRequest,
    ) -> sqlx::Result<ApplicationRecord> {
        let current = self.get_by_id(id).await?;
        let name = input.name.unwrap_or(current.name);
        let build_type = input
            .build_type
            .as_ref()
            .map(proto_to_build_type)
            .unwrap_or_else(|| current.build_type.clone());
        let trigger_type = input
            .trigger_type
            .clone()
            .unwrap_or_else(|| current.trigger_type.clone());
        let network_ids = serialize_json_string_vec(if input.network_ids.is_empty() {
            None
        } else {
            Some(&input.network_ids)
        })?
        .unwrap_or(current.network_ids);
        let detach_rustploy_network = match input.detach_rustploy_network {
            Some(value) => i64::from(value),
            None => current.detach_rustploy_network,
        };

        // Old patch rewrote every column on the single applications row with
        // input.or(current); each unified-schema statement below is only
        // executed with a concrete value so NULL never has to be bound.
        self.queries
            .app_core_patch_application(&AppCorePatchApplicationParams {
                name,
                network_ids,
                detach_rustploy_network,
                id,
            })
            .await?;
        if let Some(description) = input.description {
            self.queries.app_core_patch_description(&description, id).await?;
        }
        if let Some(env_var) = input.env_var {
            self.queries.update_service_env_var(&env_var, id).await?;
        }
        if let Some(server_id) = input.server_id {
            self.queries.update_service_server(server_id, id).await?;
        }
        if build_type != current.build_type {
            self.queries.app_core_patch_build_type(&build_type, id).await?;
        }
        if trigger_type != current.trigger_type {
            self.queries
                .app_core_patch_trigger_type(&trigger_type, id)
                .await?;
        }
        if let Some(build_server_id) = input.build_server_id {
            self.queries
                .app_core_patch_build_server(build_server_id, id)
                .await?;
        }
        if let Some(registry_id) = input.registry_id {
            self.queries.app_core_patch_registry(registry_id, id).await?;
        }

        let record = self
            .load_record(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        self.cache
            .insert(
                CacheKey::Application(id),
                CacheEnum::Application(record.clone()),
            )
            .await;
        Ok(record)
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        let app = self.get_by_id(id).await?;
        let dependencies = self
            .queries
            .service_dependency_counts(&dependency_counts_params(id))
            .await?;
        // Old ResourceDependencyCounts::blocks_delete().
        if dependencies.active_deployments > 0 || dependencies.enabled_backups > 0 {
            return Err(sqlx::Error::Protocol(format!(
                "application has active dependencies: active_deployments={}, enabled_backups={}",
                dependencies.active_deployments, dependencies.enabled_backups
            )));
        }
        let previews = auto_di::resolve::<crate::services::compute::preview::PreviewDeploymentService>()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        previews
            .ensure_application_can_be_deleted(id)
            .await
            .map_err(sqlx::Error::Protocol)?;
        previews
            .remove_for_base_application(id)
            .await
            .map_err(sqlx::Error::Protocol)?;

        let app_name = app.app_name.clone();
        let server_id = app.server_id;
        let _db_ref = self.db.clone();

        tokio::spawn(async move {
            let Ok(executor) =
                crate::services::compute::compose::remote::agent_executor_for(server_id).await
            else {
                tracing::warn!(application = %app_name, "could not resolve Agent for application cleanup");
                return;
            };
            let docker_cli = crate::utils::docker::DockerCli::from_executor(executor);

            let candidates = [
                app_name.clone(),
                format!("{}_app", app_name),
                format!("{}-app", app_name),
            ];

            for candidate in &candidates {
                let _ = tokio::time::timeout(
                    std::time::Duration::from_secs(4),
                    docker_cli.container(candidate).stop().run(),
                )
                .await;
                let _ = tokio::time::timeout(
                    std::time::Duration::from_secs(4),
                    docker_cli
                        .container(candidate)
                        .remove()
                        .force()
                        .volumes()
                        .run(),
                )
                .await;
            }
        });

        // service_application / service_git rows cascade via FK.
        self.queries.delete_service(id).await?;
        self.cache.invalidate(&CacheKey::Application(id)).await;
        Ok(())
    }

    pub async fn dependencies(&self, id: i64) -> sqlx::Result<ServiceDependencyCountsRow> {
        self.get_by_id(id).await?;
        self.queries
            .service_dependency_counts(&dependency_counts_params(id))
            .await
    }
}

pub(super) fn generate_app_name(name: &str) -> String {
    let raw = name
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric(), "-");
    let clean = raw.trim_matches('-');
    let suffix = Uuid::new_v4()
        .to_string()
        .split('-')
        .next()
        .unwrap()
        .to_string();
    format!("{}-{}", clean, suffix)
}
