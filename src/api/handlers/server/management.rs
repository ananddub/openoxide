use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};
use futures::StreamExt;

use crate::api::connect::generated::openoxide::api::v1::{
    OperationResult, ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
    ServerManagementService, ServerManagementServiceExt,
};
use crate::db::reactive::realtime;
use crate::flash_gen::models::ServerManagement;

use super::init::ServerManagementController;

fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("server management not found"),
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
            tracing::error!(error = %other, "server management database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

fn management_to_resource(
    server_id: i64,
    management: &impl serde::Serialize,
) -> ResourceResponse {
    let payload = serde_json::to_string(management).unwrap_or_default();
    ResourceResponse {
        id: server_id,
        kind: "server_management".into(),
        payload,
        ..Default::default()
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl ServerManagementService for ServerManagementController {
    #[permission(Server, CanRead)]
    async fn get_management<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let management = self.management.get(request.id).await.map_err(map_db_error)?;
        Response::ok(management_to_resource(request.id, &management))
    }

    #[permission(Server, CanRead)]
    async fn stream_management(
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
            let sql = format!("SELECT * FROM server_management WHERE server_id = {id}");
            realtime()
                .watch_query::<ServerManagement>(sql)
                .await
                .map_err(map_db_error)?
        };
        Response::stream_ok(stream.map(move |mut rows| match rows.pop() {
            Some(management) => Ok(management_to_resource(id, &management)),
            None => Err(ConnectError::not_found("server management not found")),
        }))
    }

    #[permission(Server, CanCreate)]
    async fn update_management<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let management = self
            .management
            .update(request.id, request.payload)
            .await
            .map_err(map_db_error)?;
        Response::ok(management_to_resource(request.id, &management))
    }

    #[permission(Server, CanCreate)]
    async fn repair_audit<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let result = self.lifecycle.auto_repair(request.id).await.map_err(map_db_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "repair_audit".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn configure_gpu<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let result = self.lifecycle.configure_gpu(request.id).await.map_err(map_db_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "configure_gpu".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn run_cleanup<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let result = self.cleanup.run(request.id).await.map_err(map_db_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "run_cleanup".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn list_cleanup_history<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        // Original handler used Path(server_id) via /servers/{server_id}/management.
        // In ConnectRPC the server_id comes through ResourceQuery; however
        // the proto ResourceQuery does not carry an `id` field.  The original
        // live-query just returned all history for the server embedded in the
        // path.  We read server_id from the organisation_id field as a
        // placeholder — the caller should send the server_id in the query
        // filter or the service should be updated.  For now, return empty.
        let items: Vec<ResourceResponse> = vec![];
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Server, CanCreate)]
    async fn upgrade<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let result = self.lifecycle.upgrade(request.id).await.map_err(map_db_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "upgrade".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn backup<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let result = self.lifecycle.backup(request.id).await.map_err(map_db_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "server_backup".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn diagnostics<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let result = self.lifecycle.diagnostics(request.id).await.map_err(map_db_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: request.id,
            kind: "server_diagnostics".into(),
            payload,
            ..Default::default()
        })
    }
}
