use std::sync::Arc;

use auto_di::singleton;

use crate::{
    api::dto::compose::management::{ComposeExportBundleDto, ImportComposeDto},
    core::cache::{AppStateCache, CacheEnum, CacheKey},
    repository::{
        ComposeProjectRepository, DomainRepository, EnvironmentRepository, MountRepository,
        PatchRepository,
    },
    services::compose::{ComposeRecord, queries::generate_app_name},
};

const SCHEMA_VERSION: u32 = 1;

pub struct ComposeTransferService {
    projects: Arc<ComposeProjectRepository>,
    environments: Arc<EnvironmentRepository>,
    domains: Arc<DomainRepository>,
    mounts: Arc<MountRepository>,
    patches: Arc<PatchRepository>,
    cache: Arc<AppStateCache>,
}

#[singleton]
impl ComposeTransferService {
    fn new(
        projects: Arc<ComposeProjectRepository>,
        environments: Arc<EnvironmentRepository>,
        domains: Arc<DomainRepository>,
        mounts: Arc<MountRepository>,
        patches: Arc<PatchRepository>,
        cache: Arc<AppStateCache>,
    ) -> Self {
        Self {
            projects,
            environments,
            domains,
            mounts,
            patches,
            cache,
        }
    }

    pub async fn export(
        &self,
        id: i64,
        include_secrets: bool,
    ) -> sqlx::Result<ComposeExportBundleDto> {
        let mut compose = self
            .projects
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let domains = self.domains.list_by_compose_raw(id).await?;
        let mounts = self.mounts.fetch_for_compose(id).await?;
        let patches = self.patches.list_by_compose(id).await?;
        compose.refresh_token = None;
        if !include_secrets {
            compose.env_var = None;
            compose.compose_file = redact_compose_secrets(&compose.compose_file);
        }
        Ok(ComposeExportBundleDto {
            schema_version: SCHEMA_VERSION,
            exported_at: chrono::Utc::now().timestamp(),
            secrets_included: include_secrets,
            compose,
            domains,
            mounts,
            patches,
        })
    }

    pub async fn import(&self, input: ImportComposeDto) -> sqlx::Result<ComposeRecord> {
        let bundle: ComposeExportBundleDto = serde_json::from_str(&input.archive)
            .map_err(|error| sqlx::Error::Protocol(format!("invalid compose archive: {error}")))?;
        if bundle.schema_version != SCHEMA_VERSION {
            return Err(sqlx::Error::Protocol(format!(
                "unsupported compose export schema version {}",
                bundle.schema_version
            )));
        }
        self.environments
            .get_by_id(input.target_environment_id)
            .await?
            .ok_or_else(|| sqlx::Error::Protocol("target environment not found".into()))?;
        let mut compose = bundle.compose.clone();
        let name = input.name.unwrap_or(compose.name);
        let now = chrono::Utc::now().timestamp();
        compose.id = None;
        compose.name = name.clone();
        compose.app_name = generate_app_name(&name);
        compose.environment_id = input.target_environment_id;
        compose.server_id = input.target_server_id;
        compose.refresh_token = None;
        compose.compose_status = "IDLE".into();
        compose.created_at = now;
        compose.updated_at = now;
        let id = self.projects.create(&compose).await?;
        if let Err(error) = self.import_relations(id, bundle).await {
            let _ = self.projects.delete(id).await;
            return Err(error);
        }
        let compose = self
            .projects
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let record = ComposeRecord::from(compose);
        self.cache
            .insert(CacheKey::Compose(id), CacheEnum::Compose(record.clone()))
            .await;
        Ok(record)
    }

    async fn import_relations(&self, id: i64, bundle: ComposeExportBundleDto) -> sqlx::Result<()> {
        for domain in bundle.domains {
            self.domains
                .create_and_return(
                    domain.host,
                    domain.https,
                    domain.port,
                    domain.path,
                    domain.internal_path,
                    domain.custom_entrypoint,
                    domain.service_name,
                    domain.custom_cert_resolver,
                    domain.strip_path,
                    domain.middlewares,
                    "COMPOSE".into(),
                    domain.certificate_type,
                    None,
                    Some(id),
                )
                .await?;
        }
        for mount in bundle.mounts {
            self.mounts
                .create_for_compose(
                    id,
                    &mount.mount_type,
                    mount.host_path.as_deref(),
                    mount.volume_name.as_deref(),
                    mount.file_path.as_deref(),
                    mount.content.as_deref(),
                    &mount.mount_path,
                )
                .await?;
        }
        for mut patch in bundle.patches {
            patch.id = None;
            patch.application_id = None;
            patch.compose_id = Some(id);
            self.patches.create(&patch).await?;
        }
        Ok(())
    }
}

fn redact_compose_secrets(value: &str) -> String {
    value
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("-") && line.contains("/run/secrets/") {
                line.to_string()
            } else if line.to_ascii_lowercase().contains("password")
                || line.to_ascii_lowercase().contains("secret")
                || line.to_ascii_lowercase().contains("token")
            {
                line.split_once(':')
                    .map(|(key, _)| format!("{key}: REDACTED"))
                    .unwrap_or_else(|| line.to_string())
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
