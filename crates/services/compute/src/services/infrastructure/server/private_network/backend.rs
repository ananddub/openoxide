use crate::utils::{
    exec::CommandExecutor,
    os::{
        OsCli,
        wireguard::{WireGuardConfigBuilder, WireGuardPeerBuilder},
    },
};
use zeroize::Zeroize;

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
    pub ssh_port: u16,
    /// Resolved IPv4 of the endpoint, used as the DNAT target for SSH over
    /// the tunnel. `None` when the endpoint has no IPv4 record.
    pub gateway_host: Option<String>,
}

/// Remote-side forwarding/NAT state that must be removed alongside the
/// WireGuard interface.
#[derive(Debug, Clone)]
pub struct WireGuardGateway {
    pub interface: String,
    pub tunnel_ip: String,
    pub host_ip: Option<String>,
    pub ssh_port: u16,
}

#[derive(Debug, Clone)]
pub struct KernelWireGuardHealth {
    pub interface_exists: bool,
    pub peer_public_keys: Vec<String>,
    pub latest_handshake: Option<i64>,
    pub allowed_ips: Vec<String>,
}

pub(super) trait ManagedWireGuardBackend {
    async fn install(&self, plan: &WireGuardInstallPlan<'_>) -> sqlx::Result<String>;
    async fn teardown(
        &self,
        interface: &str,
        gateway: Option<&WireGuardGateway>,
    ) -> sqlx::Result<()>;
    async fn health(&self, interface: &str) -> sqlx::Result<KernelWireGuardHealth>;
    async fn rotate(&self, plan: &WireGuardInstallPlan<'_>) -> sqlx::Result<String>;
}

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

        // WireGuard runs through the OpenOxide agent container, which ships
        // `wireguard-tools` pre-installed — there is nothing to install on the
        // host root. If `wg` is still missing here, the agent is unreachable or
        // an outdated image, so surface that instead of a host apt command.
        if local.has_command("wg").run().await.is_err() {
            return Err(Self::protocol(
                "WireGuard tooling is unavailable on the local panel agent. Update/restart the panel agent (its image bundles wireguard-tools) and retry.",
            ));
        }
        if remote.has_command("wg").run().await.is_err() {
            return Err(Self::protocol(
                "WireGuard tooling is unavailable on the target server's agent. Make sure the OpenOxide agent is deployed and reachable on that server (its image bundles wireguard-tools), then retry.",
            ));
        }

        // Opening the WireGuard UDP port in the host firewall is best-effort:
        // many servers have no `ufw` (cloud-provider firewall, raw iptables, or
        // none at all), so a missing tool or a failed rule must NOT abort the
        // whole setup — WireGuard still works, the port just isn't managed by
        // us. Only log it so the user can open it manually if their firewall
        // actually blocks it.
        if let Err(error) = OsCli::new(self.remote)
            .firewall()
            .allow_port(plan.port)
            .protocol(os::firewall::NetworkProtocol::Udp)
            .run()
            .await
        {
            tracing::warn!(
                port = plan.port,
                %error,
                "could not open the WireGuard UDP port in the remote firewall (no ufw / rule failed); continuing — open it manually if your firewall blocks it"
            );
        }

        // Reuse the keypair from a previous install when its config survived,
        // so repair/retry keeps the remote public key stable. Rotation removes
        // the configs first, so it still gets fresh keys.
        let local_private = local
            .wireguard()
            .interface(plan.interface)
            .snapshot_config()
            .await
            .ok()
            .and_then(|config| private_key_from_config(&config));
        let remote_private = remote
            .wireguard()
            .interface(plan.interface)
            .snapshot_config()
            .await
            .ok()
            .and_then(|config| private_key_from_config(&config));

        // Self-heal orphaned state: a previous setup attempt may have created
        // the interface on one or both hosts and then failed a later step
        // (firewall, DB, validation), leaving an interface the panel never
        // recorded as active. Blocking the retry with "already exists" traps
        // the user in a loop they can't escape (repair/rotate aren't offered
        // for a network that was never activated). Since the caller explicitly
        // asked to set up, tear any stale interface down on both hosts first,
        // then install fresh.
        let local_exists = local
            .wireguard()
            .interface(plan.interface)
            .exists()
            .await
            .map_err(Self::protocol)?;
        let remote_exists = remote
            .wireguard()
            .interface(plan.interface)
            .exists()
            .await
            .map_err(Self::protocol)?;
        if local_exists || remote_exists {
            tracing::warn!(
                interface = plan.interface,
                local_exists,
                remote_exists,
                "stale WireGuard interface found before setup; tearing it down and reinstalling"
            );
            let _ = local.wireguard().interface(plan.interface).remove().await;
            let _ = remote.wireguard().interface(plan.interface).remove().await;
        }

        let panel_private = match local_private {
            Some(key) => key,
            None => local
                .wireguard()
                .key()
                .generate()
                .await
                .map_err(Self::protocol)?,
        };
        let panel_public = local
            .wireguard()
            .key()
            .public_from_private(&panel_private)
            .await
            .map_err(Self::protocol)?;
        let remote_private = match remote_private {
            Some(key) => key,
            None => remote
                .wireguard()
                .key()
                .generate()
                .await
                .map_err(Self::protocol)?,
        };
        let remote_public = remote
            .wireguard()
            .key()
            .public_from_private(&remote_private)
            .await
            .map_err(Self::protocol)?;

        let local_peer_builder = std::iter::once(format!("{}/32", plan.remote_host))
            .chain(plan.routes.iter().cloned())
            .fold(
                WireGuardPeerBuilder::new(remote_public.clone())
                    .endpoint(plan.endpoint)
                    .persistent_keepalive(plan.keepalive),
                |peer, route| peer.allowed_ip(route),
            );
        let remote_peer_builder =
            WireGuardPeerBuilder::new(panel_public).allowed_ip(plan.panel_host.clone());

        local
            .wireguard()
            .interface(plan.interface)
            .install(
                &WireGuardConfigBuilder::new(panel_private)
                    .address(plan.panel_address.clone())
                    .mtu(1420)
                    .peer(local_peer_builder.build().map_err(Self::protocol)?)
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
                    .listen_port(plan.port)
                    .mtu(1420)
                    .peer(remote_peer_builder.build().map_err(Self::protocol)?)
                    .build()
                    .map_err(Self::protocol)?,
            )
            .await
        {
            let _ = local.wireguard().interface(plan.interface).remove().await;
            return Err(Self::protocol(format!(
                "Could not start WireGuard on the remote server at {}: {error}. Verify that UDP port {} is free and the endpoint is reachable",
                plan.endpoint, plan.port
            )));
        }

        // Make the remote agent container a gateway for its host: forward
        // tunnel traffic out of the WireGuard interface and MASQUERADE it, so
        // packets arriving over the tunnel (e.g. SSH to the host) are routed on
        // to the host network. Best-effort — our direct ip/wg bring-up skips
        // wg-quick's PostUp hooks, and some hosts lack iptables; a failure here
        // must not fail the whole setup (the tunnel itself is already up).
        // Make the remote agent a gateway: forward + NAT so tunnel traffic
        // reaches the host, and DNAT the tunnel IP's SSH to the host's sshd.
        Self::enable_gateway(
            self.remote,
            &WireGuardGateway {
                interface: plan.interface.to_owned(),
                tunnel_ip: plan.remote_host.clone(),
                host_ip: plan.gateway_host.clone(),
                ssh_port: plan.ssh_port,
            },
        )
        .await;

        Ok(remote_public)
    }

    async fn teardown(
        &self,
        interface: &str,
        gateway: Option<&WireGuardGateway>,
    ) -> sqlx::Result<()> {
        // Remove the gateway rules first: they are independent of the
        // interface and would otherwise survive teardown, silently forwarding
        // tunnel traffic after the network was deleted.
        if let Some(gateway) = gateway {
            Self::disable_gateway(self.remote, gateway).await;
        }
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
        // Keep the gateway rules: rotation only changes keys, so the existing
        // SSH forwarding stays valid and survives a rollback untouched.
        self.teardown(plan.interface, None).await?;
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

impl KernelWireGuardBackend<'_> {
    /// Enable in-container forwarding + NAT so the agent routes tunnel traffic
    /// to its host. The remote agent interface holds the tunnel IP (e.g.
    /// 10.100.1.2); SSH to that IP must reach the host's sshd, so we DNAT the
    /// tunnel IP's configured SSH port to the host's real IP and MASQUERADE the
    /// forwarded traffic. Idempotent and best-effort (ignores already-exists /
    /// no iptables).
    async fn enable_gateway(exec: &CommandExecutor, gateway: &WireGuardGateway) {
        let _ = exec
            .run(
                "sh",
                ["-c", "sysctl -w net.ipv4.ip_forward=1 2>/dev/null || true"],
            )
            .await;
        for rule in gateway_install_rules(gateway) {
            let _ = exec.run("sh", ["-c", &rule]).await;
        }
    }

    /// Undo [`Self::enable_gateway`]. Best-effort: a missing rule (or a host
    /// without iptables) must not block deleting the network.
    pub(super) async fn disable_gateway(exec: &CommandExecutor, gateway: &WireGuardGateway) {
        for rule in gateway_remove_rules(gateway) {
            let _ = exec.run("sh", ["-c", &rule]).await;
        }
    }

    pub(super) async fn restore_pair(
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

    pub(super) fn rotation_error(
        reason: impl std::fmt::Display,
        rollback: Result<(), String>,
    ) -> sqlx::Error {
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

fn gateway_nat_host(gateway: &WireGuardGateway) -> Option<&str> {
    let host_ip = gateway.host_ip.as_deref()?;
    (!host_ip.is_empty() && host_ip != gateway.tunnel_ip.as_str() && !gateway.tunnel_ip.is_empty())
        .then_some(host_ip)
}

fn gateway_install_rules(gateway: &WireGuardGateway) -> Vec<String> {
    let interface = &gateway.interface;
    // Forwarding to/from the WireGuard interface only. (We must NOT broadly
    // MASQUERADE 10.0.0.0/8 — Docker's own networks here are 10.0.0.0/24
    // (swarm ingress) and 10.0.1.0/24 (panel↔agent); a blanket rule mangles
    // their source IPs and kills ALL container networking.)
    let mut rules = vec![
        format!(
            "iptables -C FORWARD -i {interface} -j ACCEPT 2>/dev/null || iptables -A FORWARD -i {interface} -j ACCEPT"
        ),
        format!(
            "iptables -C FORWARD -o {interface} -j ACCEPT 2>/dev/null || iptables -A FORWARD -o {interface} -j ACCEPT"
        ),
    ];
    if let Some(host_ip) = gateway_nat_host(gateway) {
        let tunnel_ip = &gateway.tunnel_ip;
        let port = gateway.ssh_port;
        rules.push(format!(
            "iptables -t nat -C PREROUTING -d {tunnel_ip} -p tcp --dport {port} -j DNAT --to-destination {host_ip}:{port} 2>/dev/null || iptables -t nat -A PREROUTING -d {tunnel_ip} -p tcp --dport {port} -j DNAT --to-destination {host_ip}:{port}"
        ));
        rules.push(format!(
            "iptables -t nat -C OUTPUT -d {tunnel_ip} -p tcp --dport {port} -j DNAT --to-destination {host_ip}:{port} 2>/dev/null || iptables -t nat -A OUTPUT -d {tunnel_ip} -p tcp --dport {port} -j DNAT --to-destination {host_ip}:{port}"
        ));
        rules.push(format!(
            "iptables -t nat -C POSTROUTING -d {host_ip} -p tcp --dport {port} -j MASQUERADE 2>/dev/null || iptables -t nat -A POSTROUTING -d {host_ip} -p tcp --dport {port} -j MASQUERADE"
        ));
    }
    rules
}

fn gateway_remove_rules(gateway: &WireGuardGateway) -> Vec<String> {
    let interface = &gateway.interface;
    let mut rules = vec![
        format!("iptables -D FORWARD -i {interface} -j ACCEPT 2>/dev/null || true"),
        format!("iptables -D FORWARD -o {interface} -j ACCEPT 2>/dev/null || true"),
    ];
    if let Some(host_ip) = gateway_nat_host(gateway) {
        let tunnel_ip = &gateway.tunnel_ip;
        let port = gateway.ssh_port;
        rules.push(format!(
            "iptables -t nat -D PREROUTING -d {tunnel_ip} -p tcp --dport {port} -j DNAT --to-destination {host_ip}:{port} 2>/dev/null || true"
        ));
        rules.push(format!(
            "iptables -t nat -D OUTPUT -d {tunnel_ip} -p tcp --dport {port} -j DNAT --to-destination {host_ip}:{port} 2>/dev/null || true"
        ));
        rules.push(format!(
            "iptables -t nat -D POSTROUTING -d {host_ip} -p tcp --dport {port} -j MASQUERADE 2>/dev/null || true"
        ));
    }
    rules
}

fn private_key_from_config(config: &str) -> Option<String> {
    config.lines().find_map(|line| {
        let (name, value) = line.split_once('=')?;
        if name.trim() != "PrivateKey" {
            return None;
        }
        let value = value.trim();
        (!value.is_empty() && !value.chars().any(char::is_whitespace)).then(|| value.to_owned())
    })
}

#[cfg(test)]
mod tests {
    use super::{
        KernelWireGuardBackend, WireGuardGateway, gateway_install_rules, gateway_remove_rules,
        private_key_from_config,
    };

    const PRIVATE_KEY: &str = "DM5qhLAE20PG9BbfBCger+Ac9D2NDOwCtY1rbYDLf34=";

    fn gateway() -> WireGuardGateway {
        WireGuardGateway {
            interface: "openoxide2".into(),
            tunnel_ip: "10.77.2.2".into(),
            host_ip: Some("203.0.113.10".into()),
            ssh_port: 2222,
        }
    }

    #[test]
    fn gateway_rules_use_the_configured_ssh_port() {
        let rules = gateway_install_rules(&gateway());
        assert!(
            rules.iter().any(
                |rule| rule.contains("--dport 2222 -j DNAT --to-destination 203.0.113.10:2222")
            )
        );
        assert!(rules.iter().any(|rule| rule.contains("-A POSTROUTING")));
        assert!(rules.iter().all(|rule| !rule.contains("--dport 22 ")));

        let cleanup = gateway_remove_rules(&gateway());
        assert!(cleanup.iter().all(|rule| rule.contains("-D ")));
        assert!(
            cleanup
                .iter()
                .any(|rule| rule.contains("--dport 2222 -j DNAT"))
        );
    }

    #[test]
    fn gateway_rules_skip_nat_without_a_host_ip() {
        let gateway = WireGuardGateway {
            host_ip: None,
            ..gateway()
        };
        let rules = gateway_install_rules(&gateway);
        assert!(rules.iter().all(|rule| !rule.contains("-t nat")));
        assert_eq!(rules.len(), 2);
    }

    #[test]
    fn reads_private_keys_from_existing_configs() {
        let config = concat!(
            "[Interface]\nPrivateKey = DM5qhLAE20PG9BbfBCger+Ac9D2NDOwCtY1rbYDLf34=\n",
            "Address = 10.77.2.1/24\n\n[Peer]\n",
            "PublicKey = nwQHKWewX3F+JSitCr3JSYVfwJg1Gc9kU12xjz5HpmM=\n"
        );
        assert_eq!(
            private_key_from_config(&config).as_deref(),
            Some(PRIVATE_KEY)
        );
        assert_eq!(
            private_key_from_config("[Interface]\nAddress = 10.77.2.1/24\n"),
            None
        );
        assert_eq!(
            private_key_from_config("[Interface]\nPrivateKey = \n"),
            None
        );
    }

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
