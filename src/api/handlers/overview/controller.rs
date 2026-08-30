use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    ResourceListResponse, ResourceQuery, ResourceResponse,
    OverviewOperationsService, OverviewOperationsServiceExt,
};

use super::helper::*;
use super::init::OverviewController;

#[auto_grpc::service(connect = true, auth = true)]
impl OverviewOperationsService for OverviewController {
    #[permission(Project, CanRead)]
    async fn list_organization_services<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        let items = self
            .service
            .get_all_services(organization_id)
            .await
            .map_err(map_db_error)?;
        let items = items
            .into_iter()
            .map(|dto| {
                let payload = serde_json::to_string(&dto).unwrap_or_default();
                ResourceResponse {
                    id: dto.id,
                    kind: "overview_service".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Project, CanRead)]
    async fn list_organization_domains<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        let items = self
            .service
            .get_all_domains(organization_id)
            .await
            .map_err(map_db_error)?;
        let items = items
            .into_iter()
            .map(|dto| {
                let payload = serde_json::to_string(&dto).unwrap_or_default();
                ResourceResponse {
                    id: dto.id,
                    kind: "overview_domain".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Project, CanRead)]
    async fn list_organization_backups<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        let items = self
            .service
            .get_all_backups(organization_id)
            .await
            .map_err(map_db_error)?;
        let items = items
            .into_iter()
            .map(|dto| {
                let payload = serde_json::to_string(&dto).unwrap_or_default();
                ResourceResponse {
                    id: dto.id,
                    kind: "overview_backup".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }
}
