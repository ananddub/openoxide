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
        // Check existence
        self.get_by_id(id).await?;
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

        let res = docker.login(registry, username, password).await;
        match res {
            Ok(output) => {
                if output.success() {
                    let _ = docker.logout(registry).await;
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
}
