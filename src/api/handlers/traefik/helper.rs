use connectrpc::{ConnectError, ErrorCode};

pub fn map_error(error: String) -> ConnectError {
    ConnectError::new(ErrorCode::Internal, error)
}

pub fn map_bad_request(error: String) -> ConnectError {
    ConnectError::invalid_argument(error)
}
