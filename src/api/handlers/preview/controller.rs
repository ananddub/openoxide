use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CreatePreviewRequest, CreatePreviewResponse, DeletePreviewRequest, Empty, GetPreviewRequest,
    GetPreviewResponse, ListPreviewsRequest, ListPreviewsResponse, PreviewDeploymentService,
    PreviewDeploymentServiceExt, RedeployPreviewRequest, RedeployPreviewResponse,
};

use super::helper::*;
use super::init::PreviewDeploymentController;

#[auto_grpc::service(connect = true, auth = true)]
impl PreviewDeploymentService for PreviewDeploymentController {
    #[permission(Application, CanRead)]
    async fn list_previews<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListPreviewsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListPreviewsResponse> + Send + use<'a>> {
        let items = self.service.list(true).await.map_err(map_sqlx)?;
        let previews = match request.application_id {
            Some(application_id) => items
                .into_iter()
                .filter(|item| item.base_application_id == application_id)
                .map(view_to_proto)
                .collect(),
            None => items.into_iter().map(view_to_proto).collect(),
        };
        Response::ok(ListPreviewsResponse {
            previews,
            ..Default::default()
        })
    }

    #[permission(Application, CanRead)]
    async fn get_preview<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetPreviewRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GetPreviewResponse> + Send + use<'a>> {
        let view = self.service.get(request.id).await.map_err(map_sqlx)?;
        Response::ok(GetPreviewResponse {
            preview: ::buffa::MessageField::some(view_to_proto(view)),
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
    async fn create_preview<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreatePreviewRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<CreatePreviewResponse> + Send + use<'a>> {
        let ev = event_from_request(request.branch, request.commit)?;
        let outcome = self
            .service
            .deploy_application(request.application_id, &ev)
            .await
            .map_err(map_service)?;
        Response::ok(CreatePreviewResponse {
            preview: ::buffa::MessageField::some(outcome_to_proto(outcome)),
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
    async fn redeploy_preview<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RedeployPreviewRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<RedeployPreviewResponse> + Send + use<'a>> {
        let outcome = self.service.redeploy(request.id).await.map_err(map_service)?;
        Response::ok(RedeployPreviewResponse {
            preview: ::buffa::MessageField::some(outcome_to_proto(outcome)),
            ..Default::default()
        })
    }

    #[permission(Application, CanDelete)]
    async fn delete_preview<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeletePreviewRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.remove(request.id, true).await.map_err(map_service)?;
        Response::ok(Empty::default())
    }
}
