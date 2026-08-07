use std::collections::HashSet;

use crate::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::OsCli;
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
        match WireGuardQuickBuilder::new(self.executor, WireGuardQuickAction::Up, self.name.clone())
            .run()
            .await
        {
            Ok(output) => Ok(output),
            Err(error) => {
                let _ = os.file(&path).delete().run().await;
                Err(error)
            }
        }
    }

    pub async fn remove(self) -> ExecResult<ExecOutput> {
        validate_interface_name(&self.name)?;
        let _ = WireGuardQuickBuilder::new(
            self.executor,
            WireGuardQuickAction::Down,
            self.name.clone(),
        )
        .run()
        .await;
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

fn parse_handshakes(value: &str) -> ExecResult<Vec<WireGuardHandshake>> {
    let mut keys = HashSet::new();
    value
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (public_key, timestamp) = line.split_once('\t').ok_or_else(|| {
                crate::exec::ExecError::CommandFailed {
                    code: None,
                    stderr: "invalid WireGuard handshake output".into(),
                }
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
    use super::{parse_allowed_ips, parse_handshakes, parse_peer_public_keys};

    const KEY_ONE: &str = "DM5qhLAE20PG9BbfBCger+Ac9D2NDOwCtY1rbYDLf34=";
    const KEY_TWO: &str = "nwQHKWewX3F+JSitCr3JSYVfwJg1Gc9kU12xjz5HpmM=";

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
