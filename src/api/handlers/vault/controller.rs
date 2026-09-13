use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};
use futures::StreamExt;

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, OperationResult, ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
    VaultService, VaultServiceExt,
};
use crate::db::reactive::realtime;
use crate::flash_gen::models::VaultProviders;
use crate::services::integrations::vault::{
    CreateVaultProviderDto, UpdateVaultProviderDto, VaultProviderAssignmentDto,
};

use super::helper::*;
use super::init::VaultController;

#[auto_grpc::service(connect = true, auth = true)]
impl VaultService for VaultController {
    #[permission(Server, CanRead)]
    async fn stream_list_providers(
        &self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceListResponse> + Send + use<>,
        >,
    > {
        let organization_id = auth.organization_id;
        let stream = {
            let sql = format!(
                "SELECT * FROM vault_providers WHERE organization_id = {organization_id}"
            );
            realtime()
                .watch_query::<VaultProviders>(sql)
                .await
                .map_err(|e| map_vault_error(e.into()))?
        };
        Response::stream_ok(stream.map(move |rows| {
            Ok(ResourceListResponse {
                items: rows
                    .into_iter()
                    .map(vault_provider_dto_from_model)
                    .map(provider_to_resource)
                    .collect(),
                ..Default::default()
            })
        }))
    }

    #[permission(Server, CanRead)]
    async fn stream_provider(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceResponse> + Send + use<>,
        >,
    > {
        let organization_id = auth.organization_id;
        let id = request.id;
        let stream = {
            let sql = format!(
                "SELECT * FROM vault_providers WHERE id = {id} AND organization_id = {organization_id}"
            );
            realtime()
                .watch_query::<VaultProviders>(sql)
                .await
                .map_err(|e| map_vault_error(e.into()))?
        };
        Response::stream_ok(stream.map(move |mut rows| match rows.pop() {
            Some(item) => Ok(provider_to_resource(vault_provider_dto_from_model(item))),
            None => Err(ConnectError::not_found("Vault provider not found")),
        }))
    }

    #[permission(Server, CanRead)]
    async fn list_providers<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
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
        let org_id = auth.organization_id;
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
        let org_id = auth.organization_id;
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
        let org_id = auth.organization_id;
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
        let org_id = auth.organization_id;
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
        let org_id = auth.organization_id;
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
        let org_id = auth.organization_id;
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
        let org_id = auth.organization_id;
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
        let org_id = auth.organization_id;
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
