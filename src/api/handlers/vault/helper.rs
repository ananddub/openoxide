use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    VaultProvider as VaultProviderProto, VaultProviderAssignment as VaultProviderAssignmentProto,
    VaultProviderType as ProtoVaultProviderType,
};
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
        VaultServiceError::ProviderError(msg) => ConnectError::invalid_argument(msg),
    }
}

pub fn provider_type_to_proto(value: VaultProviderType) -> ProtoVaultProviderType {
    match value {
        VaultProviderType::Hashicorp => ProtoVaultProviderType::Hashicorp,
        VaultProviderType::Infisical => ProtoVaultProviderType::Infisical,
        VaultProviderType::Doppler => ProtoVaultProviderType::Doppler,
        VaultProviderType::Aws => ProtoVaultProviderType::Aws,
        VaultProviderType::AwsParameterStore => ProtoVaultProviderType::AwsParameterStore,
        VaultProviderType::Scaleway => ProtoVaultProviderType::Scaleway,
        VaultProviderType::Azure => ProtoVaultProviderType::Azure,
    }
}

pub fn proto_to_provider_type(
    value: ::buffa::EnumValue<ProtoVaultProviderType>,
) -> Result<VaultProviderType, ConnectError> {
    match value {
        ::buffa::EnumValue::Known(ProtoVaultProviderType::Hashicorp) => {
            Ok(VaultProviderType::Hashicorp)
        }
        ::buffa::EnumValue::Known(ProtoVaultProviderType::Infisical) => {
            Ok(VaultProviderType::Infisical)
        }
        ::buffa::EnumValue::Known(ProtoVaultProviderType::Doppler) => Ok(VaultProviderType::Doppler),
        ::buffa::EnumValue::Known(ProtoVaultProviderType::Aws) => Ok(VaultProviderType::Aws),
        ::buffa::EnumValue::Known(ProtoVaultProviderType::AwsParameterStore) => {
            Ok(VaultProviderType::AwsParameterStore)
        }
        ::buffa::EnumValue::Known(ProtoVaultProviderType::Scaleway) => {
            Ok(VaultProviderType::Scaleway)
        }
        ::buffa::EnumValue::Known(ProtoVaultProviderType::Azure) => Ok(VaultProviderType::Azure),
        _ => Err(ConnectError::invalid_argument("unsupported Vault provider type")),
    }
}

pub fn assignment_to_proto(dto: VaultProviderAssignmentDto) -> VaultProviderAssignmentProto {
    VaultProviderAssignmentProto {
        project_id: dto.project_id,
        environment_ids: dto.environment_ids,
        ..Default::default()
    }
}

pub fn provider_to_proto(dto: VaultProviderDto) -> VaultProviderProto {
    VaultProviderProto {
        id: dto.id,
        name: dto.name,
        provider_type: ::buffa::EnumValue::Known(provider_type_to_proto(dto.provider_type)),
        api_url: dto.api_url,
        auth_token: dto.auth_token,
        namespace: dto.namespace,
        config_json: dto.config_json,
        organization_id: dto.organization_id,
        created_at: dto.created_at,
        updated_at: dto.updated_at,
        assignments: dto.assignments.into_iter().map(assignment_to_proto).collect(),
        ..Default::default()
    }
}
