use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};
use futures::StreamExt;
use std::sync::Arc;

use crate::api::connect::generated::openoxide::api::v1::{
    ResourceListResponse, ResourceQuery, ResourceResponse,
    OverviewOperationsService, OverviewOperationsServiceExt,
};
use crate::db::reactive::realtime;
use crate::flash_gen::models::{Domains, Services};
use crate::flash_gen::overview::OverviewListBackupsRow;

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
    async fn stream_list_organization_services(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceListResponse> + Send + use<>,
        >,
    > {
        let organization_id = request.organization_id.unwrap_or(0);
        let service = Arc::clone(&self.service);
        let stream = {
            let sql = format!(
                "SELECT s.* FROM services s JOIN environments e ON s.environment_id = e.id JOIN projects p ON e.project_id = p.id LEFT JOIN servers sv ON s.server_id = sv.id LEFT JOIN service_database d ON d.service_id = s.id WHERE p.organization_id = {organization_id} AND NOT (s.service_type = 'DATABASE' AND d.db_type = 'LIBSQL')"
            );
            realtime()
                .watch_query::<Services>(sql)
                .await
                .map_err(map_db_error)?
        };
        Response::stream_ok(stream.then(move |_rows| {
            let service = Arc::clone(&service);
            async move {
                match service.get_all_services(organization_id).await {
                    Ok(dtos) => {
                        let items = dtos
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
                        Ok(ResourceListResponse { items, ..Default::default() })
                    }
                    Err(e) => Err(map_db_error(e)),
                }
            }
        }))
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
    async fn stream_list_organization_domains(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceListResponse> + Send + use<>,
        >,
    > {
        let organization_id = request.organization_id.unwrap_or(0);
        let service = Arc::clone(&self.service);
        let stream = {
            let sql = format!(
                "SELECT dom.* FROM domains dom LEFT JOIN services s ON dom.service_id = s.id LEFT JOIN environments e ON s.environment_id = e.id LEFT JOIN projects p ON e.project_id = p.id WHERE p.organization_id = {organization_id}"
            );
            realtime()
                .watch_query::<Domains>(sql)
                .await
                .map_err(map_db_error)?
        };
        Response::stream_ok(stream.then(move |_rows| {
            let service = Arc::clone(&service);
            async move {
                match service.get_all_domains(organization_id).await {
                    Ok(dtos) => {
                        let items = dtos
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
                        Ok(ResourceListResponse { items, ..Default::default() })
                    }
                    Err(e) => Err(map_db_error(e)),
                }
            }
        }))
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

    #[permission(Project, CanRead)]
    async fn stream_list_organization_backups(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceListResponse> + Send + use<>,
        >,
    > {
        let organization_id = request.organization_id.unwrap_or(0);
        let service = Arc::clone(&self.service);
        let stream = {
            let sql = format!(
                "SELECT b.id AS id, b.app_name AS name, b.enabled AS enabled, b.destination_id AS destination_id, b.created_at AS created_at FROM backups b WHERE b.organization_id = {organization_id} UNION ALL SELECT vb.id AS id, vb.name AS name, vb.enabled AS enabled, vb.destination_id AS destination_id, vb.created_at AS created_at FROM volume_backups vb WHERE vb.organization_id = {organization_id}"
            );
            realtime()
                .watch_query::<OverviewListBackupsRow>(sql)
                .await
                .map_err(map_db_error)?
        };
        Response::stream_ok(stream.then(move |_rows| {
            let service = Arc::clone(&service);
            async move {
                match service.get_all_backups(organization_id).await {
                    Ok(dtos) => {
                        let items = dtos
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
                        Ok(ResourceListResponse { items, ..Default::default() })
                    }
                    Err(e) => Err(map_db_error(e)),
                }
            }
        }))
    }
}
