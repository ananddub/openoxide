use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::ResourceResponse;
use crate::flash_gen::models::SshKeys;
use serde::{Deserialize, Serialize};

pub fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("ssh key not found"),
        sqlx::Error::Protocol(msg) => ConnectError::invalid_argument(msg.clone()),
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => {
            ConnectError::new(ErrorCode::FailedPrecondition,
                "Cannot delete SSH Key: It is currently assigned to active servers or git repositories.")
        }
        other => {
            tracing::error!(error = %other, "ssh key database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

pub fn key_to_resource(dto: SshKeyResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "ssh_key".into(),
        payload,
        ..Default::default()
    }
}

// ── Helper DTOs for deserializing ResourceRequest payloads ────────────

#[derive(Debug, Deserialize)]
pub struct GenerateSshKeyDto {
    pub name: String,
    pub description: Option<String>,
    pub key_type: String, // "ed25519" or "rsa"
}

#[derive(Debug, Deserialize)]
pub struct GeneratePairRequestDto {
    #[serde(default = "default_key_type")]
    pub key_type: String, // "ed25519" or "rsa"
}

pub fn default_key_type() -> String {
    "ed25519".into()
}

#[derive(Debug, Clone, Serialize)]
pub struct GeneratePairResponseDto {
    pub private_key: String,
    pub public_key: String,
}

#[derive(Debug, Clone, Serialize)]
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

impl From<SshKeys> for SshKeyResponseDto {
    fn from(value: SshKeys) -> Self {
        let has_priv = !value.private_key.is_empty();
        let priv_key = if has_priv {
            Some(value.private_key)
        } else {
            None
        };
        Self {
            id: value.id,
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
