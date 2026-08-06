use crate::{
    api::dto::server_management::{
        PrivateNetworkHealthStatusDto, PrivateNetworkProviderDto, PrivateNetworkRotationStateDto,
        PrivateNetworkStatusDto, ServerConnectionModeDto, ServerPrivateNetworkDto,
    },
    db::models::server_private_networks::{
        PrivateNetworkConnectionMode, PrivateNetworkHealthStatus, PrivateNetworkProvider,
        PrivateNetworkRotationState, PrivateNetworkStatus, ServerPrivateNetwork,
    },
};

pub(super) fn map_network(value: ServerPrivateNetwork) -> sqlx::Result<ServerPrivateNetworkDto> {
    Ok(ServerPrivateNetworkDto {
        server_id: value.server_id,
        connection_mode: mode_dto(PrivateNetworkConnectionMode::try_from(
            value.connection_mode.as_str(),
        )?),
        provider: value
            .provider
            .as_deref()
            .map(PrivateNetworkProvider::try_from)
            .transpose()?
            .map(provider_dto),
        private_host: value.private_host,
        tunnel_address: value.tunnel_address,
        public_key: value.public_key,
        endpoint: value.endpoint,
        listen_port: value.listen_port.map(to_u16).transpose()?,
        persistent_keepalive: value.persistent_keepalive.map(to_u16).transpose()?,
        status: status_dto(PrivateNetworkStatus::try_from(value.status.as_str())?),
        last_handshake_at: value.last_handshake_at,
        config_version: value.config_version,
        dns_name: value.dns_name,
        routes: serde_json::from_str(&value.routes)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?,
        health_status: health_dto(PrivateNetworkHealthStatus::try_from(
            value.health_status.as_str(),
        )?),
        health_error: value.health_error,
        last_health_check_at: value.last_health_check_at,
        consecutive_failures: value.consecutive_failures,
        rotation_state: rotation_dto(PrivateNetworkRotationState::try_from(
            value.rotation_state.as_str(),
        )?),
        updated_at: value.updated_at,
    })
}

pub(super) fn connection_mode(value: ServerConnectionModeDto) -> PrivateNetworkConnectionMode {
    match value {
        ServerConnectionModeDto::DirectSsh => PrivateNetworkConnectionMode::DirectSsh,
        ServerConnectionModeDto::ManagedWireguard => PrivateNetworkConnectionMode::ManagedWireguard,
        ServerConnectionModeDto::ExternalPrivateNetwork => {
            PrivateNetworkConnectionMode::ExternalPrivateNetwork
        }
    }
}

pub(super) fn provider(value: PrivateNetworkProviderDto) -> PrivateNetworkProvider {
    match value {
        PrivateNetworkProviderDto::Wireguard => PrivateNetworkProvider::Wireguard,
        PrivateNetworkProviderDto::Tailscale => PrivateNetworkProvider::Tailscale,
        PrivateNetworkProviderDto::Zerotier => PrivateNetworkProvider::Zerotier,
        PrivateNetworkProviderDto::Netbird => PrivateNetworkProvider::Netbird,
        PrivateNetworkProviderDto::Custom => PrivateNetworkProvider::Custom,
    }
}

fn mode_dto(value: PrivateNetworkConnectionMode) -> ServerConnectionModeDto {
    match value {
        PrivateNetworkConnectionMode::DirectSsh => ServerConnectionModeDto::DirectSsh,
        PrivateNetworkConnectionMode::ManagedWireguard => ServerConnectionModeDto::ManagedWireguard,
        PrivateNetworkConnectionMode::ExternalPrivateNetwork => {
            ServerConnectionModeDto::ExternalPrivateNetwork
        }
    }
}

fn provider_dto(value: PrivateNetworkProvider) -> PrivateNetworkProviderDto {
    match value {
        PrivateNetworkProvider::Wireguard => PrivateNetworkProviderDto::Wireguard,
        PrivateNetworkProvider::Tailscale => PrivateNetworkProviderDto::Tailscale,
        PrivateNetworkProvider::Zerotier => PrivateNetworkProviderDto::Zerotier,
        PrivateNetworkProvider::Netbird => PrivateNetworkProviderDto::Netbird,
        PrivateNetworkProvider::Custom => PrivateNetworkProviderDto::Custom,
    }
}

pub(super) fn status_dto(value: PrivateNetworkStatus) -> PrivateNetworkStatusDto {
    match value {
        PrivateNetworkStatus::Disabled => PrivateNetworkStatusDto::Disabled,
        PrivateNetworkStatus::Configuring => PrivateNetworkStatusDto::Configuring,
        PrivateNetworkStatus::Active => PrivateNetworkStatusDto::Active,
        PrivateNetworkStatus::Failed => PrivateNetworkStatusDto::Failed,
    }
}

pub(super) fn health_dto(value: PrivateNetworkHealthStatus) -> PrivateNetworkHealthStatusDto {
    match value {
        PrivateNetworkHealthStatus::Unknown => PrivateNetworkHealthStatusDto::Unknown,
        PrivateNetworkHealthStatus::Healthy => PrivateNetworkHealthStatusDto::Healthy,
        PrivateNetworkHealthStatus::Degraded => PrivateNetworkHealthStatusDto::Degraded,
        PrivateNetworkHealthStatus::Unreachable => PrivateNetworkHealthStatusDto::Unreachable,
        PrivateNetworkHealthStatus::ConfigDrift => PrivateNetworkHealthStatusDto::ConfigDrift,
    }
}

fn rotation_dto(value: PrivateNetworkRotationState) -> PrivateNetworkRotationStateDto {
    match value {
        PrivateNetworkRotationState::Idle => PrivateNetworkRotationStateDto::Idle,
        PrivateNetworkRotationState::Rotating => PrivateNetworkRotationStateDto::Rotating,
        PrivateNetworkRotationState::RollingBack => PrivateNetworkRotationStateDto::RollingBack,
        PrivateNetworkRotationState::Failed => PrivateNetworkRotationStateDto::Failed,
    }
}

fn to_u16(value: i64) -> sqlx::Result<u16> {
    u16::try_from(value).map_err(|error| sqlx::Error::Protocol(error.to_string()))
}
