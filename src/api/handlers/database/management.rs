use crate::core::middleware::permission::{CanCreate, CanDelete, CanRead, CanUpdate, Database};
use std::{str::FromStr, sync::Arc};

use auto_route::controller;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use super::ApiError;
use crate::{
    api::dto::database::{
        DatabaseArchiveDto, DatabaseConnectionDto, DatabaseCredentialRotationDto,
        DatabaseExportQueryDto, DatabaseMountResponseDto, DatabaseResponseDto,
        DatabaseSwarmSettingsDto, DatabaseValidationDto, ImportDatabaseDto,
        PostgresAdvancedConfigDto, PostgresAdvancedConfigResponseDto, RotateDatabaseCredentialsDto,
        UpsertDatabaseMountDto,
    },
    core::middleware::{permission::RequirePermission, validator::ValidatedJson},
    services::database::{DatabaseKind, DatabaseService},
};

pub struct DatabaseManagementController {
    service: Arc<DatabaseService>,
    mounts: Arc<crate::services::database::mount::DatabaseMountService>,
    swarm: Arc<crate::services::database::swarm_settings::DatabaseSwarmSettingsService>,
}

#[controller("/database-management")]
impl DatabaseManagementController {
    fn new(
        service: Arc<DatabaseService>,
        mounts: Arc<crate::services::database::mount::DatabaseMountService>,
        swarm: Arc<crate::services::database::swarm_settings::DatabaseSwarmSettingsService>,
    ) -> Self {
        Self {
            service,
            mounts,
            swarm,
        }
    }

    #[get("/{kind}/{id}/swarm-settings")]
    async fn get_swarm_settings(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanRead>,
        Path((kind, id)): Path<(String, i64)>,
    ) -> Result<Json<DatabaseSwarmSettingsDto>, ApiError> {
        self.swarm
            .get(parse_kind(&kind)?, id)
            .await
            .map(Json)
            .map_err(super::map_sqlx_error)
    }

    #[put("/{kind}/{id}/swarm-settings")]
    async fn update_swarm_settings(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanUpdate>,
        Path((kind, id)): Path<(String, i64)>,
        ValidatedJson(body): ValidatedJson<DatabaseSwarmSettingsDto>,
    ) -> Result<Json<DatabaseSwarmSettingsDto>, ApiError> {
        let response = body.clone();
        self.swarm
            .update(parse_kind(&kind)?, id, body)
            .await
            .map_err(super::map_sqlx_error)?;
        Ok(Json(response))
    }

    #[get("/{kind}/{id}/mounts")]
    #[live(table = "mounts")]
    async fn list_mounts(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanRead>,
        Path((kind, id)): Path<(String, i64)>,
    ) -> Result<Json<Vec<DatabaseMountResponseDto>>, ApiError> {
        self.mounts
            .list(parse_kind(&kind)?, id)
            .await
            .map(|items| items.into_iter().map(Into::into).collect())
            .map(Json)
            .map_err(super::map_sqlx_error)
    }

    #[post("/{kind}/{id}/mounts")]
    async fn create_mount(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanCreate>,
        Path((kind, id)): Path<(String, i64)>,
        ValidatedJson(body): ValidatedJson<UpsertDatabaseMountDto>,
    ) -> Result<(StatusCode, Json<DatabaseMountResponseDto>), ApiError> {
        self.mounts
            .create(parse_kind(&kind)?, id, body)
            .await
            .map(Into::into)
            .map(|item| (StatusCode::CREATED, Json(item)))
            .map_err(super::map_sqlx_error)
    }

    #[put("/{kind}/{database_id}/mounts/{id}")]
    async fn update_mount(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanUpdate>,
        Path((kind, database_id, id)): Path<(String, i64, i64)>,
        ValidatedJson(body): ValidatedJson<UpsertDatabaseMountDto>,
    ) -> Result<Json<DatabaseMountResponseDto>, ApiError> {
        self.mounts
            .update(parse_kind(&kind)?, database_id, id, body)
            .await
            .map(Into::into)
            .map(Json)
            .map_err(super::map_sqlx_error)
    }

    #[delete("/{kind}/{database_id}/mounts/{id}")]
    async fn delete_mount(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanDelete>,
        Path((kind, database_id, id)): Path<(String, i64, i64)>,
    ) -> Result<StatusCode, ApiError> {
        match self
            .mounts
            .delete(parse_kind(&kind)?, database_id, id)
            .await
        {
            Ok(true) => Ok(StatusCode::NO_CONTENT),
            Ok(false) => Err((StatusCode::NOT_FOUND, "database mount not found".into())),
            Err(error) => Err(super::map_sqlx_error(error)),
        }
    }

    #[get("/{kind}/{id}/dependencies")]
    #[live(tables = ["postgres_dbs", "mysql_dbs", "mariadb_dbs", "mongo_dbs", "redis_dbs", "libsql_dbs", "deployments", "schedules", "backups"])]
    async fn dependencies(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanRead>,
        Path((kind, id)): Path<(String, i64)>,
    ) -> Result<Json<crate::repository::ResourceDependencyCounts>, ApiError> {
        self.service
            .dependencies(parse_kind(&kind)?, id)
            .await
            .map(Json)
            .map_err(super::map_sqlx_error)
    }

    #[get("/{kind}/{id}/export")]
    async fn export(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanRead>,
        Path((kind, id)): Path<(String, i64)>,
        Query(query): Query<DatabaseExportQueryDto>,
    ) -> Result<Json<DatabaseArchiveDto>, ApiError> {
        let bundle = self
            .service
            .export(
                parse_kind(&kind)?,
                id,
                query.include_secrets.unwrap_or(false),
            )
            .await
            .map_err(super::map_sqlx_error)?;
        Ok(Json(DatabaseArchiveDto {
            format: "openoxide.database+json".into(),
            schema_version: i64::from(bundle.schema_version),
            archive: serde_json::to_string_pretty(&bundle)
                .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?,
        }))
    }

    #[post("/import")]
    async fn import(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanUpdate>,
        ValidatedJson(body): ValidatedJson<ImportDatabaseDto>,
    ) -> Result<(StatusCode, Json<DatabaseResponseDto>), ApiError> {
        self.service
            .import(body)
            .await
            .map(DatabaseResponseDto::from)
            .map(|item| (StatusCode::CREATED, Json(item)))
            .map_err(super::map_sqlx_error)
    }

    #[get("/{kind}/{id}/connection")]
    #[live(tables = ["postgres_dbs", "mysql_dbs", "mariadb_dbs", "mongo_dbs", "redis_dbs", "libsql_dbs"])]
    async fn connection(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanRead>,
        Path((kind, id)): Path<(String, i64)>,
    ) -> Result<Json<DatabaseConnectionDto>, ApiError> {
        self.service
            .connection_details(parse_kind(&kind)?, id)
            .await
            .map(Json)
            .map_err(super::map_sqlx_error)
    }

    #[post("/{kind}/{id}/credentials/rotate")]
    async fn rotate_credentials(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanUpdate>,
        Path((kind, id)): Path<(String, i64)>,
        ValidatedJson(body): ValidatedJson<RotateDatabaseCredentialsDto>,
    ) -> Result<Json<DatabaseCredentialRotationDto>, ApiError> {
        self.service
            .rotate_credentials(parse_kind(&kind)?, id, body.password)
            .await
            .map(Json)
            .map_err(super::map_sqlx_error)
    }

    #[post("/{kind}/{id}/validate")]
    async fn validate(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanRead>,
        Path((kind, id)): Path<(String, i64)>,
    ) -> Result<Json<DatabaseValidationDto>, ApiError> {
        self.service
            .validate_configuration(parse_kind(&kind)?, id)
            .await
            .map(Json)
            .map_err(super::map_sqlx_error)
    }

    #[get("/postgres/{id}/advanced-config")]
    #[live(table = "postgres_dbs")]
    async fn get_postgres_advanced_config(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanRead>,
        Path(id): Path<i64>,
    ) -> Result<Json<PostgresAdvancedConfigResponseDto>, ApiError> {
        self.service
            .get_postgres_advanced_config(id)
            .await
            .map(Json)
            .map_err(super::map_sqlx_error)
    }

    #[put("/postgres/{id}/advanced-config")]
    async fn update_postgres_advanced_config(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanUpdate>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PostgresAdvancedConfigDto>,
    ) -> Result<Json<PostgresAdvancedConfigResponseDto>, ApiError> {
        self.service
            .update_postgres_advanced_config(id, body)
            .await
            .map(Json)
            .map_err(super::map_sqlx_error)
    }
}

fn parse_kind(value: &str) -> Result<DatabaseKind, ApiError> {
    DatabaseKind::from_str(value).map_err(|_| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            format!("unsupported database kind: {value}"),
        )
    })
}
