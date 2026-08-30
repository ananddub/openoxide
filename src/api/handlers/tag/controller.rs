use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, OperationResult, ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
    TagService, TagServiceExt,
};
use crate::services::tag::{CreateTagDto, UpdateTagDto};

use super::helper::*;
use super::init::TagController;

#[auto_grpc::service(connect = true, auth = true)]
impl TagService for TagController {
    async fn list_tags<'a>(
        &'a self,
        _ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let items = self.service.list_all().await.map_err(map_db_error)?;
        let items = items.into_iter().map(TagDto::from).map(tag_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    async fn get_tag<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let tag = self.service.get_by_id(request.id).await.map_err(map_db_error)?
            .ok_or_else(|| ConnectError::not_found("tag not found"))?;
        Response::ok(tag_to_resource(TagDto::from(tag)))
    }

    async fn create_tag<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateTagDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let tag = self.service.create(dto).await.map_err(map_db_error)?;
        Response::ok(tag_to_resource(TagDto::from(tag)))
    }

    async fn update_tag<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: UpdateTagDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let tag = self.service.update(request.id, dto).await.map_err(map_db_error)?;
        Response::ok(tag_to_resource(TagDto::from(tag)))
    }

    async fn delete_tag<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.delete(request.id).await.map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    async fn add_tag<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        Response::ok(OperationResult {
            success: true,
            operation: "add_tag".into(),
            ..Default::default()
        })
    }

    async fn remove_tag<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.delete(request.id).await.map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    async fn list_project_tags<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let project_id = request.environment_id.unwrap_or(0);
        let items = self.service.list_project_tags(project_id).await.map_err(map_db_error)?;
        let items = items.into_iter().map(TagDto::from).map(tag_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    async fn attach_project_tag<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: AttachProjectTagDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service.attach_project_tag(request.id, dto.tag_id).await.map_err(map_db_error)?;
        Response::ok(OperationResult {
            success: true,
            operation: "attach_project_tag".into(),
            ..Default::default()
        })
    }

    async fn detach_project_tag<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let dto: AttachProjectTagDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service.detach_project_tag(request.id, dto.tag_id).await.map_err(map_db_error)?;
        Response::ok(Empty::default())
    }
}
