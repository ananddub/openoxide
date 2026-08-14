use crate::api::dto::server::{
    PrivateNetworkProviderDto, ServerConnectionModeDto, UpdatePrivateNetworkDto,
};

use super::addressing::tunnel_addresses;

pub(super) fn validate(input: &UpdatePrivateNetworkDto) -> sqlx::Result<()> {
    if input.connection_mode == ServerConnectionModeDto::DirectSsh {
        return Ok(());
    }
    let host = input
        .private_host
        .as_deref()
        .filter(|value| !value.trim().is_empty() && !value.chars().any(char::is_whitespace))
        .ok_or_else(|| {
            sqlx::Error::Protocol("private_host is required and must not contain whitespace".into())
        })?;
    if host.len() > 253 {
        return Err(sqlx::Error::Protocol("private_host is too long".into()));
    }
    let provider = input
        .provider
        .ok_or_else(|| sqlx::Error::Protocol("private network provider is required".into()))?;
    validate_dns_name(input.dns_name.as_deref())?;
    validate_routes(&input.routes, input.tunnel_address.as_deref())?;
    match input.connection_mode {
        ServerConnectionModeDto::ManagedWireguard => {
            if provider != PrivateNetworkProviderDto::Wireguard {
                return Err(sqlx::Error::Protocol(
                    "managed private networks require the WIREGUARD provider".into(),
                ));
            }
            let cidr = input.tunnel_address.as_deref().ok_or_else(|| {
                sqlx::Error::Protocol("managed WireGuard requires tunnel_address".into())
            })?;
            let endpoint = input.endpoint.as_deref().ok_or_else(|| {
                sqlx::Error::Protocol("managed WireGuard requires endpoint".into())
            })?;
            let (_, _, expected_host) = tunnel_addresses(cidr)?;
            if input.private_host.as_deref() != Some(expected_host.as_str()) {
                return Err(sqlx::Error::Protocol(format!(
                    "private_host must be {expected_host} for tunnel {cidr}"
                )));
            }
            // The public endpoint can be NAT/port-forwarded to a different
            // local WireGuard listen port. Validate both independently.
            endpoint_port(endpoint)?;
        }
        ServerConnectionModeDto::ExternalPrivateNetwork => {
            if provider == PrivateNetworkProviderDto::Wireguard {
                return Err(sqlx::Error::Protocol(
                    "use MANAGED_WIREGUARD for the WireGuard provider".into(),
                ));
            }
        }
        ServerConnectionModeDto::DirectSsh => {}
    }
    Ok(())
}

fn validate_dns_name(value: Option<&str>) -> sqlx::Result<()> {
    let Some(value) = value else { return Ok(()) };
    if value.len() > 253 || !value.to_ascii_lowercase().ends_with(".openoxide.internal") {
        return Err(sqlx::Error::Protocol(
            "private DNS name must end with .openoxide.internal".into(),
        ));
    }
    if value.split('.').any(|label| {
        label.is_empty()
            || label.len() > 63
            || label.starts_with('-')
            || label.ends_with('-')
            || !label
                .chars()
                .all(|character| character.is_ascii_alphanumeric() || character == '-')
    }) {
        return Err(sqlx::Error::Protocol("invalid private DNS name".into()));
    }
    Ok(())
}

fn validate_routes(routes: &[String], tunnel: Option<&str>) -> sqlx::Result<()> {
    let tunnel = tunnel
        .map(|value| value.parse::<ipnet::IpNet>())
        .transpose()
        .map_err(|error| sqlx::Error::Protocol(format!("invalid tunnel route: {error}")))?;
    let mut parsed = std::collections::BTreeSet::new();
    for route in routes {
        let route = route
            .parse::<ipnet::IpNet>()
            .map_err(|error| sqlx::Error::Protocol(format!("invalid private route: {error}")))?;
        if route.prefix_len() == 0 {
            return Err(sqlx::Error::Protocol(
                "default routes are not allowed for a management tunnel".into(),
            ));
        }
        if Some(route) == tunnel || !parsed.insert(route) {
            return Err(sqlx::Error::Protocol("duplicate private route".into()));
        }
    }
    Ok(())
}

fn endpoint_port(endpoint: &str) -> sqlx::Result<u16> {
    if let Ok(address) = endpoint.parse::<std::net::SocketAddr>() {
        return Ok(address.port());
    }
    let (host, port) = endpoint
        .rsplit_once(':')
        .ok_or_else(|| sqlx::Error::Protocol("endpoint must include a port".into()))?;
    if host.is_empty()
        || host.chars().any(char::is_whitespace)
        || !host
            .chars()
            .all(|value| value.is_ascii_alphanumeric() || matches!(value, '.' | '-'))
    {
        return Err(sqlx::Error::Protocol("invalid WireGuard endpoint".into()));
    }
    port.parse()
        .map_err(|_| sqlx::Error::Protocol("invalid WireGuard endpoint port".into()))
}

#[cfg(test)]
mod tests {
    use crate::api::dto::server::{
        PrivateNetworkProviderDto, ServerConnectionModeDto, UpdatePrivateNetworkDto,
    };

    use super::{endpoint_port, validate};

    fn managed() -> UpdatePrivateNetworkDto {
        UpdatePrivateNetworkDto {
            connection_mode: ServerConnectionModeDto::ManagedWireguard,
            provider: Some(PrivateNetworkProviderDto::Wireguard),
            private_host: Some("10.77.2.2".into()),
            tunnel_address: Some("10.77.2.0/24".into()),
            public_key: None,
            endpoint: Some("panel.example.com:51820".into()),
            listen_port: Some(51820),
            persistent_keepalive: Some(25),
            dns_name: None,
            routes: Vec::new(),
        }
    }

    #[test]
    fn validates_endpoint() {
        assert_eq!(endpoint_port("panel.example.com:51820").unwrap(), 51820);
        assert_eq!(endpoint_port("[2001:db8::1]:51820").unwrap(), 51820);
        assert!(endpoint_port("host:1\nPostUp=evil").is_err());
    }

    #[test]
    fn accepts_complete_managed_wireguard() {
        assert!(validate(&managed()).is_ok());
    }

    #[test]
    fn rejects_missing_managed_endpoint_and_accepts_nat_port_mapping() {
        let mut input = managed();
        input.endpoint = None;
        assert!(validate(&input).is_err());

        input.endpoint = Some("panel.example.com:52180".into());
        input.listen_port = Some(51820);
        assert!(validate(&input).is_ok());
    }

    #[test]
    fn rejects_wrong_derived_private_host() {
        let mut input = managed();
        input.private_host = Some("10.77.2.3".into());
        assert!(validate(&input).is_err());
    }

    #[test]
    fn external_provider_requires_private_host() {
        let mut input = managed();
        input.connection_mode = ServerConnectionModeDto::ExternalPrivateNetwork;
        input.provider = Some(PrivateNetworkProviderDto::Tailscale);
        input.private_host = None;
        input.tunnel_address = None;
        input.endpoint = None;
        assert!(validate(&input).is_err());
    }
}
