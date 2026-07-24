use std::fmt;

/// Network protocol for port bindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Protocol {
    #[default]
    Tcp,
    Udp,
    Sctp,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Tcp => "tcp",
            Self::Udp => "udp",
            Self::Sctp => "sctp",
        })
    }
}

/// Strongly-typed Docker port binding.
///
/// ```
/// Port::tcp(8080, 80)               // "8080:80/tcp"
/// Port::udp(53, 53)                 // "53:53/udp"
/// Port::tcp(80, 80).host_ip("0.0.0.0")  // "0.0.0.0:80:80/tcp"
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Port {
    pub host_ip: Option<String>,
    pub host: u16,
    pub container: u16,
    pub protocol: Protocol,
}

impl Port {
    pub fn tcp(host: u16, container: u16) -> Self {
        Self {
            host_ip: None,
            host,
            container,
            protocol: Protocol::Tcp,
        }
    }
    pub fn udp(host: u16, container: u16) -> Self {
        Self {
            host_ip: None,
            host,
            container,
            protocol: Protocol::Udp,
        }
    }
    pub fn sctp(host: u16, container: u16) -> Self {
        Self {
            host_ip: None,
            host,
            container,
            protocol: Protocol::Sctp,
        }
    }
    /// Bind to a specific host IP.
    pub fn host_ip(mut self, ip: impl Into<String>) -> Self {
        self.host_ip = Some(ip.into());
        self
    }
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.host_ip {
            Some(ip) => write!(f, "{ip}:{}:{}/{}", self.host, self.container, self.protocol),
            None => write!(f, "{}:{}/{}", self.host, self.container, self.protocol),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tcp_basic() {
        assert_eq!(Port::tcp(8080, 80).to_string(), "8080:80/tcp");
    }
    #[test]
    fn udp_basic() {
        assert_eq!(Port::udp(53, 53).to_string(), "53:53/udp");
    }
    #[test]
    fn with_host_ip() {
        assert_eq!(
            Port::tcp(80, 80).host_ip("0.0.0.0").to_string(),
            "0.0.0.0:80:80/tcp"
        );
    }
}
