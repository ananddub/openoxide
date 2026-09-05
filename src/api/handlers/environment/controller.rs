use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};
use futures::StreamExt;

use crate::api::connect::generated::openoxide::api::v1::{
    CreateEnvironmentRequest, DeleteEnvironmentRequest, Empty,
    Environment as EnvironmentProto, EnvironmentService, EnvironmentServiceExt,
    GetEnvironmentRequest, ListEnvironmentsRequest, ListEnvironmentsResponse,
    SetDefaultEnvironmentRequest, UpdateEnvironmentRequest,
};
use crate::db::reactive::realtime;
use crate::flash_gen::models::Environments;

use super::helper::*;
use super::init::EnvironmentController;

#[auto_grpc::service(connect = true, auth = true)]
impl EnvironmentService for EnvironmentController {
    #[permission(Environment, CanRead)]
    async fn stream_get_environment(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetEnvironmentRequest>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<EnvironmentProto> + Send + use<>,
        >,
    > {
        let id = request.id;
        let stream = {
            let sql = format!("SELECT * FROM environments WHERE id = {id}");
            realtime()
                .watch_query::<Environments>(sql)
                .await
                .map_err(map_db_error)?
        };
        Response::stream_ok(stream.map(move |mut rows| match rows.pop() {
            Some(item) => Ok(env_to_proto(item)),
            None => Err(ConnectError::not_found("environment not found")),
        }))
    }

    #[permission(Environment, CanRead)]
    async fn stream_list_environments(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListEnvironmentsRequest>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ListEnvironmentsResponse> + Send + use<>,
        >,
    > {
        let project_id = request.project_id;
        let stream = {
            let sql = format!("SELECT * FROM environments WHERE project_id = {project_id}");
            realtime()
                .watch_query::<Environments>(sql)
                .await
                .map_err(map_db_error)?
        };
        Response::stream_ok(stream.map(move |rows| {
            Ok(ListEnvironmentsResponse {
                environments: rows.into_iter().map(env_to_proto).collect(),
                ..Default::default()
            })
        }))
    }

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
