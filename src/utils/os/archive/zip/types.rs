#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CompressionLevel {
    Stored,
    Fast,
    #[default]
    Default,
    Best,
}

impl CompressionLevel {
    pub(crate) const fn flag(self) -> &'static str {
        match self {
            Self::Stored => "-0",
            Self::Fast => "-1",
            Self::Default => "-6",
            Self::Best => "-9",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ZipPathMode {
    #[default]
    Preserve,
    FileNamesOnly,
}
