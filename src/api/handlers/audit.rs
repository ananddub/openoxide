use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    AuditService, AuditServiceExt, ResourceListResponse, ResourceQuery, ResourceResponse,
};
use crate::db::queries::FlashQueries;
use crate::flash_gen::audit_logs::{
    AuditLogsListByOrganizationAndActionParams, AuditLogsListByOrganizationParams,
};

pub struct AuditController {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl AuditController {
    pub fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }
}

fn map_db_error(err: sqlx::Error) -> ConnectError {
    tracing::error!(error = %err, "audit log database operation failed");
    ConnectError::new(ErrorCode::Internal, "audit log database operation failed")
}

#[auto_grpc::service(connect = true, auth = true)]
impl AuditService for AuditController {
    #[permission(AuditLog, CanRead)]
    async fn list_audit_logs<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        let limit = request.limit.unwrap_or(100).clamp(1, 500);
        let offset = request.offset.unwrap_or(0).max(0);
        let action = request.query.as_deref();

        let (rows, _total) = match action {
            Some(action) => {
                let rows = self
                    .queries
                    .audit_logs_list_by_organization_and_action(
                        &AuditLogsListByOrganizationAndActionParams {
                            organization_id,
                            action: action.to_string(),
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

        let items = rows
            .into_iter()
            .map(|row| {
                let payload = serde_json::to_string(&AuditLogDto::from(row))
                    .unwrap_or_default();
                ResourceResponse {
                    id: 0,
                    kind: "audit_log".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();

        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(AuditLog, CanRead)]
    async fn export_audit_logs<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let auth = Self::extract_auth(&ctx).await?;
        let organization_id = auth.organization_id;
        let action = request.query.as_deref();

        let rows = match action {
            Some(action) => {
                self.queries
                    .audit_logs_list_by_organization_and_action(
                        &AuditLogsListByOrganizationAndActionParams {
                            organization_id,
                            action: action.to_string(),
                            limit: 500,
                            offset: 0,
                        },
                    )
                    .await
                    .map_err(map_db_error)?
            }
            None => {
                self.queries
                    .audit_logs_list_by_organization(&AuditLogsListByOrganizationParams {
                        organization_id,
                        limit: 500,
                        offset: 0,
                    })
                    .await
                    .map_err(map_db_error)?
            }
        };

        let mut csv =
            String::from("id,created_at,user_email,user_role,action,resource_type,resource_id,resource_name\n");
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

        Response::ok(ResourceResponse {
            id: 0,
            kind: "audit_log_export".into(),
            payload: csv,
            ..Default::default()
        })
    }
}

fn csv_field(value: &str) -> String {
    value.replace('"', "\"\"").replace(['\r', '\n'], " ")
}

// ── Helper DTOs for ResourceResponse payloads ────────────

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct AuditLogDto {
    id: i64,
    user_email: String,
    user_role: String,
    action: String,
    resource_type: String,
    resource_id: Option<String>,
    resource_name: Option<String>,
    metadata: Option<String>,
    organization_id: Option<i64>,
    user_id: Option<i64>,
    created_at: i64,
}

impl From<crate::flash_gen::models::AuditLogs> for AuditLogDto {
    fn from(v: crate::flash_gen::models::AuditLogs) -> Self {
        Self {
            id: v.id,
            user_email: v.user_email,
            user_role: v.user_role,
            action: v.action,
            resource_type: v.resource_type,
            resource_id: v.resource_id,
            resource_name: v.resource_name,
            metadata: v.metadata,
            organization_id: v.organization_id,
            user_id: v.user_id,
            created_at: v.created_at,
        }
    }
}
