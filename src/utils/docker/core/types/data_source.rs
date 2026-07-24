use std::path::PathBuf;

/// Defines a data source for creating secrets or configs
#[derive(Clone, Debug)]
pub enum DataSource {
    /// Read data from a file path
    File(PathBuf),
    /// Read data from a string
    String(String),
    /// Read data from raw bytes
    Bytes(Vec<u8>),
}

impl DataSource {
    pub fn from_file(path: impl Into<PathBuf>) -> Self {
        Self::File(path.into())
    }

    pub fn from_string(data: impl Into<String>) -> Self {
        Self::String(data.into())
    }

    pub fn from_bytes(data: impl Into<Vec<u8>>) -> Self {
        Self::Bytes(data.into())
    }
}
