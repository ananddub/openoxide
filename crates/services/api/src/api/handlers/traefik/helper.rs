use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    TraefikFileContent, TraefikFileNode, TraefikFileNodeType, TraefikFileTreeNode, TraefikHealth,
    TraefikLogEntry, TraefikMiddleware, TraefikMiddlewareKind, TraefikRequestsStatus,
    TraefikVersion,
};
use crate::services::networking::traefik::{
    StructuredMiddlewareResponseDto, TraefikFileContentDto, TraefikFileNodeDto,
    TraefikFileTreeNodeDto, TraefikHealthResponseDto, TraefikLogEntryDto, TraefikRequestsStatusDto,
    TraefikVersionDto,
};

pub fn map_error(error: String) -> ConnectError {
    ConnectError::new(ErrorCode::Internal, error)
}

pub fn map_bad_request(error: String) -> ConnectError {
    ConnectError::invalid_argument(error)
}

// `StructuredMiddlewareDto.kind` is an enum from a private module; the typed
// request is converted to that dto through its serde (SCREAMING_SNAKE_CASE)
// shape, with the concrete type inferred by the service call.
pub fn typed_dto<T: serde::de::DeserializeOwned>(
    value: serde_json::Value,
) -> Result<T, ConnectError> {
    serde_json::from_value(value).map_err(|error| ConnectError::invalid_argument(error.to_string()))
}

pub fn middleware_kind_name(
    value: ::buffa::EnumValue<TraefikMiddlewareKind>,
) -> Result<&'static str, ConnectError> {
    match value {
        ::buffa::EnumValue::Known(TraefikMiddlewareKind::StripPrefix) => Ok("STRIP_PREFIX"),
        ::buffa::EnumValue::Known(TraefikMiddlewareKind::AddPrefix) => Ok("ADD_PREFIX"),
        ::buffa::EnumValue::Known(TraefikMiddlewareKind::RedirectScheme) => Ok("REDIRECT_SCHEME"),
        ::buffa::EnumValue::Known(TraefikMiddlewareKind::Compress) => Ok("COMPRESS"),
        ::buffa::EnumValue::Known(TraefikMiddlewareKind::RateLimit) => Ok("RATE_LIMIT"),
        ::buffa::EnumValue::Known(TraefikMiddlewareKind::IpAllowList) => Ok("IP_ALLOW_LIST"),
        ::buffa::EnumValue::Known(TraefikMiddlewareKind::RequestHeaders) => Ok("REQUEST_HEADERS"),
        ::buffa::EnumValue::Known(TraefikMiddlewareKind::ResponseHeaders) => Ok("RESPONSE_HEADERS"),
        _ => Err(ConnectError::invalid_argument(
            "unsupported middleware kind",
        )),
    }
}

pub fn file_node_to_proto(dto: TraefikFileNodeDto) -> TraefikFileNode {
    TraefikFileNode {
        name: dto.name,
        relative_path: dto.relative_path,
        size: dto.size,
        is_readonly: dto.is_readonly,
        modified_at: dto.modified_at,
    }
}

pub fn file_tree_node_to_proto(dto: TraefikFileTreeNodeDto) -> TraefikFileTreeNode {
    TraefikFileTreeNode {
        name: dto.name,
        relative_path: dto.relative_path,
        node_type: ::buffa::EnumValue::Known(node_type_to_proto(&dto.node_type)),
        size: dto.size,
        is_readonly: dto.is_readonly,
        modified_at: dto.modified_at,
        children: dto
            .children
            .into_iter()
            .map(file_tree_node_to_proto)
            .collect(),
    }
}

fn node_type_to_proto(value: &str) -> TraefikFileNodeType {
    match value {
        "file" => TraefikFileNodeType::File,
        "directory" => TraefikFileNodeType::Directory,
        _ => TraefikFileNodeType::Unspecified,
    }
}

pub fn file_content_to_proto(path: String, dto: TraefikFileContentDto) -> TraefikFileContent {
    TraefikFileContent {
        path,
        content: dto.content,
        is_readonly: dto.is_readonly,
    }
}

pub fn health_to_proto(dto: TraefikHealthResponseDto) -> TraefikHealth {
    TraefikHealth {
        is_healthy: dto.is_healthy,
        rawdata_status: dto.rawdata_status,
        configuration_errors: dto.configuration_errors,
    }
}

pub fn requests_status_to_proto(dto: TraefikRequestsStatusDto) -> TraefikRequestsStatus {
    TraefikRequestsStatus {
        is_active: dto.is_active,
        log_path: dto.log_path,
        cron_expression: dto.cron_expression,
    }
}

pub fn log_entry_to_proto(entry: TraefikLogEntryDto) -> TraefikLogEntry {
    TraefikLogEntry {
        timestamp: entry.timestamp,
        client_ip: entry.client_ip,
        method: entry.method,
        path: entry.path,
        status: u32::from(entry.status),
        duration_ms: entry.duration_ms,
        service_name: entry.service_name,
        router_name: entry.router_name,
    }
}

pub fn version_to_proto(dto: TraefikVersionDto) -> TraefikVersion {
    TraefikVersion {
        server_id: dto.server_id,
        current_image: dto.current_image,
        desired_version: dto.desired_version,
    }
}

pub fn middleware_to_proto(dto: StructuredMiddlewareResponseDto) -> TraefikMiddleware {
    TraefikMiddleware {
        reference: dto.reference,
        labels: dto.labels.into_iter().collect(),
    }
}
