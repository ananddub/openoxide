use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageFilter {
    /// Match against image name or `name:tag`.
    Reference(String),
    Label(String, String),
    LabelKey(String),
    Before(String),
    Since(String),
    Until(String),
    Dangling(bool),
}

impl ImageFilter {
    pub fn reference(v: impl Into<String>) -> Self {
        Self::Reference(v.into())
    }
    pub fn label(k: impl Into<String>, v: impl Into<String>) -> Self {
        Self::Label(k.into(), v.into())
    }
    pub fn label_key(k: impl Into<String>) -> Self {
        Self::LabelKey(k.into())
    }
    pub fn before(v: impl Into<String>) -> Self {
        Self::Before(v.into())
    }
    pub fn since(v: impl Into<String>) -> Self {
        Self::Since(v.into())
    }
    pub fn until(v: impl Into<String>) -> Self {
        Self::Until(v.into())
    }
}

impl fmt::Display for ImageFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reference(v) => write!(f, "reference={v}"),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::LabelKey(k) => write!(f, "label={k}"),
            Self::Before(v) => write!(f, "before={v}"),
            Self::Since(v) => write!(f, "since={v}"),
            Self::Until(v) => write!(f, "until={v}"),
            Self::Dangling(b) => write!(f, "dangling={b}"),
        }
    }
}
