use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecError {
    #[error("failed to execute command: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to execute command `{program}`: {source}")]
    IoCommand {
        program: String,
        #[source]
        source: std::io::Error,
    },
    #[error("command failed (exit code {code:?}): {stderr}")]
    CommandFailed { code: Option<i32>, stderr: String },
    #[error("remote SSH execution failed: {0}")]
    Ssh(String),
    #[error("command stream consumer disconnected")]
    StreamCancelled,
    #[error("command timed out after {seconds} seconds")]
    Timeout { seconds: u64 },
    #[error("command output parsing failed: {0}")]
    Json(#[from] serde_json::Error),
}

pub type ExecResult<T> = Result<T, ExecError>;
