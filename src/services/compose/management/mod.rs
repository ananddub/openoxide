mod mount;
mod patch;
mod transfer;

pub use mount::ComposeMountService;
pub use patch::{ComposePatchService, UpsertComposePatchDto};
pub use transfer::{ComposeTransferService, ImportComposeDto};

use std::sync::Arc;

use auto_di::{resolve, singleton};
use serde::{Deserialize, Serialize};

use crate::{
    core::cache::{AppStateCache, CacheKey},
    db::queries::FlashQueries,
    flash_gen::compose_extra::ComposeMoveToEnvironmentParams,
    services::deployment::DeploymentService,
    utils::builder::{compose::transform, queue::BuilderQueue},
};

use super::{
    crud::{CreateComposeDto, PatchComposeDto}, ComposeRecord, ComposeService,
};

pub struct ComposeManagementService {
    compose: Arc<ComposeService>,
    queries: Arc<FlashQueries>,
    deployment_service: Arc<DeploymentService>,
    cache: Arc<AppStateCache>,
}

#[singleton]
impl ComposeManagementService {
    fn new(
        compose: Arc<ComposeService>,
        queries: Arc<FlashQueries>,
        deployment_service: Arc<DeploymentService>,
        cache: Arc<AppStateCache>,
    ) -> Self {
        Self {
            compose,
            queries,
            deployment_service,
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
            .create(CreateComposeDto {
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
                    PatchComposeDto {
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
        self.queries.update_compose_file(&compose_file, id).await?;
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
        self.queries.update_compose_file(&compose_file, id).await?;
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
        self.queries.update_compose_file(&compose_file, id).await?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        self.compose.get_by_id(id).await
    }

    pub async fn move_to_environment(
        &self,
        id: i64,
        target_environment_id: i64,
    ) -> sqlx::Result<ComposeRecord> {
        self.compose.get_by_id(id).await?;
        if self
            .queries
            .deployments_has_running_compose_deployment(&id.to_string())
            .await?
        {
            return Err(sqlx::Error::Protocol(
                "compose deployment is already queued or running".into(),
            ));
        }
        // Old move_to_environment_in_same_organization returned None when no
        // row matched; :one RETURNING id surfaces that as RowNotFound.
        self.queries
            .compose_move_to_environment(&ComposeMoveToEnvironmentParams {
                environment_id: target_environment_id,
                id,
                id2: target_environment_id,
            })
            .await
            .map_err(|error| match error {
                sqlx::Error::RowNotFound => sqlx::Error::Protocol(
                    "target environment not found or belongs to another organization".into(),
                ),
                other => other,
            })?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        self.compose.get_by_id(id).await
    }

    pub async fn rotate_webhook_token(&self, id: i64) -> sqlx::Result<String> {
        self.compose.get_by_id(id).await?;
        let token = generate_token()?;
        // Old rotate_refresh_token returned rows_affected == 1; a miss
        // surfaces here as RowNotFound.
        self.queries
            .compose_rotate_webhook_token(&token, id)
            .await?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        Ok(token)
    }

    pub async fn cleanup_queue(&self, id: i64) -> sqlx::Result<u64> {
        self.compose.get_by_id(id).await?;
        if let Ok(queue) = resolve::<BuilderQueue>().await {
            let _ = queue.cancel_queued_compose(id).await;
        }
        // Old cancel_queued_for_compose_count returned the UPDATE's
        // rows_affected; the queued ids are collected first so the count
        // matches the rows the follow-up statement cancels.
        let queued = self
            .queries
            .deployments_get_queued_ids_for_compose(id)
            .await?;
        self.queries
            .deployments_cancel_queued_for_compose(id)
            .await?;
        Ok(queued.len() as u64)
    }

    pub async fn clear_history(&self, id: i64) -> sqlx::Result<u64> {
        self.compose.get_by_id(id).await?;
        let deployment_ids = self
            .queries
            .deployments_list_finished_ids_for_compose(id)
            .await?;
        // Old delete_finished_for_compose returned rows_affected, which the
        // id list above mirrors for the follow-up DELETE.
        let affected = deployment_ids.len() as u64;
        self.queries
            .deployments_delete_finished_for_compose(id)
            .await?;
        for deployment_id in deployment_ids {
            let path = crate::utils::paths::openoxide_paths().deployment_log_file(deployment_id);
            if let Err(error) = tokio::fs::remove_file(path).await
                && error.kind() != std::io::ErrorKind::NotFound
            {
                tracing::warn!(deployment_id, %error, "could not delete compose deployment log");
            }
        }
        Ok(affected)
    }

    pub async fn force_kill(
        &self,
        id: i64,
    ) -> sqlx::Result<crate::services::deployment::CancelDeploymentResult> {
        self.compose.get_by_id(id).await?;
        // Old running_for_compose returned Option; :one surfaces a miss as
        // RowNotFound, mapped to the original Protocol error.
        let deployment = self
            .queries
            .deployments_running_for_compose(id)
            .await
            .map_err(|error| match error {
                sqlx::Error::RowNotFound => {
                    sqlx::Error::Protocol("compose has no running deployment".into())
                }
                other => other,
            })?;
        self.deployment_service.cancel(deployment.id).await
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

// ── JSON payload contract (moved from the retired api::dto layer) ──────

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComposeResourceKind {
    Config,
    Secret,
}
impl ComposeResourceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Config => "CONFIG",
            Self::Secret => "SECRET",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct InstallComposeTemplateDto {
    pub name: String,
    pub description: Option<String>,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub compose_file: String,
    pub env_var: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ComposePreviewDto {
    pub compose_file: String,
    pub randomize: bool,
    pub suffix: Option<String>,
    pub isolated_deployment: bool,
    pub isolated_deployments_volume: bool,
    pub app_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ComposePreviewResponseDto {
    pub compose_file: String,
    pub services: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertComposeResourceDto {
    pub kind: ComposeResourceKind,
    pub name: String,
    pub file: Option<String>,
    pub external: bool,
    pub services: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteComposeResourceDto {
    pub kind: ComposeResourceKind,
    pub name: String,
}
