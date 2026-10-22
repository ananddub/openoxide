use auto_di::singleton;

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub access_secret: String,
    pub refresh_secret: String,
    pub access_expiry_mins: i64,
    pub refresh_expiry_days: i64,
    pub debug_skip_time_check: bool,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            access_secret: "access-secret-change-me".into(),
            refresh_secret: "refresh-secret-change-me".into(),
            access_expiry_mins: 15,
            refresh_expiry_days: 7,
            debug_skip_time_check: false,
        }
    }
}

#[singleton]
impl JwtConfig {
    pub fn new() -> Self {
        let debug_skip_time_check = env_bool("JWT_DEBUG_SKIP_TIME_CHECK")
            .or_else(|| env_bool("DEBUG"))
            .unwrap_or(false);

        if debug_skip_time_check {
            tracing::warn!("JWT debug mode enabled: token expiry time validation is disabled");
        }

        Self {
            access_secret: std::env::var("JWT_ACCESS_SECRET")
                .unwrap_or_else(|_| Self::default().access_secret),
            refresh_secret: std::env::var("JWT_REFRESH_SECRET")
                .unwrap_or_else(|_| Self::default().refresh_secret),
            access_expiry_mins: std::env::var("JWT_ACCESS_EXPIRY_MINS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(15),
            refresh_expiry_days: std::env::var("JWT_REFRESH_EXPIRY_DAYS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(7),
            debug_skip_time_check,
        }
    }
}

fn env_bool(key: &str) -> Option<bool> {
    let value = std::env::var(key).ok()?;
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "y" | "on" => Some(true),
        "0" | "false" | "no" | "n" | "off" => Some(false),
        _ => None,
    }
}
