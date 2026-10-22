use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    AddTagRequest, AddTagResponse, AttachProjectTagRequest, AttachProjectTagResponse,
    CreateTagRequest, CreateTagResponse, DeleteTagRequest, DetachProjectTagRequest, Empty,
    GetTagRequest, GetTagResponse, ListProjectTagsRequest, ListProjectTagsResponse,
    ListTagsRequest, ListTagsResponse, RemoveTagRequest, TagService, TagServiceExt,
    UpdateTagRequest, UpdateTagResponse,
};
use crate::services::workspace::tag::{CreateTagDto, UpdateTagDto};

use super::helper::*;
use super::init::TagController;

#[auto_grpc::service(connect = true, auth = true)]
impl TagService for TagController {
    async fn list_tags<'a>(
        &'a self,
        _ctx: RequestContext,
        _query: ServiceRequest<'_, ListTagsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListTagsResponse> + Send + use<'a>> {
        let items = self.service.list_all().await.map_err(map_db_error)?;
        let tags = items
            .into_iter()
            .map(TagDto::from)
            .map(tag_to_proto)
            .collect();
        Response::ok(ListTagsResponse { tags })
    }

    async fn get_tag<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, GetTagRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GetTagResponse> + Send + use<'a>> {
        let tag = self
            .service
            .get_by_id(request.id)
            .await
            .map_err(map_db_error)?
            .ok_or_else(|| ConnectError::not_found("tag not found"))?;
        Response::ok(GetTagResponse {
            tag: ::buffa::MessageField::some(tag_to_proto(TagDto::from(tag))),
        })
    }

    async fn create_tag<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, CreateTagRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<CreateTagResponse> + Send + use<'a>> {
        let dto = CreateTagDto {
            name: request.name.to_owned(),
            color: request.color.to_owned(),
            organization_id: request.organization_id,
        };
        let tag = self.service.create(dto).await.map_err(map_db_error)?;
        Response::ok(CreateTagResponse {
            tag: ::buffa::MessageField::some(tag_to_proto(TagDto::from(tag))),
        })
    }

    async fn update_tag<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, UpdateTagRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<UpdateTagResponse> + Send + use<'a>> {
        let dto = UpdateTagDto {
            name: request.name.map(str::to_owned),
            color: request.color.map(str::to_owned),
        };
        let tag = self
            .service
            .update(request.id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(UpdateTagResponse {
            tag: ::buffa::MessageField::some(tag_to_proto(TagDto::from(tag))),
        })
    }

    async fn delete_tag<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, DeleteTagRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .delete(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    async fn add_tag<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, AddTagRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AddTagResponse> + Send + use<'a>> {
        Response::ok(AddTagResponse { success: true })
    }

    async fn remove_tag<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, RemoveTagRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .delete(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    async fn list_project_tags<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ListProjectTagsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListProjectTagsResponse> + Send + use<'a>> {
        let items = self
            .service
            .list_project_tags(request.project_id)
            .await
            .map_err(map_db_error)?;
        let tags = items
            .into_iter()
            .map(TagDto::from)
            .map(tag_to_proto)
            .collect();
        Response::ok(ListProjectTagsResponse { tags })
    }

    async fn attach_project_tag<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, AttachProjectTagRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AttachProjectTagResponse> + Send + use<'a>> {
        self.service
            .attach_project_tag(request.project_id, request.tag_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(AttachProjectTagResponse { success: true })
    }

    async fn detach_project_tag<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, DetachProjectTagRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .detach_project_tag(request.project_id, request.tag_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }
}
