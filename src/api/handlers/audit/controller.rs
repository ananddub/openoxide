use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    AuditExportAuditLogsRequest, AuditExportAuditLogsResponse, AuditListAuditLogsRequest,
    AuditListAuditLogsResponse, AuditService, AuditServiceExt,
};
use crate::flash_gen::audit_logs::{
    AuditLogsListByOrganizationAndActionParams, AuditLogsListByOrganizationParams,
};

use super::helper::*;
use super::init::AuditController;

#[auto_grpc::service(connect = true, auth = true)]
impl AuditService for AuditController {
    #[permission(AuditLog, CanRead)]
    async fn list_audit_logs<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AuditListAuditLogsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AuditListAuditLogsResponse> + Send + use<'a>>
    {
        let organization_id = request.organization_id.unwrap_or(0);
        let limit = request.limit.unwrap_or(100).clamp(1, 500);
        let offset = request.offset.unwrap_or(0).max(0);
        let action = request.action;

        let (rows, _total) = match action {
            Some(action) => {
                let rows = self
                    .queries
                    .audit_logs_list_by_organization_and_action(
                        &AuditLogsListByOrganizationAndActionParams {
                            organization_id,
                            action: action.to_owned(),
                            limit,
                            offset,
                        },
                    )
                    .await
                    .map_err(map_db_error)?;
                let total = self
                    .queries
                    .audit_logs_count_by_organization_and_action(organization_id, action)
                    .await
                    .map_err(map_db_error)?;
                (rows, total)
            }
            None => {
                let rows = self
                    .queries
                    .audit_logs_list_by_organization(&AuditLogsListByOrganizationParams {
                        organization_id,
                        limit,
                        offset,
                    })
                    .await
                    .map_err(map_db_error)?;
                let total = self
                    .queries
                    .audit_logs_count_by_organization(organization_id)
                    .await
                    .map_err(map_db_error)?;
                (rows, total)
            }
        };

        let logs = rows.into_iter().map(log_to_proto).collect();
        Response::ok(AuditListAuditLogsResponse { logs })
    }

    #[permission(AuditLog, CanRead)]
    async fn export_audit_logs<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AuditExportAuditLogsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AuditExportAuditLogsResponse> + Send + use<'a>>
    {
        let organization_id = auth.organization_id;
        let action = request.action;

        let rows = match action {
            Some(action) => self
                .queries
                .audit_logs_list_by_organization_and_action(
                    &AuditLogsListByOrganizationAndActionParams {
                        organization_id,
                        action: action.to_owned(),
                        limit: 500,
                        offset: 0,
                    },
                )
                .await
                .map_err(map_db_error)?,
            None => self
                .queries
                .audit_logs_list_by_organization(&AuditLogsListByOrganizationParams {
                    organization_id,
                    limit: 500,
                    offset: 0,
                })
                .await
                .map_err(map_db_error)?,
        };

        let mut csv = String::from(
            "id,created_at,user_email,user_role,action,resource_type,resource_id,resource_name\n",
        );
        for row in rows {
            csv.push_str(&format!(
                "{},{},\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                row.id,
                row.created_at,
                csv_field(&row.user_email),
                csv_field(&row.user_role),
                csv_field(&row.action),
                csv_field(&row.resource_type),
                csv_field(row.resource_id.as_deref().unwrap_or("")),
                csv_field(row.resource_name.as_deref().unwrap_or(""))
            ));
        }

        Response::ok(AuditExportAuditLogsResponse { csv })
    }
}
