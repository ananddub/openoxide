use std::sync::Arc;

use auto_di::singleton;
use serde::Deserialize;

use crate::{
    db::{
        models::ssh_keys::SshKey,
        queries::FlashQueries,
    },
    flash_gen::{
        models::SshKeys,
        ssh_keys::{SshKeysCreateAndReturnParams, SshKeysUpdateAndReturnParams},
    },
    utils::ssh,
};

#[derive(Debug, Deserialize)]
pub struct CreateSshKeyDto {
    pub name: String,
    pub description: Option<String>,
    pub public_key: String,
    #[serde(default)]
    pub private_key: String,
}

#[derive(Debug, Deserialize)]
pub struct PatchSshKeyDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub public_key: Option<String>,
    pub private_key: Option<String>,
}

fn to_model(row: SshKeys) -> SshKey {
    SshKey {
        id: row.id,
        name: row.name,
        description: row.description,
        private_key: row.private_key,
        public_key: row.public_key,
        last_used_at: row.last_used_at,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}

pub struct SshKeyService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl SshKeyService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<SshKey> {
        let row = self.queries.ssh_keys_get_by_id(id).await?;
        Ok(to_model(row))
    }

    pub async fn list(&self) -> sqlx::Result<Vec<SshKey>> {
        let rows = self.queries.ssh_keys_list_ordered().await?;
        Ok(rows.into_iter().map(to_model).collect())
    }

    pub async fn create(&self, input: CreateSshKeyDto) -> sqlx::Result<SshKey> {
        let now = chrono::Utc::now().timestamp();
        let row = self
            .queries
            .ssh_keys_create_and_return(&SshKeysCreateAndReturnParams {
                name: input.name,
                description: input.description.unwrap_or_default(),
                private_key: input.private_key,
                public_key: input.public_key,
                created_at: now,
                updated_at: now,
            })
            .await?;
        Ok(to_model(row))
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

        let now = chrono::Utc::now().timestamp();
        let row = self
            .queries
            .ssh_keys_create_and_return(&SshKeysCreateAndReturnParams {
                name,
                description: description.unwrap_or_default(),
                private_key,
                public_key,
                created_at: now,
                updated_at: now,
            })
            .await?;
        Ok(to_model(row))
    }

    pub async fn patch(&self, id: i64, input: PatchSshKeyDto) -> sqlx::Result<SshKey> {
        let current = self.get_by_id(id).await?;
        let name = input.name.unwrap_or(current.name);
        let description = input.description.or(current.description);
        let private_key = input.private_key.unwrap_or(current.private_key);
        let public_key = input.public_key.unwrap_or(current.public_key);

        let row = self
            .queries
            .ssh_keys_update_and_return(&SshKeysUpdateAndReturnParams {
                name,
                description: description.unwrap_or_default(),
                private_key,
                public_key,
                id,
            })
            .await?;
        Ok(to_model(row))
    }

    pub async fn mark_used(&self, id: i64) -> sqlx::Result<SshKey> {
        let row = self.queries.ssh_keys_touch_and_return(id).await?;
        Ok(to_model(row))
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        let server_use = self.queries.ssh_keys_server_use_count(id).await?;
        let git_use = self.queries.ssh_keys_git_use_count(id).await?;
        if server_use > 0 || git_use > 0 {
            return Err(sqlx::Error::Protocol(
                "Cannot delete SSH Key: It is currently in use by an active server, application, or compose stack.".into(),
            ));
        }
        self.queries.ssh_keys_delete(id).await
    }
}
