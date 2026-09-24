use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    Registry as RegistryProto, RegistryUsageReference as RegistryUsageReferenceProto,
};
use crate::flash_gen::models::Registries;
use crate::flash_gen::registries::RegistriesUsageReferencesRow;

pub fn map_sqlx_error(error: sqlx::Error) -> ConnectError {
    match &error {
        sqlx::Error::RowNotFound => ConnectError::not_found("registry not found"),
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        // Validation and usage-conflict failures surface as Protocol errors;
        // the client supplied the offending value either way.
        sqlx::Error::Protocol(msg) => ConnectError::invalid_argument(msg.clone()),
        other => {
            tracing::error!(error = %other, "registry database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

pub fn registry_to_proto(registry: Registries) -> RegistryProto {
    RegistryProto {
        id: registry.id,
        registry_name: registry.registry_name,
        image_prefix: registry.image_prefix,
        username: registry.username,
        password: registry.password,
        registry_url: registry.registry_url,
        registry_type: registry.registry_type,
        created_at: registry.created_at,
        updated_at: registry.updated_at,
        ..Default::default()
    }
}

pub fn usage_to_proto(row: RegistriesUsageReferencesRow) -> RegistryUsageReferenceProto {
    RegistryUsageReferenceProto {
        resource_id: row.resource_id.map(|id| id.to_string()).unwrap_or_default(),
        resource_name: row.resource_name,
        resource_type: row.resource_type,
        usage_kind: row.usage_kind,
        ..Default::default()
    }
}
