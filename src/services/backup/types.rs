use serde::{Deserialize, Serialize};

macro_rules! backup_enum {
    ($name:ident { $($variant:ident => $value:literal),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, poem_openapi::Enum)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        #[oai(rename_all = "SCREAMING_SNAKE_CASE")]
        pub enum $name { $($variant),+ }
        impl $name { pub const fn as_str(self) -> &'static str { match self { $(Self::$variant => $value),+ } } }
        impl TryFrom<&str> for $name {
            type Error = String;
            fn try_from(value: &str) -> Result<Self, Self::Error> { match value.trim().to_ascii_uppercase().as_str() { $($value => Ok(Self::$variant),)+ other => Err(format!("invalid {}: {other}", stringify!($name))) } }
        }
    };
}

backup_enum!(BackupKind { Database => "DATABASE", Volume => "VOLUME", Panel => "PANEL", ComposeConfig => "COMPOSE_CONFIG" });
backup_enum!(BackupOperation { Backup => "BACKUP", Restore => "RESTORE" });
backup_enum!(BackupExecutionStatus { Running => "RUNNING", Succeeded => "SUCCEEDED", Failed => "FAILED" });
