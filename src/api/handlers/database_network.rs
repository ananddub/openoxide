use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    DatabaseNetworkService, DatabaseNetworkServiceExt, Empty, ResourceListResponse,
    ResourceQuery, ResourceRequest, ResourceResponse,
};
use crate::services::database_network::{
    CreateDatabaseNetworkDto, DatabaseNetworkResponseDto, PatchDatabaseNetworkDto,
    DatabaseNetworkService as AppDatabaseNetworkService,
};

pub struct DatabaseNetworkController {
    service: Arc<AppDatabaseNetworkService>,
}

#[singleton]
impl DatabaseNetworkController {
    pub fn new(service: Arc<AppDatabaseNetworkService>) -> Self {
        Self { service }
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ConnectError {
    match &error {
        sqlx::Error::RowNotFound => ConnectError::not_found("database network not found"),
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Protocol(msg) => ConnectError::new(ErrorCode::AlreadyExists, msg.clone()),
        other => {
            tracing::error!(error = %other, "database network operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

fn network_to_resource(dto: DatabaseNetworkResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "database_network".into(),
        payload,
        ..Default::default()
    }
}

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
