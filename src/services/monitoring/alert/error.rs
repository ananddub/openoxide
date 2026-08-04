use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlertParseError {
    Target(String),
    Metric(String),
    Operator(String),
    MissingId(String),
}

impl fmt::Display for AlertParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Target(value) => write!(f, "unknown alert target {value:?}"),
            Self::Metric(value) => write!(f, "unknown alert metric {value:?}"),
            Self::Operator(value) => write!(f, "unknown alert operator {value:?}"),
            Self::MissingId(name) => write!(f, "alert rule {name:?} has no id"),
        }
    }
}

impl std::error::Error for AlertParseError {}
