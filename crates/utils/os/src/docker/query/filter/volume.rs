use crate::docker::VolumeDriver;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VolumeFilter {
    Driver(VolumeDriver),
    Label(String, String),
    LabelKey(String),
    Name(String),
    Dangling(bool),
}

impl VolumeFilter {
    pub fn driver(v: impl Into<VolumeDriver>) -> Self {
        Self::Driver(v.into())
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
}

impl fmt::Display for VolumeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Driver(v) => write!(f, "driver={}", v.as_str()),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::LabelKey(k) => write!(f, "label={k}"),
            Self::Name(v) => write!(f, "name={v}"),
            Self::Dangling(b) => write!(f, "dangling={b}"),
        }
    }
}
