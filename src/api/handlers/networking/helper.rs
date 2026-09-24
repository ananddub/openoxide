use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    NetworkingCdnProvider as ProtoCdnProvider, NetworkingDomainDiagnostic,
    NetworkingRootNetworkStatus,
};

// The service input DTOs (`CdnPurgeDto`, `DomainDiagnosticDto`) are declared in
// private modules of `services::networking::diagnostics`, so typed requests are
// handed to them through their serde shape with the concrete type inferred by
// the service call — no JSON parsing of the wire request happens here.
pub fn typed_dto<T: serde::de::DeserializeOwned>(
    value: serde_json::Value,
) -> Result<T, ConnectError> {
    serde_json::from_value(value).map_err(|error| ConnectError::invalid_argument(error.to_string()))
}

pub fn map_error(error: String) -> ConnectError {
    tracing::warn!(%error, "networking operation failed");
    ConnectError::new(ErrorCode::Internal, error)
}

/// Wire spelling of `CdnProvider` (SCREAMING_SNAKE_CASE serde tag).
pub fn proto_to_cdn_provider(
    value: ::buffa::EnumValue<ProtoCdnProvider>,
) -> Result<&'static str, ConnectError> {
    match value {
        ::buffa::EnumValue::Known(ProtoCdnProvider::Cloudflare) => Ok("CLOUDFLARE"),
        ::buffa::EnumValue::Known(ProtoCdnProvider::Fastly) => Ok("FASTLY"),
        ::buffa::EnumValue::Known(ProtoCdnProvider::Bunny) => Ok("BUNNY"),
        _ => Err(ConnectError::invalid_argument("unsupported CDN provider")),
    }
}

// The response DTOs (`DomainDiagnosticResponseDto`, `RootNetworkStatusDto`)
// live in private service modules, so their names cannot be written here.
// Both derive `Serialize`, so the mapping reads them through their serde
// shape at the one call boundary where the concrete type is inferred.

pub fn diagnostic_to_proto(dto: &impl serde::Serialize) -> NetworkingDomainDiagnostic {
    let value = serde_json::to_value(dto).unwrap_or(serde_json::Value::Null);
    NetworkingDomainDiagnostic {
        host: json_str(&value, "host"),
        resolved_addresses: value["resolved_addresses"]
            .as_array()
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|entry| entry.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default(),
        dns_ok: json_bool(&value, "dns_ok"),
        tcp_ok: json_bool(&value, "tcp_ok"),
        http_ok: json_bool(&value, "http_ok"),
        http_status: value["http_status"].as_u64().and_then(|n| u32::try_from(n).ok()),
        error: value["error"].as_str().map(str::to_owned),
        ..Default::default()
    }
}

pub fn root_network_to_proto(dto: &impl serde::Serialize) -> NetworkingRootNetworkStatus {
    let value = serde_json::to_value(dto).unwrap_or(serde_json::Value::Null);
    NetworkingRootNetworkStatus {
        name: json_str(&value, "name"),
        exists: json_bool(&value, "exists"),
        healthy: json_bool(&value, "healthy"),
        repaired: json_bool(&value, "repaired"),
        driver: value["driver"].as_str().map(str::to_owned),
        scope: value["scope"].as_str().map(str::to_owned),
        attachable: value["attachable"].as_bool(),
        connected_resources: value["connected_resources"].as_i64().unwrap_or_default(),
        issue: value["issue"].as_str().map(str::to_owned),
        ..Default::default()
    }
}

fn json_str(value: &serde_json::Value, key: &str) -> String {
    value[key].as_str().unwrap_or_default().to_owned()
}

fn json_bool(value: &serde_json::Value, key: &str) -> bool {
    value[key].as_bool().unwrap_or_default()
}
