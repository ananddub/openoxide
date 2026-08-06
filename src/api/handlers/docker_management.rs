use std::sync::Arc;

use auto_route::controller;
use axum::{
    Json,
    body::Body,
    extract::{Multipart, Path, Query},
    http::{HeaderValue, Response, StatusCode, header},
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

#[derive(Debug, Deserialize, Object)]
struct DockerFileQuery {
    server_id: Option<i64>,
    path: String,
}

pub struct DockerManagementController {
    service: Arc<DockerManagementService>,
}

#[controller("/docker")]
impl DockerManagementController {
    #[post("/containers/{id}/files")]
    async fn upload_container_file(
        &self,
        RequirePermission(_, _): RequirePermission<ServerCreatePermission>,
        Path(id): Path<String>,
        Query(query): Query<DockerTargetQuery>,
        mut multipart: Multipart,
    ) -> Result<Json<DockerActionResponseDto>, ApiError> {
        let mut destination = None;
        let mut filename = None;
        let mut bytes = None;
        while let Some(field) = multipart
            .next_field()
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
        {
            match field.name() {
                Some("destination") => {
                    destination = Some(
                        field
                            .text()
                            .await
                            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?,
                    )
                }
                Some("file") => {
                    filename = field.file_name().map(str::to_owned);
                    bytes = Some(
                        field
                            .bytes()
                            .await
                            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?,
                    );
                }
                _ => {}
            }
        }
        let destination =
            destination.ok_or((StatusCode::BAD_REQUEST, "destination is required".into()))?;
        let bytes = bytes.ok_or((StatusCode::BAD_REQUEST, "file is required".into()))?;
        self.service
            .upload_container_bytes(
                query.server_id,
                &id,
                &destination,
                filename.as_deref().unwrap_or("upload.bin"),
                &bytes,
            )
            .await
            .map(Json)
            .map_err(api_error)
    }

    #[get("/containers/{id}/files")]
    async fn download_container_file(
        &self,
        RequirePermission(_, _): RequirePermission<ServerReadPermission>,
        Path(id): Path<String>,
        Query(query): Query<DockerFileQuery>,
    ) -> Result<Response<Body>, ApiError> {
        let bytes = self
            .service
            .download_container_bytes(query.server_id, &id, &query.path)
            .await
            .map_err(api_error)?;
        let filename = query
            .path
            .rsplit('/')
            .next()
            .unwrap_or("download.bin")
            .replace(['\r', '\n', '"'], "_");
        let mut response = Response::new(Body::from(bytes));
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );
        response.headers_mut().insert(
            header::CONTENT_DISPOSITION,
            HeaderValue::from_str(&format!("attachment; filename=\"{filename}\""))
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?,
        );
        Ok(response)
    }
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
