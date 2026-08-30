use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, OperationResult, ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
    VaultService, VaultServiceExt,
};
use crate::services::vault::{
    CreateVaultProviderDto, UpdateVaultProviderDto, VaultProviderAssignmentDto, VaultProviderDto,
    VaultService as AppVaultService, VaultServiceError,
};

pub struct VaultController {
    service: Arc<AppVaultService>,
}

#[singleton]
impl VaultController {
    pub fn new(service: Arc<AppVaultService>) -> Self {
        Self { service }
    }
}

fn map_vault_error(error: VaultServiceError) -> ConnectError {
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

fn provider_to_resource(dto: VaultProviderDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "vault_provider".into(),
        payload,
        ..Default::default()
    }
}

fn assignment_to_resource(dto: VaultProviderAssignmentDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.project_id,
        kind: "vault_assignment".into(),
        payload,
        ..Default::default()
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl VaultService for VaultController {
    #[permission(Server, CanRead)]
    async fn list_providers<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let org_id = _query.organization_id.unwrap_or(0);
        let items = self
            .service
            .list_providers(org_id)
            .await
            .map_err(map_vault_error)?;
        let items = items.into_iter().map(provider_to_resource).collect();
        Response::ok(ResourceListResponse {
            items,
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn get_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let org_id = request.organization_id.unwrap_or(0);
        let provider = self
            .service
            .get_provider(request.id, org_id)
            .await
            .map_err(map_vault_error)?;
        Response::ok(provider_to_resource(provider))
    }

    #[permission(Server, CanCreate)]
    async fn create_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let org_id = request.organization_id.unwrap_or(0);
        let dto: CreateVaultProviderDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let provider = self
            .service
            .create_provider(org_id, dto)
            .await
            .map_err(map_vault_error)?;
        Response::ok(provider_to_resource(provider))
    }

    #[permission(Server, CanUpdate)]
    async fn update_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let org_id = request.organization_id.unwrap_or(0);
        let dto: UpdateVaultProviderDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let provider = self
            .service
            .update_provider(request.id, org_id, dto)
            .await
            .map_err(map_vault_error)?;
        Response::ok(provider_to_resource(provider))
    }

    #[permission(Server, CanDelete)]
    async fn delete_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let org_id = request.organization_id.unwrap_or(0);
        self.service
            .delete_provider(request.id, org_id)
            .await
            .map_err(map_vault_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Server, CanRead)]
    async fn test_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let org_id = request.organization_id.unwrap_or(0);
        let result = self
            .service
            .test_connection(request.id, org_id)
            .await
            .map_err(map_vault_error)?;
        Response::ok(OperationResult {
            success: result.success,
            operation: "test_provider".into(),
            message: Some(result.message),
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn test_connection<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: CreateVaultProviderDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let result = self
            .service
            .test_credentials(
                dto.provider_type,
                &dto.api_url,
                &dto.auth_token,
                dto.namespace,
                dto.config_json.as_deref(),
            )
            .await
            .map_err(map_vault_error)?;
        Response::ok(OperationResult {
            success: result.success,
            operation: "test_connection".into(),
            message: Some(result.message),
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn list_secrets<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let org_id = request.organization_id.unwrap_or(0);
        let result = self
            .service
            .list_secret_names(request.id, org_id)
            .await
            .map_err(map_vault_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        let items = vec![ResourceResponse {
            id: request.id,
            kind: "vault_secrets".into(),
            payload,
            ..Default::default()
        }];
        Response::ok(ResourceListResponse {
            items,
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn list_assignments<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let org_id = request.organization_id.unwrap_or(0);
        let assignments = self
            .service
            .get_assignments(request.id, org_id)
            .await
            .map_err(map_vault_error)?;
        let items = assignments.into_iter().map(assignment_to_resource).collect();
        Response::ok(ResourceListResponse {
            items,
            ..Default::default()
        })
    }

    #[permission(Server, CanUpdate)]
    async fn update_assignments<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let org_id = request.organization_id.unwrap_or(0);
        let assignments: Vec<VaultProviderAssignmentDto> = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let result = self
            .service
            .set_assignments(request.id, org_id, assignments)
            .await
            .map_err(map_vault_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "update_assignments".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }
}
