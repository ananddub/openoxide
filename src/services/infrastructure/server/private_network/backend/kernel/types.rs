pub struct WireGuardInstallPlan<'a> {
    pub interface: &'a str,
    pub panel_address: String,
    pub remote_address: String,
    pub panel_host: String,
    pub remote_host: String,
    pub endpoint: &'a str,
    pub port: u16,
    pub keepalive: u16,
    pub routes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct KernelWireGuardHealth {
    pub interface_exists: bool,
    pub peer_public_keys: Vec<String>,
    pub latest_handshake: Option<i64>,
    pub allowed_ips: Vec<String>,
}
