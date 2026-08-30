use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CreatePreviewRequest, Empty, GetPreviewRequest, ListPreviewsRequest, ListPreviewsResponse,
    PreviewDeployment, PreviewDeploymentService, PreviewDeploymentServiceExt,
    PreviewOperationRequest,
};
use crate::services::preview::{
    PreviewDeploymentService as AppPreviewService, PreviewDeploymentOutcome, PreviewDeploymentView,
};
use crate::services::webhook::{GitProviderKind, PullRequestEvent};

pub struct PreviewDeploymentController {
    service: Arc<AppPreviewService>,
}

#[singleton]
impl PreviewDeploymentController {
    pub fn new(service: Arc<AppPreviewService>) -> Self {
        Self { service }
    }
}

fn view_to_proto(v: PreviewDeploymentView) -> PreviewDeployment {
    PreviewDeployment {
        id: v.id,
        application_id: v.base_application_id,
        status: v.status,
        branch: Some(v.source_branch.clone()),
        url: if v.domain.is_empty() { None } else { Some(v.domain) },
        error: None,
        created_at: v.created_at,
        updated_at: v.updated_at,
        ..Default::default()
    }
}

fn outcome_to_proto(o: PreviewDeploymentOutcome) -> PreviewDeployment {
    PreviewDeployment {
        id: o.preview.id,
        application_id: o.preview.base_application_id,
        status: o.preview.status,
        branch: Some(o.preview.source_branch.clone()),
        url: if o.preview.domain.is_empty() { None } else { Some(o.preview.domain) },
        error: None,
        created_at: o.preview.created_at,
        updated_at: o.preview.updated_at,
        ..Default::default()
    }
}

fn map_sqlx(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("preview deployment not found"),
        other => {
            tracing::error!(error = %other, "preview deployment database operation failed");
            ConnectError::new(ErrorCode::Internal, "preview deployment operation failed")
        }
    }
}

fn map_service(err: String) -> ConnectError {
    let lower = err.to_ascii_lowercase();
    if lower.contains("not found") {
        ConnectError::not_found(err)
    } else if err.contains("already queued or running") {
        ConnectError::new(ErrorCode::AlreadyExists, err)
    } else {
        ConnectError::invalid_argument(err)
    }
}

fn event_from_request(
    branch: &str,
    commit: Option<&str>,
) -> Result<PullRequestEvent, ConnectError> {
    let provider: GitProviderKind = "github"
        .parse()
        .map_err(|e: String| ConnectError::invalid_argument(e))?;
    Ok(PullRequestEvent {
        provider,
        // The proto request only carries application_id/branch/commit;
        // the remaining webhook fields fall back to neutral defaults.
        owner: String::new(),
        repository: String::new(),
        number: "0".into(),
        action: "opened".into(),
        source_branch: branch.to_owned(),
        source_owner: None,
        source_repository: None,
        target_branch: branch.to_owned(),
        commit: commit.map(str::to_owned),
        author: None,
    })
}

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
