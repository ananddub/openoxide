use super::error::AlertParseError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, poem_openapi::Enum, ts_rs::TS,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TargetKind {
    Server,
    Application,
    Database,
    Compose,
}

impl TargetKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Server => "SERVER",
            Self::Application => "APPLICATION",
            Self::Database => "DATABASE",
            Self::Compose => "COMPOSE",
        }
    }

    pub fn is_host_level(&self) -> bool {
        matches!(self, Self::Server)
    }
}

impl FromStr for TargetKind {
    type Err = AlertParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw.trim().to_ascii_uppercase().as_str() {
            "SERVER" => Ok(Self::Server),
            "APPLICATION" | "APP" => Ok(Self::Application),
            "DATABASE" | "DB" => Ok(Self::Database),
            "COMPOSE" => Ok(Self::Compose),
            other => Err(AlertParseError::Target(other.to_owned())),
        }
    }
}

impl fmt::Display for TargetKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
