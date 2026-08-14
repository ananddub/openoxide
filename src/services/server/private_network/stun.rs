use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use tokio::net::UdpSocket;
use tracing::{debug, warn};

const PUBLIC_STUN_SERVERS: &[&str] = &[
    "stun.l.google.com:19302",
    "stun1.l.google.com:19302",
    "stun2.l.google.com:19302",
    "stunserver.stunprotocol.org:3478",
];

const STUN_MAGIC_COOKIE: u32 = 0x2112A442;
const STUN_BINDING_REQUEST: u16 = 0x0001;
const STUN_BINDING_RESPONSE: u16 = 0x0101;
const ATTR_XOR_MAPPED_ADDRESS: u16 = 0x0020;
const ATTR_MAPPED_ADDRESS: u16 = 0x0001;

/// Query public STUN servers over UDP to discover external mapped SocketAddr (Public IP:Port)
pub async fn discover_public_endpoint(preferred_port: u16) -> Option<String> {
    let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), preferred_port);
    let socket = match UdpSocket::bind(bind_addr).await {
        Ok(s) => s,
        Err(_) => match UdpSocket::bind("0.0.0.0:0").await {
            Ok(s) => s,
            Err(err) => {
                warn!("failed to bind UDP socket for STUN discovery: {err}");
                return None;
            }
        },
    };

    for stun_host in PUBLIC_STUN_SERVERS {
        if let Ok(mut addrs) = tokio::net::lookup_host(stun_host).await {
            if let Some(target) = addrs.next() {
                if let Ok(mapped) = query_stun_server(&socket, target).await {
                    debug!("discovered STUN endpoint {mapped} via {stun_host}");
                    return Some(mapped.to_string());
                }
            }
        }
    }

    None
}

/// Send UDP hole punching probe packets to a target endpoint to open firewall / NAT mappings
pub async fn punch_nat_hole(target_endpoint: &str, listen_port: u16) -> Result<(), String> {
    let target_addr: SocketAddr = match target_endpoint.parse() {
        Ok(addr) => addr,
        Err(_) => {
            let mut addrs = tokio::net::lookup_host(target_endpoint)
                .await
                .map_err(|e| format!("DNS lookup failed for {target_endpoint}: {e}"))?;
            addrs
                .next()
                .ok_or_else(|| format!("no addresses for {target_endpoint}"))?
        }
    };

    let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), listen_port);
    let socket = match UdpSocket::bind(bind_addr).await {
        Ok(s) => s,
        Err(_) => UdpSocket::bind("0.0.0.0:0")
            .await
            .map_err(|e| format!("failed to bind UDP socket for hole punching: {e}"))?,
    };

    let dummy_packet = [0u8; 32];
    for _ in 0..3 {
        let _ = socket.send_to(&dummy_packet, target_addr).await;
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    debug!("sent NAT hole-punching probes to {target_addr}");
    Ok(())
}

/// Perform STUN query using RFC 5389 protocol
async fn query_stun_server(socket: &UdpSocket, target: SocketAddr) -> Result<SocketAddr, String> {
    let mut req = [0u8; 20];
    req[0..2].copy_from_slice(&STUN_BINDING_REQUEST.to_be_bytes());
    req[2..4].copy_from_slice(&0u16.to_be_bytes());
    req[4..8].copy_from_slice(&STUN_MAGIC_COOKIE.to_be_bytes());

    let trans_id = uuid::Uuid::new_v4();
    req[8..20].copy_from_slice(&trans_id.as_bytes()[..12]);

    socket
        .send_to(&req, target)
        .await
        .map_err(|e| format!("failed to send STUN request: {e}"))?;

    let mut buf = [0u8; 512];
    let timeout = tokio::time::timeout(Duration::from_millis(1500), socket.recv_from(&mut buf));

    let (len, src) = match timeout.await {
        Ok(Ok(res)) => res,
        _ => return Err("STUN response timeout".into()),
    };

    if src != target || len < 20 {
        return Err("invalid STUN response source or length".into());
    }

    let msg_type = u16::from_be_bytes([buf[0], buf[1]]);
    if msg_type != STUN_BINDING_RESPONSE {
        return Err("not a STUN binding response".into());
    }

    let cookie = u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]);
    let mut offset = 20;

    while offset + 4 <= len {
        let attr_type = u16::from_be_bytes([buf[offset], buf[offset + 1]]);
        let attr_len = u16::from_be_bytes([buf[offset + 2], buf[offset + 3]]) as usize;
        offset += 4;

        if offset + attr_len > len {
            break;
        }

        let attr_data = &buf[offset..offset + attr_len];
        offset += (attr_len + 3) & !3; // 4-byte boundary padding

        if attr_type == ATTR_XOR_MAPPED_ADDRESS && attr_data.len() >= 8 {
            let family = attr_data[1];
            let xor_port = u16::from_be_bytes([attr_data[2], attr_data[3]]);
            let port = xor_port ^ (STUN_MAGIC_COOKIE >> 16) as u16;

            if family == 0x01 && attr_data.len() >= 8 {
                let xor_ip = u32::from_be_bytes([attr_data[4], attr_data[5], attr_data[6], attr_data[7]]);
                let ip_u32 = xor_ip ^ cookie;
                let ip = Ipv4Addr::from(ip_u32);
                return Ok(SocketAddr::new(IpAddr::V4(ip), port));
            }
        } else if attr_type == ATTR_MAPPED_ADDRESS && attr_data.len() >= 8 {
            let family = attr_data[1];
            let port = u16::from_be_bytes([attr_data[2], attr_data[3]]);
            if family == 0x01 && attr_data.len() >= 8 {
                let ip = Ipv4Addr::new(attr_data[4], attr_data[5], attr_data[6], attr_data[7]);
                return Ok(SocketAddr::new(IpAddr::V4(ip), port));
            }
        }
    }

    Err("no mapped address attribute in STUN response".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stun_discovery() {
        let res = discover_public_endpoint(51820).await;
        println!("STUN discovery result: {:?}", res);
    }
}
