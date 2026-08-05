use crate::utils::exec::ExecError;

#[derive(Debug, thiserror::Error)]
pub enum ZipError {
    #[error("source path not set")]
    MissingSource,
    #[error("destination path not set")]
    MissingDestination,
    #[error("execution failed: {0}")]
    Exec(#[from] ExecError),
    #[error("command failed: {0}")]
    Failed(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
