use std::sync::Arc;

use auto_di::singleton;
use serde::{Deserialize, Serialize};

use crate::{
    db::queries::FlashQueries,
    flash_gen::{
        models::ServerPrivateNetworks,
        server_private_networks::{
            ServerPrivateNetworksAcquireOperationParams, ServerPrivateNetworksFinishRotationParams,
            ServerPrivateNetworksInsertParams, ServerPrivateNetworksRenewOperationParams,
            ServerPrivateNetworksSetHealthHandshakeParams, ServerPrivateNetworksSetHealthParams,
            ServerPrivateNetworksSetRuntimeFullParams,
            ServerPrivateNetworksSetRuntimeStatusHandshakeParams,
            ServerPrivateNetworksSetRuntimeStatusKeyParams,
            ServerPrivateNetworksUpdateConfigParams,
        },
    },
    utils::exec::CommandExecutor,
};

use super::{
    addressing::{interface_name, panel_host, tunnel_addresses},
    backend::{KernelWireGuardBackend, ManagedWireGuardBackend, WireGuardInstallPlan},
    enums::{
        PrivateNetworkConnectionMode, PrivateNetworkHealthStatus, PrivateNetworkOperation,
        PrivateNetworkProvider, PrivateNetworkRotationState, PrivateNetworkStatus,
    },
    mapping::{connection_mode, health_dto, map_network, provider},
    retry::RetryPolicy,
    validation::validate,
};

const OPERATION_LEASE_SECONDS: i64 = 30;
const OPERATION_HEARTBEAT_SECONDS: u64 = 10;
const STALE_HANDSHAKE_SECONDS: i64 = 180;
const AUTO_REPAIR_FAILURE_THRESHOLD: i64 = 3;

pub struct ServerPrivateNetworkService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl ServerPrivateNetworkService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn get(&self, server_id: i64) -> sqlx::Result<Option<ServerPrivateNetworkDto>> {
        self.assert_server(server_id).await?;
        self.networks_get(server_id)
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
        // For managed WireGuard, fill any missing tunnel address / derived
        // private host / UDP listen port / endpoint with safe non-overlapping
        // values BEFORE validation. The UI only has to pick "Managed
        // WireGuard"; the user may still override any field, but blank fields
        // never produce "private_host must be X" / "listen port required" /
        // "already exists" errors.
        let mut input = input;
        if input.connection_mode == ServerConnectionModeDto::ManagedWireguard {
            self.autofill_managed_defaults(server_id, &mut input).await?;
        }
        validate(&input)?;
        if input.connection_mode == ServerConnectionModeDto::ManagedWireguard {
            self.assert_unique_managed_network(server_id, &input)
                .await?;
        }
        if input.connection_mode == ServerConnectionModeDto::DirectSsh {
            self.networks_disable(server_id).await?;
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
            self.networks_upsert(
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
        let iface = interface_name(server_id);
        let local_exec = crate::services::compute::compose::remote::agent_executor_for(None)
            .await
            .map_err(sqlx::Error::Protocol)?;
        let local_os = crate::utils::os::OsCli::new(&local_exec);
        let _ = local_os
            .wireguard()
            .quick(os::wireguard::WireGuardQuickAction::Down, &iface)
            .run()
            .await;

        if let Ok(executor) = crate::services::compute::compose::remote::remote_executor(
            &self.queries.pool,
            server_id,
        )
        .await
        {
            let remote_exec = CommandExecutor::Remote(Box::new(executor));
            let remote_os = crate::utils::os::OsCli::new(&remote_exec);
            let _ = remote_os
                .wireguard()
                .quick(os::wireguard::WireGuardQuickAction::Down, &iface)
                .run()
                .await;
        }

        self.networks_disable(server_id).await
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
            self.networks_set_runtime_state(
                server_id,
                PrivateNetworkStatus::Configuring,
                None,
                None,
            )
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
        let _ = self.networks_force_release_operation(server_id).await;
        self.with_operation(server_id, PrivateNetworkOperation::Setup, async {
            let (local, remote) = self.executors(server_id).await?;
            let backend = KernelWireGuardBackend::new(&local, &remote);
            let interface = interface_name(server_id);
            let _ = backend.teardown(&interface).await;
            self.networks_set_runtime_state(
                server_id,
                PrivateNetworkStatus::Configuring,
                None,
                None,
            )
            .await?;
            self.setup_locked(server_id).await
        })
        .await
    }

    pub async fn rotate_wireguard(&self, server_id: i64) -> sqlx::Result<ServerPrivateNetworkDto> {
        self.require_managed_wireguard(server_id).await?;
        self.with_operation(server_id, PrivateNetworkOperation::Rotate, async {
            self.networks_begin_rotation(server_id).await?;
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
                self.networks_set_runtime_state(
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
                    self.networks_finish_rotation(server_id, public_key).await?;
                    self.get_required(server_id).await
                }
                Err(error) => {
                    self.networks_fail_rotation(server_id).await?;
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
        self.networks_set_health(server_id, status, error.as_deref(), kernel.latest_handshake)
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
        let networks = match self.networks_list_managed().await {
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
                        && let Err(error) = self
                            .networks_set_runtime_state(
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
            return self.networks_disable(server_id).await;
        }
        self.with_operation(server_id, PrivateNetworkOperation::Teardown, async {
            let (local, remote) = self.executors(server_id).await?;
            KernelWireGuardBackend::new(&local, &remote)
                .teardown(&interface_name(server_id))
                .await?;
            self.networks_disable(server_id).await?;
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
        let remote_ip = self
            .queries
            .servers_get_by_id(server_id)
            .await
            .map(|server| server.ip_address)
            .unwrap_or_default();
        let port = network
            .listen_port
            .and_then(|value| value.try_into().ok())
            .unwrap_or(51820);

        let default_endpoint = format!("{remote_ip}:{port}");
        let raw_endpoint = network.endpoint.as_deref().unwrap_or("").trim();
        let endpoint = if raw_endpoint.is_empty()
            || raw_endpoint.contains("example.com")
            || raw_endpoint.contains("pannel.example")
            || raw_endpoint.contains("example")
            || raw_endpoint.contains("exmaple")
        {
            &default_endpoint
        } else {
            raw_endpoint
        };

        if let Err(error) = resolve_wireguard_endpoint(endpoint).await {
            let message = error.to_string();
            self.record_setup_failure(server_id, None, &message).await?;
            return Err(error);
        }

        let (panel_address, remote_address, remote_host) = tunnel_addresses(cidr)?;
        let (local, remote) = self.executors(server_id).await?;
        self.remove_orphaned_local_interfaces(&local).await?;
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
                port,
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
                    let message = format!(
                        "WireGuard was installed, but SSH over the tunnel ({remote_host}) is not reachable: {error}. Verify that UDP port {port} reaches the remote server and that its firewall allows the port"
                    );
                    self.record_setup_failure(server_id, Some(&public_key), &message)
                        .await?;
                    return Err(sqlx::Error::Protocol(message));
                }
                let health = match backend.health(&interface).await {
                    Ok(health) => health,
                    Err(error) => {
                        let message = format!(
                            "WireGuard was installed, but its health check failed: {error}"
                        );
                        self.record_setup_failure(server_id, Some(&public_key), &message)
                            .await?;
                        return Err(sqlx::Error::Protocol(message));
                    }
                };
                self.networks_set_runtime_state(
                    server_id,
                    PrivateNetworkStatus::Active,
                    Some(&public_key),
                    health.latest_handshake,
                )
                .await?;
                self.networks_set_health(
                    server_id,
                    PrivateNetworkHealthStatus::Healthy,
                    None,
                    health.latest_handshake,
                )
                .await?;
                self.get_required(server_id).await
            }
            Err(error) => {
                let message = format!("Managed WireGuard setup failed: {error}");
                self.record_setup_failure(server_id, None, &message).await?;
                Err(sqlx::Error::Protocol(message))
            }
        }
    }

    async fn record_setup_failure(
        &self,
        server_id: i64,
        public_key: Option<&str>,
        message: &str,
    ) -> sqlx::Result<()> {
        self.networks_set_runtime_state(server_id, PrivateNetworkStatus::Failed, public_key, None)
            .await?;
        self.networks_set_health(
            server_id,
            PrivateNetworkHealthStatus::Unreachable,
            Some(message),
            None,
        )
        .await
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

    async fn verify_external_transport(&self, network: &ServerPrivateNetworks) -> sqlx::Result<()> {
        let host = required(network.private_host.as_deref(), "private_host")?;
        self.verify_private_ssh(network.server_id, host).await?;
        self.networks_set_health(
            network.server_id,
            PrivateNetworkHealthStatus::Healthy,
            None,
            None,
        )
        .await
    }

    async fn external_health(
        &self,
        network: &ServerPrivateNetworks,
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
        self.networks_set_health(network.server_id, status, error.as_deref(), None)
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
            .networks_acquire_operation(server_id, operation, OPERATION_LEASE_SECONDS)
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
                    match self.networks_renew_operation(server_id, operation, OPERATION_LEASE_SECONDS).await {
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
        let release = self.networks_release_operation(server_id, operation).await;
        match (result, release) {
            (Err(error), _) => Err(error),
            (Ok(_), Err(error)) => Err(error),
            (Ok(value), Ok(())) => Ok(value),
        }
    }

    async fn executors(&self, server_id: i64) -> sqlx::Result<(CommandExecutor, CommandExecutor)> {
        let local = crate::services::compute::compose::remote::agent_executor_for(None)
            .await
            .map_err(sqlx::Error::Protocol)?;
        let remote = crate::services::compute::compose::remote::agent_executor(server_id)
            .await
            .map_err(sqlx::Error::Protocol)?;
        Ok((local, remote))
    }

    async fn get_model(&self, server_id: i64) -> sqlx::Result<ServerPrivateNetworks> {
        if let Some(existing) = self.networks_get(server_id).await? {
            Ok(existing)
        } else {
            self.assert_server(server_id).await?;
            let tunnel_ip = format!("10.77.{server_id}.1/24");
            let default_network = self
                .networks_upsert(
                    server_id,
                    PrivateNetworkConnectionMode::ManagedWireguard,
                    Some(PrivateNetworkProvider::Wireguard),
                    None,
                    Some(&tunnel_ip),
                    None,
                    None,
                    Some(51820),
                    Some(20),
                    None,
                    "[]",
                    PrivateNetworkStatus::Configuring,
                )
                .await?;
            Ok(default_network)
        }
    }

    async fn assert_unique_managed_network(
        &self,
        server_id: i64,
        input: &UpdatePrivateNetworkDto,
    ) -> sqlx::Result<()> {
        let tunnel = input.tunnel_address.as_deref();
        for network in self.networks_list_managed().await? {
            if network.server_id == server_id {
                continue;
            }
            if network.tunnel_address.as_deref() == tunnel {
                return Err(sqlx::Error::Protocol(format!(
                    "WireGuard tunnel network {} is already assigned to server {}",
                    tunnel.unwrap_or_default(),
                    network.server_id,
                )));
            }
        }
        Ok(())
    }

    async fn remove_orphaned_local_interfaces(&self, local: &CommandExecutor) -> sqlx::Result<()> {
        let active = self
            .networks_list_managed()
            .await?
            .into_iter()
            .map(|network| interface_name(network.server_id))
            .collect::<std::collections::HashSet<_>>();
        let output = match os::OsCli::new(local)
            .wireguard()
            .show(os::wireguard::WireGuardShowTarget::Interfaces)
            .run()
            .await
        {
            Ok(out) => out,
            Err(_) => return Ok(()),
        };
        for interface in orphaned_managed_interfaces(&output.stdout, &active) {
            tracing::warn!(interface, "removing orphaned managed WireGuard interface");
            let _ = os::OsCli::new(local)
                .wireguard()
                .interface(&interface)
                .remove()
                .await;
        }
        Ok(())
    }

    async fn get_required(&self, server_id: i64) -> sqlx::Result<ServerPrivateNetworkDto> {
        map_network(self.get_model(server_id).await?)
    }

    async fn assert_server(&self, server_id: i64) -> sqlx::Result<()> {
        self.queries.servers_get_by_id(server_id).await.map(|_| ())
    }

    /// Fill blank managed-WireGuard fields with safe, non-overlapping values so
    /// the user only has to choose "Managed WireGuard". Any field the user
    /// already set is preserved (so they can still customise).
    async fn autofill_managed_defaults(
        &self,
        server_id: i64,
        input: &mut UpdatePrivateNetworkDto,
    ) -> sqlx::Result<()> {
        // Provider is always WIREGUARD for managed mode.
        if input.provider.is_none() {
            input.provider = Some(PrivateNetworkProviderDto::Wireguard);
        }

        // Gather tunnel CIDRs + listen ports already used by other managed
        // networks so we never collide.
        let existing = self
            .queries
            .server_private_networks_list_managed("MANAGED_WIREGUARD", "DISABLED")
            .await
            .unwrap_or_default();
        let used_cidrs: std::collections::HashSet<String> = existing
            .iter()
            .filter(|n| n.server_id != server_id)
            .filter_map(|n| n.tunnel_address.clone())
            .collect();
        let used_ports: std::collections::HashSet<i64> = existing
            .iter()
            .filter(|n| n.server_id != server_id)
            .filter_map(|n| n.listen_port)
            .collect();

        // Allocate a free /24 inside 10.100.0.0/16. We re-pick not only when
        // the tunnel is blank but also when the supplied CIDR is already
        // assigned to a DIFFERENT server — so a stale/duplicate value from the
        // UI self-corrects instead of surfacing "already assigned to server N".
        let needs_cidr = match input
            .tunnel_address
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            None => true,
            Some(given) => used_cidrs.contains(given),
        };
        if needs_cidr {
            let mut chosen = None;
            for octet in 0..=255u32 {
                let candidate = format!("10.100.{octet}.0/24");
                if !used_cidrs.contains(&candidate) {
                    chosen = Some(candidate);
                    break;
                }
            }
            input.tunnel_address = Some(chosen.ok_or_else(|| {
                sqlx::Error::Protocol("no free tunnel network available in 10.100.0.0/16".into())
            })?);
            // The derived host must follow the new CIDR, so clear a stale one.
            input.private_host = None;
        }

        // Derive the private host (.2) from the tunnel CIDR when blank or wrong.
        if let Some(cidr) = input.tunnel_address.as_deref() {
            let (_, _, expected_host) = tunnel_addresses(cidr)?;
            let host_ok = input
                .private_host
                .as_deref()
                .map(str::trim)
                .is_some_and(|h| h == expected_host);
            if !host_ok {
                input.private_host = Some(expected_host);
            }
        }

        // Allocate a free UDP listen port starting at 51820.
        let port_taken = input
            .listen_port
            .is_some_and(|p| used_ports.contains(&i64::from(p)));
        if input.listen_port.is_none() || port_taken {
            let mut port = 51820u16;
            while used_ports.contains(&i64::from(port)) {
                port = port.checked_add(1).ok_or_else(|| {
                    sqlx::Error::Protocol("no free WireGuard UDP port available".into())
                })?;
            }
            input.listen_port = Some(port);
            // A re-picked port invalidates a user/UI endpoint built on the old
            // one; let the endpoint block below rebuild it.
            if port_taken {
                input.endpoint = None;
            }
        }

        // Default endpoint to the server's own IP + the listen port when blank,
        // so validation (endpoint port must match listen port) passes.
        if input
            .endpoint
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .is_none()
            && let Ok(server) = self.queries.servers_get_by_id(server_id).await
        {
            let ip = server.ip_address;
            if !ip.trim().is_empty() {
                input.endpoint = Some(format!("{ip}:{}", input.listen_port.unwrap_or(51820)));
            }
        }

        // Sensible keepalive default for NAT traversal.
        if input.persistent_keepalive.is_none() {
            input.persistent_keepalive = Some(25);
        }

        Ok(())
    }

    async fn verify_private_ssh(&self, server_id: i64, host: &str) -> sqlx::Result<()> {
        let (_, port, _username, _private_key, _public_key) =
            crate::services::infrastructure::server::direct_ssh_credentials(
                &self.queries,
                server_id,
            )
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let port: u16 = port
            .try_into()
            .map_err(|error: std::num::TryFromIntError| sqlx::Error::Protocol(error.to_string()))?;

        // The WireGuard interface lives inside the LOCAL AGENT container, not
        // the panel process — so the reachability check must run from the agent
        // (which can route the tunnel IP), otherwise the panel, having no route
        // to 10.x.x.x, always times out. Run a TCP-connect probe to the tunnel
        // IP's SSH port from the agent's netns.
        let agent = crate::services::compute::compose::remote::agent_executor_for(None)
            .await
            .map_err(sqlx::Error::Protocol)?;
        let probe = format!(
            // Try bash /dev/tcp first (no extra tooling), fall back to nc.
            "timeout 5 bash -c 'exec 3<>/dev/tcp/{host}/{port}' 2>/dev/null || timeout 5 nc -z {host} {port} 2>/dev/null"
        );
        RetryPolicy::fast_network()
            .run(|attempt| {
                let agent = agent.clone();
                let probe = probe.clone();
                async move {
                    let result = agent.run("sh", ["-c", probe.as_str()]).await;
                    if let Err(ref error) = result {
                        tracing::debug!(attempt, %error, "private-network tunnel reachability attempt failed");
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

    async fn networks_get(&self, server_id: i64) -> sqlx::Result<Option<ServerPrivateNetworks>> {
        match self.queries.server_private_networks_get(server_id).await {
            Ok(row) => Ok(Some(normalize_model(row))),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }

    async fn networks_list_managed(&self) -> sqlx::Result<Vec<ServerPrivateNetworks>> {
        Ok(self
            .queries
            .server_private_networks_list_managed(
                PrivateNetworkConnectionMode::ManagedWireguard.as_str(),
                PrivateNetworkStatus::Disabled.as_str(),
            )
            .await?
            .into_iter()
            .map(normalize_model)
            .collect())
    }

    async fn networks_disable(&self, server_id: i64) -> sqlx::Result<()> {
        self.queries
            .server_private_networks_disable(server_id)
            .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn networks_upsert(
        &self,
        server_id: i64,
        mode: PrivateNetworkConnectionMode,
        provider: Option<PrivateNetworkProvider>,
        private_host: Option<&str>,
        tunnel_address: Option<&str>,
        public_key: Option<&str>,
        endpoint: Option<&str>,
        listen_port: Option<i64>,
        persistent_keepalive: Option<i64>,
        dns_name: Option<&str>,
        routes: &str,
        status: PrivateNetworkStatus,
    ) -> sqlx::Result<ServerPrivateNetworks> {
        if self.networks_get(server_id).await?.is_some() {
            self.queries
                .server_private_networks_update_config(&ServerPrivateNetworksUpdateConfigParams {
                    connection_mode: mode.as_str().to_owned(),
                    provider: provider
                        .map(|value| value.as_str())
                        .unwrap_or_default()
                        .to_owned(),
                    private_host: private_host.unwrap_or_default().to_owned(),
                    tunnel_address: tunnel_address.unwrap_or_default().to_owned(),
                    public_key: public_key.unwrap_or_default().to_owned(),
                    endpoint: endpoint.unwrap_or_default().to_owned(),
                    listen_port: listen_port.unwrap_or_default(),
                    persistent_keepalive: persistent_keepalive.unwrap_or_default(),
                    dns_name: dns_name.unwrap_or_default().to_owned(),
                    routes: routes.to_owned(),
                    status: status.as_str().to_owned(),
                    server_id,
                })
                .await?;
        } else {
            self.queries
                .server_private_networks_insert(&ServerPrivateNetworksInsertParams {
                    server_id,
                    connection_mode: mode.as_str().to_owned(),
                    provider: provider
                        .map(|value| value.as_str())
                        .unwrap_or_default()
                        .to_owned(),
                    private_host: private_host.unwrap_or_default().to_owned(),
                    tunnel_address: tunnel_address.unwrap_or_default().to_owned(),
                    public_key: public_key.unwrap_or_default().to_owned(),
                    endpoint: endpoint.unwrap_or_default().to_owned(),
                    listen_port: listen_port.unwrap_or_default(),
                    persistent_keepalive: persistent_keepalive.unwrap_or_default(),
                    dns_name: dns_name.unwrap_or_default().to_owned(),
                    routes: routes.to_owned(),
                    status: status.as_str().to_owned(),
                })
                .await?;
        }
        Ok(normalize_model(
            self.queries.server_private_networks_get(server_id).await?,
        ))
    }

    async fn networks_set_runtime_state(
        &self,
        server_id: i64,
        status: PrivateNetworkStatus,
        public_key: Option<&str>,
        last_handshake_at: Option<i64>,
    ) -> sqlx::Result<()> {
        match (public_key, last_handshake_at) {
            (None, None) => {
                self.queries
                    .server_private_networks_set_runtime_status(status.as_str(), server_id)
                    .await
            }
            (Some(public_key), None) => {
                self.queries
                    .server_private_networks_set_runtime_status_key(
                        &ServerPrivateNetworksSetRuntimeStatusKeyParams {
                            status: status.as_str().to_owned(),
                            public_key: public_key.to_owned(),
                            server_id,
                        },
                    )
                    .await
            }
            (None, Some(last_handshake_at)) => {
                self.queries
                    .server_private_networks_set_runtime_status_handshake(
                        &ServerPrivateNetworksSetRuntimeStatusHandshakeParams {
                            status: status.as_str().to_owned(),
                            last_handshake_at,
                            server_id,
                        },
                    )
                    .await
            }
            (Some(public_key), Some(last_handshake_at)) => {
                self.queries
                    .server_private_networks_set_runtime_full(
                        &ServerPrivateNetworksSetRuntimeFullParams {
                            status: status.as_str().to_owned(),
                            public_key: public_key.to_owned(),
                            last_handshake_at,
                            server_id,
                        },
                    )
                    .await
            }
        }
    }

    async fn networks_acquire_operation(
        &self,
        server_id: i64,
        operation: PrivateNetworkOperation,
        lease_seconds: i64,
    ) -> sqlx::Result<bool> {
        let lease_until = chrono::Utc::now().timestamp() + lease_seconds;
        Ok(self
            .queries
            .server_private_networks_acquire_operation(
                &ServerPrivateNetworksAcquireOperationParams {
                    operation: operation.as_str().to_owned(),
                    operation_lease_until: lease_until,
                    server_id,
                    operation_lease_until2: lease_until,
                },
            )
            .await
            .is_ok())
    }

    async fn networks_release_operation(
        &self,
        server_id: i64,
        operation: PrivateNetworkOperation,
    ) -> sqlx::Result<()> {
        self.queries
            .server_private_networks_release_operation(server_id, operation.as_str())
            .await
    }

    async fn networks_force_release_operation(&self, server_id: i64) -> sqlx::Result<()> {
        self.queries
            .server_private_networks_force_release_operation(server_id)
            .await
    }

    async fn networks_renew_operation(
        &self,
        server_id: i64,
        operation: PrivateNetworkOperation,
        lease_seconds: i64,
    ) -> sqlx::Result<bool> {
        Ok(self
            .queries
            .server_private_networks_renew_operation(&ServerPrivateNetworksRenewOperationParams {
                operation_lease_until: chrono::Utc::now().timestamp() + lease_seconds,
                server_id,
                operation: operation.as_str().to_owned(),
            })
            .await
            .is_ok())
    }

    async fn networks_set_health(
        &self,
        server_id: i64,
        status: PrivateNetworkHealthStatus,
        error: Option<&str>,
        last_handshake_at: Option<i64>,
    ) -> sqlx::Result<()> {
        let status_str = status.as_str();
        if let Some(last_handshake_at) = last_handshake_at {
            self.queries
                .server_private_networks_set_health_handshake(
                    &ServerPrivateNetworksSetHealthHandshakeParams {
                        health_status: status_str.to_owned(),
                        health_error: error.unwrap_or_default().to_owned(),
                        last_handshake_at,
                        healthy: status_str.to_owned(),
                        server_id,
                    },
                )
                .await
        } else {
            self.queries
                .server_private_networks_set_health(&ServerPrivateNetworksSetHealthParams {
                    health_status: status_str.to_owned(),
                    health_error: error.unwrap_or_default().to_owned(),
                    healthy: status_str.to_owned(),
                    server_id,
                })
                .await
        }
    }

    async fn networks_begin_rotation(&self, server_id: i64) -> sqlx::Result<()> {
        self.queries
            .server_private_networks_begin_rotation(
                PrivateNetworkRotationState::Rotating.as_str(),
                server_id,
            )
            .await
    }

    async fn networks_finish_rotation(&self, server_id: i64, public_key: &str) -> sqlx::Result<()> {
        self.queries
            .server_private_networks_finish_rotation(&ServerPrivateNetworksFinishRotationParams {
                public_key: public_key.to_owned(),
                rotation_state: PrivateNetworkRotationState::Idle.as_str().to_owned(),
                server_id,
            })
            .await
    }

    async fn networks_fail_rotation(&self, server_id: i64) -> sqlx::Result<()> {
        self.queries
            .server_private_networks_fail_rotation(
                PrivateNetworkRotationState::Failed.as_str(),
                server_id,
            )
            .await
    }
}

fn orphaned_managed_interfaces(
    interfaces: &str,
    active: &std::collections::HashSet<String>,
) -> Vec<String> {
    interfaces
        .split_whitespace()
        .filter(|interface| {
            (interface.starts_with("openoxide") || interface.starts_with("rpwg"))
                && !active.contains(*interface)
        })
        .map(str::to_owned)
        .collect()
}

async fn resolve_wireguard_endpoint(endpoint: &str) -> sqlx::Result<()> {
    let target = if endpoint.contains(':') {
        endpoint.to_string()
    } else {
        format!("{endpoint}:51820")
    };
    tokio::net::lookup_host(&target)
        .await
        .map_err(|error| {
            sqlx::Error::Protocol(format!(
                "Remote WireGuard endpoint '{endpoint}' could not be resolved: {error}. Enter the remote server's reachable public IP or DNS name with its UDP port, for example 163.44.122.248:51820"
            ))
        })?
        .next()
        .ok_or_else(|| {
            sqlx::Error::Protocol(format!(
                "Remote WireGuard endpoint '{endpoint}' resolved to no addresses. Check the hostname and DNS configuration"
            ))
        })?;
    Ok(())
}

fn should_auto_repair(status: PrivateNetworkHealthStatusDto, consecutive_failures: i64) -> bool {
    status == PrivateNetworkHealthStatusDto::ConfigDrift
        && consecutive_failures >= AUTO_REPAIR_FAILURE_THRESHOLD
}

fn required<'a>(value: Option<&'a str>, field: &str) -> sqlx::Result<&'a str> {
    value.ok_or_else(|| sqlx::Error::Protocol(format!("{field} is required")))
}

// The flash generator emits non-optional bind parameters for nullable
// columns, so absent values are stored as '' / 0. Normalizing them back to
// None on read keeps the service-layer Option semantics of the old model.
fn clean_str(value: Option<String>) -> Option<String> {
    value.filter(|value| !value.is_empty())
}

fn clean_port(value: Option<i64>) -> Option<i64> {
    value.filter(|value| *value != 0)
}

fn normalize_model(row: ServerPrivateNetworks) -> ServerPrivateNetworks {
    ServerPrivateNetworks {
        server_id: row.server_id,
        connection_mode: row.connection_mode,
        provider: clean_str(row.provider),
        private_host: clean_str(row.private_host),
        tunnel_address: clean_str(row.tunnel_address),
        public_key: clean_str(row.public_key),
        endpoint: clean_str(row.endpoint),
        listen_port: clean_port(row.listen_port),
        persistent_keepalive: clean_port(row.persistent_keepalive),
        status: row.status,
        last_handshake_at: row.last_handshake_at,
        config_version: row.config_version,
        created_at: row.created_at,
        updated_at: row.updated_at,
        dns_name: clean_str(row.dns_name),
        routes: row.routes,
        health_status: row.health_status,
        health_error: clean_str(row.health_error),
        last_health_check_at: row.last_health_check_at,
        consecutive_failures: row.consecutive_failures,
        operation: row.operation,
        operation_lease_until: row.operation_lease_until,
        config_hash: clean_str(row.config_hash),
        rotation_state: row.rotation_state,
    }
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

// ── JSON payload contract (moved from the retired api::dto layer) ──────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServerConnectionModeDto {
    DirectSsh,
    ManagedWireguard,
    ExternalPrivateNetwork,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrivateNetworkProviderDto {
    Wireguard,
    Tailscale,
    Zerotier,
    Netbird,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrivateNetworkStatusDto {
    Disabled,
    Configuring,
    Active,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrivateNetworkHealthStatusDto {
    Unknown,
    Healthy,
    Degraded,
    Unreachable,
    ConfigDrift,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrivateNetworkRotationStateDto {
    Idle,
    Rotating,
    RollingBack,
    Failed,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePrivateNetworkDto {
    pub connection_mode: ServerConnectionModeDto,
    pub provider: Option<PrivateNetworkProviderDto>,
    pub private_host: Option<String>,
    pub tunnel_address: Option<String>,
    pub public_key: Option<String>,
    pub endpoint: Option<String>,
    pub listen_port: Option<u16>,
    pub persistent_keepalive: Option<u16>,
    pub dns_name: Option<String>,
    pub routes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ServerPrivateNetworkDto {
    pub server_id: i64,
    pub connection_mode: ServerConnectionModeDto,
    pub provider: Option<PrivateNetworkProviderDto>,
    pub private_host: Option<String>,
    pub tunnel_address: Option<String>,
    pub public_key: Option<String>,
    pub endpoint: Option<String>,
    pub listen_port: Option<u16>,
    pub persistent_keepalive: Option<u16>,
    pub status: PrivateNetworkStatusDto,
    pub last_handshake_at: Option<i64>,
    pub config_version: i64,
    pub dns_name: Option<String>,
    pub routes: Vec<String>,
    pub health_status: PrivateNetworkHealthStatusDto,
    pub health_error: Option<String>,
    pub last_health_check_at: Option<i64>,
    pub consecutive_failures: i64,
    pub rotation_state: PrivateNetworkRotationStateDto,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrivateNetworkHealthDto {
    pub status: PrivateNetworkHealthStatusDto,
    pub interface_exists: bool,
    pub private_ssh_reachable: bool,
    pub latest_handshake_at: Option<i64>,
    pub checked_at: i64,
    pub error: Option<String>,
}

impl ServerPrivateNetworkService {
    /// Deserializes an `UpdatePrivateNetworkDto` from a generic
    /// `ResourceRequest` payload. Kept as an associated function so the
    /// ConnectRPC handler can surface serde failures as `invalid_argument`
    /// before any database work starts.
    pub fn parse_update_payload(
        payload: &str,
    ) -> Result<UpdatePrivateNetworkDto, serde_json::Error> {
        serde_json::from_str(payload)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PrivateNetworkHealthStatusDto, PrivateNetworkStatusDto, ServerConnectionModeDto,
        UpdatePrivateNetworkDto, orphaned_managed_interfaces, should_auto_repair,
    };
    use std::collections::HashSet;

    #[test]
    fn removes_only_openoxide_interfaces_without_database_owners() {
        let active = HashSet::from(["openoxide14".to_owned()]);
        assert_eq!(
            orphaned_managed_interfaces("wg0 openoxide15 openoxide14 tailscale0", &active),
            vec!["openoxide15"],
        );
    }

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

    #[test]
    fn accepts_frontend_enum_casing() {
        let input: UpdatePrivateNetworkDto = serde_json::from_value(serde_json::json!({
            "connection_mode": "MANAGED_WIREGUARD",
            "provider": "WIREGUARD",
            "private_host": "10.77.2.2",
            "tunnel_address": "10.77.2.0/24",
            "public_key": null,
            "endpoint": "panel.example.com:51820",
            "listen_port": 51820,
            "persistent_keepalive": 25,
            "dns_name": null,
            "routes": []
        }))
        .unwrap();
        assert_eq!(
            input.connection_mode,
            ServerConnectionModeDto::ManagedWireguard
        );
    }

    #[test]
    fn serializes_response_enums_for_frontend() {
        assert_eq!(
            serde_json::to_string(&PrivateNetworkStatusDto::Configuring).unwrap(),
            "\"CONFIGURING\""
        );
    }
}
