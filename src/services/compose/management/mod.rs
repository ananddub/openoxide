mod mount;
mod patch;
mod transfer;

pub use mount::ComposeMountService;
pub use patch::ComposePatchService;
pub use transfer::ComposeTransferService;

use std::sync::Arc;

use auto_di::{resolve, singleton};

use crate::{
    api::dto::compose::management::{
        ComposePreviewDto, ComposePreviewResponseDto, DeleteComposeResourceDto,
        InstallComposeTemplateDto, UpsertComposeResourceDto,
    },
    core::cache::{AppStateCache, CacheKey},
    repository::{ComposeProjectRepository, DeploymentRepository},
    utils::builder::{compose::transform, queue::BuilderQueue},
};

use super::{ComposeRecord, ComposeService};

pub struct ComposeManagementService {
    compose: Arc<ComposeService>,
    projects: Arc<ComposeProjectRepository>,
    deployments: Arc<DeploymentRepository>,
    cache: Arc<AppStateCache>,
}

#[singleton]
impl ComposeManagementService {
    fn new(
        compose: Arc<ComposeService>,
        projects: Arc<ComposeProjectRepository>,
        deployments: Arc<DeploymentRepository>,
        cache: Arc<AppStateCache>,
    ) -> Self {
        Self {
            compose,
            projects,
            deployments,
            cache,
        }
    }

    pub fn preview(&self, input: ComposePreviewDto) -> sqlx::Result<ComposePreviewResponseDto> {
        let suffix = input.randomize.then(|| {
            input
                .suffix
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(generate_suffix)
        });
        let app_name = input.isolated_deployment.then(|| {
            input
                .app_name
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "compose-preview".into())
        });
        let compose_file = transform::transform_compose(
            &input.compose_file,
            suffix.as_deref(),
            app_name.as_deref(),
            input.isolated_deployments_volume,
        )
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let services = transform::list_services(&compose_file)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(ComposePreviewResponseDto {
            compose_file,
            services,
        })
    }

    pub async fn install_template(
        &self,
        input: InstallComposeTemplateDto,
    ) -> sqlx::Result<ComposeRecord> {
        let created = self
            .compose
            .create(crate::api::dto::compose::CreateComposeDto {
                name: input.name,
                description: input.description,
                environment_id: input.environment_id,
                server_id: input.server_id,
                source_type: "RAW".into(),
                compose_type: "DOCKER-COMPOSE".into(),
                compose_file: input.compose_file,
            })
            .await?;
        if let Some(env_var) = input.env_var {
            self.compose
                .patch(
                    created.id,
                    crate::api::dto::compose::PatchComposeDto {
                        name: None,
                        description: None,
                        env_var: Some(env_var),
                        compose_file: None,
                        compose_type: None,
                        trigger_type: None,
                        command: None,
                        enable_submodules: None,
                        compose_path: None,
                        suffix: None,
                        randomize: None,
                        isolated_deployment: None,
                        isolated_deployments_volume: None,
                        watch_paths: None,
                        service_networks: None,
                        server_id: None,
                    },
                )
                .await
        } else {
            Ok(created)
        }
    }

    pub async fn remove_service(&self, id: i64, service_name: &str) -> sqlx::Result<ComposeRecord> {
        let current = self.compose.get_by_id(id).await?;
        if crate::utils::builder::spec::SourceType::from_str(current.source_type.as_str())
            .ok_or_else(|| sqlx::Error::Protocol("invalid compose source type".into()))?
            != crate::utils::builder::spec::SourceType::Raw
        {
            return Err(sqlx::Error::Protocol(
                "service removal requires a RAW compose source".into(),
            ));
        }
        let compose_file = transform::remove_service(&current.compose_file, service_name)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        self.projects.update_compose_file(id, &compose_file).await?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        self.compose.get_by_id(id).await
    }

    pub async fn upsert_resource(
        &self,
        id: i64,
        input: UpsertComposeResourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        let current = self.compose.get_by_id(id).await?;
        if crate::utils::builder::spec::SourceType::from_str(current.source_type.as_str())
            .ok_or_else(|| sqlx::Error::Protocol("invalid compose source type".into()))?
            != crate::utils::builder::spec::SourceType::Raw
        {
            return Err(sqlx::Error::Protocol(
                "config and secret editing requires a RAW compose source".into(),
            ));
        }
        let compose_file = transform::upsert_resource(
            &current.compose_file,
            input.kind.as_str(),
            &input.name,
            input.file.as_deref(),
            input.external,
            &input.services,
        )
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        self.projects.update_compose_file(id, &compose_file).await?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        self.compose.get_by_id(id).await
    }

    pub async fn remove_resource(
        &self,
        id: i64,
        input: DeleteComposeResourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        let current = self.compose.get_by_id(id).await?;
        if crate::utils::builder::spec::SourceType::from_str(current.source_type.as_str())
            .ok_or_else(|| sqlx::Error::Protocol("invalid compose source type".into()))?
            != crate::utils::builder::spec::SourceType::Raw
        {
            return Err(sqlx::Error::Protocol(
                "config and secret editing requires a RAW compose source".into(),
            ));
        }
        let compose_file =
            transform::remove_resource(&current.compose_file, input.kind.as_str(), &input.name)
                .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        self.projects.update_compose_file(id, &compose_file).await?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        self.compose.get_by_id(id).await
    }

    pub async fn move_to_environment(
        &self,
        id: i64,
        target_environment_id: i64,
    ) -> sqlx::Result<ComposeRecord> {
        self.compose.get_by_id(id).await?;
        if self.deployments.has_running_compose_deployment(id).await? {
            return Err(sqlx::Error::Protocol(
                "compose deployment is already queued or running".into(),
            ));
        }
        let project = self
            .projects
            .move_to_environment_in_same_organization(id, target_environment_id)
            .await?
            .ok_or_else(|| {
                sqlx::Error::Protocol(
                    "target environment not found or belongs to another organization".into(),
                )
            })?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        Ok(project.into())
    }

    pub async fn rotate_webhook_token(&self, id: i64) -> sqlx::Result<String> {
        self.compose.get_by_id(id).await?;
        let token = generate_token()?;
        if !self.projects.rotate_refresh_token(id, &token).await? {
            return Err(sqlx::Error::RowNotFound);
        }
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        Ok(token)
    }

    pub async fn cleanup_queue(&self, id: i64) -> sqlx::Result<u64> {
        self.compose.get_by_id(id).await?;
        if let Ok(queue) = resolve::<BuilderQueue>().await {
            let _ = queue.cancel_queued_compose(id).await;
        }
        self.deployments.cancel_queued_for_compose_count(id).await
    }

    pub async fn clear_history(&self, id: i64) -> sqlx::Result<u64> {
        self.compose.get_by_id(id).await?;
        let deployment_ids = self.deployments.list_finished_ids_for_compose(id).await?;
        let affected = self.deployments.delete_finished_for_compose(id).await?;
        for deployment_id in deployment_ids {
            let path = crate::utils::paths::rustploy_paths().deployment_log_file(deployment_id);
            if let Err(error) = tokio::fs::remove_file(path).await
                && error.kind() != std::io::ErrorKind::NotFound
            {
                tracing::warn!(deployment_id, %error, "could not delete compose deployment log");
            }
        }
        Ok(affected)
    }
}

fn generate_suffix() -> String {
    let mut bytes = [0_u8; 4];
    let _ = getrandom::fill(&mut bytes);
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn generate_token() -> sqlx::Result<String> {
    let mut bytes = [0_u8; 32];
    getrandom::fill(&mut bytes).map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}
