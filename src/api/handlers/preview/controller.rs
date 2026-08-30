use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CreatePreviewRequest, Empty, GetPreviewRequest, ListPreviewsRequest, ListPreviewsResponse,
    PreviewDeployment, PreviewDeploymentService, PreviewDeploymentServiceExt,
    PreviewOperationRequest,
};

use super::helper::*;
use super::init::PreviewDeploymentController;

#[auto_grpc::service(connect = true, auth = true)]
impl PreviewDeploymentService for PreviewDeploymentController {
    #[permission(Application, CanRead)]
    async fn list_previews<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, ListPreviewsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListPreviewsResponse> + Send + use<'a>> {
        let items = self.service.list(true).await.map_err(map_sqlx)?;
        let previews = items.into_iter().map(view_to_proto).collect();
        Response::ok(ListPreviewsResponse { previews, ..Default::default() })
    }

    #[permission(Application, CanRead)]
    async fn get_preview<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetPreviewRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<PreviewDeployment> + Send + use<'a>> {
        let view = self.service.get(request.id).await.map_err(map_sqlx)?;
        Response::ok(view_to_proto(view))
    }

    #[permission(Application, CanDeploy)]
    async fn create_preview<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreatePreviewRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<PreviewDeployment> + Send + use<'a>> {
        let ev = event_from_request(request.branch, request.commit)?;
        let outcome = self
            .service
            .deploy_application(request.application_id, &ev)
            .await
            .map_err(map_service)?;
        Response::ok(outcome_to_proto(outcome))
    }

    #[permission(Application, CanDeploy)]
    async fn redeploy_preview<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, PreviewOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<PreviewDeployment> + Send + use<'a>> {
        let outcome = self.service.redeploy(request.id).await.map_err(map_service)?;
        Response::ok(outcome_to_proto(outcome))
    }

    #[permission(Application, CanDelete)]
    async fn delete_preview<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, PreviewOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.remove(request.id, true).await.map_err(map_service)?;
        Response::ok(Empty::default())
    }
}
