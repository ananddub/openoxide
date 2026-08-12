use std::sync::Arc;

use auto_di::singleton;

use crate::{
    api::dto::server::{
        PrivateNetworkHealthDto, PrivateNetworkHealthStatusDto, PrivateNetworkRotationStateDto,
        PrivateNetworkStatusDto, ServerConnectionModeDto, ServerPrivateNetworkDto,
        UpdatePrivateNetworkDto,
    },
    db::models::server_private_networks::{
        PrivateNetworkConnectionMode, PrivateNetworkHealthStatus, PrivateNetworkOperation,
        PrivateNetworkStatus,
    },
    repository::{ServerPrivateNetworkRepository, ServerRepository},
    utils::exec::{CommandExecutor, LocalExecutor, RemoteExecutor, SshAuth, SshHostKey},
};

use super::{
    addressing::{interface_name, panel_host, tunnel_addresses},
    backend::{KernelWireGuardBackend, ManagedWireGuardBackend, WireGuardInstallPlan},
    mapping::{connection_mode, health_dto, map_network, provider},
    retry::RetryPolicy,
    validation::validate,
};

const OPERATION_LEASE_SECONDS: i64 = 30;
const OPERATION_HEARTBEAT_SECONDS: u64 = 10;
const STALE_HANDSHAKE_SECONDS: i64 = 180;
const AUTO_REPAIR_FAILURE_THRESHOLD: i64 = 3;

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
            return Ok(direct_network(server_id));
        }
        let status = if input.connection_mode == ServerConnectionModeDto::ExternalPrivateNetwork {
            self.verify_private_ssh(server_id, input.private_host.as_deref().expect("validated"))
                .await?;
            PrivateNetworkStatus::Active
        } else {
            PrivateNetworkStatus::Configuring
        };
        let routes = serde_json::to_string(&input.routes)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let dns_name = input.dns_name.as_deref().map(str::to_ascii_lowercase);
        let network = map_network(
            self.networks
                .upsert(
                    server_id,
                    connection_mode(input.connection_mode),
                    input.provider.map(provider),
                    input.private_host.as_deref(),
                    input.tunnel_address.as_deref(),
                    input.public_key.as_deref(),
                    input.endpoint.as_deref(),
                    input.listen_port.map(i64::from),
                    input.persistent_keepalive.map(i64::from),
                    dns_name.as_deref(),
                    &routes,
                    status,
                )
                .await?,
        )?;
        Ok(network)
    }

    pub async fn disable(&self, server_id: i64) -> sqlx::Result<()> {
        self.assert_server(server_id).await?;
        self.networks.disable(server_id).await
    }

    pub async fn setup_transport(&self, server_id: i64) -> sqlx::Result<ServerPrivateNetworkDto> {
        let network = self.get_model(server_id).await?;
        if PrivateNetworkConnectionMode::try_from(network.connection_mode.as_str())?
            == PrivateNetworkConnectionMode::ExternalPrivateNetwork
        {
            self.verify_external_transport(&network).await?;
            return self.get_required(server_id).await;
        }
        self.with_operation(
            server_id,
            PrivateNetworkOperation::Setup,
            self.setup_locked(server_id),
        )
        .await
    }

    pub async fn repair_transport(&self, server_id: i64) -> sqlx::Result<ServerPrivateNetworkDto> {
        let network = self.get_model(server_id).await?;
        if PrivateNetworkConnectionMode::try_from(network.connection_mode.as_str())?
            == PrivateNetworkConnectionMode::ExternalPrivateNetwork
        {
            self.verify_external_transport(&network).await?;
            return self.get_required(server_id).await;
        }
        self.with_operation(server_id, PrivateNetworkOperation::Repair, async {
            let (local, remote) = self.executors(server_id).await?;
            let backend = KernelWireGuardBackend::new(&local, &remote);
            let interface = interface_name(server_id);
            let _ = backend.teardown(&interface).await;
            self.networks
                .set_runtime_state(server_id, PrivateNetworkStatus::Configuring, None, None)
                .await?;
            self.setup_locked(server_id).await
        })
        .await
    }

    pub async fn re_setup_transport(
        &self,
        server_id: i64,
    ) -> sqlx::Result<ServerPrivateNetworkDto> {
        self.require_managed_wireguard(server_id).await?;
        let _ = self.networks.force_release_operation(server_id).await;
        self.with_operation(server_id, PrivateNetworkOperation::Setup, async {
            let (local, remote) = self.executors(server_id).await?;
            let backend = KernelWireGuardBackend::new(&local, &remote);
            let interface = interface_name(server_id);
            let _ = backend.teardown(&interface).await;
            self.networks
                .set_runtime_state(server_id, PrivateNetworkStatus::Configuring, None, None)
                .await?;
            self.setup_locked(server_id).await
        })
        .await
    }

    pub async fn rotate_wireguard(&self, server_id: i64) -> sqlx::Result<ServerPrivateNetworkDto> {
        self.require_managed_wireguard(server_id).await?;
        self.with_operation(server_id, PrivateNetworkOperation::Rotate, async {
            self.networks.begin_rotation(server_id).await?;
            let result = async {
                let network = self.get_model(server_id).await?;
                let cidr = required(network.tunnel_address.as_deref(), "tunnel_address")?;
                let endpoint = required(network.endpoint.as_deref(), "endpoint")?;
                let (panel_address, remote_address, remote_host) = tunnel_addresses(cidr)?;
                let (local, remote) = self.executors(server_id).await?;
                let backend = KernelWireGuardBackend::new(&local, &remote);
                let interface = interface_name(server_id);
                let public_key = backend
                    .rotate(&WireGuardInstallPlan {
                        interface: &interface,
                        panel_address,
                        remote_address,
                        panel_host: panel_host(cidr)?,
                        remote_host: remote_host.clone(),
                        endpoint,
                        port: network
                            .listen_port
                            .and_then(|value| value.try_into().ok())
                            .unwrap_or(51820),
                        keepalive: network
                            .persistent_keepalive
                            .and_then(|value| value.try_into().ok())
                            .unwrap_or(20),
                        routes: serde_json::from_str(&network.routes)
                            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?,
                    })
                    .await?;
                self.verify_private_ssh(server_id, &remote_host).await?;
                self.networks
                    .set_runtime_state(
                        server_id,
                        PrivateNetworkStatus::Active,
                        Some(&public_key),
                        None,
                    )
                    .await?;
                self.get_required(server_id).await
            }
            .await;
            match result {
                Ok(network) => {
                    let public_key = network.public_key.as_deref().ok_or_else(|| {
                        sqlx::Error::Protocol("rotation produced no public key".into())
                    })?;
                    self.networks.finish_rotation(server_id, public_key).await?;
                    self.get_required(server_id).await
                }
                Err(error) => {
                    self.networks.fail_rotation(server_id).await?;
                    Err(error)
                }
            }
        })
        .await
    }

    pub async fn health(&self, server_id: i64) -> sqlx::Result<PrivateNetworkHealthDto> {
        let network = self.get_model(server_id).await?;
        if PrivateNetworkConnectionMode::try_from(network.connection_mode.as_str())?
            == PrivateNetworkConnectionMode::ExternalPrivateNetwork
        {
            return self.external_health(&network).await;
        }
        let (local, remote) = self.executors(server_id).await?;
        let backend = KernelWireGuardBackend::new(&local, &remote);
        let kernel = backend.health(&interface_name(server_id)).await?;
        let ssh = match network.private_host.as_deref() {
            Some(host) => self.verify_private_ssh(server_id, host).await,
            None => Err(sqlx::Error::Protocol("private_host is missing".into())),
        };
        let now = chrono::Utc::now().timestamp();
        let stale = kernel
            .latest_handshake
            .map(|timestamp| now.saturating_sub(timestamp) > STALE_HANDSHAKE_SECONDS)
            .unwrap_or(true);
        let desired_routes: Vec<String> = serde_json::from_str(&network.routes)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let host_route = network
            .private_host
            .as_deref()
            .map(|host| format!("{host}/32"));
        let routes_match = desired_routes
            .iter()
            .all(|route| kernel.allowed_ips.contains(route))
            && host_route
                .as_ref()
                .is_none_or(|route| kernel.allowed_ips.contains(route));
        let peer_matches = network
            .public_key
            .as_ref()
            .is_none_or(|public_key| kernel.peer_public_keys.contains(public_key));
        let (status, error) = if !kernel.interface_exists {
            (
                PrivateNetworkHealthStatus::ConfigDrift,
                Some("WireGuard interface is missing".to_owned()),
            )
        } else if !routes_match {
            (
                PrivateNetworkHealthStatus::ConfigDrift,
                Some("WireGuard routes differ from desired configuration".to_owned()),
            )
        } else if !peer_matches {
            (
                PrivateNetworkHealthStatus::ConfigDrift,
                Some("WireGuard peer key differs from desired configuration".to_owned()),
            )
        } else if let Err(error) = &ssh {
            (
                PrivateNetworkHealthStatus::Unreachable,
                Some(error.to_string()),
            )
        } else if stale {
            (
                PrivateNetworkHealthStatus::Degraded,
                Some("WireGuard handshake is stale".to_owned()),
            )
        } else {
            (PrivateNetworkHealthStatus::Healthy, None)
        };
        self.networks
            .set_health(server_id, status, error.as_deref(), kernel.latest_handshake)
            .await?;
        Ok(PrivateNetworkHealthDto {
            status: health_dto(status),
            interface_exists: kernel.interface_exists,
            private_ssh_reachable: ssh.is_ok(),
            latest_handshake_at: kernel.latest_handshake,
            checked_at: now,
            error,
        })
    }

    pub async fn check_all_health(&self) {
        let networks = match self.networks.list_managed().await {
            Ok(networks) => networks,
            Err(error) => {
                tracing::warn!(%error, "could not list managed WireGuard networks");
                return;
            }
        };
        for network in networks {
            match self.health(network.server_id).await {
                Ok(health) => {
                    let refreshed = match self.get_model(network.server_id).await {
                        Ok(network) => network,
                        Err(error) => {
                            tracing::warn!(server_id = network.server_id, %error, "could not reload managed WireGuard state");
                            continue;
                        }
                    };
                    if should_auto_repair(health.status, refreshed.consecutive_failures) {
                        if let Err(error) = self.repair_transport(network.server_id).await {
                            tracing::warn!(server_id = network.server_id, %error, "automatic WireGuard repair failed");
                        }
                    } else if health.status == PrivateNetworkHealthStatusDto::Healthy
                        && matches!(
                            PrivateNetworkStatus::try_from(refreshed.status.as_str()),
                            Ok(PrivateNetworkStatus::Failed)
                        )
                    {
                        if let Err(error) = self
                            .networks
                            .set_runtime_state(
                                network.server_id,
                                PrivateNetworkStatus::Active,
                                refreshed.public_key.as_deref(),
                                refreshed.last_handshake_at,
                            )
                            .await
                        {
                            tracing::warn!(server_id = network.server_id, %error, "could not recover WireGuard runtime status");
                        }
                    }
                }
                Err(error) => {
                    tracing::warn!(server_id = network.server_id, %error, "managed WireGuard health check failed");
                }
            }
        }
    }

    pub async fn teardown_transport(&self, server_id: i64) -> sqlx::Result<()> {
        let network = self.get_model(server_id).await?;
        if PrivateNetworkConnectionMode::try_from(network.connection_mode.as_str())?
            == PrivateNetworkConnectionMode::ExternalPrivateNetwork
        {
            return self.networks.disable(server_id).await;
        }
        self.with_operation(server_id, PrivateNetworkOperation::Teardown, async {
            let (local, remote) = self.executors(server_id).await?;
            KernelWireGuardBackend::new(&local, &remote)
                .teardown(&interface_name(server_id))
                .await?;
            self.networks.disable(server_id).await?;
            Ok(())
        })
        .await
    }

    async fn setup_locked(&self, server_id: i64) -> sqlx::Result<ServerPrivateNetworkDto> {
        let network = self.get_model(server_id).await?;
        if PrivateNetworkConnectionMode::try_from(network.connection_mode.as_str())?
            != PrivateNetworkConnectionMode::ManagedWireguard
        {
            return Err(sqlx::Error::Protocol(
                "server is not configured for managed WireGuard".into(),
            ));
        }
        if PrivateNetworkStatus::try_from(network.status.as_str())? == PrivateNetworkStatus::Active
        {
            self.health(server_id).await?;
            return self.get_required(server_id).await;
        }
        let cidr = required(network.tunnel_address.as_deref(), "tunnel_address")?;
        let raw_endpoint = network.endpoint.as_deref().unwrap_or("").trim();
        let remote_ip = self
            .servers
            .get_by_id(server_id)
            .await?
            .map(|s| s.ip_address)
            .unwrap_or_default();
        let port = network
            .listen_port
            .and_then(|value| value.try_into().ok())
            .unwrap_or(51820);

        let fallback_endpoint = format!("{remote_ip}:{port}");
        let endpoint = if raw_endpoint.is_empty()
            || raw_endpoint.contains("example.com")
            || raw_endpoint.contains("pannel.example")
            || raw_endpoint.contains("example")
        {
            &fallback_endpoint
        } else {
            raw_endpoint
        };
        let (panel_address, remote_address, remote_host) = tunnel_addresses(cidr)?;
        let (local, remote) = self.executors(server_id).await?;
        let backend = KernelWireGuardBackend::new(&local, &remote);
        let interface = interface_name(server_id);
        let public_key = backend
            .install(&WireGuardInstallPlan {
                interface: &interface,
                panel_address,
                remote_address,
                panel_host: panel_host(cidr)?,
                remote_host: remote_host.clone(),
                endpoint,
                port: network
                    .listen_port
                    .and_then(|value| value.try_into().ok())
                    .unwrap_or(51820),
                keepalive: network
                    .persistent_keepalive
                    .and_then(|value| value.try_into().ok())
                    .unwrap_or(20),
                routes: serde_json::from_str(&network.routes)
                    .map_err(|error| sqlx::Error::Protocol(error.to_string()))?,
            })
            .await;
        match public_key {
            Ok(public_key) => {
                if let Err(error) = self.verify_private_ssh(server_id, &remote_host).await {
                    self.networks
                        .set_runtime_state(server_id, PrivateNetworkStatus::Failed, None, None)
                        .await?;
                    return Err(error);
                }
                let health = match backend.health(&interface).await {
                    Ok(health) => health,
                    Err(error) => {
                        self.networks
                            .set_runtime_state(server_id, PrivateNetworkStatus::Failed, None, None)
                            .await?;
                        return Err(error);
                    }
                };
                self.networks
                    .set_runtime_state(
                        server_id,
                        PrivateNetworkStatus::Active,
                        Some(&public_key),
                        health.latest_handshake,
                    )
                    .await?;
                self.get_required(server_id).await
            }
            Err(error) => {
                self.networks
                    .set_runtime_state(server_id, PrivateNetworkStatus::Failed, None, None)
                    .await?;
                Err(error)
            }
        }
    }

    async fn require_managed_wireguard(&self, server_id: i64) -> sqlx::Result<()> {
        let network = self.get_model(server_id).await?;
        if PrivateNetworkConnectionMode::try_from(network.connection_mode.as_str())?
            != PrivateNetworkConnectionMode::ManagedWireguard
        {
            return Err(sqlx::Error::Protocol(
                "key rotation is only available for OpenOxide-managed WireGuard".into(),
            ));
        }
        Ok(())
    }

    async fn verify_external_transport(
        &self,
        network: &crate::db::models::server_private_networks::ServerPrivateNetwork,
    ) -> sqlx::Result<()> {
        let host = required(network.private_host.as_deref(), "private_host")?;
        self.verify_private_ssh(network.server_id, host).await?;
        self.networks
            .set_health(
                network.server_id,
                PrivateNetworkHealthStatus::Healthy,
                None,
                None,
            )
            .await
    }

    async fn external_health(
        &self,
        network: &crate::db::models::server_private_networks::ServerPrivateNetwork,
    ) -> sqlx::Result<PrivateNetworkHealthDto> {
        let now = chrono::Utc::now().timestamp();
        let reachable = match network.private_host.as_deref() {
            Some(host) => self.verify_private_ssh(network.server_id, host).await,
            None => Err(sqlx::Error::Protocol("private_host is missing".into())),
        };
        let (status, error) = match reachable.as_ref() {
            Ok(()) => (PrivateNetworkHealthStatus::Healthy, None),
            Err(error) => (
                PrivateNetworkHealthStatus::Unreachable,
                Some(error.to_string()),
            ),
        };
        self.networks
            .set_health(network.server_id, status, error.as_deref(), None)
            .await?;
        Ok(PrivateNetworkHealthDto {
            status: health_dto(status),
            interface_exists: true,
            private_ssh_reachable: reachable.is_ok(),
            latest_handshake_at: None,
            checked_at: now,
            error,
        })
    }

    async fn with_operation<T>(
        &self,
        server_id: i64,
        operation: PrivateNetworkOperation,
        future: impl std::future::Future<Output = sqlx::Result<T>>,
    ) -> sqlx::Result<T> {
        if !self
            .networks
            .acquire_operation(server_id, operation, OPERATION_LEASE_SECONDS)
            .await?
        {
            return Err(sqlx::Error::Protocol(
                "another private-network operation is running".into(),
            ));
        }
        tokio::pin!(future);
        let mut heartbeat =
            tokio::time::interval(std::time::Duration::from_secs(OPERATION_HEARTBEAT_SECONDS));
        heartbeat.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        heartbeat.tick().await;
        let result = loop {
            tokio::select! {
                result = &mut future => break result,
                _ = heartbeat.tick() => {
                    match self.networks.renew_operation(server_id, operation, OPERATION_LEASE_SECONDS).await {
                        Ok(true) => {}
                        Ok(false) => {
                            break Err(sqlx::Error::Protocol(
                                "private-network operation lease was lost".into(),
                            ));
                        }
                        Err(error) => {
                            tracing::warn!(server_id, operation = operation.as_str(), %error, "could not renew private-network operation lease");
                        }
                    }
                }
            }
        };
        let release = self.networks.release_operation(server_id, operation).await;
        match (result, release) {
            (Err(error), _) => Err(error),
            (Ok(_), Err(error)) => Err(error),
            (Ok(value), Ok(())) => Ok(value),
        }
    }

    async fn executors(&self, server_id: i64) -> sqlx::Result<(CommandExecutor, CommandExecutor)> {
        let remote = self.direct_executor(server_id).await?;
        let remote = if remote.username() == "root" {
            remote
        } else {
            remote.with_sudo()
        };
        Ok((
            CommandExecutor::Local(LocalExecutor::new().with_non_interactive_sudo()),
            CommandExecutor::Remote(remote),
        ))
    }

    async fn get_model(
        &self,
        server_id: i64,
    ) -> sqlx::Result<crate::db::models::server_private_networks::ServerPrivateNetwork> {
        self.networks
            .get(server_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    async fn get_required(&self, server_id: i64) -> sqlx::Result<ServerPrivateNetworkDto> {
        map_network(self.get_model(server_id).await?)
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
        let port = port
            .try_into()
            .map_err(|error: std::num::TryFromIntError| sqlx::Error::Protocol(error.to_string()))?;
        RetryPolicy::network()
            .run(|attempt| {
                let executor = RemoteExecutor::new(
                    host,
                    port,
                    username.clone(),
                    SshAuth::key_pair(private_key.clone(), public_key.clone()),
                    SshHostKey::InsecureAcceptAny,
                );
                async move {
                    let result = executor.run("true", std::iter::empty::<&str>()).await;
                    if let Err(error) = &result {
                        tracing::debug!(attempt, %error, "private-network SSH verification attempt failed");
                    }
                    result
                }
            })
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
            port.try_into()
                .map_err(|error: std::num::TryFromIntError| {
                    sqlx::Error::Protocol(error.to_string())
                })?,
            username,
            SshAuth::key_pair(private_key, public_key),
            SshHostKey::InsecureAcceptAny,
        ))
    }
}

fn should_auto_repair(status: PrivateNetworkHealthStatusDto, consecutive_failures: i64) -> bool {
    status == PrivateNetworkHealthStatusDto::ConfigDrift
        && consecutive_failures >= AUTO_REPAIR_FAILURE_THRESHOLD
}

fn required<'a>(value: Option<&'a str>, field: &str) -> sqlx::Result<&'a str> {
    value.ok_or_else(|| sqlx::Error::Protocol(format!("{field} is required")))
}

fn direct_network(server_id: i64) -> ServerPrivateNetworkDto {
    ServerPrivateNetworkDto {
        server_id,
        connection_mode: ServerConnectionModeDto::DirectSsh,
        provider: None,
        private_host: None,
        tunnel_address: None,
        public_key: None,
        endpoint: None,
        listen_port: None,
        persistent_keepalive: None,
        status: PrivateNetworkStatusDto::Disabled,
        last_handshake_at: None,
        config_version: 0,
        dns_name: None,
        routes: Vec::new(),
        health_status: PrivateNetworkHealthStatusDto::Unknown,
        health_error: None,
        last_health_check_at: None,
        consecutive_failures: 0,
        rotation_state: PrivateNetworkRotationStateDto::Idle,
        updated_at: chrono::Utc::now().timestamp(),
    }
}

#[cfg(test)]
mod tests {
    use crate::api::dto::server::PrivateNetworkHealthStatusDto;

    use super::should_auto_repair;

    #[test]
    fn auto_repairs_only_persistent_configuration_drift() {
        assert!(!should_auto_repair(
            PrivateNetworkHealthStatusDto::ConfigDrift,
            2
        ));
        assert!(should_auto_repair(
            PrivateNetworkHealthStatusDto::ConfigDrift,
            3
        ));
        assert!(!should_auto_repair(
            PrivateNetworkHealthStatusDto::Unreachable,
            10
        ));
        assert!(!should_auto_repair(
            PrivateNetworkHealthStatusDto::Healthy,
            10
        ));
    }
}
