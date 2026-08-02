use std::net::{IpAddr, Ipv4Addr, ToSocketAddrs};

/// Rejects webhook targets that resolve to addresses an attacker could use to
/// reach services only the panel host can see. Loopback, link-local (including
/// the cloud metadata endpoint 169.254.169.254), multicast, unspecified and
/// 0.0.0.0/8 are all refused.
///
/// RFC1918 private ranges are deliberately allowed: self-hosted webhooks
/// (internal Mattermost, Gotify on the LAN, etc.) legitimately live there, and
/// this panel is itself a self-hosting tool.
pub fn is_blocked_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            v4.is_loopback()
                || v4.is_link_local()
                || v4.is_multicast()
                || v4.is_unspecified()
                || v4.is_broadcast()
                // 0.0.0.0/8 — routes to the local host on Linux, so it is an
                // SSRF vector even though only 0.0.0.0 is "unspecified".
                || v4.octets()[0] == 0
        }
        IpAddr::V6(v6) => {
            v6.is_loopback()
                || v6.is_multicast()
                || v6.is_unspecified()
                // fe80::/10 link-local
                || (v6.segments()[0] & 0xffc0) == 0xfe80
                // IPv4-mapped addresses smuggling a blocked v4 target
                || v6
                    .to_ipv4_mapped()
                    .is_some_and(|m| is_blocked_ip(&IpAddr::V4(m)))
        }
    }
}

/// Validates a user-supplied webhook URL before we send anything to it.
///
/// Checks the scheme is http/https and that every address the host resolves to
/// is allowed. Note this is a pre-flight check, not a guarantee: DNS could
/// return a different answer when reqwest dials (a DNS-rebinding race). Closing
/// that fully needs a custom connector; this blocks the straightforward attacks.
pub fn validate_webhook_url(raw_url: &str) -> Result<(), String> {
    let url = reqwest::Url::parse(raw_url).map_err(|e| format!("invalid webhook URL: {e}"))?;

    let scheme = url.scheme().to_ascii_lowercase();
    if scheme != "http" && scheme != "https" {
        return Err(format!(
            "invalid webhook URL scheme {scheme:?}: only http and https are allowed"
        ));
    }

    let host = url
        .host_str()
        .ok_or_else(|| "webhook URL has no host".to_string())?;

    // If the host is already a literal IP, check it directly — no DNS needed.
    if let Ok(ip) = host.parse::<IpAddr>() {
        if is_blocked_ip(&ip) {
            return Err("webhook target resolves to a blocked address range".to_string());
        }
        return Ok(());
    }

    let port = url.port_or_known_default().unwrap_or(443);
    let addrs = (host, port)
        .to_socket_addrs()
        .map_err(|e| format!("could not resolve webhook host {host}: {e}"))?;

    let mut saw_any = false;
    for addr in addrs {
        saw_any = true;
        if is_blocked_ip(&addr.ip()) {
            return Err("webhook target resolves to a blocked address range".to_string());
        }
    }

    if !saw_any {
        return Err(format!("webhook host {host} resolved to no addresses"));
    }

    Ok(())
}

/// Convenience for tests / callers that only have an IPv4 literal.
pub fn is_blocked_v4(v4: Ipv4Addr) -> bool {
    is_blocked_ip(&IpAddr::V4(v4))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_loopback_and_metadata() {
        assert!(is_blocked_v4("127.0.0.1".parse().unwrap()));
        assert!(is_blocked_v4("169.254.169.254".parse().unwrap()));
        assert!(is_blocked_v4("0.0.0.0".parse().unwrap()));
        assert!(is_blocked_v4("0.1.2.3".parse().unwrap()));
        assert!(is_blocked_v4("255.255.255.255".parse().unwrap()));
    }

    #[test]
    fn allows_private_and_public() {
        // self-hosted webhooks on a LAN must keep working
        assert!(!is_blocked_v4("192.168.1.10".parse().unwrap()));
        assert!(!is_blocked_v4("10.0.0.5".parse().unwrap()));
        assert!(!is_blocked_v4("172.16.0.1".parse().unwrap()));
        assert!(!is_blocked_v4("1.1.1.1".parse().unwrap()));
    }

    #[test]
    fn blocks_ipv6_loopback_and_mapped() {
        assert!(is_blocked_ip(&"::1".parse().unwrap()));
        assert!(is_blocked_ip(&"::ffff:127.0.0.1".parse().unwrap()));
        assert!(is_blocked_ip(&"fe80::1".parse().unwrap()));
    }

    #[test]
    fn rejects_non_http_schemes() {
        assert!(validate_webhook_url("file:///etc/passwd").is_err());
        assert!(validate_webhook_url("gopher://example.com").is_err());
    }

    #[test]
    fn rejects_loopback_literal_url() {
        assert!(validate_webhook_url("http://127.0.0.1:8080/hook").is_err());
        assert!(validate_webhook_url("http://169.254.169.254/latest/meta-data").is_err());
    }
}
