use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};
use futures::StreamExt;

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, OperationResult, ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
    TagService, TagServiceExt,
};
use crate::db::reactive::realtime;
use crate::flash_gen::models::Tags;
use crate::services::workspace::tag::{CreateTagDto, UpdateTagDto};

use super::helper::*;
use super::init::TagController;

#[auto_grpc::service(connect = true, auth = true)]
impl TagService for TagController {
    async fn stream_list_tags(
        &self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceListResponse> + Send + use<>,
        >,
    > {
        let organization_id = auth.organization_id;
        let stream = {
            let sql = format!("SELECT * FROM tags WHERE organization_id = {organization_id}");
            realtime()
                .watch_query::<Tags>(sql)
                .await
                .map_err(map_db_error)?
        };
        Response::stream_ok(stream.map(move |rows| {
            Ok(ResourceListResponse {
                items: rows.into_iter().map(TagDto::from).map(tag_to_resource).collect(),
                ..Default::default()
            })
        }))
    }

    async fn stream_tag(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceResponse> + Send + use<>,
        >,
    > {
        let organization_id = auth.organization_id;
        let id = request.id;
        let stream = {
            let sql = format!(
                "SELECT * FROM tags WHERE id = {id} AND organization_id = {organization_id}"
            );
            realtime()
                .watch_query::<Tags>(sql)
                .await
                .map_err(map_db_error)?
        };
        Response::stream_ok(stream.map(move |mut rows| match rows.pop() {
            Some(item) => Ok(tag_to_resource(TagDto::from(item))),
            None => Err(ConnectError::not_found("tag not found")),
        }))
    }

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

    async fn stream_list_project_tags(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceListResponse> + Send + use<>,
        >,
    > {
        let project_id = request.environment_id.unwrap_or(0);
        let stream = {
            let sql = format!(
                "SELECT t.* FROM tags t JOIN project_tags pt ON pt.tag_id = t.id WHERE pt.project_id = {project_id}"
            );
            realtime()
                .watch_query::<Tags>(sql)
                .await
                .map_err(map_db_error)?
        };
        Response::stream_ok(stream.map(move |rows| {
            Ok(ResourceListResponse {
                items: rows.into_iter().map(TagDto::from).map(tag_to_resource).collect(),
                ..Default::default()
            })
        }))
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
