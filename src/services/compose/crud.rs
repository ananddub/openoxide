use crate::api::dto::compose::{CreateComposeDto, PatchComposeDto};
use crate::core::cache::{CacheEnum, CacheKey};

use super::{ComposeRecord, ComposeService, queries::generate_app_name};

impl ComposeService {
    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<ComposeRecord> {
        let key = CacheKey::Compose(id);
        let res = self
            .cache
            .try_get_with(key, async {
                let project = self
                    .repo_compose
                    .get_by_id(id)
                    .await?
                    .ok_or(sqlx::Error::RowNotFound)?;
                let mut record = ComposeRecord::from(project);

                if record.compose_file.trim().is_empty() {
                    let paths = crate::utils::paths::RustployPaths::from_env();
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

    pub async fn list_by_environment(
        &self,
        environment_id: i64,
    ) -> sqlx::Result<Vec<ComposeRecord>> {
        let list = self
            .repo_compose
            .list_by_environment(environment_id)
            .await?;
        Ok(list.into_iter().map(ComposeRecord::from).collect())
    }

    pub async fn create(&self, input: CreateComposeDto) -> sqlx::Result<ComposeRecord> {
        let app_name = generate_app_name(&input.name);
        let project = self
            .repo_compose
            .create_simple(
                input.name,
                app_name,
                input.description,
                input.environment_id,
                input.server_id,
                input.source_type,
                input.compose_type,
                input.compose_file,
            )
            .await?;
        Ok(ComposeRecord::from(project))
    }

    pub async fn patch(&self, id: i64, input: PatchComposeDto) -> sqlx::Result<ComposeRecord> {
        let current = self
            .repo_compose
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let name = input.name.unwrap_or(current.name);
        let description = input.description.or(current.description);
        let env_var = input.env_var.or(current.env_var);
        let compose_file = input.compose_file.unwrap_or(current.compose_file);
        let compose_type = input.compose_type.unwrap_or(current.compose_type);
        let trigger_type = input.trigger_type.unwrap_or(current.trigger_type);
        let command = input.command.unwrap_or(current.command);
        let compose_path = input.compose_path.unwrap_or(current.compose_path);
        let server_id = input.server_id.or(current.server_id);
        let service_networks =
            crate::api::dto::compose::serialize_service_networks(input.service_networks.as_ref())?
                .unwrap_or(current.service_networks);

        let suffix = match (input.suffix, input.randomize) {
            (Some(value), _) if !value.trim().is_empty() => Some(value),
            (_, Some(1)) if current.suffix.trim().is_empty() => Some(generate_random_suffix()),
            (value, _) => value,
        };

        self.repo_compose
            .patch(
                id,
                name,
                description,
                env_var,
                compose_file,
                compose_type,
                trigger_type,
                command,
                input.enable_submodules,
                compose_path,
                suffix,
                input.randomize,
                input.isolated_deployment,
                input.isolated_deployments_volume,
                input.watch_paths,
                service_networks,
                server_id,
            )
            .await?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        self.get_by_id(id).await
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        let dependencies = self.repo_dependencies.compose(id).await?;
        if dependencies.blocks_delete() {
            return Err(sqlx::Error::Protocol(format!(
                "compose project has active dependencies: active_deployments={}, enabled_backups={}",
                dependencies.active_deployments, dependencies.enabled_backups
            )));
        }
        self.repo_compose.delete(id).await?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        Ok(())
    }

    pub async fn dependencies(
        &self,
        id: i64,
    ) -> sqlx::Result<crate::repository::ResourceDependencyCounts> {
        self.get_by_id(id).await?;
        self.repo_dependencies.compose(id).await
    }
}

fn generate_random_suffix() -> String {
    let mut bytes = [0_u8; 4];
    let _ = getrandom::fill(&mut bytes);
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
