use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CreateProjectRequest, DeleteProjectRequest, Empty, GetProjectRequest, ListProjectsRequest,
    ListProjectsResponse, Project as ProjectProto, ProjectService, ProjectServiceExt,
    UpdateProjectRequest,
};
use crate::core::cache::{AppStateCache, CacheEnum, CacheKey};
use crate::services::project::ProjectService as AppProjectService;

pub struct ProjectController {
    service: Arc<AppProjectService>,
    cache: Arc<AppStateCache>,
}

#[singleton]
impl ProjectController {
    pub fn new(service: Arc<AppProjectService>, cache: Arc<AppStateCache>) -> Self {
        Self { service, cache }
    }
}

fn project_to_proto(p: crate::flash_gen::models::Projects) -> ProjectProto {
    ProjectProto {
        id: p.id,
        name: p.name,
        description: p.description,
        environment_variables: p.env_var,
        organization_id: p.organization_id,
        created_at: p.created_at,
        updated_at: p.updated_at,
        ..Default::default()
    }
}

fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("project not found"),
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        other => {
            tracing::error!(error = %other, "project database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl ProjectService for ProjectController {
    #[permission(Project, CanRead)]
    async fn get_project<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetProjectRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ProjectProto> + Send + use<'a>> {
        let project = self.service.get_by_id(request.id).await.map_err(map_db_error)?;
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
            return Response::ok(ListProjectsResponse { projects, ..Default::default() });
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
        Response::ok(ListProjectsResponse { projects, ..Default::default() })
    }

    #[permission(Project, CanCreate)]
    async fn create_project<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateProjectRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ProjectProto> + Send + use<'a>> {
        let org_id = request.organization_id;
        let project = self
            .service
            .create(request.to_owned_message())
            .await
            .map_err(map_db_error)?;
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
        let project = self.service.update(dto.id, dto).await.map_err(map_db_error)?;
        self.cache.invalidate_all().await;
        Response::ok(project_to_proto(project))
    }

    #[permission(Project, CanDelete)]
    async fn delete_project<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteProjectRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.delete(request.id).await.map_err(map_db_error)?;

        if let Ok(runner) = auto_di::resolve::<crate::services::schedule::ScheduleRunner>().await {
            if let Err(error) = runner.refresh_jobs().await {
                tracing::error!(%error, "could not refresh scheduler after project deletion");
            }
        }

        self.cache.invalidate_all().await;
        Response::ok(Empty::default())
    }
}
