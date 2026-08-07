use std::path::PathBuf;

use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

use super::{FirewallMark, WireGuardPeerUpdate};

pub struct WireGuardSetBuilder<'a> {
    executor: &'a CommandExecutor,
    interface: String,
    listen_port: Option<u16>,
    firewall_mark: Option<FirewallMark>,
    private_key_file: Option<PathBuf>,
    peers: Vec<WireGuardPeerUpdate>,
}

impl<'a> WireGuardSetBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, interface: impl Into<String>) -> Self {
        Self {
            executor,
            interface: interface.into(),
            listen_port: None,
            firewall_mark: None,
            private_key_file: None,
            peers: Vec::new(),
        }
    }

    pub fn listen_port(mut self, port: u16) -> Self {
        self.listen_port = Some(port);
        self
    }

    pub fn firewall_mark(mut self, mark: FirewallMark) -> Self {
        self.firewall_mark = Some(mark);
        self
    }

    pub fn private_key_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.private_key_file = Some(path.into());
        self
    }

    pub fn peer(mut self, peer: WireGuardPeerUpdate) -> Self {
        self.peers.push(peer);
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        super::validate_interface_name(&self.interface)?;
        self.validate()?;
        self.executor.run("wg", self.args()).await
    }

    fn validate(&self) -> ExecResult<()> {
        if self.listen_port == Some(0) {
            return Err(super::command_error("WireGuard listen port cannot be zero"));
        }
        for peer in &self.peers {
            super::validate_key(&peer.public_key)
                .map_err(|_| super::command_error("invalid WireGuard peer key"))?;
            if peer.remove
                && (peer.preshared_key_file.is_some()
                    || peer.endpoint.is_some()
                    || peer.persistent_keepalive.is_some()
                    || !peer.allowed_ips.is_empty())
            {
                return Err(super::command_error(
                    "removed WireGuard peer cannot contain update flags",
                ));
            }
            if let Some(endpoint) = &peer.endpoint {
                super::validate_endpoint_value(endpoint)?;
            }
        }
        Ok(())
    }

    pub(super) fn args(&self) -> Vec<String> {
        let mut args = vec!["set".into(), self.interface.clone()];
        if let Some(port) = self.listen_port {
            args.extend(["listen-port".into(), port.to_string()]);
        }
        if let Some(mark) = &self.firewall_mark {
            args.extend(["fwmark".into(), mark.to_string()]);
        }
        if let Some(path) = &self.private_key_file {
            args.extend(["private-key".into(), path.to_string_lossy().into_owned()]);
        }
        for peer in &self.peers {
            args.extend(["peer".into(), peer.public_key.clone()]);
            if peer.remove {
                args.push("remove".into());
                continue;
            }
            if let Some(path) = &peer.preshared_key_file {
                args.extend(["preshared-key".into(), path.to_string_lossy().into_owned()]);
            }
            if let Some(endpoint) = &peer.endpoint {
                args.extend(["endpoint".into(), endpoint.clone()]);
            }
            if let Some(seconds) = peer.persistent_keepalive {
                args.extend(["persistent-keepalive".into(), seconds.to_string()]);
            }
            if !peer.allowed_ips.is_empty() {
                args.extend([
                    "allowed-ips".into(),
                    peer.allowed_ips
                        .iter()
                        .map(|value| value.as_arg())
                        .collect::<Vec<_>>()
                        .join(","),
                ]);
            }
        }
        args
    }
}
