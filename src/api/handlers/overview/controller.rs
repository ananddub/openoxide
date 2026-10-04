use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    OverviewListBackupsRequest, OverviewListBackupsResponse, OverviewListDomainsRequest,
    OverviewListDomainsResponse, OverviewListOrganizationBackupsRequest,
    OverviewListOrganizationBackupsResponse, OverviewListOrganizationDomainsRequest,
    OverviewListOrganizationDomainsResponse, OverviewListOrganizationServicesRequest,
    OverviewListOrganizationServicesResponse, OverviewListServicesRequest,
    OverviewListServicesResponse, OverviewService, OverviewServiceExt,
};

use super::helper::*;
use super::init::OverviewController;

#[auto_grpc::service(connect = true, auth = true)]
impl OverviewService for OverviewController {
    #[permission(Project, CanRead)]
    async fn list_services<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, OverviewListServicesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OverviewListServicesResponse> + Send + use<'a>>
    {
        let organization_id = request.organization_id.unwrap_or(0);
        let items = self
            .service
            .get_all_services(organization_id)
            .await
            .map_err(map_db_error)?;
        let services = items.into_iter().map(service_item_to_proto).collect();
        Response::ok(OverviewListServicesResponse { services })
    }

    #[permission(Project, CanRead)]
    async fn list_domains<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, OverviewListDomainsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OverviewListDomainsResponse> + Send + use<'a>>
    {
        let organization_id = request.organization_id.unwrap_or(0);
        let items = self
            .service
            .get_all_domains(organization_id)
            .await
            .map_err(map_db_error)?;
        let domains = items.into_iter().map(domain_item_to_proto).collect();
        Response::ok(OverviewListDomainsResponse { domains })
    }

    #[permission(Project, CanRead)]
    async fn list_backups<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, OverviewListBackupsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OverviewListBackupsResponse> + Send + use<'a>>
    {
        let organization_id = request.organization_id.unwrap_or(0);
        let items = self
            .service
            .get_all_backups(organization_id)
            .await
            .map_err(map_db_error)?;
        let backups = items.into_iter().map(backup_item_to_proto).collect();
        Response::ok(OverviewListBackupsResponse { backups })
    }

    #[permission(Project, CanRead)]
    async fn list_organization_services<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, OverviewListOrganizationServicesRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<OverviewListOrganizationServicesResponse> + Send + use<'a>,
    > {
        let organization_id = request.organization_id.unwrap_or(0);
        let items = self
            .service
            .get_all_services(organization_id)
            .await
            .map_err(map_db_error)?;
        let services = items.into_iter().map(service_item_to_proto).collect();
        Response::ok(OverviewListOrganizationServicesResponse { services })
    }

    #[permission(Project, CanRead)]
    async fn list_organization_domains<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, OverviewListOrganizationDomainsRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<OverviewListOrganizationDomainsResponse> + Send + use<'a>,
    > {
        let organization_id = request.organization_id.unwrap_or(0);
        let items = self
            .service
            .get_all_domains(organization_id)
            .await
            .map_err(map_db_error)?;
        let domains = items.into_iter().map(domain_item_to_proto).collect();
        Response::ok(OverviewListOrganizationDomainsResponse { domains })
    }

    #[permission(Project, CanRead)]
    async fn list_organization_backups<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, OverviewListOrganizationBackupsRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<OverviewListOrganizationBackupsResponse> + Send + use<'a>,
    > {
        let organization_id = request.organization_id.unwrap_or(0);
        let items = self
            .service
            .get_all_backups(organization_id)
            .await
            .map_err(map_db_error)?;
        let backups = items.into_iter().map(backup_item_to_proto).collect();
        Response::ok(OverviewListOrganizationBackupsResponse { backups })
    }
}
