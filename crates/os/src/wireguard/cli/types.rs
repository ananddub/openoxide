use std::{fmt, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WireGuardShowTarget {
    Interface(String),
    All,
    Interfaces,
}

impl fmt::Display for WireGuardShowTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Interface(value) => formatter.write_str(value),
            Self::All => formatter.write_str("all"),
            Self::Interfaces => formatter.write_str("interfaces"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireGuardShowField {
    PublicKey,
    PrivateKey,
    ListenPort,
    FirewallMark,
    Peers,
    PresharedKeys,
    Endpoints,
    AllowedIps,
    LatestHandshakes,
    Transfer,
    PersistentKeepalive,
    Dump,
}

impl WireGuardShowField {
    pub(super) const fn as_arg(self) -> &'static str {
        match self {
            Self::PublicKey => "public-key",
            Self::PrivateKey => "private-key",
            Self::ListenPort => "listen-port",
            Self::FirewallMark => "fwmark",
            Self::Peers => "peers",
            Self::PresharedKeys => "preshared-keys",
            Self::Endpoints => "endpoints",
            Self::AllowedIps => "allowed-ips",
            Self::LatestHandshakes => "latest-handshakes",
            Self::Transfer => "transfer",
            Self::PersistentKeepalive => "persistent-keepalive",
            Self::Dump => "dump",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireGuardQuickAction {
    Up,
    Down,
    Save,
    Strip,
}

impl WireGuardQuickAction {
    pub(super) const fn as_arg(self) -> &'static str {
        match self {
            Self::Up => "up",
            Self::Down => "down",
            Self::Save => "save",
            Self::Strip => "strip",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireGuardConfigAction {
    Set,
    Add,
    Sync,
}

impl WireGuardConfigAction {
    pub(super) const fn as_arg(self) -> &'static str {
        match self {
            Self::Set => "setconf",
            Self::Add => "addconf",
            Self::Sync => "syncconf",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllowedIpAction {
    Replace,
    Add,
    Remove,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FirewallMark {
    Value(u32),
    Off,
}

impl fmt::Display for FirewallMark {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(value) => write!(formatter, "{value}"),
            Self::Off => formatter.write_str("off"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllowedIpChange {
    pub action: AllowedIpAction,
    pub network: ipnet::IpNet,
}

impl AllowedIpChange {
    pub fn replace(network: ipnet::IpNet) -> Self {
        Self {
            action: AllowedIpAction::Replace,
            network,
        }
    }

    pub fn add(network: ipnet::IpNet) -> Self {
        Self {
            action: AllowedIpAction::Add,
            network,
        }
    }

    pub fn remove(network: ipnet::IpNet) -> Self {
        Self {
            action: AllowedIpAction::Remove,
            network,
        }
    }

    pub(super) fn as_arg(&self) -> String {
        let prefix = match self.action {
            AllowedIpAction::Replace => "",
            AllowedIpAction::Add => "+",
            AllowedIpAction::Remove => "-",
        };
        format!("{prefix}{}", self.network.trunc())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireGuardPeerUpdate {
    pub public_key: String,
    pub remove: bool,
    pub preshared_key_file: Option<PathBuf>,
    pub endpoint: Option<String>,
    pub persistent_keepalive: Option<u16>,
    pub allowed_ips: Vec<AllowedIpChange>,
}

impl WireGuardPeerUpdate {
    pub fn new(public_key: impl Into<String>) -> Self {
        Self {
            public_key: public_key.into(),
            remove: false,
            preshared_key_file: None,
            endpoint: None,
            persistent_keepalive: None,
            allowed_ips: Vec::new(),
        }
    }

    pub fn remove(mut self) -> Self {
        self.remove = true;
        self
    }

    pub fn preshared_key_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.preshared_key_file = Some(path.into());
        self
    }

    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    pub fn persistent_keepalive(mut self, seconds: u16) -> Self {
        self.persistent_keepalive = Some(seconds);
        self
    }

    pub fn allowed_ip(mut self, change: AllowedIpChange) -> Self {
        self.allowed_ips.push(change);
        self
    }
}
