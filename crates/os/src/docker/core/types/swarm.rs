use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResolveImage {
    Always,
    Changed,
    Never,
}

impl fmt::Display for ResolveImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResolveImage::Always => write!(f, "always"),
            ResolveImage::Changed => write!(f, "changed"),
            ResolveImage::Never => write!(f, "never"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeAvailability {
    Active,
    Pause,
    Drain,
}

impl fmt::Display for NodeAvailability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeAvailability::Active => write!(f, "active"),
            NodeAvailability::Pause => write!(f, "pause"),
            NodeAvailability::Drain => write!(f, "drain"),
        }
    }
}

impl TryFrom<&str> for NodeAvailability {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "active" => Ok(NodeAvailability::Active),
            "pause" => Ok(NodeAvailability::Pause),
            "drain" => Ok(NodeAvailability::Drain),
            _ => Err(format!("Invalid node availability: {}", s)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, poem_openapi::Enum)]
#[serde(rename_all = "lowercase")]
#[oai(rename_all = "lowercase")]
pub enum NodeRole {
    Worker,
    Manager,
}

impl Default for NodeRole {
    fn default() -> Self {
        Self::Worker
    }
}

impl fmt::Display for NodeRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeRole::Worker => write!(f, "worker"),
            NodeRole::Manager => write!(f, "manager"),
        }
    }
}

impl TryFrom<&str> for NodeRole {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "worker" => Ok(NodeRole::Worker),
            "manager" => Ok(NodeRole::Manager),
            _ => Err(format!("Invalid node role: {}", s)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, poem_openapi::Enum)]
#[serde(rename_all = "lowercase")]
#[oai(rename_all = "lowercase")]
pub enum SwarmRole {
    Worker,
    Manager,
}

impl fmt::Display for SwarmRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwarmRole::Worker => write!(f, "worker"),
            SwarmRole::Manager => write!(f, "manager"),
        }
    }
}

impl TryFrom<&str> for SwarmRole {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "worker" => Ok(SwarmRole::Worker),
            "manager" => Ok(SwarmRole::Manager),
            _ => Err(format!("Invalid swarm role: {}", s)),
        }
    }
}
