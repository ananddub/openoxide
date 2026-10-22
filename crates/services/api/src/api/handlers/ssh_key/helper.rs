use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    SshKey as SshKeyProto, SshKeyType as ProtoSshKeyType,
};
use crate::flash_gen::models::SshKeys;

pub fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("ssh key not found"),
        sqlx::Error::Protocol(msg) => ConnectError::invalid_argument(msg.clone()),
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => ConnectError::new(
            ErrorCode::FailedPrecondition,
            "Cannot delete SSH Key: It is currently assigned to active servers or git repositories.",
        ),
        other => {
            tracing::error!(error = %other, "ssh key database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

pub fn key_to_proto(row: SshKeys) -> SshKeyProto {
    let has_priv = !row.private_key.is_empty();
    let private_key = if has_priv {
        Some(row.private_key)
    } else {
        None
    };
    SshKeyProto {
        id: row.id,
        name: row.name,
        description: row.description,
        public_key: row.public_key,
        private_key,
        has_private_key: has_priv,
        last_used_at: row.last_used_at,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}

/// Maps the typed key-type enum to the lowercase string accepted by the
/// SSH key service and `utils::ssh::generate_keypair`.
pub fn key_type_to_str(
    value: ::buffa::EnumValue<ProtoSshKeyType>,
) -> Result<&'static str, ConnectError> {
    match value {
        ::buffa::EnumValue::Known(ProtoSshKeyType::Ed25519) => Ok("ed25519"),
        ::buffa::EnumValue::Known(ProtoSshKeyType::Rsa) => Ok("rsa"),
        _ => Err(ConnectError::invalid_argument(
            "unsupported key type; supported: ED25519, RSA",
        )),
    }
}
