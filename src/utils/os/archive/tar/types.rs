#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TarCompression {
    None,
    #[default]
    Gzip,
    Bzip2,
    Xz,
    Zstd,
}

impl TarCompression {
    pub(crate) fn create_args(self) -> Vec<String> {
        match self {
            Self::None => vec!["-cf".into()],
            Self::Gzip => vec!["-czf".into()],
            Self::Bzip2 => vec!["-cjf".into()],
            Self::Xz => vec!["-cJf".into()],
            Self::Zstd => vec!["--zstd".into(), "-cf".into()],
        }
    }

    pub(crate) fn list_args(self) -> Vec<String> {
        match self {
            Self::None => vec!["-tf".into()],
            Self::Gzip => vec!["-tzf".into()],
            Self::Bzip2 => vec!["-tjf".into()],
            Self::Xz => vec!["-tJf".into()],
            Self::Zstd => vec!["--zstd".into(), "-tf".into()],
        }
    }

    pub(crate) fn extract_args(self) -> Vec<String> {
        match self {
            Self::None => vec!["-xf".into()],
            Self::Gzip => vec!["-xzf".into()],
            Self::Bzip2 => vec!["-xjf".into()],
            Self::Xz => vec!["-xJf".into()],
            Self::Zstd => vec!["--zstd".into(), "-xf".into()],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TarOverwritePolicy {
    Replace,
    KeepExisting,
    UnlinkFirst,
}
