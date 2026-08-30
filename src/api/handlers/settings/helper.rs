use connectrpc::{ConnectError, ErrorCode};

pub fn map_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::Protocol(msg) => ConnectError::invalid_argument(msg.clone()),
        other => {
            tracing::error!(error = %other, "settings operation failed");
            ConnectError::new(ErrorCode::Internal, "settings operation failed")
        }
    }
}
