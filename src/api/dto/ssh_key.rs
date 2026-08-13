use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::db::models::ssh_keys::SshKey;

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateSshKeyDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    #[validate(length(min = 1))]
    pub public_key: String,
    #[serde(default)]
    pub private_key: String,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct GenerateSshKeyDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub key_type: String, // "ed25519" or "rsa"
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct GeneratePairRequestDto {
    #[serde(default = "default_key_type")]
    pub key_type: String, // "ed25519" or "rsa"
}

fn default_key_type() -> String {
    "ed25519".into()
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct GeneratePairResponseDto {
    pub private_key: String,
    pub public_key: String,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchSshKeyDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    pub public_key: Option<String>,
    pub private_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct SshKeyResponseDto {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub public_key: String,
    pub private_key: Option<String>,
    pub has_private_key: bool,
    pub last_used_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<SshKey> for SshKeyResponseDto {
    fn from(value: SshKey) -> Self {
        let has_priv = !value.private_key.is_empty();
        let priv_key = if has_priv {
            Some(value.private_key)
        } else {
            None
        };
        Self {
            id: value.id.expect("persisted ssh key must have an id"),
            name: value.name,
            description: value.description,
            public_key: value.public_key,
            private_key: priv_key,
            has_private_key: has_priv,
            last_used_at: value.last_used_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
