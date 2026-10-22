use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CreateEnvironmentRequest, DeleteEnvironmentRequest, Empty, Environment as EnvironmentProto,
    EnvironmentService, EnvironmentServiceExt, GetEnvironmentRequest, ListEnvironmentsRequest,
    ListEnvironmentsResponse, SetDefaultEnvironmentRequest, UpdateEnvironmentRequest,
};

use super::helper::*;
use super::init::EnvironmentController;

#[auto_grpc::service(connect = true, auth = true)]
impl EnvironmentService for EnvironmentController {
    #[permission(Environment, CanRead)]
    async fn get_environment<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetEnvironmentRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<EnvironmentProto> + Send + use<'a>> {
        let env = self
            .service
            .get_by_id(request.id)
            .await
            .map_err(map_db_error)?;
        ensure_environment_org(&env, auth.organization_id).await?;
        Response::ok(env_to_proto(env))
    }

    #[permission(Environment, CanRead)]
    async fn list_environments<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListEnvironmentsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListEnvironmentsResponse> + Send + use<'a>> {
        ensure_project_org(request.project_id, auth.organization_id).await?;
        let items = self
            .service
            .list_by_project(request.project_id)
            .await
            .map_err(map_db_error)?;
        let environments = items.into_iter().map(env_to_proto).collect();
        Response::ok(ListEnvironmentsResponse { environments })
    }

    #[permission(Environment, CanUpdate)]
    async fn create_environment<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateEnvironmentRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<EnvironmentProto> + Send + use<'a>> {
        let mut dto = request.to_owned_message();
        ensure_project_org(dto.project_id, auth.organization_id).await?;
        dto.name = validated_environment_name(&dto.name)?;
        let env = self.service.create(dto).await.map_err(map_db_error)?;
        Response::ok(env_to_proto(env))
    }

    #[permission(Environment, CanUpdate)]
    async fn update_environment<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateEnvironmentRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<EnvironmentProto> + Send + use<'a>> {
        let mut dto = request.to_owned_message();
        let current = self.service.get_by_id(dto.id).await.map_err(map_db_error)?;
        ensure_environment_org(&current, auth.organization_id).await?;
        if let Some(name) = dto.name.as_deref() {
            dto.name = Some(validated_environment_name(name)?);
        }
        let env = self
            .service
            .update(dto.id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(env_to_proto(env))
    }

    #[permission(Environment, CanUpdate)]
    async fn set_default_environment<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, SetDefaultEnvironmentRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<EnvironmentProto> + Send + use<'a>> {
        let current = self
            .service
            .get_by_id(request.id)
            .await
            .map_err(map_db_error)?;
        ensure_environment_org(&current, auth.organization_id).await?;
        let env = self
            .service
            .set_default(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(env_to_proto(env))
    }

    #[permission(Environment, CanUpdate)]
    async fn delete_environment<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteEnvironmentRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let current = self
            .service
            .get_by_id(request.id)
            .await
            .map_err(map_db_error)?;
        ensure_environment_org(&current, auth.organization_id).await?;
        self.service
            .delete(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }
}
