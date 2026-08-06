use serde::{Deserialize, Serialize};

macro_rules! string_enum {
    ($name:ident { $($variant:ident => $value:literal),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name { $($variant),+ }

        impl $name {
            pub const fn as_str(self) -> &'static str {
                match self { $(Self::$variant => $value),+ }
            }
        }

        impl TryFrom<&str> for $name {
            type Error = sqlx::Error;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok(Self::$variant),)+
                    _ => Err(sqlx::Error::Protocol(format!(
                        "invalid {} value: {value}", stringify!($name)
                    ))),
                }
            }
        }
    };
}

string_enum!(PrivateNetworkConnectionMode {
    DirectSsh => "DIRECT_SSH",
    ManagedWireguard => "MANAGED_WIREGUARD",
    ExternalPrivateNetwork => "EXTERNAL_PRIVATE_NETWORK",
});
string_enum!(PrivateNetworkProvider {
    Wireguard => "WIREGUARD",
    Tailscale => "TAILSCALE",
    Zerotier => "ZEROTIER",
    Netbird => "NETBIRD",
    Custom => "CUSTOM",
});
string_enum!(PrivateNetworkStatus {
    Disabled => "DISABLED",
    Configuring => "CONFIGURING",
    Active => "ACTIVE",
    Failed => "FAILED",
});
string_enum!(PrivateNetworkHealthStatus {
    Unknown => "UNKNOWN",
    Healthy => "HEALTHY",
    Degraded => "DEGRADED",
    Unreachable => "UNREACHABLE",
    ConfigDrift => "CONFIG_DRIFT",
});
string_enum!(PrivateNetworkRotationState {
    Idle => "IDLE",
    Rotating => "ROTATING",
    RollingBack => "ROLLING_BACK",
    Failed => "FAILED",
});
string_enum!(PrivateNetworkOperation {
    Setup => "SETUP",
    Repair => "REPAIR",
    Rotate => "ROTATE",
    Teardown => "TEARDOWN",
});

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct ServerPrivateNetwork {
    pub server_id: i64,
    pub connection_mode: String,
    pub provider: Option<String>,
    pub private_host: Option<String>,
    pub tunnel_address: Option<String>,
    pub public_key: Option<String>,
    pub endpoint: Option<String>,
    pub listen_port: Option<i64>,
    pub persistent_keepalive: Option<i64>,
    pub status: String,
    pub last_handshake_at: Option<i64>,
    pub config_version: i64,
    pub dns_name: Option<String>,
    pub routes: String,
    pub health_status: String,
    pub health_error: Option<String>,
    pub last_health_check_at: Option<i64>,
    pub consecutive_failures: i64,
    pub operation: Option<String>,
    pub operation_lease_until: Option<i64>,
    pub config_hash: Option<String>,
    pub rotation_state: String,
    pub created_at: i64,
    pub updated_at: i64,
}
