use crate::db::queries::FlashQueries;
use crate::flash_gen::models::Registries;
use crate::flash_gen::registries::{
    RegistriesCreateParams, RegistriesUpdateParams, RegistriesUsageReferencesParams,
};
use auto_di::singleton;
use serde::Deserialize;
use std::sync::Arc;

use super::types::{CreateRegistryDto, PatchRegistryDto};

pub struct RegistryService {
    queries: Arc<FlashQueries>,
}

#[derive(Deserialize)]
struct RepositoryCatalogResponse {
    #[serde(default)]
    repositories: Vec<String>,
}

#[derive(Deserialize)]
struct RepositoryTagsResponse {
    #[serde(default)]
    tags: Vec<String>,
}

#[singleton]
impl RegistryService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Registries> {
        self.queries.registries_get_by_id(id).await
    }

    pub async fn list(&self) -> sqlx::Result<Vec<Registries>> {
        self.queries.registries_get_all().await
    }

    pub async fn create(&self, input: CreateRegistryDto) -> sqlx::Result<Registries> {
        let now = chrono::Utc::now().timestamp();
        let new_id = self
            .queries
            .registries_create(&RegistriesCreateParams {
                registry_name: input.registry_name,
                image_prefix: input.image_prefix.unwrap_or_default(),
                username: input.username,
                password: input.password,
                registry_url: input.registry_url,
                registry_type: input.registry_type,
                created_at: now,
                updated_at: now,
            })
            .await?;
        self.queries.registries_get_by_id(new_id).await
    }

    pub async fn patch(&self, id: i64, input: PatchRegistryDto) -> sqlx::Result<Registries> {
        let mut current = self.get_by_id(id).await?;
        let now = chrono::Utc::now().timestamp();

        if let Some(v) = input.registry_name {
            current.registry_name = v;
        }
        if let Some(v) = input.image_prefix {
            current.image_prefix = v;
        }
        if let Some(v) = input.username {
            current.username = v;
        }
        if let Some(v) = input.password {
            current.password = v;
        }
        if let Some(v) = input.registry_url {
            current.registry_url = v;
        }
        if let Some(v) = input.registry_type {
            current.registry_type = v;
        }
        current.updated_at = now;

        self.update_row(id, &current).await?;
        self.queries.registries_get_by_id(id).await
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        let usage = self
            .queries
            .registries_usage_references(&RegistriesUsageReferencesParams {
                registry_id: id,
                rollback_registry_id: id,
                build_registry_id: id,
            })
            .await?;
        if !usage.is_empty() {
            return Err(sqlx::Error::Protocol(format!(
                "registry is used by {} application configuration(s)",
                usage.len()
            )));
        }
        self.queries.registries_delete(id).await
    }

    pub async fn test_connection_raw(
        &self,
        registry_url: &str,
        username: &str,
        password: &str,
    ) -> Result<(), String> {
        let executor = crate::services::compute::compose::remote::agent_executor_for(None).await?;
        let docker = crate::utils::docker::DockerCli::from_executor(executor);
        let registry = if registry_url.trim().is_empty() || registry_url.contains("docker.io") {
            None
        } else {
            Some(registry_url)
        };

        let mut login_builder = docker
            .system()
            .login()
            .username(username)
            .password(password);
        if let Some(r) = registry {
            login_builder = login_builder.registry(r);
        }
        let res = login_builder.run().await;
        match res {
            Ok(output) => {
                if output.success() {
                    let mut logout_builder = docker.system().logout();
                    if let Some(r) = registry {
                        logout_builder = logout_builder.registry(r);
                    }
                    let _ = logout_builder.run().await;
                    Ok(())
                } else {
                    Err(format!("Login failed: {}", output.stderr))
                }
            }
            Err(e) => Err(format!("Docker execution error: {}", e.to_string())),
        }
    }

    pub async fn test_connection(&self, id: i64) -> Result<(), String> {
        let reg = self.get_by_id(id).await.map_err(|e| e.to_string())?;
        self.test_connection_raw(&reg.registry_url, &reg.username, &reg.password)
            .await
    }

    pub async fn repositories(&self, id: i64) -> Result<Vec<String>, String> {
        let registry = self
            .get_by_id(id)
            .await
            .map_err(|error| error.to_string())?;
        let url = api_url(&registry.registry_url, "/v2/_catalog?n=1000");
        let value: RepositoryCatalogResponse = reqwest::Client::new()
            .get(url)
            .basic_auth(&registry.username, Some(&registry.password))
            .send()
            .await
            .map_err(|error| error.to_string())?
            .error_for_status()
            .map_err(|error| error.to_string())?
            .json()
            .await
            .map_err(|error| error.to_string())?;
        Ok(value.repositories)
    }

    pub async fn tags(&self, id: i64, repository: &str) -> Result<Vec<String>, String> {
        let registry = self
            .get_by_id(id)
            .await
            .map_err(|error| error.to_string())?;
        if repository.is_empty() || repository.contains("..") {
            return Err("invalid repository name".into());
        }
        let url = api_url(
            &registry.registry_url,
            &format!("/v2/{repository}/tags/list"),
        );
        let value: RepositoryTagsResponse = reqwest::Client::new()
            .get(url)
            .basic_auth(&registry.username, Some(&registry.password))
            .send()
            .await
            .map_err(|error| error.to_string())?
            .error_for_status()
            .map_err(|error| error.to_string())?
            .json()
            .await
            .map_err(|error| error.to_string())?;
        Ok(value.tags)
    }

    pub async fn rotate_credentials(
        &self,
        id: i64,
        username: String,
        password: String,
    ) -> Result<Registries, String> {
        let current = self
            .get_by_id(id)
            .await
            .map_err(|error| error.to_string())?;
        self.test_connection_raw(&current.registry_url, &username, &password)
            .await?;
        self.patch(
            id,
            PatchRegistryDto {
                registry_name: None,
                image_prefix: None,
                username: Some(username),
                password: Some(password),
                registry_url: None,
                registry_type: None,
            },
        )
        .await
        .map_err(|error| error.to_string())
    }

    pub async fn usage(
        &self,
        id: i64,
    ) -> sqlx::Result<Vec<crate::flash_gen::registries::RegistriesUsageReferencesRow>> {
        self.get_by_id(id).await?;
        self.queries
            .registries_usage_references(&RegistriesUsageReferencesParams {
                registry_id: id,
                rollback_registry_id: id,
                build_registry_id: id,
            })
            .await
    }
}

impl RegistryService {
    async fn update_row(&self, id: i64, r: &Registries) -> sqlx::Result<()> {
        self.queries
            .registries_update(&RegistriesUpdateParams {
                registry_name: r.registry_name.clone(),
                image_prefix: r.image_prefix.clone().unwrap_or_default(),
                username: r.username.clone(),
                password: r.password.clone(),
                registry_url: r.registry_url.clone(),
                registry_type: r.registry_type.clone(),
                created_at: r.created_at,
                updated_at: r.updated_at,
                id,
            })
            .await
    }
}

fn api_url(base: &str, path: &str) -> String {
    let base = base.trim().trim_end_matches('/');
    let base = if base.starts_with("http://") || base.starts_with("https://") {
        base.to_owned()
    } else {
        format!("https://{base}")
    };
    format!("{base}{path}")
}
