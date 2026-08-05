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
    utils::{
        exec::{CommandExecutor, LocalExecutor},
        os::{
            OsCli,
            wireguard::{WireGuardConfig, WireGuardPeer},
        },
    },
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

    pub async fn setup_wireguard(&self, server_id: i64) -> sqlx::Result<ServerPrivateNetworkDto> {
        let network = self
            .networks
            .get(server_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        if network.connection_mode != "MANAGED_WIREGUARD" {
            return Err(sqlx::Error::Protocol(
                "server is not configured for managed WireGuard".into(),
            ));
        }
        if network.status == "ACTIVE" {
            let host = network
                .private_host
                .as_deref()
                .ok_or_else(|| sqlx::Error::Protocol("private_host is required".into()))?;
            self.verify_private_ssh(server_id, host).await?;
            return map_network(network);
        }
        let cidr = network
            .tunnel_address
            .as_deref()
            .ok_or_else(|| sqlx::Error::Protocol("tunnel_address is required".into()))?;
        let endpoint = network
            .endpoint
            .as_deref()
            .ok_or_else(|| sqlx::Error::Protocol("endpoint is required".into()))?;
        let (panel_address, remote_address, remote_host) = tunnel_addresses(cidr)?;
        if network.private_host.as_deref() != Some(remote_host.as_str()) {
            return Err(sqlx::Error::Protocol(format!(
                "private_host must be {remote_host} for tunnel {cidr}"
            )));
        }
        let remote = self.direct_executor(server_id).await?;
        let local = CommandExecutor::Local(LocalExecutor::new());
        let local_os = OsCli::new(&local);
        let remote_executor = CommandExecutor::Remote(remote.clone().with_sudo());
        let remote_os = OsCli::new(&remote_executor);
        let interface = interface_name(server_id);
        let result = async {
            remote_os
                .package("wireguard-tools")
                .install()
                .run()
                .await
                .map_err(protocol)?;
            let _ = local_os.wireguard().interface(&interface).remove().await;
            let _ = remote_os.wireguard().interface(&interface).remove().await;
            let panel_private = local_os
                .wireguard()
                .key()
                .generate()
                .await
                .map_err(protocol)?;
            let panel_public = local_os
                .wireguard()
                .key()
                .public_from_private(&panel_private)
                .await
                .map_err(protocol)?;
            let remote_private = remote_os
                .wireguard()
                .key()
                .generate()
                .await
                .map_err(protocol)?;
            let remote_public = remote_os
                .wireguard()
                .key()
                .public_from_private(&remote_private)
                .await
                .map_err(protocol)?;
            let port = network
                .listen_port
                .and_then(|v| u16::try_from(v).ok())
                .unwrap_or(51820);
            local_os
                .wireguard()
                .interface(&interface)
                .install(&WireGuardConfig {
                    private_key: panel_private,
                    addresses: vec![panel_address],
                    listen_port: Some(port),
                    peers: vec![WireGuardPeer {
                        public_key: remote_public.clone(),
                        allowed_ips: vec![format!("{remote_host}/32")],
                        endpoint: None,
                        persistent_keepalive: None,
                    }],
                })
                .await
                .map_err(protocol)?;
            remote_os
                .wireguard()
                .interface(&interface)
                .install(&WireGuardConfig {
                    private_key: remote_private,
                    addresses: vec![remote_address],
                    listen_port: None,
                    peers: vec![WireGuardPeer {
                        public_key: panel_public,
                        allowed_ips: vec![panel_host(cidr)?],
                        endpoint: Some(endpoint.to_owned()),
                        persistent_keepalive: Some(
                            network
                                .persistent_keepalive
                                .and_then(|v| u16::try_from(v).ok())
                                .unwrap_or(20),
                        ),
                    }],
                })
                .await
                .map_err(protocol)?;
            RemoteExecutor::new(
                remote_host.clone(),
                remote.port(),
                remote.username(),
                remote.auth().clone(),
                SshHostKey::InsecureAcceptAny,
            )
            .run("true", std::iter::empty::<&str>())
            .await
            .map_err(protocol)?;
            Ok::<String, sqlx::Error>(remote_public)
        }
        .await;
        match result {
            Ok(public_key) => {
                self.networks
                    .set_runtime_state(
                        server_id,
                        "ACTIVE",
                        Some(&public_key),
                        Some(chrono::Utc::now().timestamp()),
                    )
                    .await?
            }
            Err(error) => {
                let _ = local_os.wireguard().interface(&interface).remove().await;
                let _ = remote_os.wireguard().interface(&interface).remove().await;
                self.networks
                    .set_runtime_state(server_id, "FAILED", None, None)
                    .await?;
                return Err(error);
            }
        }
        map_network(
            self.networks
                .get(server_id)
                .await?
                .ok_or(sqlx::Error::RowNotFound)?,
        )
    }

    pub async fn teardown_wireguard(&self, server_id: i64) -> sqlx::Result<()> {
        let remote = self.direct_executor(server_id).await?;
        let local = CommandExecutor::Local(LocalExecutor::new());
        let remote_executor = CommandExecutor::Remote(remote.with_sudo());
        let interface = interface_name(server_id);
        let _ = OsCli::new(&local)
            .wireguard()
            .interface(&interface)
            .remove()
            .await;
        let _ = OsCli::new(&remote_executor)
            .wireguard()
            .interface(&interface)
            .remove()
            .await;
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

    async fn direct_executor(&self, server_id: i64) -> sqlx::Result<RemoteExecutor> {
        let (host, port, username, private_key, public_key) = self
            .servers
            .get_direct_ssh_credentials(server_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        Ok(RemoteExecutor::new(
            host,
            u16::try_from(port).map_err(|e| sqlx::Error::Protocol(e.to_string()))?,
            username,
            SshAuth::key_pair(private_key, public_key),
            SshHostKey::InsecureAcceptAny,
        ))
    }
}

fn interface_name(server_id: i64) -> String {
    let encoded = format!("{:x}", server_id.unsigned_abs());
    let suffix = if encoded.len() > 11 {
        &encoded[encoded.len() - 11..]
    } else {
        &encoded
    };
    format!("rpwg{suffix}")
}

fn tunnel_addresses(cidr: &str) -> sqlx::Result<(String, String, String)> {
    let network: ipnet::Ipv4Net = cidr
        .parse()
        .map_err(|e| sqlx::Error::Protocol(format!("invalid IPv4 tunnel network: {e}")))?;
    if network.prefix_len() > 30 {
        return Err(sqlx::Error::Protocol(
            "WireGuard tunnel network must contain at least two host addresses".into(),
        ));
    }
    let base = u32::from(network.network());
    let panel = std::net::Ipv4Addr::from(base + 1);
    let remote = std::net::Ipv4Addr::from(base + 2);
    Ok((
        format!("{panel}/{}", network.prefix_len()),
        format!("{remote}/{}", network.prefix_len()),
        remote.to_string(),
    ))
}

fn panel_host(cidr: &str) -> sqlx::Result<String> {
    tunnel_addresses(cidr)
        .map(|(panel, _, _)| format!("{}/32", panel.split('/').next().unwrap_or_default()))
}

fn protocol(error: impl std::fmt::Display) -> sqlx::Error {
    sqlx::Error::Protocol(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::{endpoint_port, interface_name, tunnel_addresses, validate};
    use crate::api::dto::server_management::{
        PrivateNetworkProviderDto, ServerConnectionModeDto, UpdatePrivateNetworkDto,
    };

    #[test]
    fn allocates_tunnel_hosts_from_network() {
        let (panel, remote, host) = tunnel_addresses("10.77.8.0/24").unwrap();
        assert_eq!(panel, "10.77.8.1/24");
        assert_eq!(remote, "10.77.8.2/24");
        assert_eq!(host, "10.77.8.2");
        assert!(interface_name(i64::MAX).len() <= 15);
        assert!(tunnel_addresses("10.77.8.0/31").is_err());
        assert!(tunnel_addresses("not-a-network").is_err());
    }

    #[test]
    fn validates_managed_network_invariants() {
        let valid = UpdatePrivateNetworkDto {
            connection_mode: ServerConnectionModeDto::ManagedWireguard,
            provider: Some(PrivateNetworkProviderDto::Wireguard),
            private_host: Some("10.77.8.2".into()),
            tunnel_address: Some("10.77.8.0/24".into()),
            public_key: None,
            endpoint: Some("panel.example.com:51820".into()),
            listen_port: Some(51820),
            persistent_keepalive: Some(20),
        };
        assert!(validate(&valid).is_ok());

        let mut wrong_host = valid.clone();
        wrong_host.private_host = Some("10.77.8.3".into());
        assert!(validate(&wrong_host).is_err());

        let mut mismatched_port = valid.clone();
        mismatched_port.listen_port = Some(51821);
        assert!(validate(&mismatched_port).is_err());

        let mut injected = valid;
        injected.endpoint = Some("panel.example.com:51820\nPostUp=evil".into());
        assert!(validate(&injected).is_err());
        assert_eq!(endpoint_port("[2001:db8::1]:51820").unwrap(), 51820);
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
            let cidr = input.tunnel_address.as_deref().expect("checked above");
            let (_, _, expected_host) = tunnel_addresses(cidr)?;
            if input.private_host.as_deref() != Some(expected_host.as_str()) {
                return Err(sqlx::Error::Protocol(format!(
                    "private_host must be {expected_host} for tunnel {cidr}"
                )));
            }
            let endpoint = input.endpoint.as_deref().expect("checked above");
            let endpoint_port = endpoint_port(endpoint)?;
            let listen_port = input.listen_port.unwrap_or(51820);
            if endpoint_port != listen_port {
                return Err(sqlx::Error::Protocol(format!(
                    "endpoint port {endpoint_port} must match listen_port {listen_port}"
                )));
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

fn endpoint_port(endpoint: &str) -> sqlx::Result<u16> {
    if let Ok(address) = endpoint.parse::<std::net::SocketAddr>() {
        return Ok(address.port());
    }
    let (host, port) = endpoint
        .rsplit_once(':')
        .ok_or_else(|| sqlx::Error::Protocol("endpoint must include a port".into()))?;
    if host.is_empty()
        || host.chars().any(char::is_whitespace)
        || !host
            .chars()
            .all(|value| value.is_ascii_alphanumeric() || matches!(value, '.' | '-'))
    {
        return Err(sqlx::Error::Protocol("invalid WireGuard endpoint".into()));
    }
    port.parse::<u16>()
        .map_err(|_| sqlx::Error::Protocol("invalid WireGuard endpoint port".into()))
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
