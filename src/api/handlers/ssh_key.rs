use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::ssh_key::{CreateSshKeyDto, GenerateSshKeyDto, PatchSshKeyDto, SshKeyResponseDto},
    core::middleware::{
        permission::{
            RequirePermission, ServerCreatePermission, ServerDeletePermission, ServerReadPermission,
        },
        validator::ValidatedJson,
    },
    services::ssh_key::SshKeyService,
};

type ApiError = (StatusCode, String);

pub struct SshKeyController {
    service: Arc<SshKeyService>,
}

#[controller("/ssh-keys")]
impl SshKeyController {
    fn new(service: Arc<SshKeyService>) -> Self {
        Self { service }
    }

    #[get]
    async fn list(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerReadPermission>,
    ) -> Result<Json<Vec<SshKeyResponseDto>>, ApiError> {
        self.service
            .list()
            .await
            .map(|items| items.into_iter().map(SshKeyResponseDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/{id}")]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerReadPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<SshKeyResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(SshKeyResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        ValidatedJson(body): ValidatedJson<CreateSshKeyDto>,
    ) -> Result<(StatusCode, Json<SshKeyResponseDto>), ApiError> {
        self.service
            .create(body)
            .await
            .map(SshKeyResponseDto::from)
            .map(|key| (StatusCode::CREATED, Json(key)))
            .map_err(map_sqlx_error)
    }

    #[post("/generate")]
    async fn generate(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        ValidatedJson(body): ValidatedJson<GenerateSshKeyDto>,
    ) -> Result<(StatusCode, Json<SshKeyResponseDto>), ApiError> {
        self.service
            .generate(body.name, body.description, &body.key_type)
            .await
            .map(SshKeyResponseDto::from)
            .map(|key| (StatusCode::CREATED, Json(key)))
            .map_err(map_sqlx_error)
    }

    #[post("/generate-pair")]
    async fn generate_pair(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Json(body): Json<crate::api::dto::ssh_key::GeneratePairRequestDto>,
    ) -> Result<Json<crate::api::dto::ssh_key::GeneratePairResponseDto>, ApiError> {
        let (private_key, public_key) = crate::utils::ssh::generate_keypair(&body.key_type)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        Ok(Json(crate::api::dto::ssh_key::GeneratePairResponseDto { private_key, public_key }))
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchSshKeyDto>,
    ) -> Result<Json<SshKeyResponseDto>, ApiError> {
        self.service
            .patch(id, body)
            .await
            .map(SshKeyResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/{id}/mark-used")]
    async fn mark_used(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<SshKeyResponseDto>, ApiError> {
        self.service
            .mark_used(id)
            .await
            .map(SshKeyResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerDeletePermission>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .delete(id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_sqlx_error)
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "ssh key not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        sqlx::Error::Database(ref database_error) if database_error.is_foreign_key_violation() => {
            (
                StatusCode::BAD_REQUEST,
                "Cannot delete SSH Key: It is currently assigned to active servers or git repositories.".into(),
            )
        }
        other => {
            tracing::error!(error = %other, "ssh key database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
