use crate::{
    api::dto::registry::{CreateRegistryDto, PatchRegistryDto},
    db::models::registries::Registry,
    db::repository::registries::RegistryRepository,
};
use auto_di::singleton;
use std::sync::Arc;

pub struct RegistryService {
    repo_reg: Arc<RegistryRepository>,
}

#[singleton]
impl RegistryService {
    fn new(repo_reg: Arc<RegistryRepository>) -> Self {
        Self { repo_reg }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Registry> {
        self.repo_reg
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn list(&self) -> sqlx::Result<Vec<Registry>> {
        self.repo_reg.get_all().await
    }

    pub async fn create(&self, input: CreateRegistryDto) -> sqlx::Result<Registry> {
        let now = chrono::Utc::now().timestamp();
        let item = Registry {
            id: None,
            registry_name: input.registry_name,
            image_prefix: input.image_prefix,
            username: input.username,
            password: input.password,
            registry_url: input.registry_url,
            registry_type: input.registry_type,
            created_at: now,
            updated_at: now,
        };
        let new_id = self.repo_reg.create(&item).await?;
        self.repo_reg
            .get_by_id(new_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn patch(&self, id: i64, input: PatchRegistryDto) -> sqlx::Result<Registry> {
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

        self.repo_reg.update(id, &current).await?;
        self.repo_reg
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        let usage = self.repo_reg.usage_references(id).await?;
        if !usage.is_empty() {
            return Err(sqlx::Error::Protocol(format!(
                "registry is used by {} application configuration(s)",
                usage.len()
            )));
        }
        self.repo_reg.delete(id).await
    }

    pub async fn test_connection_raw(
        &self,
        registry_url: &str,
        username: &str,
        password: &str,
    ) -> Result<(), String> {
        let docker = crate::utils::docker::DockerCli::new_local();
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
        let value: serde_json::Value = reqwest::Client::new()
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
        Ok(value
            .get("repositories")
            .and_then(|value| value.as_array())
            .into_iter()
            .flatten()
            .filter_map(|value| value.as_str().map(str::to_owned))
            .collect())
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
        let value: serde_json::Value = reqwest::Client::new()
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
        Ok(value
            .get("tags")
            .and_then(|value| value.as_array())
            .into_iter()
            .flatten()
            .filter_map(|value| value.as_str().map(str::to_owned))
            .collect())
    }

    pub async fn rotate_credentials(
        &self,
        id: i64,
        username: String,
        password: String,
    ) -> Result<Registry, String> {
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
    ) -> sqlx::Result<Vec<crate::db::repository::registries::RegistryUsage>> {
        self.get_by_id(id).await?;
        self.repo_reg.usage_references(id).await
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
