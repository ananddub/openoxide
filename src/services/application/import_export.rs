use std::sync::Arc;

use auto_di::singleton;

use crate::{
    api::dto::application::import_export::{ApplicationExportBundleDto, ImportApplicationDto},
    core::cache::{AppStateCache, CacheEnum, CacheKey},
    repository::{
        ApplicationMiddlewareRepository, ApplicationRepository, DomainRepository,
        EnvironmentRepository, MountRepository, PatchRepository, PortRepository,
        RedirectRepository, SecurityRepository,
    },
    services::application::{ApplicationRecord, queries::generate_app_name},
};

const APPLICATION_EXPORT_SCHEMA_VERSION: u32 = 1;

pub struct ApplicationTransferService {
    applications: Arc<ApplicationRepository>,
    environments: Arc<EnvironmentRepository>,
    domains: Arc<DomainRepository>,
    ports: Arc<PortRepository>,
    mounts: Arc<MountRepository>,
    redirects: Arc<RedirectRepository>,
    security: Arc<SecurityRepository>,
    patches: Arc<PatchRepository>,
    middlewares: Arc<ApplicationMiddlewareRepository>,
    cache: Arc<AppStateCache>,
}

#[singleton]
impl ApplicationTransferService {
    #[allow(clippy::too_many_arguments)]
    fn new(
        applications: Arc<ApplicationRepository>,
        environments: Arc<EnvironmentRepository>,
        domains: Arc<DomainRepository>,
        ports: Arc<PortRepository>,
        mounts: Arc<MountRepository>,
        redirects: Arc<RedirectRepository>,
        security: Arc<SecurityRepository>,
        patches: Arc<PatchRepository>,
        middlewares: Arc<ApplicationMiddlewareRepository>,
        cache: Arc<AppStateCache>,
    ) -> Self {
        Self {
            applications,
            environments,
            domains,
            ports,
            mounts,
            redirects,
            security,
            patches,
            middlewares,
            cache,
        }
    }

    pub async fn export(
        &self,
        application_id: i64,
        include_secrets: bool,
    ) -> sqlx::Result<ApplicationExportBundleDto> {
        let mut application = self
            .applications
            .get_by_id(application_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let domains = self.domains.list_by_application_raw(application_id).await?;
        let ports = self.ports.list_by_application(application_id).await?;
        let mounts = self.mounts.fetch_for_application(application_id).await?;
        let redirects = self.redirects.list_by_application(application_id).await?;
        let mut security = self.security.list_by_application(application_id).await?;
        let patches = self.patches.list_by_application(application_id).await?;
        let middlewares = self.middlewares.list_by_application(application_id).await?;

        application.refresh_token = None;
        if !include_secrets {
            application.env_var = None;
            application.build_secrets = None;
            application.preview_env = None;
            application.preview_build_secrets = None;
            application.docker_password = None;
            for item in &mut security {
                item.password.clear();
            }
        }

        Ok(ApplicationExportBundleDto {
            schema_version: APPLICATION_EXPORT_SCHEMA_VERSION,
            exported_at: chrono::Utc::now().timestamp(),
            secrets_included: include_secrets,
            application,
            domains,
            ports,
            mounts,
            redirects,
            security,
            patches,
            middlewares,
        })
    }

    pub async fn import(&self, input: ImportApplicationDto) -> sqlx::Result<ApplicationRecord> {
        let bundle: ApplicationExportBundleDto =
            serde_json::from_str(&input.archive).map_err(|error| {
                sqlx::Error::Protocol(format!("invalid application archive: {error}"))
            })?;
        if bundle.schema_version != APPLICATION_EXPORT_SCHEMA_VERSION {
            return Err(sqlx::Error::Protocol(format!(
                "unsupported application export schema version {}",
                bundle.schema_version
            )));
        }
        self.environments
            .get_by_id(input.target_environment_id)
            .await?
            .ok_or_else(|| sqlx::Error::Protocol("target environment not found".into()))?;

        let mut application = bundle.application.clone();
        let name = input.name.unwrap_or(application.name);
        let now = chrono::Utc::now().timestamp();
        application.id = None;
        application.name = name.clone();
        application.app_name = generate_app_name(&name);
        application.app_status = "IDLE".into();
        application.environment_id = input.target_environment_id;
        application.server_id = input.target_server_id;
        application.build_server_id = None;
        application.registry_id = None;
        application.rollback_registry_id = None;
        application.build_registry_id = None;
        application.github_provider_id = None;
        application.gitlab_provider_id = None;
        application.gitea_provider_id = None;
        application.bitbucket_provider_id = None;
        application.custom_git_ssh_key_id = None;
        application.refresh_token = None;
        application.created_at = now;
        application.updated_at = now;

        let application_id = self.applications.create(&application).await?;
        let result = self.import_relations(application_id, bundle).await;
        if let Err(error) = result {
            if let Err(cleanup_error) = self.applications.delete(application_id).await {
                tracing::error!(application_id, %cleanup_error, "failed to clean up partial application import");
            }
            return Err(error);
        }

        let application = self
            .applications
            .get_by_id(application_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let record = ApplicationRecord::from(application);
        self.cache
            .insert(
                CacheKey::Application(application_id),
                CacheEnum::Application(record.clone()),
            )
            .await;
        Ok(record)
    }

    async fn import_relations(
        &self,
        application_id: i64,
        bundle: ApplicationExportBundleDto,
    ) -> sqlx::Result<()> {
        for item in bundle.domains {
            self.domains
                .create_and_return(
                    item.host,
                    item.https,
                    item.port,
                    item.path,
                    item.internal_path,
                    item.custom_entrypoint,
                    item.service_name,
                    item.custom_cert_resolver,
                    item.strip_path,
                    item.middlewares,
                    "APPLICATION".into(),
                    item.certificate_type,
                    Some(application_id),
                    None,
                )
                .await?;
        }
        for mut item in bundle.ports {
            item.id = None;
            item.application_id = application_id;
            self.ports.create(&item).await?;
        }
        for item in bundle.mounts {
            self.mounts
                .create_for_application(
                    application_id,
                    &item.mount_type,
                    item.host_path.as_deref(),
                    item.volume_name.as_deref(),
                    item.file_path.as_deref(),
                    item.content.as_deref(),
                    &item.mount_path,
                )
                .await?;
        }
        for mut item in bundle.redirects {
            item.id = None;
            item.application_id = application_id;
            self.redirects.create(&item).await?;
        }
        for mut item in bundle.security {
            if item.password.is_empty() {
                continue;
            }
            item.id = None;
            item.application_id = application_id;
            self.security.create(&item).await?;
        }
        for mut item in bundle.patches {
            item.id = None;
            item.application_id = Some(application_id);
            item.compose_id = None;
            self.patches.create(&item).await?;
        }
        for item in bundle.middlewares {
            self.middlewares
                .create(
                    application_id,
                    &item.name,
                    &item.middleware_type,
                    item.enabled,
                    &item.config,
                )
                .await?;
        }
        Ok(())
    }
}
