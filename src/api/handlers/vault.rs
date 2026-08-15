use crate::api::dto::vault::{
    CreateVaultProviderDto, UpdateVaultProviderDto, VaultProviderDto, VaultSecretListDto, VaultTestResultDto,
};
use crate::core::middleware::permission::{
    CanCreate, CanDelete, CanRead, CanUpdate, PermissionOrganization, RequirePermission, Server,
};
use crate::core::middleware::validator::ValidatedJson;
use crate::services::vault::{VaultService, VaultServiceError};
use auto_route::controller;
use axum::{
    Extension, Json,
    extract::Path,
    http::StatusCode,
};
use std::sync::Arc;

type ApiError = (StatusCode, String);

pub struct VaultController {
    service: Arc<VaultService>,
}

#[controller("/vault-providers")]
impl VaultController {
    fn new(service: Arc<VaultService>) -> Self {
        Self { service }
    }

    #[get]
    #[live(tables = ["vault_providers"])]
    async fn list(
        &self,
        RequirePermission(_, _): RequirePermission<Server, CanRead>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
    ) -> Result<Json<Vec<VaultProviderDto>>, ApiError> {
        self.service
            .list_providers(organization_id)
            .await
            .map(Json)
            .map_err(map_vault_error)
    }

    #[get("/{id}")]
    #[live(tables = ["vault_providers"])]
    async fn get(
        &self,
        RequirePermission(_, _): RequirePermission<Server, CanRead>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(id): Path<i64>,
    ) -> Result<Json<VaultProviderDto>, ApiError> {
        self.service
            .get_provider(id, organization_id)
            .await
            .map(Json)
            .map_err(map_vault_error)
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_, _): RequirePermission<Server, CanCreate>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        ValidatedJson(body): ValidatedJson<CreateVaultProviderDto>,
    ) -> Result<(StatusCode, Json<VaultProviderDto>), ApiError> {
        self.service
            .create_provider(organization_id, body)
            .await
            .map(|dto| (StatusCode::CREATED, Json(dto)))
            .map_err(map_vault_error)
    }

    #[put("/{id}")]
    async fn update(
        &self,
        RequirePermission(_, _): RequirePermission<Server, CanUpdate>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<UpdateVaultProviderDto>,
    ) -> Result<Json<VaultProviderDto>, ApiError> {
        self.service
            .update_provider(id, organization_id, body)
            .await
            .map(Json)
            .map_err(map_vault_error)
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_, _): RequirePermission<Server, CanDelete>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .delete_provider(id, organization_id)
            .await
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(map_vault_error)
    }

    #[post("/{id}/test")]
    async fn test(
        &self,
        RequirePermission(_, _): RequirePermission<Server, CanRead>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(id): Path<i64>,
    ) -> Result<Json<VaultTestResultDto>, ApiError> {
        self.service
            .test_connection(id, organization_id)
            .await
            .map(Json)
            .map_err(map_vault_error)
    }

    #[post("/test-connection")]
    async fn test_connection(
        &self,
        RequirePermission(_, _): RequirePermission<Server, CanRead>,
        ValidatedJson(body): ValidatedJson<CreateVaultProviderDto>,
    ) -> Result<Json<VaultTestResultDto>, ApiError> {
        self.service
            .test_credentials(&body.provider_type, &body.api_url, &body.auth_token, body.namespace)
            .await
            .map(Json)
            .map_err(map_vault_error)
    }

    #[get("/{id}/secrets")]
    async fn list_secrets(
        &self,
        RequirePermission(_, _): RequirePermission<Server, CanRead>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(id): Path<i64>,
    ) -> Result<Json<VaultSecretListDto>, ApiError> {
        self.service
            .list_secret_names(id, organization_id)
            .await
            .map(Json)
            .map_err(map_vault_error)
    }
}

fn map_vault_error(error: VaultServiceError) -> ApiError {
    match error {
        VaultServiceError::NotFound => (StatusCode::NOT_FOUND, "Vault provider not found".into()),
        VaultServiceError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)),
        VaultServiceError::Http(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("HTTP error: {}", e)),
        VaultServiceError::ProviderError(msg) => (StatusCode::BAD_REQUEST, msg),
    }
}
