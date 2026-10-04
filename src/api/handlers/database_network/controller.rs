use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    DatabaseNetworkCreateNetworkRequest, DatabaseNetworkCreateNetworkResponse,
    DatabaseNetworkDeleteNetworkRequest, DatabaseNetworkDependencies,
    DatabaseNetworkGetNetworkRequest, DatabaseNetworkGetNetworkResponse,
    DatabaseNetworkListDependenciesRequest, DatabaseNetworkListDependenciesResponse,
    DatabaseNetworkListNetworksRequest, DatabaseNetworkListNetworksResponse,
    DatabaseNetworkListServerNetworksRequest, DatabaseNetworkListServerNetworksResponse,
    DatabaseNetworkService, DatabaseNetworkServiceExt, DatabaseNetworkUpdateNetworkRequest,
    DatabaseNetworkUpdateNetworkResponse, Empty,
};
use crate::services::networking::database_network::{
    CreateDatabaseNetworkDto, PatchDatabaseNetworkDto,
};

use super::helper::*;
use super::init::DatabaseNetworkController;

#[auto_grpc::service(connect = true, auth = true)]
impl DatabaseNetworkService for DatabaseNetworkController {
    #[permission(Database, CanRead)]
    async fn list_networks<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, DatabaseNetworkListNetworksRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<DatabaseNetworkListNetworksResponse> + Send + use<'a>,
    > {
        let items = self.service.list().await.map_err(map_sqlx_error)?;
        let networks = items
            .into_iter()
            .map(crate::services::networking::database_network::DatabaseNetworkResponseDto::from)
            .map(network_to_proto)
            .collect();
        Response::ok(DatabaseNetworkListNetworksResponse { networks })
    }

    #[permission(Database, CanRead)]
    async fn list_server_networks<'a>(
        &'a self,
        ctx: RequestContext,
        query: ServiceRequest<'_, DatabaseNetworkListServerNetworksRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<DatabaseNetworkListServerNetworksResponse> + Send + use<'a>,
    > {
        let server_id = query.server_id;
        let items = self
            .service
            .list_by_server(Some(server_id))
            .await
            .map_err(map_sqlx_error)?;
        let networks = items
            .into_iter()
            .map(crate::services::networking::database_network::DatabaseNetworkResponseDto::from)
            .map(network_to_proto)
            .collect();
        Response::ok(DatabaseNetworkListServerNetworksResponse { networks })
    }

    #[permission(Database, CanRead)]
    async fn get_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DatabaseNetworkGetNetworkRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<DatabaseNetworkGetNetworkResponse> + Send + use<'a>>
    {
        let network = self
            .service
            .get_by_id(request.id)
            .await
            .map_err(map_sqlx_error)?;
        Response::ok(DatabaseNetworkGetNetworkResponse {
            network: ::buffa::MessageField::some(network_to_proto(
                crate::services::networking::database_network::DatabaseNetworkResponseDto::from(
                    network,
                ),
            )),
        })
    }

    #[permission(Database, CanCreate)]
    async fn create_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DatabaseNetworkCreateNetworkRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<DatabaseNetworkCreateNetworkResponse> + Send + use<'a>,
    > {
        let dto = CreateDatabaseNetworkDto {
            name: request.name.to_owned(),
            docker_network_name: request.docker_network_name.to_owned(),
            description: request.description.map(str::to_owned),
            external: request.external,
            server_id: request.server_id,
        };
        let created = self.service.create(dto).await.map_err(map_sqlx_error)?;
        Response::ok(DatabaseNetworkCreateNetworkResponse {
            network: ::buffa::MessageField::some(network_to_proto(
                crate::services::networking::database_network::DatabaseNetworkResponseDto::from(
                    created,
                ),
            )),
        })
    }

    #[permission(Database, CanCreate)]
    async fn update_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DatabaseNetworkUpdateNetworkRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<DatabaseNetworkUpdateNetworkResponse> + Send + use<'a>,
    > {
        let dto = PatchDatabaseNetworkDto {
            name: request.name.map(str::to_owned),
            docker_network_name: request.docker_network_name.map(str::to_owned),
            description: request.description.map(str::to_owned),
            external: request.external,
            server_id: request.server_id,
        };
        let updated = self
            .service
            .patch(request.id, dto)
            .await
            .map_err(map_sqlx_error)?;
        Response::ok(DatabaseNetworkUpdateNetworkResponse {
            network: ::buffa::MessageField::some(network_to_proto(
                crate::services::networking::database_network::DatabaseNetworkResponseDto::from(
                    updated,
                ),
            )),
        })
    }

    #[permission(Database, CanDelete)]
    async fn delete_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DatabaseNetworkDeleteNetworkRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .delete(request.id)
            .await
            .map_err(map_sqlx_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Database, CanRead)]
    async fn list_dependencies<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DatabaseNetworkListDependenciesRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<DatabaseNetworkListDependenciesResponse> + Send + use<'a>,
    > {
        let counts = self
            .service
            .dependencies(request.id)
            .await
            .map_err(map_sqlx_error)?;
        Response::ok(DatabaseNetworkListDependenciesResponse {
            dependencies: ::buffa::MessageField::some(DatabaseNetworkDependencies {
                applications: counts.applications,
                compose_services: counts.compose_services,
                databases: counts.databases,
            }),
        })
    }
}
