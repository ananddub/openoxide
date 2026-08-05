use std::sync::Arc;

use auto_route::controller;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    api::dto::compose::{
        ComposeResponseDto,
        management::{
            ComposeArchiveDto, ComposeCleanupDto, ComposeExportQueryDto, ComposeMountResponseDto,
            ComposePatchResponseDto, ComposePreviewDto, ComposePreviewResponseDto, ComposeTokenDto,
            DeleteComposeResourceDto, ImportComposeDto, InstallComposeTemplateDto, MoveComposeDto,
            RemoveComposeServiceDto, UpsertComposeMountDto, UpsertComposePatchDto,
            UpsertComposeResourceDto,
        },
    },
    core::middleware::{
        permission::{
            AppCreatePermission, AppDeletePermission, AppDeployPermission, AppReadPermission,
            RequirePermission,
        },
        validator::ValidatedJson,
    },
    services::compose::management::{
        ComposeManagementService, ComposeMountService, ComposePatchService, ComposeTransferService,
    },
};

type ApiError = (StatusCode, String);

pub struct ComposeManagementController {
    service: Arc<ComposeManagementService>,
    transfer: Arc<ComposeTransferService>,
}

pub struct ComposePatchController {
    service: Arc<ComposePatchService>,
}

#[controller("/compose/{compose_id}/patches")]
impl ComposePatchController {
    fn new(service: Arc<ComposePatchService>) -> Self {
        Self { service }
    }

    #[get]
    async fn list(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(compose_id): Path<i64>,
    ) -> Result<Json<Vec<ComposePatchResponseDto>>, ApiError> {
        self.service
            .list(compose_id)
            .await
            .map(|items| Json(items.into_iter().map(Into::into).collect()))
            .map_err(map_error)
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(compose_id): Path<i64>,
        ValidatedJson(body): ValidatedJson<UpsertComposePatchDto>,
    ) -> Result<(StatusCode, Json<ComposePatchResponseDto>), ApiError> {
        self.service
            .create(compose_id, body)
            .await
            .map(|item| (StatusCode::CREATED, Json(item.into())))
            .map_err(map_error)
    }

    #[put("/{id}")]
    async fn update(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path((compose_id, id)): Path<(i64, i64)>,
        ValidatedJson(body): ValidatedJson<UpsertComposePatchDto>,
    ) -> Result<Json<ComposePatchResponseDto>, ApiError> {
        self.service
            .update(compose_id, id, body)
            .await
            .map(|item| Json(item.into()))
            .map_err(map_error)
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeletePermission>,
        Path((compose_id, id)): Path<(i64, i64)>,
    ) -> Result<StatusCode, ApiError> {
        match self.service.delete(compose_id, id).await {
            Ok(true) => Ok(StatusCode::NO_CONTENT),
            Ok(false) => Err((StatusCode::NOT_FOUND, "compose patch not found".into())),
            Err(error) => Err(map_error(error)),
        }
    }
}

#[controller("/compose")]
impl ComposeManagementController {
    fn new(service: Arc<ComposeManagementService>, transfer: Arc<ComposeTransferService>) -> Self {
        Self { service, transfer }
    }

    #[post("/preview")]
    async fn preview(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        ValidatedJson(body): ValidatedJson<ComposePreviewDto>,
    ) -> Result<Json<ComposePreviewResponseDto>, ApiError> {
        self.service.preview(body).map(Json).map_err(map_error)
    }

    #[post("/templates/install")]
    async fn install_template(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        ValidatedJson(body): ValidatedJson<InstallComposeTemplateDto>,
    ) -> Result<(StatusCode, Json<ComposeResponseDto>), ApiError> {
        self.service
            .install_template(body)
            .await
            .map(ComposeResponseDto::from)
            .map(|item| (StatusCode::CREATED, Json(item)))
            .map_err(map_error)
    }

    #[get("/{id}/export")]
    async fn export(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(id): Path<i64>,
        Query(query): Query<ComposeExportQueryDto>,
    ) -> Result<Json<ComposeArchiveDto>, ApiError> {
        let bundle = self
            .transfer
            .export(id, query.include_secrets)
            .await
            .map_err(map_error)?;
        let archive = serde_json::to_string_pretty(&bundle)
            .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
        Ok(Json(ComposeArchiveDto {
            format: "rustploy.compose+json".into(),
            schema_version: i64::from(bundle.schema_version),
            archive,
        }))
    }

    #[post("/import")]
    async fn import(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        ValidatedJson(body): ValidatedJson<ImportComposeDto>,
    ) -> Result<(StatusCode, Json<ComposeResponseDto>), ApiError> {
        self.transfer
            .import(body)
            .await
            .map(ComposeResponseDto::from)
            .map(|item| (StatusCode::CREATED, Json(item)))
            .map_err(map_error)
    }

    #[post("/{id}/services/remove")]
    async fn remove_service(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<RemoveComposeServiceDto>,
    ) -> Result<Json<ComposeResponseDto>, ApiError> {
        self.service
            .remove_service(id, &body.service_name)
            .await
            .map(ComposeResponseDto::from)
            .map(Json)
            .map_err(map_error)
    }

    #[put("/{id}/resources")]
    async fn upsert_resource(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<UpsertComposeResourceDto>,
    ) -> Result<Json<ComposeResponseDto>, ApiError> {
        self.service
            .upsert_resource(id, body)
            .await
            .map(ComposeResponseDto::from)
            .map(Json)
            .map_err(map_error)
    }

    #[delete("/{id}/resources")]
    async fn remove_resource(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeletePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<DeleteComposeResourceDto>,
    ) -> Result<Json<ComposeResponseDto>, ApiError> {
        self.service
            .remove_resource(id, body)
            .await
            .map(ComposeResponseDto::from)
            .map(Json)
            .map_err(map_error)
    }

    #[post("/{id}/move")]
    async fn move_compose(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<MoveComposeDto>,
    ) -> Result<Json<ComposeResponseDto>, ApiError> {
        self.service
            .move_to_environment(id, body.target_environment_id)
            .await
            .map(ComposeResponseDto::from)
            .map(Json)
            .map_err(map_error)
    }

    #[post("/{id}/webhook-token/rotate")]
    async fn rotate_token(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<ComposeTokenDto>, ApiError> {
        self.service
            .rotate_webhook_token(id)
            .await
            .map(|token| Json(ComposeTokenDto { token }))
            .map_err(map_error)
    }

    #[post("/{id}/deployments/queue/cleanup")]
    async fn cleanup_queue(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<ComposeCleanupDto>, ApiError> {
        self.service
            .cleanup_queue(id)
            .await
            .map(|affected| {
                Json(ComposeCleanupDto {
                    affected: affected as i64,
                })
            })
            .map_err(map_error)
    }

    #[delete("/{id}/deployments/history")]
    async fn clear_history(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeletePermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<ComposeCleanupDto>, ApiError> {
        self.service
            .clear_history(id)
            .await
            .map(|affected| {
                Json(ComposeCleanupDto {
                    affected: affected as i64,
                })
            })
            .map_err(map_error)
    }
}

pub struct ComposeMountController {
    service: Arc<ComposeMountService>,
}

#[controller("/compose/{compose_id}/mounts")]
impl ComposeMountController {
    fn new(service: Arc<ComposeMountService>) -> Self {
        Self { service }
    }

    #[get]
    async fn list(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(compose_id): Path<i64>,
    ) -> Result<Json<Vec<ComposeMountResponseDto>>, ApiError> {
        self.service
            .list(compose_id)
            .await
            .map(|items| Json(items.into_iter().map(Into::into).collect()))
            .map_err(map_error)
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(compose_id): Path<i64>,
        ValidatedJson(body): ValidatedJson<UpsertComposeMountDto>,
    ) -> Result<(StatusCode, Json<ComposeMountResponseDto>), ApiError> {
        self.service
            .create(compose_id, body)
            .await
            .map(|item| (StatusCode::CREATED, Json(item.into())))
            .map_err(map_error)
    }

    #[put("/{id}")]
    async fn update(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path((compose_id, id)): Path<(i64, i64)>,
        ValidatedJson(body): ValidatedJson<UpsertComposeMountDto>,
    ) -> Result<Json<ComposeMountResponseDto>, ApiError> {
        self.service
            .update(compose_id, id, body)
            .await
            .map(|item| Json(item.into()))
            .map_err(map_error)
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeletePermission>,
        Path((compose_id, id)): Path<(i64, i64)>,
    ) -> Result<StatusCode, ApiError> {
        match self.service.delete(compose_id, id).await {
            Ok(true) => Ok(StatusCode::NO_CONTENT),
            Ok(false) => Err((StatusCode::NOT_FOUND, "compose mount not found".into())),
            Err(error) => Err(map_error(error)),
        }
    }
}

fn map_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "compose resource not found".into()),
        sqlx::Error::Protocol(message) => (StatusCode::BAD_REQUEST, message),
        sqlx::Error::Database(ref error) if error.is_unique_violation() => {
            (StatusCode::CONFLICT, error.message().into())
        }
        other => {
            tracing::error!(error = %other, "compose management operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
