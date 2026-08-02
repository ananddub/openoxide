use std::sync::Arc;

use auto_di::singleton;

use crate::utils::ssh;
use crate::{
    api::dto::ssh_key::{CreateSshKeyDto, PatchSshKeyDto},
    db::models::ssh_keys::SshKey,
    repository::SshKeyRepository,
};

pub struct SshKeyService {
    repo_ssh: Arc<SshKeyRepository>,
}

#[singleton]
impl SshKeyService {
    fn new(repo_ssh: Arc<SshKeyRepository>) -> Self {
        Self { repo_ssh }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<SshKey> {
        self.repo_ssh
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn list(&self) -> sqlx::Result<Vec<SshKey>> {
        self.repo_ssh.list_ordered().await
    }

    pub async fn create(&self, input: CreateSshKeyDto) -> sqlx::Result<SshKey> {
        self.repo_ssh
            .create_and_return(
                input.name,
                input.description,
                input.private_key,
                input.public_key,
            )
            .await
    }

    pub async fn generate(
        &self,
        name: String,
        description: Option<String>,
        key_type: &str,
    ) -> sqlx::Result<SshKey> {
        let kt = key_type.to_lowercase();
        if kt != "ed25519" && kt != "rsa" {
            return Err(sqlx::Error::Configuration(
                "Invalid key type. Supported types are 'ed25519' and 'rsa'".into(),
            ));
        }
        let (private_key, public_key) =
            ssh::generate_keypair(&kt).map_err(|e| sqlx::Error::Configuration(e.into()))?;

        self.repo_ssh
            .create_and_return(name, description, private_key, public_key)
            .await
    }

    pub async fn patch(&self, id: i64, input: PatchSshKeyDto) -> sqlx::Result<SshKey> {
        let current = self.get_by_id(id).await?;
        let name = input.name.unwrap_or(current.name);
        let description = input.description.or(current.description);
        let private_key = input.private_key.unwrap_or(current.private_key);
        let public_key = input.public_key.unwrap_or(current.public_key);

        self.repo_ssh
            .update_and_return(id, name, description, private_key, public_key)
            .await
    }

    pub async fn mark_used(&self, id: i64) -> sqlx::Result<SshKey> {
        self.repo_ssh.touch_and_return(id).await
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        if self.repo_ssh.is_in_use(id).await? {
            return Err(sqlx::Error::Protocol(
                "Cannot delete SSH Key: It is currently in use by an active server, application, or compose stack.".into(),
            ));
        }
        self.repo_ssh.delete(id).await
    }
}
