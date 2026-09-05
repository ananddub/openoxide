use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};
use futures::StreamExt;

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, OperationResult, ResourceRequest, ResourceResponse,
    ServerPrivateNetworkService, ServerPrivateNetworkServiceExt,
};
use crate::db::reactive::realtime;
use crate::flash_gen::models::ServerPrivateNetworks;
use crate::services::infrastructure::server::ServerPrivateNetworkService as AppServerPrivateNetworkService;

use super::init::ServerPrivateNetworkController;

fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("server private network not found"),
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => {
            tracing::warn!(error = %e.message(), "foreign key violation");
            ConnectError::not_found("referenced resource not found")
        }
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            tracing::warn!(error = %e.message(), "unique constraint violation");
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        other => {
            tracing::error!(error = %other, "server private network database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

fn network_to_resource(server_id: i64, network: &impl serde::Serialize) -> ResourceResponse {
    let payload = serde_json::to_string(network).unwrap_or_default();
    ResourceResponse {
        id: server_id,
        kind: "server_private_network".into(),
        payload,
        ..Default::default()
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl ServerPrivateNetworkService for ServerPrivateNetworkController {
    #[permission(Server, CanRead)]
    async fn get_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let network = self
            .service
            .get(request.id)
            .await
            .map_err(map_db_error)?;
        match network {
            Some(dto) => Response::ok(network_to_resource(dto.server_id, &dto)),
            None => Response::ok(ResourceResponse {
                id: request.id,
                kind: "server_private_network".into(),
                payload: String::new(),
                ..Default::default()
            }),
        }
    }

    #[permission(Server, CanRead)]
    async fn stream_network(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceResponse> + Send + use<>,
        >,
    > {
        let id = request.id;
        let stream = {
            let sql = format!("SELECT * FROM server_private_networks WHERE server_id = {id}");
            realtime()
                .watch_query::<ServerPrivateNetworks>(sql)
                .await
                .map_err(map_db_error)?
        };
        Response::stream_ok(stream.map(move |mut rows| match rows.pop() {
            Some(network) => Ok(network_to_resource(network.server_id, &network)),
            None => Ok(ResourceResponse {
                id,
                kind: "server_private_network".into(),
                payload: String::new(),
                ..Default::default()
            }),
        }))
    }

    #[permission(Server, CanCreate)]
    async fn update_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto = AppServerPrivateNetworkService::parse_update_payload(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let network = self
            .service
            .update(request.id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(network_to_resource(network.server_id, &network))
    }

    #[permission(Server, CanCreate)]
    async fn delete_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .disable(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Server, CanCreate)]
    async fn setup<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let network = self
            .service
            .setup_transport(request.id)
            .await
            .map_err(map_db_error)?;
        let payload = serde_json::to_string(&network).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "setup".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn health<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let health = self
            .service
            .health(request.id)
            .await
            .map_err(map_db_error)?;
        let payload = serde_json::to_string(&health).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "health".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn repair<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let network = self
            .service
            .repair_transport(request.id)
            .await
            .map_err(map_db_error)?;
        let payload = serde_json::to_string(&network).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "repair".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn re_setup<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let network = self
            .service
            .re_setup_transport(request.id)
            .await
            .map_err(map_db_error)?;
        let payload = serde_json::to_string(&network).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "re_setup".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn rotate_keys<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let network = self
            .service
            .rotate_wireguard(request.id)
            .await
            .map_err(map_db_error)?;
        let payload = serde_json::to_string(&network).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "rotate_keys".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn teardown<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        self.service
            .teardown_transport(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(OperationResult {
            success: true,
            operation: "teardown".into(),
            ..Default::default()
        })
    }
}
