use axum::http::StatusCode;

pub(super) type ApiError = (StatusCode, String);

pub(super) fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "server or SSH key not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_foreign_key_violation() => {
            tracing::warn!(error = %database_error.message(), "foreign key violation");
            (StatusCode::NOT_FOUND, "ssh key not found".into())
        }
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            tracing::warn!(error = %database_error.message(), "unique constraint violation");
            (StatusCode::CONFLICT, database_error.message().into())
        }
        sqlx::Error::Database(ref database_error) if database_error.is_check_violation() => {
            tracing::warn!(error = %database_error.message(), "check constraint violation");
            (StatusCode::BAD_REQUEST, database_error.message().into())
        }
        sqlx::Error::Protocol(ref message) => {
            tracing::warn!(error = %message, "operation protocol/validation error");
            (StatusCode::CONFLICT, message.clone())
        }
        other => {
            tracing::error!(error = %other, "server database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
