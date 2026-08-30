use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CreateEnvironmentRequest, DeleteEnvironmentRequest, Empty,
    Environment as EnvironmentProto, EnvironmentService, EnvironmentServiceExt,
    GetEnvironmentRequest, ListEnvironmentsRequest, ListEnvironmentsResponse,
    SetDefaultEnvironmentRequest, UpdateEnvironmentRequest,
};
use crate::services::environment::EnvironmentService as AppEnvironmentService;

pub struct EnvironmentController {
    service: Arc<AppEnvironmentService>,
}

#[singleton]
impl EnvironmentController {
    pub fn new(service: Arc<AppEnvironmentService>) -> Self {
        Self { service }
    }
}

fn env_to_proto(e: crate::db::models::environments::Environment) -> EnvironmentProto {
    EnvironmentProto {
        id: e.id,
        name: e.name,
        description: e.description,
        environment_variables: e.env_var,
        is_default: e.is_default != 0,
        project_id: e.project_id,
        created_at: e.created_at,
        updated_at: e.updated_at,
        ..Default::default()
    }
}

fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("environment not found"),
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => {
            ConnectError::not_found("project not found")
        }
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Protocol(msg) => ConnectError::invalid_argument(msg.clone()),
        other => {
            tracing::error!(error = %other, "environment database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl EnvironmentService for EnvironmentController {
    #[permission(Environment, CanRead)]
    async fn get_environment<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetEnvironmentRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<EnvironmentProto> + Send + use<'a>> {
        let env = self.service.get_by_id(request.id).await.map_err(map_db_error)?;
        Response::ok(env_to_proto(env))
    }

    #[permission(Environment, CanRead)]
    async fn list_environments<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListEnvironmentsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListEnvironmentsResponse> + Send + use<'a>> {
        let items = self.service.list_by_project(request.project_id).await.map_err(map_db_error)?;
        let environments = items.into_iter().map(env_to_proto).collect();
        Response::ok(ListEnvironmentsResponse { environments, ..Default::default() })
    }

    #[permission(Environment, CanWrite)]
    async fn create_environment<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateEnvironmentRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<EnvironmentProto> + Send + use<'a>> {
        let env = self
            .service
            .create(request.to_owned_message())
            .await
            .map_err(map_db_error)?;
        Response::ok(env_to_proto(env))
    }

    #[permission(Environment, CanWrite)]
    async fn update_environment<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateEnvironmentRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<EnvironmentProto> + Send + use<'a>> {
        let dto = request.to_owned_message();
        let env = self.service.update(dto.id, dto).await.map_err(map_db_error)?;
        Response::ok(env_to_proto(env))
    }

    #[permission(Environment, CanWrite)]
    async fn set_default_environment<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, SetDefaultEnvironmentRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<EnvironmentProto> + Send + use<'a>> {
        let env = self.service.set_default(request.id).await.map_err(map_db_error)?;
        Response::ok(env_to_proto(env))
    }

    #[permission(Environment, CanWrite)]
    async fn delete_environment<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteEnvironmentRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.delete(request.id).await.map_err(map_db_error)?;
        Response::ok(Empty::default())
    }
}
