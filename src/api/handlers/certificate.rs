use crate::{
    api::dto::certificate::{CertificateResponseDto, CreateCertificateDto, PatchCertificateDto},
    core::middleware::validator::ValidatedJson,
    services::certificate::CertificateService,
    utils::jwt::claim::Claims,
};
use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};
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
    async fn list(&self, _claims: Claims) -> Result<Json<Vec<CertificateResponseDto>>, ApiError> {
        self.service
            .list()
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
    async fn get(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<Json<CertificateResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(CertificateResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        _claims: Claims,
        ValidatedJson(body): ValidatedJson<CreateCertificateDto>,
    ) -> Result<(StatusCode, Json<CertificateResponseDto>), ApiError> {
        self.service
            .create(body)
            .await
            .map(CertificateResponseDto::from)
            .map(|cert| (StatusCode::CREATED, Json(cert)))
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchCertificateDto>,
    ) -> Result<Json<CertificateResponseDto>, ApiError> {
        self.service
            .patch(id, body)
            .await
            .map(CertificateResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[delete("/{id}")]
    async fn delete(&self, _claims: Claims, Path(id): Path<i64>) -> Result<StatusCode, ApiError> {
        self.service
            .delete(id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_sqlx_error)
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "certificate not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        other => {
            tracing::error!(error = %other, "certificate database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
