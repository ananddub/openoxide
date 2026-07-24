use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigFilter {
    Id(String),
    Label(String, String),
    LabelKey(String),
    Name(String),
}

impl ConfigFilter {
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
}

impl fmt::Display for ConfigFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(v) => write!(f, "id={v}"),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::LabelKey(k) => write!(f, "label={k}"),
            Self::Name(v) => write!(f, "name={v}"),
        }
    }
}
