use auto_route::controller;
use axum::{Json, extract::Query, http::StatusCode};
use std::sync::Arc;

use crate::{
    api::dto::traefik::{
        StructuredMiddlewareDto, StructuredMiddlewareResponseDto, TraefikFileContentDto,
        TraefikFileNodeDto, TraefikFileQueryDto, TraefikFileTreeNodeDto, TraefikHealthResponseDto,
        TraefikRequestsStatusDto, TraefikStatsLogsQueryDto, TraefikStatsLogsResponseDto,
        TraefikToggleRequestsDto, TraefikVersionDto, TraefikWriteFileDto, UpdateTraefikVersionDto,
    },
    core::middleware::permission::{
        RequirePermission, TraefikReadPermission, TraefikWritePermission,
    },
    services::traefik::TraefikService,
};

type ApiError = (StatusCode, String);

pub struct TraefikController {
    service: Arc<TraefikService>,
}

#[controller("/traefik")]
impl TraefikController {
    fn new(service: Arc<TraefikService>) -> Self {
        Self { service }
    }

    #[get("/files")]
    async fn list_files(
        &self,
        RequirePermission(_claims, _): RequirePermission<TraefikReadPermission>,
        Query(params): Query<TraefikFileQueryDto>,
    ) -> Result<Json<Vec<TraefikFileNodeDto>>, ApiError> {
        let files = self
            .service
            .list_files(params.server_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
        Ok(Json(files))
    }

    #[get("/files/tree")]
    async fn list_file_tree(
        &self,
        RequirePermission(_claims, _): RequirePermission<TraefikReadPermission>,
        Query(params): Query<TraefikFileQueryDto>,
    ) -> Result<Json<Vec<TraefikFileTreeNodeDto>>, ApiError> {
        let files = self
            .service
            .list_file_tree(params.server_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
        Ok(Json(files))
    }

    #[get("/files/content")]
    async fn read_file_content(
        &self,
        RequirePermission(_claims, _): RequirePermission<TraefikReadPermission>,
        Query(params): Query<TraefikFileQueryDto>,
    ) -> Result<Json<TraefikFileContentDto>, ApiError> {
        let path = params.path.ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "File 'path' parameter is required".to_string(),
            )
        })?;

        let file_content = self
            .service
            .read_file(params.server_id, &path)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

        Ok(Json(file_content))
    }

    #[put("/files/content")]
    async fn write_file_content(
        &self,
        RequirePermission(_claims, _): RequirePermission<TraefikWritePermission>,
        Json(body): Json<TraefikWriteFileDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .write_file(body)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

        Ok(StatusCode::NO_CONTENT)
    }

    #[get("/health")]
    async fn check_health(
        &self,
        RequirePermission(_claims, _): RequirePermission<TraefikReadPermission>,
        Query(params): Query<TraefikFileQueryDto>,
    ) -> Result<Json<TraefikHealthResponseDto>, ApiError> {
        let health = self
            .service
            .check_health(params.server_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        Ok(Json(health))
    }

    #[get("/requests/status")]
    async fn get_requests_status(
        &self,
        RequirePermission(_claims, _): RequirePermission<TraefikReadPermission>,
        Query(params): Query<TraefikFileQueryDto>,
    ) -> Result<Json<TraefikRequestsStatusDto>, ApiError> {
        let status = self
            .service
            .have_activated_requests(params.server_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        Ok(Json(status))
    }

    #[post("/requests/toggle")]
    async fn toggle_requests(
        &self,
        RequirePermission(_claims, _): RequirePermission<TraefikWritePermission>,
        Json(body): Json<TraefikToggleRequestsDto>,
    ) -> Result<Json<bool>, ApiError> {
        let res = self
            .service
            .toggle_requests(body)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        Ok(Json(res))
    }

    #[get("/requests/logs")]
    async fn read_stats_logs(
        &self,
        RequirePermission(_claims, _): RequirePermission<TraefikReadPermission>,
        Query(params): Query<TraefikStatsLogsQueryDto>,
    ) -> Result<Json<TraefikStatsLogsResponseDto>, ApiError> {
        let logs = self
            .service
            .read_stats_logs(params)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        Ok(Json(logs))
    }

    #[get("/version")]
    async fn version(
        &self,
        RequirePermission(_claims, _): RequirePermission<TraefikReadPermission>,
        Query(params): Query<TraefikFileQueryDto>,
    ) -> Result<Json<TraefikVersionDto>, ApiError> {
        self.service
            .version(params.server_id)
            .await
            .map(Json)
            .map_err(|error| (StatusCode::BAD_GATEWAY, error))
    }

    #[post("/version")]
    async fn update_version(
        &self,
        RequirePermission(_claims, _): RequirePermission<TraefikWritePermission>,
        Json(body): Json<UpdateTraefikVersionDto>,
    ) -> Result<Json<TraefikVersionDto>, ApiError> {
        self.service
            .update_version(body)
            .await
            .map(Json)
            .map_err(|error| (StatusCode::BAD_GATEWAY, error))
    }

    #[post("/middlewares/render")]
    async fn render_middleware(
        &self,
        RequirePermission(_claims, _): RequirePermission<TraefikWritePermission>,
        Json(body): Json<StructuredMiddlewareDto>,
    ) -> Result<Json<StructuredMiddlewareResponseDto>, ApiError> {
        TraefikService::structured_middleware(body)
            .map(Json)
            .map_err(|error| (StatusCode::BAD_REQUEST, error))
    }
}
