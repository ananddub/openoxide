pub(super) fn interface_name(server_id: i64) -> String {
    let id = server_id.unsigned_abs();
    let suffix = if id < 1_000_000 {
        format!("{id}")
    } else {
        let hex = format!("{id:x}");
        if hex.len() > 6 {
            hex[hex.len() - 6..].to_string()
        } else {
            hex
        }
    };
    format!("openoxide{suffix}")
}

pub(super) fn tunnel_addresses(cidr: &str) -> sqlx::Result<(String, String, String)> {
    let network: ipnet::Ipv4Net = cidr
        .parse()
        .map_err(|error| sqlx::Error::Protocol(format!("invalid IPv4 tunnel network: {error}")))?;
    if network.prefix_len() > 30 {
        return Err(sqlx::Error::Protocol(
            "WireGuard tunnel network must contain at least two host addresses".into(),
        ));
    }
    let base = u32::from(network.network());
    let panel = std::net::Ipv4Addr::from(base + 1);
    let remote = std::net::Ipv4Addr::from(base + 2);
    Ok((
        format!("{panel}/{}", network.prefix_len()),
        format!("{remote}/{}", network.prefix_len()),
        remote.to_string(),
    ))
}

pub(super) fn panel_host(cidr: &str) -> sqlx::Result<String> {
    tunnel_addresses(cidr)
        .map(|(panel, _, _)| format!("{}/32", panel.split('/').next().unwrap_or_default()))
}

#[cfg(test)]
mod tests {
    use super::{interface_name, tunnel_addresses};

    #[test]
    fn allocates_tunnel_hosts() {
        assert_eq!(
            tunnel_addresses("10.77.8.0/24").unwrap(),
            (
                "10.77.8.1/24".into(),
                "10.77.8.2/24".into(),
                "10.77.8.2".into()
            )
        );
        assert!(interface_name(i64::MAX).len() <= 15);
        assert!(tunnel_addresses("10.77.8.0/31").is_err());
    }
}
