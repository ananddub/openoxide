use std::sync::Arc;

use auto_route::controller;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use poem_openapi::Object;
use serde::Deserialize;

use crate::{
    api::dto::docker_management::{
        ContainerActionRequestDto, ContainerRemoveRequestDto, DockerActionResponseDto,
        DockerPruneRequestDto, DockerPruneResponseDto,
    },
    core::middleware::{
        permission::{RequirePermission, ServerCreatePermission, ServerReadPermission},
        validator::ValidatedJson,
    },
    services::docker_management::DockerManagementService,
    utils::docker::{ContainerSummary, ImageSummary, NetworkSummary, VolumeSummary},
};

type ApiError = (StatusCode, String);

#[derive(Debug, Deserialize, Object)]
struct DockerTargetQuery {
    server_id: Option<i64>,
}

pub struct DockerManagementController {
    service: Arc<DockerManagementService>,
}

#[controller("/docker")]
impl DockerManagementController {
    fn new(service: Arc<DockerManagementService>) -> Self {
        Self { service }
    }

    #[get("/containers")]
    async fn containers(
        &self,
        RequirePermission(_, _): RequirePermission<ServerReadPermission>,
        Query(query): Query<DockerTargetQuery>,
    ) -> Result<Json<Vec<ContainerSummary>>, ApiError> {
        self.service
            .containers(query.server_id)
            .await
            .map(Json)
            .map_err(api_error)
    }

    #[get("/images")]
    async fn images(
        &self,
        RequirePermission(_, _): RequirePermission<ServerReadPermission>,
        Query(query): Query<DockerTargetQuery>,
    ) -> Result<Json<Vec<ImageSummary>>, ApiError> {
        self.service
            .images(query.server_id)
            .await
            .map(Json)
            .map_err(api_error)
    }

    #[get("/networks")]
    async fn networks(
        &self,
        RequirePermission(_, _): RequirePermission<ServerReadPermission>,
        Query(query): Query<DockerTargetQuery>,
    ) -> Result<Json<Vec<NetworkSummary>>, ApiError> {
        self.service
            .networks(query.server_id)
            .await
            .map(Json)
            .map_err(api_error)
    }

    #[get("/volumes")]
    async fn volumes(
        &self,
        RequirePermission(_, _): RequirePermission<ServerReadPermission>,
        Query(query): Query<DockerTargetQuery>,
    ) -> Result<Json<Vec<VolumeSummary>>, ApiError> {
        self.service
            .volumes(query.server_id)
            .await
            .map(Json)
            .map_err(api_error)
    }

    #[get("/containers/{id}/inspect")]
    async fn inspect_container(
        &self,
        RequirePermission(_, _): RequirePermission<ServerReadPermission>,
        Path(id): Path<String>,
        Query(query): Query<DockerTargetQuery>,
    ) -> Result<Json<serde_json::Value>, ApiError> {
        self.service
            .container_inspect(query.server_id, &id)
            .await
            .map(Json)
            .map_err(api_error)
    }

    #[get("/images/{id}/inspect")]
    async fn inspect_image(
        &self,
        RequirePermission(_, _): RequirePermission<ServerReadPermission>,
        Path(id): Path<String>,
        Query(query): Query<DockerTargetQuery>,
    ) -> Result<Json<serde_json::Value>, ApiError> {
        self.service
            .image_inspect(query.server_id, &id)
            .await
            .map(Json)
            .map_err(api_error)
    }

    #[get("/networks/{id}/inspect")]
    async fn inspect_network(
        &self,
        RequirePermission(_, _): RequirePermission<ServerReadPermission>,
        Path(id): Path<String>,
        Query(query): Query<DockerTargetQuery>,
    ) -> Result<Json<serde_json::Value>, ApiError> {
        self.service
            .network_inspect(query.server_id, &id)
            .await
            .map(Json)
            .map_err(api_error)
    }

    #[get("/volumes/{id}/inspect")]
    async fn inspect_volume(
        &self,
        RequirePermission(_, _): RequirePermission<ServerReadPermission>,
        Path(id): Path<String>,
        Query(query): Query<DockerTargetQuery>,
    ) -> Result<Json<serde_json::Value>, ApiError> {
        self.service
            .volume_inspect(query.server_id, &id)
            .await
            .map(Json)
            .map_err(api_error)
    }

    #[post("/containers/{id}/action")]
    async fn container_action(
        &self,
        RequirePermission(_, _): RequirePermission<ServerCreatePermission>,
        Path(id): Path<String>,
        Query(query): Query<DockerTargetQuery>,
        ValidatedJson(body): ValidatedJson<ContainerActionRequestDto>,
    ) -> Result<Json<DockerActionResponseDto>, ApiError> {
        self.service
            .container_action(query.server_id, &id, body.action)
            .await
            .map(Json)
            .map_err(api_error)
    }

    #[delete("/containers/{id}")]
    async fn remove_container(
        &self,
        RequirePermission(_, _): RequirePermission<ServerCreatePermission>,
        Path(id): Path<String>,
        Query(query): Query<DockerTargetQuery>,
        ValidatedJson(body): ValidatedJson<ContainerRemoveRequestDto>,
    ) -> Result<Json<DockerActionResponseDto>, ApiError> {
        self.service
            .remove_container(query.server_id, &id, body.force, body.volumes)
            .await
            .map(Json)
            .map_err(api_error)
    }

    #[get("/disk-usage")]
    async fn disk_usage(
        &self,
        RequirePermission(_, _): RequirePermission<ServerReadPermission>,
        Query(query): Query<DockerTargetQuery>,
    ) -> Result<Json<serde_json::Value>, ApiError> {
        self.service
            .disk_usage(query.server_id)
            .await
            .map(Json)
            .map_err(api_error)
    }

    #[post("/prune")]
    async fn prune(
        &self,
        RequirePermission(_, _): RequirePermission<ServerCreatePermission>,
        Query(query): Query<DockerTargetQuery>,
        ValidatedJson(body): ValidatedJson<DockerPruneRequestDto>,
    ) -> Result<Json<DockerPruneResponseDto>, ApiError> {
        self.service
            .prune(query.server_id, body)
            .await
            .map(Json)
            .map_err(api_error)
    }
}

fn api_error(error: String) -> ApiError {
    if error.starts_with("invalid") {
        (StatusCode::BAD_REQUEST, error)
    } else {
        (StatusCode::BAD_GATEWAY, error)
    }
}
