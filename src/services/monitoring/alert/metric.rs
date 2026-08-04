use super::error::AlertParseError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, poem_openapi::Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MetricKind {
    Cpu,
    Memory,
    Disk,
}

impl MetricKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Cpu => "CPU",
            Self::Memory => "MEMORY",
            Self::Disk => "DISK",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Cpu => "CPU",
            Self::Memory => "Memory",
            Self::Disk => "Disk",
        }
    }
}

impl FromStr for MetricKind {
    type Err = AlertParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw.trim().to_ascii_uppercase().as_str() {
            "CPU" | "CPU_PERCENT" => Ok(Self::Cpu),
            "MEMORY" | "MEM" | "MEMORY_PERCENT" | "RAM" => Ok(Self::Memory),
            "DISK" | "DISK_PERCENT" | "STORAGE" => Ok(Self::Disk),
            other => Err(AlertParseError::Metric(other.to_owned())),
        }
    }
}

impl fmt::Display for MetricKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
