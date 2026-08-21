use crate::{
    api::dto::certificate::{
        CertificateRenewalDto, CertificateResponseDto, CreateCertificateDto, PatchCertificateDto,
        RenewCertificateDto,
    },
    core::middleware::{
        permission::{
            CanCreate, CanDelete, CanRead, CanUpdate, Certificate, PermissionOrganization,
            RequirePermission,
        },
        validator::ValidatedJson,
    },
    services::certificate::CertificateService,
};
use auto_route::controller;
use axum::{Extension, Json, extract::Path, http::StatusCode};
use std::sync::Arc;

type ApiError = (StatusCode, String);

pub struct CertificateController {
    service: Arc<CertificateService>,
}

#[controller("/certificates")]
impl CertificateController {
    fn new(service: Arc<CertificateService>) -> Self {
        Self { service }
    }

    #[get]
    #[live(tables = ["certificates","certificate_renewals"])]
    async fn list(
        &self,
        RequirePermission(_, _): RequirePermission<Certificate, CanRead>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
    ) -> Result<Json<Vec<CertificateResponseDto>>, ApiError> {
        self.service
            .list(organization_id)
            .await
            .map(|items| {
                items
                    .into_iter()
                    .map(CertificateResponseDto::from)
                    .collect()
            })
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/{id}")]
    #[live(tables = ["certificates","certificate_renewals"])]
    async fn get(
        &self,
        RequirePermission(_, _): RequirePermission<Certificate, CanRead>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(id): Path<i64>,
    ) -> Result<Json<CertificateResponseDto>, ApiError> {
        self.service
            .get_by_id(id, organization_id)
            .await
            .map(CertificateResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_, _): RequirePermission<Certificate, CanCreate>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        ValidatedJson(body): ValidatedJson<CreateCertificateDto>,
    ) -> Result<(StatusCode, Json<CertificateResponseDto>), ApiError> {
        self.service
            .create(body, organization_id)
            .await
            .map(CertificateResponseDto::from)
            .map(|cert| (StatusCode::CREATED, Json(cert)))
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        RequirePermission(_, _): RequirePermission<Certificate, CanUpdate>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchCertificateDto>,
    ) -> Result<Json<CertificateResponseDto>, ApiError> {
        self.service
            .patch(id, body, organization_id)
            .await
            .map(CertificateResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_, _): RequirePermission<Certificate, CanDelete>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .delete(id, organization_id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_sqlx_error)
    }

    #[get("/{id}/dependencies")]
    #[live(tables = ["certificates","certificate_renewals"])]
    async fn dependencies(
        &self,
        RequirePermission(_, _): RequirePermission<Certificate, CanRead>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(id): Path<i64>,
    ) -> Result<Json<crate::repository::CertificateDependencyCounts>, ApiError> {
        self.service
            .dependencies(id, organization_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/{id}/renew")]
    async fn renew(
        &self,
        RequirePermission(_, _): RequirePermission<Certificate, CanUpdate>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(id): Path<i64>,
        Json(body): Json<RenewCertificateDto>,
    ) -> Result<Json<CertificateResponseDto>, ApiError> {
        self.service
            .renew(id, body, organization_id)
            .await
            .map(CertificateResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/{id}/renewals")]
    #[live(tables = ["certificates","certificate_renewals"])]
    async fn renewals(
        &self,
        RequirePermission(_, _): RequirePermission<Certificate, CanRead>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(id): Path<i64>,
    ) -> Result<Json<Vec<CertificateRenewalDto>>, ApiError> {
        self.service
            .renewal_history(id, organization_id)
            .await
            .map(|rows| Json(rows.into_iter().map(Into::into).collect()))
            .map_err(map_sqlx_error)
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "certificate not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        sqlx::Error::Protocol(message) => (StatusCode::BAD_REQUEST, message),
        other => {
            tracing::error!(error = %other, "certificate database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
