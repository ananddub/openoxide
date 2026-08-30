use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    DatabaseNetworkService, DatabaseNetworkServiceExt, Empty, ResourceListResponse,
    ResourceQuery, ResourceRequest, ResourceResponse,
};
use crate::services::database_network::{
    CreateDatabaseNetworkDto, DatabaseNetworkResponseDto, PatchDatabaseNetworkDto,
};

use super::helper::*;
use super::init::DatabaseNetworkController;

#[auto_grpc::service(connect = true, auth = true)]
impl DatabaseNetworkService for DatabaseNetworkController {
    #[permission(Database, CanRead)]
    async fn list_networks<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let items = self.service.list().await.map_err(map_sqlx_error)?;
        let items = items.into_iter().map(DatabaseNetworkResponseDto::from).map(network_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Database, CanRead)]
    async fn list_server_networks<'a>(
        &'a self,
        ctx: RequestContext,
        query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let server_id = query.organization_id.unwrap_or(0);
        let items = self.service.list_by_server(Some(server_id)).await.map_err(map_sqlx_error)?;
        let items = items.into_iter().map(DatabaseNetworkResponseDto::from).map(network_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Database, CanRead)]
    async fn get_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let network = self.service.get_by_id(request.id).await.map_err(map_sqlx_error)?;
        Response::ok(network_to_resource(DatabaseNetworkResponseDto::from(network)))
    }

    #[permission(Database, CanCreate)]
    async fn create_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateDatabaseNetworkDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let created = self.service.create(dto).await.map_err(map_sqlx_error)?;
        Response::ok(network_to_resource(DatabaseNetworkResponseDto::from(created)))
    }

    #[permission(Database, CanCreate)]
    async fn update_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: PatchDatabaseNetworkDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let updated = self.service.patch(request.id, dto).await.map_err(map_sqlx_error)?;
        Response::ok(network_to_resource(DatabaseNetworkResponseDto::from(updated)))
    }

    #[permission(Database, CanDelete)]
    async fn delete_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.delete(request.id).await.map_err(map_sqlx_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Database, CanRead)]
    async fn list_dependencies<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let counts = self.service.dependencies(request.id).await.map_err(map_sqlx_error)?;
        let payload = serde_json::to_string(&counts).unwrap_or_default();
        let item = ResourceResponse {
            id: request.id,
            kind: "network_dependency_counts".into(),
            payload,
            ..Default::default()
        };
        Response::ok(ResourceListResponse { items: vec![item], ..Default::default() })
    }
}
