mod config_file;
mod key_command;
mod quick;
mod set;
mod show;
mod show_config;
mod types;

pub use config_file::WireGuardConfigFileBuilder;
pub use quick::WireGuardQuickBuilder;
pub use set::WireGuardSetBuilder;
pub use show::WireGuardShowBuilder;
pub use show_config::WireGuardShowConfigBuilder;
pub use types::{
    AllowedIpAction, AllowedIpChange, FirewallMark, WireGuardConfigAction, WireGuardPeerUpdate,
    WireGuardQuickAction, WireGuardShowField, WireGuardShowTarget,
};

use crate::exec::{ExecError, ExecResult};

pub(super) fn command_error(message: impl Into<String>) -> ExecError {
    ExecError::CommandFailed {
        code: None,
        stderr: message.into(),
    }
}

pub(super) fn validate_interface_name(name: &str) -> ExecResult<()> {
    if !name.is_empty()
        && name.len() <= 15
        && name
            .chars()
            .all(|value| value.is_ascii_alphanumeric() || matches!(value, '-' | '_' | '=' | '+'))
    {
        Ok(())
    } else {
        Err(command_error("invalid WireGuard interface name"))
    }
}

pub(super) fn validate_interface_or_config(value: &str) -> ExecResult<()> {
    if value.ends_with(".conf") {
        if value.is_empty() || value.contains(['\n', '\r', '\0']) {
            return Err(command_error("invalid WireGuard configuration path"));
        }
        Ok(())
    } else {
        validate_interface_name(value)
    }
}

pub(super) fn validate_endpoint_value(value: &str) -> ExecResult<()> {
    super::config::validate_endpoint(value).map_err(|_| command_error("invalid WireGuard endpoint"))
}

pub(super) use super::config::validate_key;
pub(super) use key_command::{WireGuardKeyAction, WireGuardKeyCommand};

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::exec::{CommandExecutor, LocalExecutor};

    use super::*;

    fn executor() -> CommandExecutor {
        CommandExecutor::Local(LocalExecutor::new())
    }

    #[test]
    fn show_builder_supports_every_wireguard_field() {
        let executor = executor();
        let fields = [
            (WireGuardShowField::PublicKey, "public-key"),
            (WireGuardShowField::PrivateKey, "private-key"),
            (WireGuardShowField::ListenPort, "listen-port"),
            (WireGuardShowField::FirewallMark, "fwmark"),
            (WireGuardShowField::Peers, "peers"),
            (WireGuardShowField::PresharedKeys, "preshared-keys"),
            (WireGuardShowField::Endpoints, "endpoints"),
            (WireGuardShowField::AllowedIps, "allowed-ips"),
            (WireGuardShowField::LatestHandshakes, "latest-handshakes"),
            (WireGuardShowField::Transfer, "transfer"),
            (
                WireGuardShowField::PersistentKeepalive,
                "persistent-keepalive",
            ),
            (WireGuardShowField::Dump, "dump"),
        ];
        for (field, expected) in fields {
            assert_eq!(
                WireGuardShowBuilder::new(&executor, WireGuardShowTarget::Interface("wg0".into()))
                    .field(field)
                    .args(),
                ["show", "wg0", expected]
            );
        }
    }

    #[test]
    fn set_builder_supports_all_interface_and_peer_flags() {
        let executor = executor();
        let peer = WireGuardPeerUpdate::new("nwQHKWewX3F+JSitCr3JSYVfwJg1Gc9kU12xjz5HpmM=")
            .preshared_key_file("/run/wg.psk")
            .endpoint("vpn.example.com:51820")
            .persistent_keepalive(25)
            .allowed_ip(AllowedIpChange::replace(
                ipnet::IpNet::from_str("10.0.0.0/24").unwrap(),
            ))
            .allowed_ip(AllowedIpChange::add(
                ipnet::IpNet::from_str("10.1.0.0/24").unwrap(),
            ))
            .allowed_ip(AllowedIpChange::remove(
                ipnet::IpNet::from_str("10.2.0.0/24").unwrap(),
            ));
        assert_eq!(
            WireGuardSetBuilder::new(&executor, "wg0")
                .listen_port(51820)
                .firewall_mark(FirewallMark::Value(42))
                .private_key_file("/run/wg.key")
                .peer(peer)
                .args(),
            [
                "set",
                "wg0",
                "listen-port",
                "51820",
                "fwmark",
                "42",
                "private-key",
                "/run/wg.key",
                "peer",
                "nwQHKWewX3F+JSitCr3JSYVfwJg1Gc9kU12xjz5HpmM=",
                "preshared-key",
                "/run/wg.psk",
                "endpoint",
                "vpn.example.com:51820",
                "persistent-keepalive",
                "25",
                "allowed-ips",
                "10.0.0.0/24,+10.1.0.0/24,-10.2.0.0/24",
            ]
        );
    }

    #[test]
    fn config_and_quick_builders_cover_every_action() {
        let executor = executor();
        for (action, expected) in [
            (WireGuardQuickAction::Up, "up"),
            (WireGuardQuickAction::Down, "down"),
            (WireGuardQuickAction::Save, "save"),
            (WireGuardQuickAction::Strip, "strip"),
        ] {
            assert_eq!(
                WireGuardQuickBuilder::new(&executor, action, "wg0").args(),
                [expected, "wg0"]
            );
        }
        for (action, expected) in [
            (WireGuardConfigAction::Set, "setconf"),
            (WireGuardConfigAction::Add, "addconf"),
            (WireGuardConfigAction::Sync, "syncconf"),
        ] {
            assert_eq!(
                WireGuardConfigFileBuilder::new(&executor, action, "wg0", "/run/wg.conf").args(),
                [expected, "wg0", "/run/wg.conf"]
            );
        }
        assert_eq!(
            WireGuardShowConfigBuilder::new(&executor, "wg0").args(),
            ["showconf", "wg0"]
        );
    }
}
