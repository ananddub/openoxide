use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::tag::{AttachProjectTagDto, CreateTagDto, TagDto, UpdateTagDto},
    services::tag::TagService,
};

type ApiError = (StatusCode, String);

fn map_sqlx_error(err: sqlx::Error) -> ApiError {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub struct TagController {
    service: Arc<TagService>,
}

#[controller("/tags")]
impl TagController {
    fn new(service: Arc<TagService>) -> Self {
        Self { service }
    }

    #[get("")]
    #[live(tables = ["tags","project_tags"])]
    async fn list_all(&self) -> Result<Json<Vec<TagDto>>, ApiError> {
        let items = self.service.list_all().await.map_err(map_sqlx_error)?;

        Ok(Json(items.into_iter().map(TagDto::from).collect()))
    }

    #[get("/{id}")]
    #[live(tables = ["tags","project_tags"])]
    async fn get_by_id(&self, Path(id): Path<i64>) -> Result<Json<TagDto>, ApiError> {
        let tag = self.service.get_by_id(id).await.map_err(map_sqlx_error)?;

        match tag {
            Some(t) => Ok(Json(TagDto::from(t))),
            None => Err((StatusCode::NOT_FOUND, "Tag not found".to_string())),
        }
    }

    #[post("")]
    async fn create(&self, Json(body): Json<CreateTagDto>) -> Result<Json<TagDto>, ApiError> {
        let created = self.service.create(body).await.map_err(map_sqlx_error)?;

        Ok(Json(TagDto::from(created)))
    }

    #[patch("/{id}")]
    async fn update(
        &self,
        Path(id): Path<i64>,
        Json(body): Json<UpdateTagDto>,
    ) -> Result<Json<TagDto>, ApiError> {
        let updated = self
            .service
            .update(id, body)
            .await
            .map_err(map_sqlx_error)?;

        Ok(Json(TagDto::from(updated)))
    }

    #[delete("/{id}")]
    async fn delete(&self, Path(id): Path<i64>) -> Result<StatusCode, ApiError> {
        self.service
            .delete(id)
            .await
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(map_sqlx_error)
    }

    #[get("/project/{project_id}")]
    #[live(tables = ["tags","project_tags"])]
    async fn list_project_tags(
        &self,
        Path(project_id): Path<i64>,
    ) -> Result<Json<Vec<TagDto>>, ApiError> {
        let items = self
            .service
            .list_project_tags(project_id)
            .await
            .map_err(map_sqlx_error)?;

        Ok(Json(items.into_iter().map(TagDto::from).collect()))
    }

    #[post("/project/{project_id}")]
    async fn attach_project_tag(
        &self,
        Path(project_id): Path<i64>,
        Json(body): Json<AttachProjectTagDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .attach_project_tag(project_id, body.tag_id)
            .await
            .map(|_| StatusCode::CREATED)
            .map_err(map_sqlx_error)
    }

    #[delete("/project/{project_id}/{tag_id}")]
    async fn detach_project_tag(
        &self,
        Path((project_id, tag_id)): Path<(i64, i64)>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .detach_project_tag(project_id, tag_id)
            .await
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(map_sqlx_error)
    }
}
