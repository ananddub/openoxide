use axum::http::StatusCode;

pub(super) type ApiError = (StatusCode, String);

pub(super) fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "server or SSH key not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_foreign_key_violation() => {
            (StatusCode::NOT_FOUND, "ssh key not found".into())
        }
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        sqlx::Error::Database(ref database_error) if database_error.is_check_violation() => {
            (StatusCode::BAD_REQUEST, database_error.message().into())
        }
        sqlx::Error::Protocol(message) => (StatusCode::CONFLICT, message),
        other => {
            tracing::error!(error = %other, "server database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
