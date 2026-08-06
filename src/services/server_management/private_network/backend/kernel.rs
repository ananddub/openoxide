use crate::utils::{
    exec::CommandExecutor,
    os::{
        OsCli,
        wireguard::{WireGuardConfig, WireGuardPeer},
    },
};
use zeroize::Zeroize;

use super::ManagedWireGuardBackend;

pub struct WireGuardInstallPlan<'a> {
    pub interface: &'a str,
    pub panel_address: String,
    pub remote_address: String,
    pub panel_host: String,
    pub remote_host: String,
    pub endpoint: &'a str,
    pub port: u16,
    pub keepalive: u16,
    pub routes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct KernelWireGuardHealth {
    pub interface_exists: bool,
    pub peer_public_keys: Vec<String>,
    pub latest_handshake: Option<i64>,
    pub allowed_ips: Vec<String>,
}

pub struct KernelWireGuardBackend<'a> {
    local: &'a CommandExecutor,
    remote: &'a CommandExecutor,
}

impl<'a> KernelWireGuardBackend<'a> {
    pub fn new(local: &'a CommandExecutor, remote: &'a CommandExecutor) -> Self {
        Self { local, remote }
    }

    fn protocol(error: impl std::fmt::Display) -> sqlx::Error {
        sqlx::Error::Protocol(error.to_string())
    }

    async fn restore_pair(
        &self,
        interface: &str,
        local_backup: &mut String,
        remote_backup: &mut String,
    ) -> Result<(), String> {
        let local = OsCli::new(self.local)
            .wireguard()
            .interface(interface)
            .restore_config(local_backup)
            .await;
        let remote = OsCli::new(self.remote)
            .wireguard()
            .interface(interface)
            .restore_config(remote_backup)
            .await;
        match (local, remote) {
            (Ok(_), Ok(_)) => Ok(()),
            (Err(local), Ok(_)) => Err(format!("panel restore failed: {local}")),
            (Ok(_), Err(remote)) => Err(format!("remote restore failed: {remote}")),
            (Err(local), Err(remote)) => Err(format!(
                "panel restore failed: {local}; remote restore failed: {remote}"
            )),
        }
    }

    fn rotation_error(reason: impl std::fmt::Display, rollback: Result<(), String>) -> sqlx::Error {
        match rollback {
            Ok(()) => sqlx::Error::Protocol(format!(
                "{reason}; previous WireGuard configuration was restored"
            )),
            Err(rollback_error) => sqlx::Error::Protocol(format!(
                "{reason}; WireGuard rollback was incomplete: {rollback_error}"
            )),
        }
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
            .install(&WireGuardConfig {
                private_key: panel_private,
                addresses: vec![plan.panel_address.clone()],
                listen_port: Some(plan.port),
                peers: vec![WireGuardPeer {
                    public_key: remote_public.clone(),
                    allowed_ips: std::iter::once(format!("{}/32", plan.remote_host))
                        .chain(plan.routes.iter().cloned())
                        .collect(),
                    endpoint: None,
                    persistent_keepalive: None,
                }],
            })
            .await
            .map_err(Self::protocol)?;

        if let Err(error) = remote
            .wireguard()
            .interface(plan.interface)
            .install(&WireGuardConfig {
                private_key: remote_private,
                addresses: vec![plan.remote_address.clone()],
                listen_port: None,
                peers: vec![WireGuardPeer {
                    public_key: panel_public,
                    allowed_ips: vec![plan.panel_host.clone()],
                    endpoint: Some(plan.endpoint.to_owned()),
                    persistent_keepalive: Some(plan.keepalive),
                }],
            })
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

#[cfg(test)]
mod tests {
    use super::KernelWireGuardBackend;

    #[test]
    fn rotation_error_reports_incomplete_rollback() {
        let restored = KernelWireGuardBackend::rotation_error("rotation failed", Ok(()));
        assert!(restored.to_string().contains("was restored"));

        let incomplete = KernelWireGuardBackend::rotation_error(
            "rotation failed",
            Err("remote restore failed".into()),
        );
        assert!(incomplete.to_string().contains("rollback was incomplete"));
        assert!(incomplete.to_string().contains("remote restore failed"));
    }
}
