use super::types::CommandExecutor;
use tokio_util::sync::CancellationToken;

/// Interface name prefixes that indicate a VPN/overlay link: Tailscale,
/// WireGuard, ZeroTier, Netbird (`wt*`), Nebula, IPsec, PPP, and generic
/// tun/tap devices — rather than a regular LAN/public NIC.
const VPN_IFACE_PREFIXES: &[&str] = &[
    "tailscale",
    "wg",
    "zt",
    "wt",
    "nebula",
    "ipsec",
    "ppp",
    "tun",
    "tap",
];

fn is_vpn_iface(name: &str) -> bool {
    VPN_IFACE_PREFIXES
        .iter()
        .any(|prefix| name.starts_with(prefix))
}

/// Shared CGNAT range (100.64.0.0/10) that Tailscale and Netbird both default
/// to for their mesh addresses, regardless of what the interface is named.
fn is_cgnat_addr(ip: &str) -> bool {
    let mut octets = ip.split('.');
    let a = octets.next().and_then(|s| s.parse::<u16>().ok());
    let b = octets.next().and_then(|s| s.parse::<u16>().ok());
    matches!((a, b), (Some(100), Some(b)) if (64..=127).contains(&b))
}

struct IfaceAddr<'a> {
    iface: &'a str,
    ip: &'a str,
}

/// Parses `ip -4 -o addr show` output into (interface, address) pairs.
fn parse_addrs(ip_addr_show_output: &str) -> impl Iterator<Item = IfaceAddr<'_>> {
    ip_addr_show_output.lines().filter_map(|line| {
        let mut fields = line.split_whitespace();
        fields.next()?; // index, e.g. "3:"
        let iface = fields.next()?;
        fields.next()?; // "inet"
        let cidr = fields.next()?;
        let ip = cidr.split('/').next()?;
        (!ip.is_empty()).then_some(IfaceAddr { iface, ip })
    })
}

/// Best-effort VPN/overlay address lookup: first by known interface naming
/// conventions, then by the shared CGNAT range used when the interface was
/// renamed or belongs to a tool we don't recognize by name.
fn first_vpn_addr(ip_addr_show_output: &str) -> Option<String> {
    let addrs: Vec<_> = parse_addrs(ip_addr_show_output).collect();
    addrs
        .iter()
        .find(|a| is_vpn_iface(a.iface))
        .or_else(|| addrs.iter().find(|a| is_cgnat_addr(a.ip)))
        .map(|a| a.ip.to_string())
}

fn first_non_loopback(hostname_dash_i_output: &str) -> Option<String> {
    hostname_dash_i_output
        .split_whitespace()
        .find(|ip| !ip.starts_with("127."))
        .map(str::to_owned)
}

/// Best-effort advertise-address detection: prefers a VPN/overlay interface
/// (by name, or by CGNAT range as a fallback signal) over the first
/// `hostname -I` entry, since the latter is usually the LAN/public NIC and
/// unreachable from peers that only share a VPN mesh. Callers can always
/// override this via an explicit `advertise_addr` setting.
pub async fn detect_advertise_addr(executor: &CommandExecutor) -> String {
    if let Ok(out) = executor.run("ip", ["-4", "-o", "addr", "show"]).await {
        if let Some(ip) = first_vpn_addr(&out.stdout) {
            return ip;
        }
    }
    executor
        .run("hostname", ["-I"])
        .await
        .ok()
        .and_then(|out| first_non_loopback(&out.stdout))
        .unwrap_or_else(|| "127.0.0.1".into())
}

pub async fn detect_advertise_addr_cancelled(
    executor: &CommandExecutor,
    cancel: &CancellationToken,
) -> String {
    if let Ok(out) = executor
        .run_cancelled("ip", ["-4", "-o", "addr", "show"], cancel)
        .await
    {
        if let Some(ip) = first_vpn_addr(&out.stdout) {
            return ip;
        }
    }
    executor
        .run_cancelled("hostname", ["-I"], cancel)
        .await
        .ok()
        .and_then(|out| first_non_loopback(&out.stdout))
        .unwrap_or_else(|| "127.0.0.1".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefers_tailscale_interface() {
        let output = "1: lo    inet 127.0.0.1/8 scope host lo\n\
             2: eth0    inet 192.168.1.20/24 scope global eth0\n\
             3: tailscale0    inet 100.102.184.2/32 scope global tailscale0";
        assert_eq!(first_vpn_addr(output), Some("100.102.184.2".to_string()));
    }

    #[test]
    fn matches_netbird_interface_by_name() {
        let output = "1: lo    inet 127.0.0.1/8 scope host lo\n\
             2: eth0    inet 192.168.1.20/24 scope global eth0\n\
             3: wt0    inet 100.75.1.5/32 scope global wt0";
        assert_eq!(first_vpn_addr(output), Some("100.75.1.5".to_string()));
    }

    #[test]
    fn falls_back_to_cgnat_range_when_iface_name_unknown() {
        let output = "1: lo    inet 127.0.0.1/8 scope host lo\n\
             2: eth0    inet 192.168.1.20/24 scope global eth0\n\
             3: custom-mesh0    inet 100.90.4.1/32 scope global custom-mesh0";
        assert_eq!(first_vpn_addr(output), Some("100.90.4.1".to_string()));
    }

    #[test]
    fn no_vpn_interface_returns_none() {
        let output = "1: lo    inet 127.0.0.1/8 scope host lo\n\
             2: eth0    inet 192.168.1.20/24 scope global eth0";
        assert_eq!(first_vpn_addr(output), None);
    }

    #[test]
    fn cgnat_range_boundaries() {
        assert!(is_cgnat_addr("100.64.0.1"));
        assert!(is_cgnat_addr("100.127.255.254"));
        assert!(!is_cgnat_addr("100.63.255.255"));
        assert!(!is_cgnat_addr("100.128.0.0"));
        assert!(!is_cgnat_addr("192.168.1.20"));
    }

    #[test]
    fn hostname_fallback_skips_loopback() {
        let output = "127.0.0.1 192.168.1.20 10.0.0.5";
        assert_eq!(first_non_loopback(output), Some("192.168.1.20".to_string()));
    }
}
