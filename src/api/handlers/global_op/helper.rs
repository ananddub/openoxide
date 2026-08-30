use connectrpc::{ConnectError, ErrorCode};
use serde::Deserialize;

use crate::services::platform::global_ops::ContainerActionDto;

pub fn api_error(error: String) -> ConnectError {
    if error.starts_with("invalid") {
        ConnectError::invalid_argument(error)
    } else {
        ConnectError::new(ErrorCode::FailedPrecondition, error)
    }
}

// ── Helper DTOs for deserializing ResourceRequest payloads ────────────

#[derive(Debug, Deserialize)]
pub struct InspectDto {
    pub server_id: Option<i64>,
    pub container_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DownloadFileDto {
    pub server_id: Option<i64>,
    pub container_id: String,
    pub path: String,
}

#[derive(Debug, serde::Serialize)]
pub struct DownloadFileResponseDto {
    pub content: String,
    pub path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContainerActionRequestDto {
    pub action: ContainerActionDto,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContainerRemoveRequestDto {
    pub force: bool,
    pub volumes: bool,
}
