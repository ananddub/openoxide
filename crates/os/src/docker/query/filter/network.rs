use crate::docker::{NetworkDriver, NetworkScope, NetworkType};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkFilter {
    Driver(NetworkDriver),
    Id(String),
    Label(String, String),
    LabelKey(String),
    Name(String),
    Scope(NetworkScope),
    Type(NetworkType),
    Dangling(bool),
}

impl NetworkFilter {
    pub fn driver(v: impl Into<NetworkDriver>) -> Self {
        Self::Driver(v.into())
    }
    pub fn id(v: impl Into<String>) -> Self {
        Self::Id(v.into())
    }
    pub fn label(k: impl Into<String>, v: impl Into<String>) -> Self {
        Self::Label(k.into(), v.into())
    }
    pub fn label_key(k: impl Into<String>) -> Self {
        Self::LabelKey(k.into())
    }
    pub fn name(v: impl Into<String>) -> Self {
        Self::Name(v.into())
    }
    pub fn dangling(v: bool) -> Self {
        Self::Dangling(v)
    }
}

impl fmt::Display for NetworkFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Driver(v) => write!(f, "driver={}", v.as_str()),
            Self::Id(v) => write!(f, "id={v}"),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::LabelKey(k) => write!(f, "label={k}"),
            Self::Name(v) => write!(f, "name={v}"),
            Self::Scope(s) => write!(f, "scope={s}"),
            Self::Type(t) => write!(f, "type={t}"),
            Self::Dangling(b) => write!(f, "dangling={b}"),
        }
    }
}
