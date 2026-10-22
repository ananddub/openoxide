use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CreateProjectRequest, DeleteProjectRequest, Empty, GetProjectRequest, ListProjectsRequest,
    ListProjectsResponse, Project as ProjectProto, ProjectService, ProjectServiceExt,
    UpdateProjectRequest,
};
use crate::core::cache::{CacheEnum, CacheKey};

use super::helper::*;
use super::init::ProjectController;

#[auto_grpc::service(connect = true, auth = true)]
impl ProjectService for ProjectController {
    #[permission(Project, CanRead)]
    async fn get_project<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetProjectRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ProjectProto> + Send + use<'a>> {
        let project = self
            .service
            .get_by_id(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(project_to_proto(project))
    }

    #[permission(Project, CanRead)]
    async fn list_projects<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListProjectsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListProjectsResponse> + Send + use<'a>> {
        let organization_id = request.organization_id;

        if let Some(CacheEnum::ProjectsList(cached)) = self
            .cache
            .get(&CacheKey::ProjectsList(organization_id))
            .await
        {
            let projects = cached.into_iter().map(project_to_proto).collect();
            return Response::ok(ListProjectsResponse { projects });
        }

        let items = self
            .service
            .list_by_organization(organization_id)
            .await
            .map_err(map_db_error)?;

        self.cache
            .insert(
                CacheKey::ProjectsList(organization_id),
                CacheEnum::ProjectsList(items.clone()),
            )
            .await;

        let projects = items.into_iter().map(project_to_proto).collect();
        Response::ok(ListProjectsResponse { projects })
    }

    #[permission(Project, CanCreate)]
    async fn create_project<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateProjectRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ProjectProto> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let mut input = request.to_owned_message();
        input.organization_id = org_id;
        let project = self.service.create(input).await.map_err(map_db_error)?;
        self.cache.invalidate(&CacheKey::ProjectsList(org_id)).await;
        Response::ok(project_to_proto(project))
    }

    #[permission(Project, CanUpdate)]
    async fn update_project<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateProjectRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ProjectProto> + Send + use<'a>> {
        let dto = request.to_owned_message();
        let project = self
            .service
            .update(dto.id, dto)
            .await
            .map_err(map_db_error)?;
        self.cache.invalidate_all().await;
        Response::ok(project_to_proto(project))
    }

    #[permission(Project, CanDelete)]
    async fn delete_project<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteProjectRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .delete(request.id)
            .await
            .map_err(map_db_error)?;

        if let Ok(runner) =
            auto_di::resolve::<crate::services::automation::schedule::ScheduleRunner>().await
            && let Err(error) = runner.refresh_jobs().await
        {
            tracing::error!(%error, "could not refresh scheduler after project deletion");
        }

        self.cache.invalidate_all().await;
        Response::ok(Empty::default())
    }
}
