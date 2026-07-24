use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::registry::{
        CreateRegistryDto, PatchRegistryDto, RegistryResponseDto, TestRegistryDto,
    },
    core::middleware::{
        permission::{
            AppCreatePermission, AppDeletePermission, AppReadPermission, RequirePermission,
        },
        validator::ValidatedJson,
    },
    services::registry::RegistryService,
};

type ApiError = (StatusCode, String);

pub struct RegistryController {
    service: Arc<RegistryService>,
}

#[controller("/registries")]
impl RegistryController {
    fn new(service: Arc<RegistryService>) -> Self {
        Self { service }
    }

    #[get]
    async fn list(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
    ) -> Result<Json<Vec<RegistryResponseDto>>, ApiError> {
        self.service
            .list()
            .await
            .map(|items| items.into_iter().map(RegistryResponseDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/{id}")]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<RegistryResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(RegistryResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        ValidatedJson(body): ValidatedJson<CreateRegistryDto>,
    ) -> Result<(StatusCode, Json<RegistryResponseDto>), ApiError> {
        self.service
            .create(body)
            .await
            .map(RegistryResponseDto::from)
            .map(|reg| (StatusCode::CREATED, Json(reg)))
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchRegistryDto>,
    ) -> Result<Json<RegistryResponseDto>, ApiError> {
        self.service
            .patch(id, body)
            .await
            .map(RegistryResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeletePermission>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .delete(id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_sqlx_error)
    }

    #[post("/{id}/test")]
    async fn test_connection(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .test_connection(id)
            .await
            .map(|()| StatusCode::OK)
            .map_err(|err| (StatusCode::BAD_REQUEST, err))
    }

    #[post("/test-raw")]
    async fn test_connection_raw(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        ValidatedJson(body): ValidatedJson<TestRegistryDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .test_connection_raw(&body.registry_url, &body.username, &body.password)
            .await
            .map(|()| StatusCode::OK)
            .map_err(|err| (StatusCode::BAD_REQUEST, err))
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "registry not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        other => {
            tracing::error!(error = %other, "registry database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
