use std::sync::Arc;

use auto_route::controller;
use axum::{Json, http::StatusCode};

use crate::{
    api::dto::networking::{
        CdnPurgeDto, CdnPurgeResponseDto, DomainDiagnosticDto, DomainDiagnosticResponseDto,
        RootNetworkDto, RootNetworkStatusDto,
    },
    core::middleware::{
        permission::{AppCreatePermission, AppReadPermission, RequirePermission},
        validator::ValidatedJson,
    },
    services::networking::{CdnPurgeService, NetworkDiagnosticService, RootNetworkService},
};

type ApiError = (StatusCode, String);

pub struct NetworkingController {
    diagnostics: Arc<NetworkDiagnosticService>,
    root_network: Arc<RootNetworkService>,
    cdn: Arc<CdnPurgeService>,
}

#[controller("/networking")]
impl NetworkingController {
    fn new(
        diagnostics: Arc<NetworkDiagnosticService>,
        root_network: Arc<RootNetworkService>,
        cdn: Arc<CdnPurgeService>,
    ) -> Self {
        Self {
            diagnostics,
            root_network,
            cdn,
        }
    }

    #[post("/cdn/purge")]
    async fn purge_cdn(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        ValidatedJson(body): ValidatedJson<CdnPurgeDto>,
    ) -> Result<Json<CdnPurgeResponseDto>, ApiError> {
        self.cdn.purge(body).await.map(Json).map_err(map_error)
    }

    #[post("/domains/diagnose")]
    async fn diagnose_domain(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        ValidatedJson(body): ValidatedJson<DomainDiagnosticDto>,
    ) -> Json<DomainDiagnosticResponseDto> {
        Json(self.diagnostics.diagnose(body).await)
    }

    #[post("/root-network/diagnose")]
    async fn diagnose_root_network(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Json(body): Json<RootNetworkDto>,
    ) -> Result<Json<RootNetworkStatusDto>, ApiError> {
        self.root_network
            .diagnose(body.server_id)
            .await
            .map(Json)
            .map_err(map_error)
    }

    #[post("/root-network/repair")]
    async fn repair_root_network(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Json(body): Json<RootNetworkDto>,
    ) -> Result<Json<RootNetworkStatusDto>, ApiError> {
        self.root_network
            .repair(body.server_id)
            .await
            .map(Json)
            .map_err(map_error)
    }
}

fn map_error(error: String) -> ApiError {
    tracing::warn!(%error, "networking operation failed");
    (StatusCode::BAD_REQUEST, error)
}
