use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};
use futures::StreamExt;
use validator::Validate;

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, OperationResult, ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
    RemoteServerService, RemoteServerServiceExt,
};
use crate::db::reactive::realtime;
use crate::flash_gen::models::Servers;
use crate::services::infrastructure::server::{
    CreateRemoteServerDto, MigrateServerDependenciesDto, PatchRemoteServerDto,
    RemoteServerAction, RemoteServerActionResponseDto, RemoteServerResponseDto,
    RemoteServerStatus, ServerDependencyMigrationDto,
};

use super::init::RemoteServerController;

fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("remote server not found"),
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => {
            tracing::warn!(error = %e.message(), "foreign key violation");
            ConnectError::not_found("referenced resource not found")
        }
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            tracing::warn!(error = %e.message(), "unique constraint violation");
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Database(e) if e.is_check_violation() => {
            tracing::warn!(error = %e.message(), "check constraint violation");
            ConnectError::invalid_argument(e.message())
        }
        sqlx::Error::Protocol(msg) => {
            tracing::warn!(error = %msg, "operation protocol/validation error");
            ConnectError::invalid_argument(msg.clone())
        }
        other => {
            tracing::error!(error = %other, "remote server database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

fn server_to_resource(dto: RemoteServerResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "remote_server".into(),
        payload,
        ..Default::default()
    }
}

fn migration_to_resource(dto: &ServerDependencyMigrationDto) -> ResourceResponse {
    let payload = serde_json::to_string(dto).unwrap_or_default();
    ResourceResponse {
        id: 0,
        kind: "server_migration".into(),
        payload,
        ..Default::default()
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl RemoteServerService for RemoteServerController {
    #[permission(Server, CanRead)]
    async fn list_remote_servers<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let items = self.service.list().await.map_err(map_db_error)?;
        let items = items
            .into_iter()
            .map(RemoteServerResponseDto::from)
            .map(server_to_resource)
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Server, CanRead)]
    async fn stream_list_remote_servers(
        &self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceListResponse> + Send + use<>,
        >,
    > {
        let stream = {
            let sql = "SELECT * FROM servers".to_string();
            realtime()
                .watch_query::<Servers>(sql)
                .await
                .map_err(map_db_error)?
        };
        Response::stream_ok(stream.map(move |rows| {
            Ok(ResourceListResponse {
                items: rows
                    .into_iter()
                    .map(RemoteServerResponseDto::from)
                    .map(server_to_resource)
                    .collect(),
                ..Default::default()
            })
        }))
    }

    #[permission(Server, CanRead)]
    async fn get_remote_server<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let server = self
            .service
            .get_by_id(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(server_to_resource(RemoteServerResponseDto::from(server)))
    }

    #[permission(Server, CanRead)]
    async fn stream_remote_server(
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
            let sql = format!("SELECT * FROM servers WHERE id = {id}");
            realtime()
                .watch_query::<Servers>(sql)
                .await
                .map_err(map_db_error)?
        };
        Response::stream_ok(stream.map(move |mut rows| match rows.pop() {
            Some(server) => Ok(server_to_resource(RemoteServerResponseDto::from(server))),
            None => Err(ConnectError::not_found("remote server not found")),
        }))
    }

    #[permission(Server, CanCreate)]
    async fn create_remote_server<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateRemoteServerDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        dto.validate()
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let server = self.service.create(dto).await.map_err(map_db_error)?;
        Response::ok(server_to_resource(RemoteServerResponseDto::from(server)))
    }

    #[permission(Server, CanUpdate)]
    async fn update_remote_server<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: PatchRemoteServerDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        dto.validate()
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let server = self.service.patch(request.id, dto).await.map_err(map_db_error)?;
        Response::ok(server_to_resource(RemoteServerResponseDto::from(server)))
    }

    #[permission(Server, CanCreate)]
    async fn activate<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let server = self
            .service
            .set_status(request.id, RemoteServerStatus::Active)
            .await
            .map_err(map_db_error)?;
        let resp = RemoteServerActionResponseDto {
            server: RemoteServerResponseDto::from(server),
            action: RemoteServerAction::Activate,
        };
        let payload = serde_json::to_string(&resp).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "activate".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn deactivate<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let server = self
            .service
            .set_status(request.id, RemoteServerStatus::Inactive)
            .await
            .map_err(map_db_error)?;
        let resp = RemoteServerActionResponseDto {
            server: RemoteServerResponseDto::from(server),
            action: RemoteServerAction::Deactivate,
        };
        let payload = serde_json::to_string(&resp).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "deactivate".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn test_connection<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let server = self
            .service
            .touch_test_connection(request.id)
            .await
            .map_err(map_db_error)?;
        let resp = RemoteServerActionResponseDto {
            server: RemoteServerResponseDto::from(server),
            action: RemoteServerAction::TestConnection,
        };
        let payload = serde_json::to_string(&resp).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "test_connection".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanDelete)]
    async fn delete_remote_server<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.delete(request.id).await.map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Server, CanCreate)]
    async fn migrate_dependencies<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: MigrateServerDependenciesDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        dto.validate()
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let migration = self
            .service
            .migrate_dependencies(request.id, dto.target_server_id)
            .await
            .map_err(map_db_error)?;
        let payload = serde_json::to_string(&migration).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "migrate_dependencies".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn get_migration<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        // The migration_id is passed in request.payload as a plain string.
        let migration_id = if request.payload.is_empty() {
            return Err(ConnectError::invalid_argument("migration_id is required"));
        } else {
            request.payload
        };
        let migration = self
            .service
            .migration_status(&migration_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(migration_to_resource(&migration))
    }

    #[permission(Server, CanCreate)]
    async fn rollback_migration<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        // The migration_id is passed in request.payload as a plain string.
        let migration_id = if request.payload.is_empty() {
            return Err(ConnectError::invalid_argument("migration_id is required"));
        } else {
            request.payload
        };
        let migration = self
            .service
            .rollback_migration(&migration_id)
            .await
            .map_err(map_db_error)?;
        let payload = serde_json::to_string(&migration).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "rollback_migration".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }
}
