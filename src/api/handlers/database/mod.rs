use axum::Json;
use axum::http::StatusCode;

pub type ApiError = (StatusCode, String);

pub fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "database not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_foreign_key_violation() => {
            (StatusCode::NOT_FOUND, "related resource not found".into())
        }
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        sqlx::Error::Database(ref database_error) if database_error.is_check_violation() => {
            (StatusCode::BAD_REQUEST, database_error.message().into())
        }
        other => {
            tracing::error!(error = %other, "managed database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}

pub async fn run_operation(
    service: &crate::services::database::DatabaseService,
    kind: crate::services::database::DatabaseKind,
    id: i64,
    operation: crate::services::database::DatabaseOperation,
) -> Result<
    (
        StatusCode,
        Json<crate::api::dto::database::DatabaseOperationResponseDto>,
    ),
    ApiError,
> {
    service
        .run_operation(kind, id, operation)
        .await
        .map(crate::api::dto::database::DatabaseOperationResponseDto::from)
        .map(|response| (StatusCode::ACCEPTED, Json(response)))
        .map_err(map_sqlx_error)
}

pub mod libsql;
pub mod mariadb;
pub mod mongo;
pub mod mysql;
pub mod postgres;
pub mod redis;
