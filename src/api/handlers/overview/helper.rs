use connectrpc::{ConnectError, ErrorCode};

pub fn map_db_error(err: sqlx::Error) -> ConnectError {
    tracing::error!(error = %err, "overview database operation failed");
    ConnectError::new(ErrorCode::Internal, "overview database operation failed")
}
