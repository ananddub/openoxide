use serde::{Deserialize, Serialize};

use crate::core::cache::{CacheEnum, CacheKey};
use crate::flash_gen::compose_extra::{
    ComposeCreateServiceNoServerParams, ComposeCreateServiceWithServerParams,
};
use crate::flash_gen::service_compose::{
    CreateServiceComposeSimpleParams, PatchServiceComposeParams,
};
use crate::flash_gen::service_git::{CreateServiceGitParams, UpdateServiceGitTriggerParams};
use crate::flash_gen::services::UpdateServiceCommandParams;

use super::{
    ComposeOperationResult, ComposeRecord, ComposeService,
    queries::{
        ComposeDependencyCounts, compose_dependency_counts, generate_app_name,
        update_git_enable_submodules, update_git_watch_paths, update_service_identity,
    },
};

impl ComposeService {
    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<ComposeRecord> {
        let key = CacheKey::Compose(id);
        let res = self
            .cache
            .try_get_with(key, async {
                let mut record = self.load_record(id).await?;

                if record.compose_file.trim().is_empty() {
                    let paths = crate::utils::paths::OpenOxidePaths::from_env();
                    let clean_path = record.compose_path.trim_start_matches("./");
                    let file_path_1 =
                        format!("{}/{}", paths.compose_source(&record.app_name), clean_path);
                    let file_path_2 = format!(
                        "{}/docker-compose.yml",
                        paths.compose_source(&record.app_name)
                    );
                    let file_path_3 = format!(
                        "{}/docker-compose.yml",
                        paths.compose_files(&record.app_name)
                    );

                    if let Ok(content) = tokio::fs::read_to_string(&file_path_1).await {
                        record.compose_file = content;
                    } else if let Ok(content) = tokio::fs::read_to_string(&file_path_2).await {
                        record.compose_file = content;
                    } else if let Ok(content) = tokio::fs::read_to_string(&file_path_3).await {
                        record.compose_file = content;
                    }
                }

                Ok::<_, sqlx::Error>(CacheEnum::Compose(record))
            })
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        if let CacheEnum::Compose(record) = res {
            Ok(record)
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }

    /// Reads the compose record straight from the database, bypassing the
    /// cache (the retired repositories did the same for their read-backs).
    pub(super) async fn load_record(&self, id: i64) -> sqlx::Result<ComposeRecord> {
        let service = self.queries.get_service_by_i_d(id).await?;
        if service.service_type != "COMPOSE" {
            return Err(sqlx::Error::RowNotFound);
        }
        let ext = self.queries.get_service_compose(id).await?;
        let git = self.queries.get_service_git_by_i_d(id).await.ok();
        Ok(ComposeRecord::from((service, ext, git)))
    }

    pub async fn list_by_environment(
        &self,
        environment_id: i64,
    ) -> sqlx::Result<Vec<ComposeRecord>> {
        let services = self
            .queries
            .list_services_by_type_and_environment("COMPOSE", environment_id)
            .await?;
        let mut list = Vec::with_capacity(services.len());
        for service in services {
            let ext = match self.queries.get_service_compose(service.id).await {
                Ok(ext) => ext,
                Err(sqlx::Error::RowNotFound) => continue,
                Err(error) => return Err(error),
            };
            let git = self.queries.get_service_git_by_i_d(service.id).await.ok();
            list.push(ComposeRecord::from((service, ext, git)));
        }
        Ok(list)
    }

    pub async fn create(&self, input: CreateComposeDto) -> sqlx::Result<ComposeRecord> {
        let app_name = generate_app_name(&input.name);
        // Flash bind params for nullable columns are non-Option; "" is the
        // NULL encoding and is normalized back to None on read.
        let description = input.description.unwrap_or_default();
        let service = match input.server_id {
            Some(server_id) => {
                self.queries
                    .compose_create_service_with_server(&ComposeCreateServiceWithServerParams {
                        name: input.name.clone(),
                        app_name: app_name.clone(),
                        description: description.clone(),
                        environment_id: input.environment_id,
                        server_id,
                    })
                    .await?
            }
            None => {
                self.queries
                    .compose_create_service_no_server(&ComposeCreateServiceNoServerParams {
                        name: input.name.clone(),
                        app_name: app_name.clone(),
                        description: description.clone(),
                        environment_id: input.environment_id,
                    })
                    .await?
            }
        };
        self.queries
            .create_service_compose_simple(&CreateServiceComposeSimpleParams {
                service_id: service.id,
                compose_type: input.compose_type.clone(),
                compose_file: input.compose_file.clone(),
            })
            .await?;
        if input.source_type != "RAW" {
            self.queries
                .create_service_git(&CreateServiceGitParams {
                    service_id: service.id,
                    source_type: input.source_type.clone(),
                    trigger_type: "PUSH".into(),
                    auto_deploy: 0,
                    watch_paths: String::new(),
                    enable_submodules: 0,
                })
                .await?;
        }
        self.get_by_id(service.id).await
    }

    pub async fn patch(&self, id: i64, input: PatchComposeDto) -> sqlx::Result<ComposeRecord> {
        let current = self.get_by_id(id).await?;
        let ext = self.queries.get_service_compose(id).await?;
        let git = self.queries.get_service_git_by_i_d(id).await.ok();
        let name = input.name.as_deref().or(Some(current.name.as_str()));
        let description = input
            .description
            .as_deref()
            .or(current.description.as_deref());
        let env_var = input.env_var.as_deref().or(current.env_var.as_deref());
        let compose_file = input.compose_file.as_deref().unwrap_or(&current.compose_file);
        let compose_type = input.compose_type.as_deref().unwrap_or(&ext.compose_type);
        let command = input.command.as_deref().or(Some(current.command.as_str()));
        let compose_path = input
            .compose_path
            .as_deref()
            .unwrap_or(&current.compose_path);
        let server_id = input.server_id.or(current.server_id);
        let service_networks = serialize_service_networks(input.service_networks.as_ref())?
            .unwrap_or_else(|| current.service_networks.clone());

        let suffix = match (input.suffix, input.randomize) {
            (Some(value), _) if !value.trim().is_empty() => Some(value),
            (_, Some(1)) if ext.suffix.trim().is_empty() => Some(generate_random_suffix()),
            (value, _) => value,
        };

        update_service_identity(&self.queries, id, name, description).await?;
        self.queries
            .update_service_env_var(env_var.unwrap_or_default(), id)
            .await?;
        self.queries
            .update_service_command(
                &UpdateServiceCommandParams {
                    command: command.unwrap_or_default().to_string(),
                    args: String::new(),
                    id,
                },
            )
            .await?;
        // update_service_server binds a non-Option i64; skip the write when
        // the resolved server is None so the NULL round-trips.
        if let Some(server_id) = server_id {
            self.queries.update_service_server(server_id, id).await?;
        }
        // PatchServiceCompose binds are non-Option, so concrete values are
        // resolved here; the COALESCE(...) clauses then become no-ops while
        // keeping the old partial-update semantics.
        self.queries
            .patch_service_compose(&PatchServiceComposeParams {
                compose_file: compose_file.to_string(),
                compose_type: compose_type.to_string(),
                compose_path: compose_path.to_string(),
                suffix: suffix.clone().unwrap_or_else(|| ext.suffix.clone()),
                randomize: input.randomize.unwrap_or(ext.randomize),
                isolated_deployment: input.isolated_deployment.unwrap_or(ext.isolated_deployment),
                isolated_deployments_volume: input
                    .isolated_deployments_volume
                    .unwrap_or(ext.isolated_deployments_volume),
                service_networks,
                service_id: id,
            })
            .await?;

        if let Some(trigger_type) = input.trigger_type.as_deref() {
            let auto_deploy = git.as_ref().and_then(|git| git.auto_deploy).unwrap_or(0);
            self.queries
                .update_service_git_trigger(&UpdateServiceGitTriggerParams {
                    trigger_type: trigger_type.to_string(),
                    auto_deploy,
                    service_id: id,
                })
                .await?;
        }
        update_git_watch_paths(&self.queries, id, input.watch_paths.as_deref()).await?;
        update_git_enable_submodules(&self.queries, id, input.enable_submodules).await?;

        self.cache.invalidate(&CacheKey::Compose(id)).await;
        self.get_by_id(id).await
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        let dependencies = compose_dependency_counts(&self.queries, id).await?;
        if dependencies.blocks_delete() {
            return Err(sqlx::Error::Protocol(format!(
                "compose project has active dependencies: active_deployments={}, enabled_backups={}",
                dependencies.active_deployments, dependencies.enabled_backups
            )));
        }
        // ON DELETE CASCADE removes service_compose/service_git and the
        // keyed relations (domains, mounts, patches, deployments).
        self.queries.delete_service(id).await?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        Ok(())
    }

    pub async fn dependencies(&self, id: i64) -> sqlx::Result<ComposeDependencyCounts> {
        self.get_by_id(id).await?;
        compose_dependency_counts(&self.queries, id).await
    }
}

fn generate_random_suffix() -> String {
    let mut bytes = [0_u8; 4];
    let _ = getrandom::fill(&mut bytes);
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

// ── JSON payload contract (moved from the retired api::dto layer) ──────

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ComposeServiceNetworkDto {
    pub service_name: String,
    pub network_ids: Vec<String>,
    pub detach_rustploy_network: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateComposeDto {
    pub name: String,
    pub description: Option<String>,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    #[serde(default = "default_source_type")]
    pub source_type: String,
    #[serde(default = "default_compose_type")]
    pub compose_type: String,
    #[serde(default)]
    pub compose_file: String,
}

#[derive(Debug, Deserialize)]
pub struct PatchComposeDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub env_var: Option<String>,
    pub compose_file: Option<String>,
    pub compose_type: Option<String>,
    pub trigger_type: Option<String>,
    pub command: Option<String>,
    pub enable_submodules: Option<i64>,
    pub compose_path: Option<String>,
    pub suffix: Option<String>,
    pub randomize: Option<i64>,
    pub isolated_deployment: Option<i64>,
    pub isolated_deployments_volume: Option<i64>,
    pub watch_paths: Option<String>,
    pub service_networks: Option<Vec<ComposeServiceNetworkDto>>,
    pub server_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ComposeResponseDto {
    pub id: i64,
    pub name: String,
    pub app_name: String,
    pub description: Option<String>,
    pub env_var: Option<String>,
    pub compose_file: String,
    pub source_type: String,
    pub compose_type: String,
    pub compose_status: String,
    pub last_error: Option<String>,
    pub trigger_type: String,
    pub repository: Option<String>,
    pub owner: Option<String>,
    pub branch: Option<String>,
    pub gitlab_repository: Option<String>,
    pub gitlab_owner: Option<String>,
    pub gitlab_branch: Option<String>,
    pub gitea_repository: Option<String>,
    pub gitea_owner: Option<String>,
    pub gitea_branch: Option<String>,
    pub bitbucket_repository: Option<String>,
    pub bitbucket_owner: Option<String>,
    pub bitbucket_branch: Option<String>,
    pub custom_git_url: Option<String>,
    pub custom_git_branch: Option<String>,
    pub build_path: Option<String>,
    pub custom_git_ssh_key_id: Option<i64>,
    pub command: String,
    pub compose_path: String,
    pub service_networks: Vec<ComposeServiceNetworkDto>,
    pub services: Vec<String>,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

pub fn detect_compose_file_content(record: &ComposeRecord) -> String {
    if !record.compose_file.trim().is_empty() {
        return record.compose_file.clone();
    }

    let clean_path = record.compose_path.trim_start_matches("./");
    let candidates = [
        format!(
            ".runtime/rustploy/compose/{}/source/docker-compose.yml",
            record.app_name
        ),
        format!(
            ".runtime/rustploy/compose/{}/source/docker-compose.yaml",
            record.app_name
        ),
        format!(
            ".runtime/rustploy/compose/{}/source/compose.yml",
            record.app_name
        ),
        format!(
            ".runtime/rustploy/compose/{}/source/compose.yaml",
            record.app_name
        ),
        format!(
            ".runtime/rustploy/compose/{}/source/{}",
            record.app_name, clean_path
        ),
    ];

    for path in &candidates {
        if let Ok(content) = std::fs::read_to_string(path) {
            if !content.trim().is_empty() {
                return content;
            }
        }
    }

    String::new()
}

pub fn detect_compose_service_names(record: &ComposeRecord) -> Vec<String> {
    let content = detect_compose_file_content(record);
    if !content.trim().is_empty() {
        if let Ok(value) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
            if let Some(services) = value.get("services").and_then(|s| s.as_mapping()) {
                let keys: Vec<String> = services
                    .keys()
                    .filter_map(|k| k.as_str().map(|s| s.to_string()))
                    .collect();
                if !keys.is_empty() {
                    return keys;
                }
            }
        }
    }

    vec!["app".to_string()]
}

impl From<ComposeRecord> for ComposeResponseDto {
    fn from(value: ComposeRecord) -> Self {
        let compose_file_content = detect_compose_file_content(&value);
        let services = detect_compose_service_names(&value);
        Self {
            id: value.id,
            name: value.name,
            app_name: value.app_name,
            description: value.description,
            env_var: value.env_var,
            compose_file: compose_file_content,
            source_type: value.source_type,
            compose_type: value.compose_type.as_str().to_string(),
            compose_status: value.compose_status,
            last_error: value.last_error,
            trigger_type: value.trigger_type,
            repository: value.repository,
            owner: value.owner,
            branch: value.branch,
            gitlab_repository: value.gitlab_repository,
            gitlab_owner: value.gitlab_owner,
            gitlab_branch: value.gitlab_branch,
            gitea_repository: value.gitea_repository,
            gitea_owner: value.gitea_owner,
            gitea_branch: value.gitea_branch,
            bitbucket_repository: value.bitbucket_repository,
            bitbucket_owner: value.bitbucket_owner,
            bitbucket_branch: value.bitbucket_branch,
            custom_git_url: value.custom_git_url,
            custom_git_branch: value.custom_git_branch,
            build_path: value.build_path,
            custom_git_ssh_key_id: value.custom_git_ssh_key_id,
            command: value.command,
            compose_path: value.compose_path,
            service_networks: parse_service_networks(&value.service_networks),
            services,
            environment_id: value.environment_id,
            server_id: value.server_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub fn serialize_service_networks(
    value: Option<&Vec<ComposeServiceNetworkDto>>,
) -> sqlx::Result<Option<String>> {
    value
        .map(serde_json::to_string)
        .transpose()
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))
}

pub fn parse_service_networks(value: &str) -> Vec<ComposeServiceNetworkDto> {
    serde_json::from_str::<Vec<ComposeServiceNetworkDto>>(value).unwrap_or_default()
}

#[derive(Debug, Clone, Serialize)]
pub struct ComposeOperationResponseDto {
    pub compose: ComposeResponseDto,
    pub deployment_id: Option<i64>,
    pub operation: String,
}

impl From<ComposeOperationResult> for ComposeOperationResponseDto {
    fn from(value: ComposeOperationResult) -> Self {
        Self {
            compose: ComposeResponseDto::from(value.compose),
            deployment_id: value.deployment_id,
            operation: value.operation.as_str().into(),
        }
    }
}

fn default_source_type() -> String {
    "GITHUB".into()
}

fn default_compose_type() -> String {
    "DOCKER-COMPOSE".into()
}
