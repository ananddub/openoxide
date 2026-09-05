use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::ResourceResponse;
use crate::flash_gen::models::VaultProviders;
use crate::services::integrations::vault::{
    VAULT_MASK_TOKEN, VaultProviderAssignmentDto, VaultProviderDto, VaultProviderType,
    VaultServiceError,
};

/// Mirror of `VaultService::into_dto` (private to the service module) so
/// realtime streams can map raw `VaultProviders` rows into the same response
/// DTO the REST list/get endpoints return (assignments extracted from
/// `config_json["_assignments"]`, auth token masked).
pub fn vault_provider_dto_from_model(p: VaultProviders) -> VaultProviderDto {
    let assignments = p
        .config_json
        .as_deref()
        .and_then(|value| serde_json::from_str::<serde_json::Value>(value).ok())
        .and_then(|value| serde_json::from_value(value["_assignments"].clone()).ok())
        .unwrap_or_default();

    VaultProviderDto {
        id: p.id,
        name: p.name,
        provider_type: p
            .provider_type
            .parse()
            .unwrap_or(VaultProviderType::Hashicorp),
        api_url: p.api_url,
        auth_token: if p.auth_token.is_empty() {
            String::new()
        } else {
            VAULT_MASK_TOKEN.to_string()
        },
        namespace: p.namespace,
        config_json: p.config_json,
        organization_id: p.organization_id,
        created_at: p.created_at,
        updated_at: p.updated_at,
        assignments,
    }
}

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
