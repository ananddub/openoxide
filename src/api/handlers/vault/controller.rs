use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::__buffa::view::VaultProviderAssignmentView;
use crate::api::connect::generated::openoxide::api::v1::{
    CreateVaultProviderRequest, CreateVaultProviderResponse, DeleteVaultProviderRequest, Empty,
    GetVaultProviderRequest, GetVaultProviderResponse, ListVaultAssignmentsRequest,
    ListVaultAssignmentsResponse, ListVaultProvidersRequest, ListVaultProvidersResponse,
    ListVaultSecretsRequest, ListVaultSecretsResponse, TestVaultConnectionRequest,
    TestVaultConnectionResponse, TestVaultProviderRequest, TestVaultProviderResponse,
    UpdateVaultAssignmentsRequest, UpdateVaultAssignmentsResponse, UpdateVaultProviderRequest,
    UpdateVaultProviderResponse, VaultService, VaultServiceExt,
};
use crate::services::integrations::vault::{
    CreateVaultProviderDto, UpdateVaultProviderDto, VaultProviderAssignmentDto,
};

use super::helper::*;
use super::init::VaultController;

fn assignments_to_dto(
    views: &[VaultProviderAssignmentView<'_>],
) -> Vec<VaultProviderAssignmentDto> {
    views
        .iter()
        .map(|a| VaultProviderAssignmentDto {
            project_id: a.project_id,
            environment_ids: a.environment_ids.to_vec(),
        })
        .collect()
}

#[auto_grpc::service(connect = true, auth = true)]
impl VaultService for VaultController {
    #[permission(Server, CanRead)]
    async fn list_providers<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ListVaultProvidersRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListVaultProvidersResponse> + Send + use<'a>>
    {
        let org_id = auth.organization_id;
        let items = self
            .service
            .list_providers(org_id)
            .await
            .map_err(map_vault_error)?;
        let providers = items.into_iter().map(provider_to_proto).collect();
        Response::ok(ListVaultProvidersResponse { providers })
    }

    #[permission(Server, CanRead)]
    async fn get_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetVaultProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GetVaultProviderResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let provider = self
            .service
            .get_provider(request.id, org_id)
            .await
            .map_err(map_vault_error)?;
        Response::ok(GetVaultProviderResponse {
            provider: ::buffa::MessageField::some(provider_to_proto(provider)),
        })
    }

    #[permission(Server, CanCreate)]
    async fn create_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateVaultProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<CreateVaultProviderResponse> + Send + use<'a>>
    {
        let org_id = auth.organization_id;
        let dto = CreateVaultProviderDto {
            name: request.name.to_owned(),
            provider_type: proto_to_provider_type(request.provider_type)?,
            api_url: request.api_url.to_owned(),
            auth_token: request.auth_token.to_owned(),
            namespace: request.namespace.map(str::to_owned),
            config_json: request.config_json.map(str::to_owned),
            assignments: assignments_to_dto(&request.assignments),
        };
        let provider = self
            .service
            .create_provider(org_id, dto)
            .await
            .map_err(map_vault_error)?;
        Response::ok(CreateVaultProviderResponse {
            provider: ::buffa::MessageField::some(provider_to_proto(provider)),
        })
    }

    #[permission(Server, CanUpdate)]
    async fn update_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateVaultProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<UpdateVaultProviderResponse> + Send + use<'a>>
    {
        let org_id = auth.organization_id;
        let dto = UpdateVaultProviderDto {
            name: request.name.map(str::to_owned),
            api_url: request.api_url.map(str::to_owned),
            auth_token: request.auth_token.map(str::to_owned),
            namespace: request.namespace.map(str::to_owned),
            config_json: request.config_json.map(str::to_owned),
            assignments: request
                .assignments
                .as_option()
                .map(|update| assignments_to_dto(&update.assignments)),
        };
        let provider = self
            .service
            .update_provider(request.id, org_id, dto)
            .await
            .map_err(map_vault_error)?;
        Response::ok(UpdateVaultProviderResponse {
            provider: ::buffa::MessageField::some(provider_to_proto(provider)),
        })
    }

    #[permission(Server, CanDelete)]
    async fn delete_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteVaultProviderRequest>,
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
        request: ServiceRequest<'_, TestVaultProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TestVaultProviderResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let result = self
            .service
            .test_connection(request.id, org_id)
            .await
            .map_err(map_vault_error)?;
        Response::ok(TestVaultProviderResponse {
            success: result.success,
            message: result.message,
        })
    }

    #[permission(Server, CanRead)]
    async fn test_connection<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, TestVaultConnectionRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TestVaultConnectionResponse> + Send + use<'a>>
    {
        let provider_type = proto_to_provider_type(request.provider_type)?;
        let api_url = request.api_url.to_owned();
        let auth_token = request.auth_token.to_owned();
        let result = self
            .service
            .test_credentials(
                provider_type,
                &api_url,
                &auth_token,
                request.namespace.map(str::to_owned),
                request.config_json,
            )
            .await
            .map_err(map_vault_error)?;
        Response::ok(TestVaultConnectionResponse {
            success: result.success,
            message: result.message,
        })
    }

    #[permission(Server, CanRead)]
    async fn list_secrets<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListVaultSecretsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListVaultSecretsResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let result = self
            .service
            .list_secret_names(request.provider_id, org_id)
            .await
            .map_err(map_vault_error)?;
        Response::ok(ListVaultSecretsResponse {
            secrets: result.secrets,
        })
    }

    #[permission(Server, CanRead)]
    async fn list_assignments<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListVaultAssignmentsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListVaultAssignmentsResponse> + Send + use<'a>>
    {
        let org_id = auth.organization_id;
        let assignments = self
            .service
            .get_assignments(request.provider_id, org_id)
            .await
            .map_err(map_vault_error)?;
        let assignments = assignments.into_iter().map(assignment_to_proto).collect();
        Response::ok(ListVaultAssignmentsResponse { assignments })
    }

    #[permission(Server, CanUpdate)]
    async fn update_assignments<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateVaultAssignmentsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<UpdateVaultAssignmentsResponse> + Send + use<'a>>
    {
        let org_id = auth.organization_id;
        let assignments = assignments_to_dto(&request.assignments);
        let result = self
            .service
            .set_assignments(request.provider_id, org_id, assignments)
            .await
            .map_err(map_vault_error)?;
        let assignments = result.into_iter().map(assignment_to_proto).collect();
        Response::ok(UpdateVaultAssignmentsResponse { assignments })
    }
}
