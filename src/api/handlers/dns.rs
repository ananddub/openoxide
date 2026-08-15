use crate::api::dto::dns::{
    CreateDnsProviderDto, DnsProviderDto, DnsTestResultDto, UpdateDnsProviderDto,
};
use crate::core::middleware::permission::{
    CanCreate, CanDelete, CanRead, CanUpdate, PermissionOrganization, RequirePermission, Server,
};
use crate::core::middleware::validator::ValidatedJson;
use crate::services::dns::{DnsService, DnsServiceError};
use auto_route::controller;
use axum::{
    Extension, Json,
    extract::Path,
    http::StatusCode,
};
use std::sync::Arc;

type ApiError = (StatusCode, String);

pub struct DnsController {
    service: Arc<DnsService>,
}

#[controller("/dns-providers")]
impl DnsController {
    fn new(service: Arc<DnsService>) -> Self {
        Self { service }
    }

    #[get]
    #[live(tables = ["dns_providers"])]
    async fn list(
        &self,
        RequirePermission(_, _): RequirePermission<Server, CanRead>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
    ) -> Result<Json<Vec<DnsProviderDto>>, ApiError> {
        self.service
            .list_providers(organization_id)
            .await
            .map(Json)
            .map_err(map_dns_error)
    }

    #[get("/{id}")]
    #[live(tables = ["dns_providers"])]
    async fn get(
        &self,
        RequirePermission(_, _): RequirePermission<Server, CanRead>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(id): Path<i64>,
    ) -> Result<Json<DnsProviderDto>, ApiError> {
        self.service
            .get_provider(id, organization_id)
            .await
            .map(Json)
            .map_err(map_dns_error)
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_, _): RequirePermission<Server, CanCreate>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        ValidatedJson(body): ValidatedJson<CreateDnsProviderDto>,
    ) -> Result<(StatusCode, Json<DnsProviderDto>), ApiError> {
        self.service
            .create_provider(organization_id, body)
            .await
            .map(|dto| (StatusCode::CREATED, Json(dto)))
            .map_err(map_dns_error)
    }

    #[put("/{id}")]
    async fn update(
        &self,
        RequirePermission(_, _): RequirePermission<Server, CanUpdate>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<UpdateDnsProviderDto>,
    ) -> Result<Json<DnsProviderDto>, ApiError> {
        self.service
            .update_provider(id, organization_id, body)
            .await
            .map(Json)
            .map_err(map_dns_error)
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
            .map_err(map_dns_error)
    }

    #[post("/{id}/test")]
    async fn test(
        &self,
        RequirePermission(_, _): RequirePermission<Server, CanRead>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(id): Path<i64>,
    ) -> Result<Json<DnsTestResultDto>, ApiError> {
        self.service
            .test_connection(id, organization_id)
            .await
            .map(Json)
            .map_err(map_dns_error)
    }

    #[post("/test-connection")]
    async fn test_connection(
        &self,
        RequirePermission(_, _): RequirePermission<Server, CanRead>,
        ValidatedJson(body): ValidatedJson<CreateDnsProviderDto>,
    ) -> Result<Json<DnsTestResultDto>, ApiError> {
        self.service
            .test_credentials(&body.provider_type, &body.credentials_json)
            .await
            .map(Json)
            .map_err(map_dns_error)
    }
}

fn map_dns_error(error: DnsServiceError) -> ApiError {
    match error {
        DnsServiceError::NotFound => (StatusCode::NOT_FOUND, "DNS provider not found".into()),
        DnsServiceError::Database(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        DnsServiceError::Http(err) => (StatusCode::BAD_GATEWAY, err.to_string()),
        DnsServiceError::ProviderError(msg) => (StatusCode::BAD_REQUEST, msg),
    }
}
