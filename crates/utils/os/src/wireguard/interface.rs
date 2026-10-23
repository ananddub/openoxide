use std::collections::HashSet;

use crate::OsCli;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::file::FileMode;
use zeroize::Zeroize;

use super::WireGuardConfig;
use super::cli::{
    WireGuardQuickAction, WireGuardQuickBuilder, WireGuardShowBuilder, WireGuardShowField,
    WireGuardShowTarget, validate_interface_name,
};
use super::config::validate_key;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireGuardHandshake {
    pub public_key: String,
    pub timestamp: Option<i64>,
}

pub struct WireGuardInterfaceBuilder<'a> {
    executor: &'a CommandExecutor,
    name: String,
}

impl<'a> WireGuardInterfaceBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, name: String) -> Self {
        Self { executor, name }
    }

    pub async fn install(self, config: &WireGuardConfig) -> ExecResult<ExecOutput> {
        validate_interface_name(&self.name)?;
        config
            .validate()
            .map_err(|error| crate::exec::ExecError::CommandFailed {
                code: None,
                stderr: error.to_string(),
            })?;
        let path = format!("/etc/wireguard/{}.conf", self.name);
        let os = OsCli::new(self.executor);
        os.dir("/etc/wireguard")
            .create()
            .with_parents()
            .run()
            .await?;
        let mut rendered = config.render();
        let write_result = os.file(&path).write(&rendered).execute().await;
        rendered.zeroize();
        write_result?;
        os.file(&path).chmod(FileMode::OwnerReadWrite).run().await?;
        // Bring the interface up with direct `ip`/`wg` commands instead of the
        // `wg-quick` wrapper. `wg-quick` shells out to `readlink` on startup,
        // which on hosts shipping uutils (Rust) coreutils tries to open a
        // locale file that the distro's AppArmor `wg-quick` profile denies —
        // aborting the whole bring-up with a confusing exit 126. The low-level
        // commands below are portable and immune to that. If this host has no
        // conf-aware tooling we still fall back to `wg-quick up`.
        match self.bring_up_direct(&os, config).await {
            Ok(output) => Ok(output),
            Err(direct_error) => {
                tracing::debug!(%direct_error, "direct WireGuard bring-up failed; trying wg-quick");
                match WireGuardQuickBuilder::new(
                    self.executor,
                    WireGuardQuickAction::Up,
                    self.name.clone(),
                )
                .run()
                .await
                {
                    Ok(output) => Ok(output),
                    Err(error) => {
                        let _ = self.bring_down_direct().await;
                        let _ = os.file(&path).delete().run().await;
                        Err(error)
                    }
                }
            }
        }
    }

    /// Bring the interface up with explicit `ip`/`wg` commands (no wg-quick).
    async fn bring_up_direct(
        &self,
        _os: &OsCli<'_>,
        config: &WireGuardConfig,
    ) -> ExecResult<ExecOutput> {
        let name = &self.name;
        let conf = format!("/etc/wireguard/{name}.conf");
        // Clean any half-up interface first (idempotent).
        let _ = self.bring_down_direct().await;

        // 1. Create the WireGuard link.
        self.executor
            .run("ip", ["link", "add", "dev", name, "type", "wireguard"])
            .await?;

        // 2. Load keys + peers via `wg setconf` from a stripped conf (only the
        //    [Interface] PrivateKey/ListenPort + [Peer] blocks — no Address/DNS/
        //    MTU, which `wg` rejects). Strip with sed, no readlink involved.
        let strip = format!(
            "sed -e '/^Address/d' -e '/^DNS/d' -e '/^MTU/d' -e '/^Table/d' -e '/^Pre/d' -e '/^Post/d' -e '/^SaveConfig/d' {conf}"
        );
        if let Err(error) = self
            .executor
            .run("bash", ["-c", &format!("wg setconf {name} <({strip})")])
            .await
        {
            let _ = self.bring_down_direct().await;
            return Err(error);
        }

        // 3. Assign the tunnel addresses.
        for address in &config.addresses {
            if let Err(error) = self
                .executor
                .run("ip", ["address", "add", address, "dev", name])
                .await
            {
                let _ = self.bring_down_direct().await;
                return Err(error);
            }
        }

        // 4. Bring the link up (with MTU when configured).
        let mtu = config.mtu.unwrap_or(1420).to_string();
        let output = match self
            .executor
            .run(
                "ip",
                ["link", "set", "mtu", mtu.as_str(), "up", "dev", name],
            )
            .await
        {
            Ok(output) => output,
            Err(error) => {
                let _ = self.bring_down_direct().await;
                return Err(error);
            }
        };

        // 5. Install the peer routes. wg-quick does this implicitly from
        //    AllowedIPs; the direct path must do it explicitly or only the
        //    interface subnet is reachable and custom routes silently do
        //    nothing. `replace` keeps retries idempotent.
        for route in route_destinations(config) {
            if let Err(error) = self
                .executor
                .run("ip", ["route", "replace", route, "dev", name])
                .await
            {
                let _ = self.bring_down_direct().await;
                return Err(error);
            }
        }
        Ok(output)
    }

    async fn bring_down_direct(&self) -> ExecResult<ExecOutput> {
        self.executor
            .run("ip", ["link", "del", "dev", self.name.as_str()])
            .await
    }

    pub async fn remove(self) -> ExecResult<ExecOutput> {
        validate_interface_name(&self.name)?;
        let quick = WireGuardQuickBuilder::new(
            self.executor,
            WireGuardQuickAction::Down,
            self.name.clone(),
        )
        .run()
        .await;
        // `wg-quick down` needs the rendered conf; an interface created
        // directly (or whose conf is already gone) would otherwise survive
        // teardown. Fall back to deleting the link itself.
        if quick.is_err() {
            let _ = self.bring_down_direct().await;
        }
        OsCli::new(self.executor)
            .file(format!("/etc/wireguard/{}.conf", self.name))
            .delete()
            .run()
            .await
    }

    pub async fn latest_handshakes(self) -> ExecResult<ExecOutput> {
        validate_interface_name(&self.name)?;
        WireGuardShowBuilder::new(
            self.executor,
            WireGuardShowTarget::Interface(self.name.clone()),
        )
        .field(WireGuardShowField::LatestHandshakes)
        .run()
        .await
    }

    pub async fn exists(&self) -> ExecResult<bool> {
        validate_interface_name(&self.name)?;
        match WireGuardShowBuilder::new(
            self.executor,
            WireGuardShowTarget::Interface(self.name.clone()),
        )
        .run()
        .await
        {
            Ok(_) => Ok(true),
            Err(crate::exec::ExecError::CommandFailed { .. }) => Ok(false),
            Err(error) => Err(error),
        }
    }

    pub async fn public_key(&self) -> ExecResult<String> {
        validate_interface_name(&self.name)?;
        let output = WireGuardShowBuilder::new(
            self.executor,
            WireGuardShowTarget::Interface(self.name.clone()),
        )
        .field(WireGuardShowField::PublicKey)
        .run()
        .await?;
        Ok(output.stdout.trim().to_owned())
    }

    pub async fn allowed_ips(&self) -> ExecResult<Vec<String>> {
        validate_interface_name(&self.name)?;
        let output = WireGuardShowBuilder::new(
            self.executor,
            WireGuardShowTarget::Interface(self.name.clone()),
        )
        .field(WireGuardShowField::AllowedIps)
        .run()
        .await?;
        parse_allowed_ips(&output.stdout)
    }

    pub async fn peer_public_keys(&self) -> ExecResult<Vec<String>> {
        validate_interface_name(&self.name)?;
        let output = WireGuardShowBuilder::new(
            self.executor,
            WireGuardShowTarget::Interface(self.name.clone()),
        )
        .field(WireGuardShowField::Peers)
        .run()
        .await?;
        parse_peer_public_keys(&output.stdout)
    }

    pub async fn parsed_handshakes(&self) -> ExecResult<Vec<WireGuardHandshake>> {
        validate_interface_name(&self.name)?;
        let output = WireGuardShowBuilder::new(
            self.executor,
            WireGuardShowTarget::Interface(self.name.clone()),
        )
        .field(WireGuardShowField::LatestHandshakes)
        .run()
        .await?;
        parse_handshakes(&output.stdout)
    }

    pub async fn save_config(&self) -> ExecResult<ExecOutput> {
        validate_interface_name(&self.name)?;
        WireGuardQuickBuilder::new(self.executor, WireGuardQuickAction::Save, self.name.clone())
            .run()
            .await
    }

    pub async fn snapshot_config(&self) -> ExecResult<String> {
        validate_interface_name(&self.name)?;
        let output = OsCli::new(self.executor)
            .file(format!("/etc/wireguard/{}.conf", self.name))
            .read()
            .execute()
            .await?;
        Ok(output.stdout)
    }

    pub async fn restore_config(self, config: &mut String) -> ExecResult<ExecOutput> {
        validate_interface_name(&self.name)?;
        let path = format!("/etc/wireguard/{}.conf", self.name);
        let os = OsCli::new(self.executor);
        let _ = WireGuardQuickBuilder::new(
            self.executor,
            WireGuardQuickAction::Down,
            self.name.clone(),
        )
        .run()
        .await;
        let result = async {
            os.file(&path).write(config.as_str()).execute().await?;
            os.file(&path).chmod(FileMode::OwnerReadWrite).run().await?;
            WireGuardQuickBuilder::new(self.executor, WireGuardQuickAction::Up, self.name.clone())
                .run()
                .await
        }
        .await;
        config.zeroize();
        result
    }
}

fn route_destinations(config: &WireGuardConfig) -> Vec<&str> {
    config
        .peers
        .iter()
        .flat_map(|peer| peer.allowed_ips.iter().map(String::as_str))
        .collect()
}

fn parse_handshakes(value: &str) -> ExecResult<Vec<WireGuardHandshake>> {
    let mut keys = HashSet::new();
    value
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (public_key, timestamp) =
                line.split_once('\t')
                    .ok_or_else(|| crate::exec::ExecError::CommandFailed {
                        code: None,
                        stderr: "invalid WireGuard handshake output".into(),
                    })?;
            validate_key(public_key).map_err(|_| command_error("invalid WireGuard peer key"))?;
            if !keys.insert(public_key) {
                return Err(command_error("duplicate WireGuard peer key"));
            }
            let timestamp = timestamp.trim().parse::<i64>().map_err(|_| {
                crate::exec::ExecError::CommandFailed {
                    code: None,
                    stderr: "invalid WireGuard handshake timestamp".into(),
                }
            })?;
            if timestamp < 0 {
                return Err(command_error("invalid WireGuard handshake timestamp"));
            }
            Ok(WireGuardHandshake {
                public_key: public_key.to_owned(),
                timestamp: (timestamp > 0).then_some(timestamp),
            })
        })
        .collect()
}

fn parse_allowed_ips(value: &str) -> ExecResult<Vec<String>> {
    let mut routes = HashSet::new();
    let mut parsed = Vec::new();
    for line in value.lines().filter(|line| !line.trim().is_empty()) {
        let (public_key, values) = line
            .split_once('\t')
            .ok_or_else(|| command_error("invalid WireGuard allowed-ips output"))?;
        validate_key(public_key).map_err(|_| command_error("invalid WireGuard peer key"))?;
        if values.trim().is_empty() || values.trim() == "(none)" {
            return Err(command_error("WireGuard peer has no allowed IPs"));
        }
        for value in values.split_whitespace() {
            let network = value
                .parse::<ipnet::IpNet>()
                .map_err(|_| command_error("invalid WireGuard allowed IP"))?
                .trunc()
                .to_string();
            if !routes.insert(network.clone()) {
                return Err(command_error("duplicate WireGuard allowed IP"));
            }
            parsed.push(network);
        }
    }
    Ok(parsed)
}

fn parse_peer_public_keys(value: &str) -> ExecResult<Vec<String>> {
    let mut keys = HashSet::new();
    value
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let key = line.trim();
            validate_key(key).map_err(|_| command_error("invalid WireGuard peer key"))?;
            if !keys.insert(key) {
                return Err(command_error("duplicate WireGuard peer key"));
            }
            Ok(key.to_owned())
        })
        .collect()
}

fn command_error(message: &str) -> crate::exec::ExecError {
    crate::exec::ExecError::CommandFailed {
        code: None,
        stderr: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_allowed_ips, parse_handshakes, parse_peer_public_keys, route_destinations};
    use crate::wireguard::{WireGuardConfig, WireGuardPeer};

    const KEY_ONE: &str = "DM5qhLAE20PG9BbfBCger+Ac9D2NDOwCtY1rbYDLf34=";
    const KEY_TWO: &str = "nwQHKWewX3F+JSitCr3JSYVfwJg1Gc9kU12xjz5HpmM=";

    #[test]
    fn installs_routes_for_every_peer_allowed_ip() {
        let config = WireGuardConfig {
            private_key: KEY_ONE.into(),
            addresses: vec!["10.77.8.1/24".into()],
            peers: vec![WireGuardPeer {
                public_key: KEY_TWO.into(),
                allowed_ips: vec!["10.77.8.2/32".into(), "192.168.1.0/24".into()],
                endpoint: None,
                persistent_keepalive: None,
            }],
            ..Default::default()
        };
        assert_eq!(
            route_destinations(&config),
            ["10.77.8.2/32", "192.168.1.0/24"]
        );
    }

    #[test]
    fn parses_latest_handshakes() {
        let values = parse_handshakes(&format!("{KEY_ONE}\t0\n{KEY_TWO}\t1720000000\n")).unwrap();
        assert_eq!(values[0].timestamp, None);
        assert_eq!(values[1].timestamp, Some(1_720_000_000));
    }

    #[test]
    fn strictly_parses_allowed_ips_and_peer_keys() {
        let routes = parse_allowed_ips(&format!(
            "{KEY_ONE}\t10.90.0.2/32 172.19.0.9/16\n{KEY_TWO}\tfd00::2/128\n"
        ))
        .unwrap();
        assert_eq!(routes, ["10.90.0.2/32", "172.19.0.0/16", "fd00::2/128"]);

        let peers = parse_peer_public_keys(&format!("{KEY_ONE}\n{KEY_TWO}\n")).unwrap();
        assert_eq!(peers, [KEY_ONE, KEY_TWO]);
    }

    #[test]
    fn rejects_malformed_or_duplicate_runtime_state() {
        assert!(parse_allowed_ips("missing-tab").is_err());
        assert!(parse_allowed_ips(&format!("{KEY_ONE}\tnot-a-cidr\n")).is_err());
        assert!(
            parse_allowed_ips(&format!(
                "{KEY_ONE}\t10.90.0.2/32\n{KEY_TWO}\t10.90.0.2/32\n"
            ))
            .is_err()
        );
        assert!(parse_peer_public_keys(&format!("{KEY_ONE}\n{KEY_ONE}\n")).is_err());
        assert!(parse_handshakes(&format!("{KEY_ONE}\t-1\n")).is_err());
    }
}
