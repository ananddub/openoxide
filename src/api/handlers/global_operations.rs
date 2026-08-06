use crate::{
    services::global_operations::{
        BulkDeploymentRequest, BulkDeploymentResult, GlobalOperationsService, GlobalResourceDto,
        GlobalSearchOptions, ServerDependencyView,
    },
    utils::jwt::claim::Claims,
};
use auto_route::controller;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize, poem_openapi::Object)]
struct GlobalSearchQuery {
    query: String,
    limit: Option<i64>,
}

pub struct GlobalOperationsController {
    service: Arc<GlobalOperationsService>,
}

#[controller("/global")]
impl GlobalOperationsController {
    fn new(service: Arc<GlobalOperationsService>) -> Self {
        Self { service }
    }
    #[get("/search")]
    async fn search(
        &self,
        _claims: Claims,
        Query(query): Query<GlobalSearchQuery>,
    ) -> Result<Json<Vec<GlobalResourceDto>>, (StatusCode, String)> {
        if query.query.trim().is_empty() {
            return Err((StatusCode::BAD_REQUEST, "query is required".into()));
        }
        self.service
            .search(GlobalSearchOptions {
                query: query.query,
                limit: query.limit.unwrap_or(50).clamp(1, 500),
            })
            .await
            .map(Json)
            .map_err(map_error)
    }
    #[post("/deployments/queue/cleanup")]
    async fn cleanup_queue(
        &self,
        _claims: Claims,
    ) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
        self.service
            .cleanup_deployment_queue()
            .await
            .map(|cancelled| Json(serde_json::json!({"cancelled": cancelled})))
            .map_err(map_error)
    }
    #[post("/deployments/bulk")]
    async fn bulk_deployments(
        &self,
        _claims: Claims,
        Json(body): Json<BulkDeploymentRequest>,
    ) -> Result<Json<Vec<BulkDeploymentResult>>, (StatusCode, String)> {
        self.service
            .bulk_deployments(body)
            .await
            .map(Json)
            .map_err(map_error)
    }
    #[get("/servers/{server_id}/dependencies")]
    async fn server_dependencies(
        &self,
        _claims: Claims,
        Path(server_id): Path<i64>,
    ) -> Result<Json<ServerDependencyView>, (StatusCode, String)> {
        self.service
            .server_dependencies(server_id)
            .await
            .map(Json)
            .map_err(map_error)
    }
}
fn map_error(error: sqlx::Error) -> (StatusCode, String) {
    if let sqlx::Error::Protocol(message) = error {
        return (StatusCode::BAD_REQUEST, message);
    }
    tracing::error!(%error, "global operation failed");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "global operation failed".into(),
    )
}
