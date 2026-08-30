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
