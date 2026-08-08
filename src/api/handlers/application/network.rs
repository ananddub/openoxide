use crate::core::middleware::permission::{Application, CanCreate, CanRead};
use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::{
        application::network::{ApplicationNetworksResponseDto, UpdateApplicationNetworksDto},
        database_network::DatabaseNetworkResponseDto,
    },
    core::middleware::{permission::RequirePermission, validator::ValidatedJson},
    services::application::network::ApplicationNetworkService,
};

type ApiError = (StatusCode, String);

pub struct ApplicationNetworkController {
    service: Arc<ApplicationNetworkService>,
}

#[controller("/applications/{application_id}/networks")]
impl ApplicationNetworkController {
    fn new(service: Arc<ApplicationNetworkService>) -> Self {
        Self { service }
    }

    #[get]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(application_id): Path<i64>,
    ) -> Result<Json<ApplicationNetworksResponseDto>, ApiError> {
        self.service
            .get(application_id)
            .await
            .map(|(detach, networks)| {
                Json(ApplicationNetworksResponseDto {
                    application_id,
                    detach_rustploy_network: detach,
                    networks: networks
                        .into_iter()
                        .map(DatabaseNetworkResponseDto::from)
                        .collect(),
                })
            })
            .map_err(map_error)
    }

    #[put]
    async fn update(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanCreate>,
        Path(application_id): Path<i64>,
        ValidatedJson(body): ValidatedJson<UpdateApplicationNetworksDto>,
    ) -> Result<Json<ApplicationNetworksResponseDto>, ApiError> {
        self.service
            .update(application_id, body)
            .await
            .map(|(detach, networks)| {
                Json(ApplicationNetworksResponseDto {
                    application_id,
                    detach_rustploy_network: detach,
                    networks: networks
                        .into_iter()
                        .map(DatabaseNetworkResponseDto::from)
                        .collect(),
                })
            })
            .map_err(map_error)
    }
}

fn map_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "application not found".into()),
        sqlx::Error::Protocol(message) => (StatusCode::BAD_REQUEST, message),
        other => {
            tracing::error!(error=%other, "application network operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
