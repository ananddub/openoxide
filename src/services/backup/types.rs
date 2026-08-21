use os::string_enum;
use serde::{Deserialize, Serialize};

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, poem_openapi::Enum, ts_rs::TS)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[oai(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum BackupKind {
        default = Database;
        Database => "DATABASE",
        Volume => "VOLUME",
        Panel => "PANEL",
        ComposeConfig => "COMPOSE_CONFIG",
    }
}

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, poem_openapi::Enum, ts_rs::TS)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[oai(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum BackupOperation {
        default = Backup;
        Backup => "BACKUP",
        Restore => "RESTORE",
    }
}

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, poem_openapi::Enum, ts_rs::TS)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[oai(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum BackupExecutionStatus {
        default = Running;
        Running => "RUNNING",
        Succeeded => "SUCCEEDED",
        Failed => "FAILED",
    }
}
