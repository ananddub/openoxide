use crate::{
    api::dto::audit::{AuditLogPageDto, AuditLogQueryDto},
    core::middleware::permission::{AuditLog, CanRead, RequirePermission},
    db::repository::AuditLogRepository,
};
use auto_route::controller;
use axum::{Json, extract::Query, http::StatusCode};
use std::sync::Arc;

pub struct AuditController {
    repository: Arc<AuditLogRepository>,
}

#[controller("/audit-logs")]
impl AuditController {
    fn new(repository: Arc<AuditLogRepository>) -> Self {
        Self { repository }
    }

    #[get]
    #[live(table = "audit_logs")]
    async fn list(
        &self,
        RequirePermission(_claims, _permission): RequirePermission<AuditLog, CanRead>,
        axum::Extension(crate::core::middleware::permission::PermissionOrganization(
            organization_id,
        )): axum::Extension<crate::core::middleware::permission::PermissionOrganization>,
        Query(query): Query<AuditLogQueryDto>,
    ) -> Result<Json<AuditLogPageDto>, (StatusCode, String)> {
        let limit = query.limit.unwrap_or(100).clamp(1, 500);
        let offset = query.offset.unwrap_or(0).max(0);
        let rows = self
            .repository
            .list_filtered(
                organization_id,
                query.action.as_deref(),
                query.resource_type.as_deref(),
                limit,
                offset,
            )
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        let total = self
            .repository
            .count_filtered(
                organization_id,
                query.action.as_deref(),
                query.resource_type.as_deref(),
            )
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        Ok(Json(AuditLogPageDto {
            items: rows.into_iter().map(Into::into).collect(),
            total,
            limit,
            offset,
        }))
    }

    #[get("/export")]
    async fn export(
        &self,
        RequirePermission(_claims, permission): RequirePermission<AuditLog, CanRead>,
        Query(query): Query<AuditLogQueryDto>,
    ) -> Result<String, (StatusCode, String)> {
        let rows = self
            .repository
            .list_filtered(
                permission.organization_id(),
                query.action.as_deref(),
                query.resource_type.as_deref(),
                500,
                0,
            )
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        let mut csv = String::from(
            "id,created_at,user_email,user_role,action,resource_type,resource_id,resource_name\n",
        );
        for row in rows {
            csv.push_str(&format!(
                "{},{},\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                row.id.unwrap_or_default(),
                row.created_at,
                csv_field(&row.user_email),
                csv_field(&row.user_role),
                csv_field(&row.action),
                csv_field(&row.resource_type),
                csv_field(row.resource_id.as_deref().unwrap_or("")),
                csv_field(row.resource_name.as_deref().unwrap_or(""))
            ));
        }
        Ok(csv)
    }
}

fn csv_field(value: &str) -> String {
    value.replace('"', "\"\"").replace(['\r', '\n'], " ")
}
