use std::sync::Arc;

use auto_di::singleton;

use crate::{
    api::dto::server_management::{
        PrivateNetworkProviderDto, ServerConnectionModeDto, ServerPrivateNetworkDto,
        UpdatePrivateNetworkDto,
    },
    db::models::server_private_networks::ServerPrivateNetwork,
    repository::{ServerPrivateNetworkRepository, ServerRepository},
    utils::exec::{RemoteExecutor, SshAuth, SshHostKey},
};

pub struct ServerPrivateNetworkService {
    servers: Arc<ServerRepository>,
    networks: Arc<ServerPrivateNetworkRepository>,
}

#[singleton]
impl ServerPrivateNetworkService {
    fn new(servers: Arc<ServerRepository>, networks: Arc<ServerPrivateNetworkRepository>) -> Self {
        Self { servers, networks }
    }

    pub async fn get(&self, server_id: i64) -> sqlx::Result<Option<ServerPrivateNetworkDto>> {
        self.assert_server(server_id).await?;
        self.networks
            .get(server_id)
            .await?
            .map(map_network)
            .transpose()
    }

    pub async fn update(
        &self,
        server_id: i64,
        input: UpdatePrivateNetworkDto,
    ) -> sqlx::Result<ServerPrivateNetworkDto> {
        self.assert_server(server_id).await?;
        validate(&input)?;
        if input.connection_mode == ServerConnectionModeDto::DirectSsh {
            self.networks.disable(server_id).await?;
            return direct_network(server_id);
        }
        let status = match input.connection_mode {
            ServerConnectionModeDto::ManagedWireguard => "CONFIGURING",
            ServerConnectionModeDto::ExternalPrivateNetwork => {
                self.verify_private_ssh(
                    server_id,
                    input
                        .private_host
                        .as_deref()
                        .expect("validated private host"),
                )
                .await?;
                "ACTIVE"
            }
            ServerConnectionModeDto::DirectSsh => unreachable!(),
        };
        map_network(
            self.networks
                .upsert(
                    server_id,
                    connection_mode(input.connection_mode),
                    input.provider.map(provider).as_deref(),
                    input.private_host.as_deref(),
                    input.tunnel_address.as_deref(),
                    input.public_key.as_deref(),
                    input.endpoint.as_deref(),
                    input.listen_port.map(i64::from),
                    input.persistent_keepalive.map(i64::from),
                    status,
                )
                .await?,
        )
    }

    pub async fn disable(&self, server_id: i64) -> sqlx::Result<()> {
        self.assert_server(server_id).await?;
        self.networks.disable(server_id).await
    }

    async fn assert_server(&self, server_id: i64) -> sqlx::Result<()> {
        self.servers
            .get_by_id(server_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
            .map(|_| ())
    }

    async fn verify_private_ssh(&self, server_id: i64, host: &str) -> sqlx::Result<()> {
        let (_, port, username, private_key, public_key) = self
            .servers
            .get_direct_ssh_credentials(server_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let port = u16::try_from(port).map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        RemoteExecutor::new(
            host,
            port,
            username,
            SshAuth::key_pair(private_key, public_key),
            SshHostKey::InsecureAcceptAny,
        )
        .run("true", std::iter::empty::<&str>())
        .await
        .map(|_| ())
        .map_err(|error| {
            sqlx::Error::Protocol(format!("private network SSH verification failed: {error}"))
        })
    }
}

fn validate(input: &UpdatePrivateNetworkDto) -> sqlx::Result<()> {
    if input.connection_mode == ServerConnectionModeDto::DirectSsh {
        return Ok(());
    }
    let host = input
        .private_host
        .as_deref()
        .filter(|value| !value.trim().is_empty() && !value.chars().any(char::is_whitespace))
        .ok_or_else(|| {
            sqlx::Error::Protocol("private_host is required and must not contain whitespace".into())
        })?;
    if host.len() > 253 {
        return Err(sqlx::Error::Protocol("private_host is too long".into()));
    }
    let provider = input
        .provider
        .ok_or_else(|| sqlx::Error::Protocol("private network provider is required".into()))?;
    match input.connection_mode {
        ServerConnectionModeDto::ManagedWireguard => {
            if provider != PrivateNetworkProviderDto::Wireguard {
                return Err(sqlx::Error::Protocol(
                    "managed private networks require the WIREGUARD provider".into(),
                ));
            }
            if input.tunnel_address.is_none() || input.endpoint.is_none() {
                return Err(sqlx::Error::Protocol(
                    "managed WireGuard requires tunnel_address and endpoint".into(),
                ));
            }
        }
        ServerConnectionModeDto::ExternalPrivateNetwork => {
            if provider == PrivateNetworkProviderDto::Wireguard {
                return Err(sqlx::Error::Protocol(
                    "use MANAGED_WIREGUARD for the WireGuard provider".into(),
                ));
            }
        }
        ServerConnectionModeDto::DirectSsh => {}
    }
    Ok(())
}

fn map_network(value: ServerPrivateNetwork) -> sqlx::Result<ServerPrivateNetworkDto> {
    Ok(ServerPrivateNetworkDto {
        server_id: value.server_id,
        connection_mode: parse_mode(&value.connection_mode)?,
        provider: value.provider.as_deref().map(parse_provider).transpose()?,
        private_host: value.private_host,
        tunnel_address: value.tunnel_address,
        public_key: value.public_key,
        endpoint: value.endpoint,
        listen_port: value.listen_port.map(to_u16).transpose()?,
        persistent_keepalive: value.persistent_keepalive.map(to_u16).transpose()?,
        status: value.status,
        last_handshake_at: value.last_handshake_at,
        config_version: value.config_version,
        updated_at: value.updated_at,
    })
}

fn direct_network(server_id: i64) -> sqlx::Result<ServerPrivateNetworkDto> {
    Ok(ServerPrivateNetworkDto {
        server_id,
        connection_mode: ServerConnectionModeDto::DirectSsh,
        provider: None,
        private_host: None,
        tunnel_address: None,
        public_key: None,
        endpoint: None,
        listen_port: None,
        persistent_keepalive: None,
        status: "DISABLED".into(),
        last_handshake_at: None,
        config_version: 0,
        updated_at: chrono::Utc::now().timestamp(),
    })
}

fn connection_mode(value: ServerConnectionModeDto) -> &'static str {
    match value {
        ServerConnectionModeDto::DirectSsh => "DIRECT_SSH",
        ServerConnectionModeDto::ManagedWireguard => "MANAGED_WIREGUARD",
        ServerConnectionModeDto::ExternalPrivateNetwork => "EXTERNAL_PRIVATE_NETWORK",
    }
}

fn provider(value: PrivateNetworkProviderDto) -> String {
    match value {
        PrivateNetworkProviderDto::Wireguard => "WIREGUARD",
        PrivateNetworkProviderDto::Tailscale => "TAILSCALE",
        PrivateNetworkProviderDto::Zerotier => "ZEROTIER",
        PrivateNetworkProviderDto::Netbird => "NETBIRD",
        PrivateNetworkProviderDto::Custom => "CUSTOM",
    }
    .into()
}

fn parse_mode(value: &str) -> sqlx::Result<ServerConnectionModeDto> {
    match value {
        "DIRECT_SSH" => Ok(ServerConnectionModeDto::DirectSsh),
        "MANAGED_WIREGUARD" => Ok(ServerConnectionModeDto::ManagedWireguard),
        "EXTERNAL_PRIVATE_NETWORK" => Ok(ServerConnectionModeDto::ExternalPrivateNetwork),
        _ => Err(sqlx::Error::Protocol(format!(
            "invalid connection mode: {value}"
        ))),
    }
}

fn parse_provider(value: &str) -> sqlx::Result<PrivateNetworkProviderDto> {
    match value {
        "WIREGUARD" => Ok(PrivateNetworkProviderDto::Wireguard),
        "TAILSCALE" => Ok(PrivateNetworkProviderDto::Tailscale),
        "ZEROTIER" => Ok(PrivateNetworkProviderDto::Zerotier),
        "NETBIRD" => Ok(PrivateNetworkProviderDto::Netbird),
        "CUSTOM" => Ok(PrivateNetworkProviderDto::Custom),
        _ => Err(sqlx::Error::Protocol(format!(
            "invalid private network provider: {value}"
        ))),
    }
}

fn to_u16(value: i64) -> sqlx::Result<u16> {
    u16::try_from(value).map_err(|error| sqlx::Error::Protocol(error.to_string()))
}
