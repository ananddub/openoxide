use crate::utils::{
    exec::CommandExecutor,
    os::{
        OsCli,
        wireguard::{WireGuardConfigBuilder, WireGuardPeerBuilder},
    },
};
use zeroize::Zeroize;

use super::{KernelWireGuardHealth, WireGuardInstallPlan};
use crate::services::server::private_network::backend::ManagedWireGuardBackend;

pub struct KernelWireGuardBackend<'a> {
    pub(super) local: &'a CommandExecutor,
    pub(super) remote: &'a CommandExecutor,
}

impl<'a> KernelWireGuardBackend<'a> {
    pub fn new(local: &'a CommandExecutor, remote: &'a CommandExecutor) -> Self {
        Self { local, remote }
    }

    pub(super) fn protocol(error: impl std::fmt::Display) -> sqlx::Error {
        sqlx::Error::Protocol(error.to_string())
    }
}

impl ManagedWireGuardBackend for KernelWireGuardBackend<'_> {
    async fn install(&self, plan: &WireGuardInstallPlan<'_>) -> sqlx::Result<String> {
        let local = OsCli::new(self.local);
        let remote = OsCli::new(self.remote);
        remote
            .package("wireguard-tools")
            .install()
            .run()
            .await
            .map_err(Self::protocol)?;

        if local
            .wireguard()
            .interface(plan.interface)
            .exists()
            .await
            .map_err(Self::protocol)?
            || remote
                .wireguard()
                .interface(plan.interface)
                .exists()
                .await
                .map_err(Self::protocol)?
        {
            return Err(sqlx::Error::Protocol(
                "WireGuard interface already exists; use repair or rotate instead of replacing it"
                    .into(),
            ));
        }

        let panel_private = local
            .wireguard()
            .key()
            .generate()
            .await
            .map_err(Self::protocol)?;
        let panel_public = local
            .wireguard()
            .key()
            .public_from_private(&panel_private)
            .await
            .map_err(Self::protocol)?;
        let remote_private = remote
            .wireguard()
            .key()
            .generate()
            .await
            .map_err(Self::protocol)?;
        let remote_public = remote
            .wireguard()
            .key()
            .public_from_private(&remote_private)
            .await
            .map_err(Self::protocol)?;

        local
            .wireguard()
            .interface(plan.interface)
            .install(
                &WireGuardConfigBuilder::new(panel_private)
                    .address(plan.panel_address.clone())
                    .listen_port(plan.port)
                    .peer(
                        std::iter::once(format!("{}/32", plan.remote_host))
                            .chain(plan.routes.iter().cloned())
                            .fold(
                                WireGuardPeerBuilder::new(remote_public.clone()),
                                |peer, route| peer.allowed_ip(route),
                            )
                            .build()
                            .map_err(Self::protocol)?,
                    )
                    .build()
                    .map_err(Self::protocol)?,
            )
            .await
            .map_err(Self::protocol)?;

        if let Err(error) = remote
            .wireguard()
            .interface(plan.interface)
            .install(
                &WireGuardConfigBuilder::new(remote_private)
                    .address(plan.remote_address.clone())
                    .peer(
                        WireGuardPeerBuilder::new(panel_public)
                            .allowed_ip(plan.panel_host.clone())
                            .endpoint(plan.endpoint)
                            .persistent_keepalive(plan.keepalive)
                            .build()
                            .map_err(Self::protocol)?,
                    )
                    .build()
                    .map_err(Self::protocol)?,
            )
            .await
        {
            let _ = local.wireguard().interface(plan.interface).remove().await;
            return Err(Self::protocol(error));
        }
        Ok(remote_public)
    }

    async fn teardown(&self, interface: &str) -> sqlx::Result<()> {
        let local = OsCli::new(self.local);
        let remote = OsCli::new(self.remote);
        let local_result = local.wireguard().interface(interface).remove().await;
        let remote_result = remote.wireguard().interface(interface).remove().await;
        local_result.map_err(Self::protocol)?;
        remote_result.map_err(Self::protocol)?;
        Ok(())
    }

    async fn health(&self, interface: &str) -> sqlx::Result<KernelWireGuardHealth> {
        let wireguard = OsCli::new(self.local).wireguard();
        let interface = wireguard.interface(interface);
        if !interface.exists().await.map_err(Self::protocol)? {
            return Ok(KernelWireGuardHealth {
                interface_exists: false,
                peer_public_keys: Vec::new(),
                latest_handshake: None,
                allowed_ips: Vec::new(),
            });
        }
        let peer_public_keys = interface.peer_public_keys().await.map_err(Self::protocol)?;
        let latest_handshake = interface
            .parsed_handshakes()
            .await
            .map_err(Self::protocol)?
            .into_iter()
            .filter_map(|value| value.timestamp)
            .max();
        let allowed_ips = interface.allowed_ips().await.map_err(Self::protocol)?;
        Ok(KernelWireGuardHealth {
            interface_exists: true,
            peer_public_keys,
            latest_handshake,
            allowed_ips,
        })
    }

    async fn rotate(&self, plan: &WireGuardInstallPlan<'_>) -> sqlx::Result<String> {
        let local = OsCli::new(self.local);
        let remote = OsCli::new(self.remote);
        let mut local_backup = local
            .wireguard()
            .interface(plan.interface)
            .snapshot_config()
            .await
            .map_err(Self::protocol)?;
        let mut remote_backup = remote
            .wireguard()
            .interface(plan.interface)
            .snapshot_config()
            .await
            .map_err(Self::protocol)?;
        self.teardown(plan.interface).await?;
        match self.install(plan).await {
            Ok(public_key) => {
                if let Err(error) = local.network().ping(&plan.remote_host).run().await {
                    let rollback = self
                        .restore_pair(plan.interface, &mut local_backup, &mut remote_backup)
                        .await;
                    return Err(Self::rotation_error(
                        format!("rotated WireGuard tunnel did not become reachable: {error}"),
                        rollback,
                    ));
                }
                local_backup.zeroize();
                remote_backup.zeroize();
                Ok(public_key)
            }
            Err(rotation_error) => {
                let rollback = self
                    .restore_pair(plan.interface, &mut local_backup, &mut remote_backup)
                    .await;
                Err(Self::rotation_error(
                    format!("WireGuard key rotation failed: {rotation_error}"),
                    rollback,
                ))
            }
        }
    }
}
