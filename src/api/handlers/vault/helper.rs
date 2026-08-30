use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::ResourceResponse;
use crate::services::integrations::vault::{VaultProviderAssignmentDto, VaultProviderDto, VaultServiceError};

pub fn map_vault_error(error: VaultServiceError) -> ConnectError {
    match error {
        VaultServiceError::NotFound => ConnectError::not_found("Vault provider not found"),
        VaultServiceError::Database(e) => match &e {
            sqlx::Error::RowNotFound => ConnectError::not_found("Vault provider not found"),
            sqlx::Error::Database(db) if db.is_unique_violation() => {
                ConnectError::new(ErrorCode::AlreadyExists, db.message())
            }
            _ => {
                tracing::error!(error = %e, "vault database operation failed");
                ConnectError::new(ErrorCode::Internal, "database operation failed")
            }
        },
        VaultServiceError::Http(e) => {
            tracing::error!(error = %e, "vault HTTP request failed");
            ConnectError::new(ErrorCode::Internal, "HTTP request failed")
        }
        VaultServiceError::ProviderError(msg) => {
            ConnectError::invalid_argument(msg)
        }
    }
}

pub fn provider_to_resource(dto: VaultProviderDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "vault_provider".into(),
        payload,
        ..Default::default()
    }
}

pub fn assignment_to_resource(dto: VaultProviderAssignmentDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.project_id,
        kind: "vault_assignment".into(),
        payload,
        ..Default::default()
    }
}
