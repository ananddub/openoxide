use os::string_enum;
use serde::{Deserialize, Serialize};

macro_rules! strict_runtime_enum {
    ($name:ident { $($variant:ident => $value:literal),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        pub enum $name { $($variant),+ }
        impl $name {
            pub const fn as_str(self) -> &'static str { match self { $(Self::$variant => $value),+ } }
        }
        impl TryFrom<&str> for $name {
            type Error = String;
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value.trim().to_ascii_uppercase().as_str() {
                    $($value => Ok(Self::$variant),)+
                    other => Err(format!("invalid {}: {other}", stringify!($name))),
                }
            }
        }
    };
}

strict_runtime_enum!(MissedRunPolicy { Skip => "SKIP", RunOnce => "RUN_ONCE" });
strict_runtime_enum!(ConcurrencyPolicy { Skip => "SKIP", Queue => "QUEUE", Allow => "ALLOW" });
strict_runtime_enum!(ScheduleTriggerKind { Cron => "CRON", Manual => "MANUAL", Missed => "MISSED", Retry => "RETRY" });
strict_runtime_enum!(ScheduleExecutionStatus { Running => "RUNNING", Succeeded => "SUCCEEDED", Failed => "FAILED", Skipped => "SKIPPED" });

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ShellType {
        default = Bash;
        Bash => "BASH",
        Sh => "SH",
    }
}

impl ShellType {
    pub fn executable(self) -> &'static str {
        match self {
            Self::Bash => "bash",
            Self::Sh => "sh",
        }
    }
}

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ScheduleType {
        default = Application;

        Application => "APPLICATION",
        Compose => "COMPOSE",
        Server => "SERVER",
        DokpanelServer => "DOKPANEL-SERVER",
    }
}

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ScheduleAction {
        default = Exec;
        Exec => "EXEC",
        Deploy => "DEPLOY",
        Redeploy => "REDEPLOY",
        Rebuild => "REBUILD",
        Reload => "RELOAD",
        Start => "START",
        Stop => "STOP",
    }
}

/// A whole runtime-policy write for one schedule. The service takes it as a
/// unit, so the retry, concurrency and notification knobs travel together.
#[derive(Debug, Clone)]
pub struct RuntimePolicyUpdate<'a> {
    pub schedule_id: i64,
    pub retry_count: i64,
    pub retry_delay_seconds: i64,
    pub missed_run_policy: &'a str,
    pub concurrency_policy: &'a str,
    pub lease_seconds: i64,
    pub notify_on_success: bool,
    pub notify_on_failure: bool,
}
