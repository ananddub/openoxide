use connectrpc::{
    ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult,
};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, ServerConnectionMode, ServerPrivateNetwork as ServerPrivateNetworkProto,
    ServerPrivateNetworkDeleteRequest, ServerPrivateNetworkGetRequest,
    ServerPrivateNetworkGetResponse, ServerPrivateNetworkHealth as ServerPrivateNetworkHealthProto,
    ServerPrivateNetworkHealthRequest, ServerPrivateNetworkHealthResponse,
    ServerPrivateNetworkHealthStatus, ServerPrivateNetworkProvider,
    ServerPrivateNetworkReSetupRequest, ServerPrivateNetworkReSetupResponse,
    ServerPrivateNetworkRepairRequest, ServerPrivateNetworkRepairResponse,
    ServerPrivateNetworkRotateKeysRequest, ServerPrivateNetworkRotateKeysResponse,
    ServerPrivateNetworkRotationState, ServerPrivateNetworkService, ServerPrivateNetworkServiceExt,
    ServerPrivateNetworkSetupRequest, ServerPrivateNetworkSetupResponse,
    ServerPrivateNetworkStatus, ServerPrivateNetworkTeardownRequest,
    ServerPrivateNetworkUpdateRequest, ServerPrivateNetworkUpdateResponse,
};
use crate::services::infrastructure::server::ServerPrivateNetworkService as AppServerPrivateNetworkService;

use super::init::ServerPrivateNetworkController;

fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("server private network not found"),
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => {
            tracing::warn!(error = %e.message(), "foreign key violation");
            ConnectError::not_found("referenced resource not found")
        }
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            tracing::warn!(error = %e.message(), "unique constraint violation");
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        other => {
            tracing::error!(error = %other, "server private network database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

// The private-network DTO + its enums live in a private service module and
// cannot be named from the handler. They all derive `Serialize` with
// SCREAMING_SNAKE_CASE, so map them to the generated proto enums by their
// serialized name without ever writing the concrete type name.
fn enum_str(value: impl serde::Serialize) -> String {
    serde_json::to_value(&value)
        .ok()
        .and_then(|v| v.as_str().map(str::to_owned))
        .unwrap_or_default()
}

fn connection_mode_to_proto(value: impl serde::Serialize) -> ServerConnectionMode {
    match enum_str(value).as_str() {
        "DIRECT_SSH" => ServerConnectionMode::DirectSsh,
        "MANAGED_WIREGUARD" => ServerConnectionMode::ManagedWireguard,
        "EXTERNAL_PRIVATE_NETWORK" => ServerConnectionMode::ExternalPrivateNetwork,
        _ => ServerConnectionMode::Unspecified,
    }
}

fn provider_to_proto(value: impl serde::Serialize) -> ServerPrivateNetworkProvider {
    match enum_str(value).as_str() {
        "WIREGUARD" => ServerPrivateNetworkProvider::Wireguard,
        "TAILSCALE" => ServerPrivateNetworkProvider::Tailscale,
        "ZEROTIER" => ServerPrivateNetworkProvider::Zerotier,
        "NETBIRD" => ServerPrivateNetworkProvider::Netbird,
        "CUSTOM" => ServerPrivateNetworkProvider::Custom,
        _ => ServerPrivateNetworkProvider::Unspecified,
    }
}

fn status_to_proto(value: impl serde::Serialize) -> ServerPrivateNetworkStatus {
    match enum_str(value).as_str() {
        "DISABLED" => ServerPrivateNetworkStatus::Disabled,
        "CONFIGURING" => ServerPrivateNetworkStatus::Configuring,
        "ACTIVE" => ServerPrivateNetworkStatus::Active,
        "FAILED" => ServerPrivateNetworkStatus::Failed,
        _ => ServerPrivateNetworkStatus::Unspecified,
    }
}

fn health_status_to_proto(value: impl serde::Serialize) -> ServerPrivateNetworkHealthStatus {
    match enum_str(value).as_str() {
        "UNKNOWN" => ServerPrivateNetworkHealthStatus::Unknown,
        "HEALTHY" => ServerPrivateNetworkHealthStatus::Healthy,
        "DEGRADED" => ServerPrivateNetworkHealthStatus::Degraded,
        "UNREACHABLE" => ServerPrivateNetworkHealthStatus::Unreachable,
        "CONFIG_DRIFT" => ServerPrivateNetworkHealthStatus::ConfigDrift,
        _ => ServerPrivateNetworkHealthStatus::Unspecified,
    }
}

fn rotation_state_to_proto(value: impl serde::Serialize) -> ServerPrivateNetworkRotationState {
    match enum_str(value).as_str() {
        "IDLE" => ServerPrivateNetworkRotationState::Idle,
        "ROTATING" => ServerPrivateNetworkRotationState::Rotating,
        "ROLLING_BACK" => ServerPrivateNetworkRotationState::RollingBack,
        "FAILED" => ServerPrivateNetworkRotationState::Failed,
        _ => ServerPrivateNetworkRotationState::Unspecified,
    }
}

macro_rules! network_to_proto {
    ($dto:expr) => {{
        let dto = $dto;
        ServerPrivateNetworkProto {
            server_id: dto.server_id,
            connection_mode: ::buffa::EnumValue::Known(connection_mode_to_proto(
                dto.connection_mode,
            )),
            provider: dto
                .provider
                .map(|p| ::buffa::EnumValue::Known(provider_to_proto(p))),
            private_host: dto.private_host,
            tunnel_address: dto.tunnel_address,
            public_key: dto.public_key,
            endpoint: dto.endpoint,
            listen_port: dto.listen_port.map(u32::from),
            persistent_keepalive: dto.persistent_keepalive.map(u32::from),
            status: ::buffa::EnumValue::Known(status_to_proto(dto.status)),
            last_handshake_at: dto.last_handshake_at,
            config_version: dto.config_version,
            dns_name: dto.dns_name,
            routes: dto.routes,
            health_status: ::buffa::EnumValue::Known(health_status_to_proto(dto.health_status)),
            health_error: dto.health_error,
            last_health_check_at: dto.last_health_check_at,
            consecutive_failures: dto.consecutive_failures,
            rotation_state: ::buffa::EnumValue::Known(rotation_state_to_proto(dto.rotation_state)),
            updated_at: dto.updated_at,
        }
    }};
}

macro_rules! health_to_proto {
    ($dto:expr) => {{
        let dto = $dto;
        ServerPrivateNetworkHealthProto {
            status: ::buffa::EnumValue::Known(health_status_to_proto(dto.status)),
            interface_exists: dto.interface_exists,
            private_ssh_reachable: dto.private_ssh_reachable,
            latest_handshake_at: dto.latest_handshake_at,
            checked_at: dto.checked_at,
            error: dto.error,
        }
    }};
}

#[auto_grpc::service(connect = true, auth = true)]
impl ServerPrivateNetworkService for ServerPrivateNetworkController {
    #[permission(Server, CanRead)]
    async fn get_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerPrivateNetworkGetRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ServerPrivateNetworkGetResponse> + Send + use<'a>>
    {
        let network = self.service.get(request.id).await.map_err(map_db_error)?;
        Response::ok(ServerPrivateNetworkGetResponse {
            network: network
                .map(|dto| ::buffa::MessageField::some(network_to_proto!(dto)))
                .unwrap_or_default(),
        })
    }

    #[permission(Server, CanCreate)]
    async fn update_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerPrivateNetworkUpdateRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ServerPrivateNetworkUpdateResponse> + Send + use<'a>,
    > {
        // The update service consumes a JSON document (its DTO lives in a
        // private module); build it from the typed request instead of
        // forwarding a raw envelope payload.
        let connection_mode = match request.connection_mode {
            ::buffa::EnumValue::Known(ServerConnectionMode::DirectSsh) => "DIRECT_SSH",
            ::buffa::EnumValue::Known(ServerConnectionMode::ManagedWireguard) => {
                "MANAGED_WIREGUARD"
            }
            ::buffa::EnumValue::Known(ServerConnectionMode::ExternalPrivateNetwork) => {
                "EXTERNAL_PRIVATE_NETWORK"
            }
            _ => {
                return Err(ConnectError::invalid_argument(
                    "unsupported connection mode",
                ));
            }
        };
        let provider = match request.provider {
            Some(::buffa::EnumValue::Known(ServerPrivateNetworkProvider::Wireguard)) => {
                Some("WIREGUARD")
            }
            Some(::buffa::EnumValue::Known(ServerPrivateNetworkProvider::Tailscale)) => {
                Some("TAILSCALE")
            }
            Some(::buffa::EnumValue::Known(ServerPrivateNetworkProvider::Zerotier)) => {
                Some("ZEROTIER")
            }
            Some(::buffa::EnumValue::Known(ServerPrivateNetworkProvider::Netbird)) => {
                Some("NETBIRD")
            }
            Some(::buffa::EnumValue::Known(ServerPrivateNetworkProvider::Custom)) => Some("CUSTOM"),
            Some(_) => return Err(ConnectError::invalid_argument("unsupported provider")),
            None => None,
        };
        let payload = serde_json::json!({
            "connection_mode": connection_mode,
            "provider": provider,
            "private_host": request.private_host,
            "tunnel_address": request.tunnel_address,
            "public_key": request.public_key,
            "endpoint": request.endpoint,
            "listen_port": request.listen_port,
            "persistent_keepalive": request.persistent_keepalive,
            "dns_name": request.dns_name,
            "routes": request.routes.iter().map(|route| route.to_string()).collect::<Vec<String>>(),
        })
        .to_string();
        let dto = AppServerPrivateNetworkService::parse_update_payload(&payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let network = self
            .service
            .update(request.id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(ServerPrivateNetworkUpdateResponse {
            network: ::buffa::MessageField::some(network_to_proto!(network)),
        })
    }

    #[permission(Server, CanCreate)]
    async fn delete_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerPrivateNetworkDeleteRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .disable(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Server, CanCreate)]
    async fn setup<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerPrivateNetworkSetupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ServerPrivateNetworkSetupResponse> + Send + use<'a>>
    {
        let network = self
            .service
            .setup_transport(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(ServerPrivateNetworkSetupResponse {
            network: ::buffa::MessageField::some(network_to_proto!(network)),
        })
    }

    #[permission(Server, CanRead)]
    async fn health<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerPrivateNetworkHealthRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ServerPrivateNetworkHealthResponse> + Send + use<'a>,
    > {
        let health = self
            .service
            .health(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(ServerPrivateNetworkHealthResponse {
            health: ::buffa::MessageField::some(health_to_proto!(health)),
        })
    }

    #[permission(Server, CanCreate)]
    async fn repair<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerPrivateNetworkRepairRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ServerPrivateNetworkRepairResponse> + Send + use<'a>,
    > {
        let network = self
            .service
            .repair_transport(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(ServerPrivateNetworkRepairResponse {
            network: ::buffa::MessageField::some(network_to_proto!(network)),
        })
    }

    #[permission(Server, CanCreate)]
    async fn re_setup<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerPrivateNetworkReSetupRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ServerPrivateNetworkReSetupResponse> + Send + use<'a>,
    > {
        let network = self
            .service
            .re_setup_transport(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(ServerPrivateNetworkReSetupResponse {
            network: ::buffa::MessageField::some(network_to_proto!(network)),
        })
    }

    #[permission(Server, CanCreate)]
    async fn rotate_keys<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerPrivateNetworkRotateKeysRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ServerPrivateNetworkRotateKeysResponse> + Send + use<'a>,
    > {
        let network = self
            .service
            .rotate_wireguard(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(ServerPrivateNetworkRotateKeysResponse {
            network: ::buffa::MessageField::some(network_to_proto!(network)),
        })
    }

    #[permission(Server, CanCreate)]
    async fn teardown<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerPrivateNetworkTeardownRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .teardown_transport(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }
}
