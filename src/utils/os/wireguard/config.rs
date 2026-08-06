#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireGuardPeer {
    pub public_key: String,
    pub allowed_ips: Vec<String>,
    pub endpoint: Option<String>,
    pub persistent_keepalive: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireGuardConfig {
    pub private_key: String,
    pub addresses: Vec<String>,
    pub listen_port: Option<u16>,
    pub peers: Vec<WireGuardPeer>,
}

impl WireGuardConfig {
    pub fn validate(&self) -> Result<(), WireGuardConfigError> {
        validate_key(&self.private_key).map_err(|_| WireGuardConfigError::InvalidPrivateKey)?;
        if self.addresses.is_empty() {
            return Err(WireGuardConfigError::MissingAddress);
        }
        if self.listen_port == Some(0) {
            return Err(WireGuardConfigError::InvalidListenPort);
        }
        let mut addresses = HashSet::new();
        for address in &self.addresses {
            let network = address
                .parse::<ipnet::IpNet>()
                .map_err(|_| WireGuardConfigError::InvalidAddress(address.clone()))?;
            if !addresses.insert(network.to_string()) {
                return Err(WireGuardConfigError::DuplicateAddress(network.to_string()));
            }
        }
        let mut routes = HashSet::new();
        let mut public_keys = HashSet::new();
        for peer in &self.peers {
            validate_key(&peer.public_key).map_err(|_| WireGuardConfigError::InvalidPublicKey)?;
            if !public_keys.insert(peer.public_key.clone()) {
                return Err(WireGuardConfigError::DuplicatePeerPublicKey);
            }
            if peer.allowed_ips.is_empty() {
                return Err(WireGuardConfigError::MissingAllowedIps);
            }
            if let Some(endpoint) = &peer.endpoint {
                validate_endpoint(endpoint)?;
            }
            for allowed in &peer.allowed_ips {
                let network = allowed
                    .parse::<ipnet::IpNet>()
                    .map_err(|_| WireGuardConfigError::InvalidAllowedIp(allowed.clone()))?;
                if network.prefix_len() == 0 {
                    return Err(WireGuardConfigError::DefaultRouteNotAllowed);
                }
                let canonical = network.trunc().to_string();
                if !routes.insert(canonical.clone()) {
                    return Err(WireGuardConfigError::DuplicateAllowedIp(canonical));
                }
            }
        }
        Ok(())
    }

    pub fn render(&self) -> String {
        let mut output = String::from("[Interface]\n");
        output.push_str(&format!("PrivateKey = {}\n", self.private_key));
        if !self.addresses.is_empty() {
            output.push_str(&format!("Address = {}\n", self.addresses.join(", ")));
        }
        if let Some(port) = self.listen_port {
            output.push_str(&format!("ListenPort = {port}\n"));
        }
        for peer in &self.peers {
            output.push_str("\n[Peer]\n");
            output.push_str(&format!("PublicKey = {}\n", peer.public_key));
            output.push_str(&format!("AllowedIPs = {}\n", peer.allowed_ips.join(", ")));
            if let Some(endpoint) = &peer.endpoint {
                output.push_str(&format!("Endpoint = {endpoint}\n"));
            }
            if let Some(keepalive) = peer.persistent_keepalive {
                output.push_str(&format!("PersistentKeepalive = {keepalive}\n"));
            }
        }
        output
    }
}

pub(super) fn validate_key(value: &str) -> Result<(), ()> {
    if value.chars().any(char::is_whitespace) {
        return Err(());
    }
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(value)
        .map_err(|_| ())?;
    (decoded.len() == 32).then_some(()).ok_or(())
}

fn validate_endpoint(value: &str) -> Result<(), WireGuardConfigError> {
    if value.is_empty() || value.chars().any(char::is_whitespace) {
        return Err(WireGuardConfigError::InvalidEndpoint);
    }
    if value.parse::<std::net::SocketAddr>().is_ok() {
        return Ok(());
    }
    let (host, port) = value
        .rsplit_once(':')
        .ok_or(WireGuardConfigError::InvalidEndpoint)?;
    if host.is_empty()
        || host.len() > 253
        || host.starts_with('-')
        || host.ends_with('-')
        || !host
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-'))
        || port.parse::<u16>().ok().filter(|port| *port != 0).is_none()
    {
        return Err(WireGuardConfigError::InvalidEndpoint);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{WireGuardConfig, WireGuardConfigError, WireGuardPeer};

    const PRIVATE_KEY: &str = "DM5qhLAE20PG9BbfBCger+Ac9D2NDOwCtY1rbYDLf34=";
    const PUBLIC_KEY: &str = "nwQHKWewX3F+JSitCr3JSYVfwJg1Gc9kU12xjz5HpmM=";
    const SECOND_PUBLIC_KEY: &str = "K7gNU3sdo+OL0wNhqoVWhr3g6s1xYv72ol/pe/Unols=";

    #[test]
    fn renders_wireguard_configuration() {
        let rendered = WireGuardConfig {
            private_key: PRIVATE_KEY.into(),
            addresses: vec!["10.77.1.1/24".into()],
            listen_port: Some(51820),
            peers: vec![WireGuardPeer {
                public_key: PUBLIC_KEY.into(),
                allowed_ips: vec!["10.77.1.2/32".into()],
                endpoint: Some("panel.example.com:51820".into()),
                persistent_keepalive: Some(25),
            }],
        }
        .render();
        assert!(rendered.contains("ListenPort = 51820"));
        assert!(rendered.contains("AllowedIPs = 10.77.1.2/32"));
        assert!(rendered.contains("PersistentKeepalive = 25"));
    }

    fn valid_config() -> WireGuardConfig {
        WireGuardConfig {
            private_key: PRIVATE_KEY.into(),
            addresses: vec!["10.77.1.1/24".into()],
            listen_port: Some(51820),
            peers: vec![WireGuardPeer {
                public_key: PUBLIC_KEY.into(),
                allowed_ips: vec!["10.77.1.2/32".into()],
                endpoint: Some("panel.example.com:51820".into()),
                persistent_keepalive: Some(20),
            }],
        }
    }

    #[test]
    fn validates_typed_keys_routes_and_endpoint() {
        assert_eq!(valid_config().validate(), Ok(()));

        let mut injected = valid_config();
        injected.peers[0].endpoint = Some("panel.example.com:51820\nPostUp = evil".into());
        assert_eq!(
            injected.validate(),
            Err(WireGuardConfigError::InvalidEndpoint)
        );

        let mut bad_key = valid_config();
        bad_key.peers[0].public_key = "not-a-wireguard-key".into();
        assert_eq!(
            bad_key.validate(),
            Err(WireGuardConfigError::InvalidPublicKey)
        );
    }

    #[test]
    fn rejects_default_and_duplicate_routes() {
        let mut full_tunnel = valid_config();
        full_tunnel.peers[0].allowed_ips = vec!["0.0.0.0/0".into()];
        assert_eq!(
            full_tunnel.validate(),
            Err(WireGuardConfigError::DefaultRouteNotAllowed)
        );

        let mut duplicate = valid_config();
        let mut second = duplicate.peers[0].clone();
        second.public_key = SECOND_PUBLIC_KEY.into();
        duplicate.peers.push(second);
        assert_eq!(
            duplicate.validate(),
            Err(WireGuardConfigError::DuplicateAllowedIp(
                "10.77.1.2/32".into()
            ))
        );
    }

    #[test]
    fn rejects_ambiguous_interface_and_peer_state() {
        let mut no_address = valid_config();
        no_address.addresses.clear();
        assert_eq!(
            no_address.validate(),
            Err(WireGuardConfigError::MissingAddress)
        );

        let mut duplicate_address = valid_config();
        duplicate_address.addresses.push("10.77.1.1/24".into());
        assert!(matches!(
            duplicate_address.validate(),
            Err(WireGuardConfigError::DuplicateAddress(_))
        ));

        let mut duplicate_peer = valid_config();
        let mut second = duplicate_peer.peers[0].clone();
        second.allowed_ips = vec!["10.77.1.3/32".into()];
        duplicate_peer.peers.push(second);
        assert_eq!(
            duplicate_peer.validate(),
            Err(WireGuardConfigError::DuplicatePeerPublicKey)
        );

        let mut missing_routes = valid_config();
        missing_routes.peers[0].allowed_ips.clear();
        assert_eq!(
            missing_routes.validate(),
            Err(WireGuardConfigError::MissingAllowedIps)
        );

        let mut zero_port = valid_config();
        zero_port.listen_port = Some(0);
        assert_eq!(
            zero_port.validate(),
            Err(WireGuardConfigError::InvalidListenPort)
        );

        let mut zero_endpoint_port = valid_config();
        zero_endpoint_port.peers[0].endpoint = Some("panel.example.com:0".into());
        assert_eq!(
            zero_endpoint_port.validate(),
            Err(WireGuardConfigError::InvalidEndpoint)
        );
    }
}
use std::collections::HashSet;

use base64::Engine;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum WireGuardConfigError {
    #[error("invalid WireGuard private key")]
    InvalidPrivateKey,
    #[error("invalid WireGuard peer public key")]
    InvalidPublicKey,
    #[error("WireGuard interface requires at least one address")]
    MissingAddress,
    #[error("duplicate WireGuard interface address: {0}")]
    DuplicateAddress(String),
    #[error("WireGuard listen port must be greater than zero")]
    InvalidListenPort,
    #[error("invalid WireGuard address: {0}")]
    InvalidAddress(String),
    #[error("invalid WireGuard allowed IP: {0}")]
    InvalidAllowedIp(String),
    #[error("default routes are not allowed for management tunnels")]
    DefaultRouteNotAllowed,
    #[error("duplicate WireGuard allowed IP: {0}")]
    DuplicateAllowedIp(String),
    #[error("duplicate WireGuard peer public key")]
    DuplicatePeerPublicKey,
    #[error("WireGuard peer requires at least one allowed IP")]
    MissingAllowedIps,
    #[error("invalid WireGuard endpoint")]
    InvalidEndpoint,
}
