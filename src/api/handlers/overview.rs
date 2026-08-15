use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::overview::{
        OverviewBackupItemDto, OverviewDomainItemDto, OverviewServiceItemDto,
    },
    core::middleware::permission::{CanRead, Project, RequirePermission},
    services::overview::OverviewService,
};

type ApiError = (StatusCode, String);

pub struct OverviewController {
    service: Arc<OverviewService>,
}

#[controller("/overview")]
impl OverviewController {
    fn new(service: Arc<OverviewService>) -> Self {
        Self { service }
    }

    #[get("/services/organization/{organization_id}")]
    #[live(tables = ["applications", "compose_projects", "postgreses", "mysqls", "mariadbs", "mongos", "redises"])]
    async fn services(
        &self,
        RequirePermission(_claims, _): RequirePermission<Project, CanRead>,
        Path(organization_id): Path<i64>,
    ) -> Result<Json<Vec<OverviewServiceItemDto>>, ApiError> {
        self.service
            .get_all_services(organization_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/domains/organization/{organization_id}")]
    #[live(tables = ["domains"])]
    async fn domains(
        &self,
        RequirePermission(_claims, _): RequirePermission<Project, CanRead>,
        Path(organization_id): Path<i64>,
    ) -> Result<Json<Vec<OverviewDomainItemDto>>, ApiError> {
        self.service
            .get_all_domains(organization_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/backups/organization/{organization_id}")]
    #[live(tables = ["database_backups", "volume_backups"])]
    async fn backups(
        &self,
        RequirePermission(_claims, _): RequirePermission<Project, CanRead>,
        Path(organization_id): Path<i64>,
    ) -> Result<Json<Vec<OverviewBackupItemDto>>, ApiError> {
        self.service
            .get_all_backups(organization_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    tracing::error!(error = %error, "overview database operation failed");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "overview database operation failed".into(),
    )
}
