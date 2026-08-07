#[derive(Debug, thiserror::Error)]
pub enum CgroupError {
    #[error("Failed to create cgroup directory: {0}")]
    CreateFailed(String),

    #[error("Failed to delete cgroup directory: {0}")]
    DeleteFailed(String),

    #[error("Failed to write cgroup file '{path}': {error}")]
    WriteFailed { path: String, error: String },

    #[error("Failed to read cgroup file '{path}': {error}")]
    ReadFailed { path: String, error: String },

    #[error("Controller '{controller}' is not available in system cgroup.controllers")]
    ControllerUnavailable { controller: String },

    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Executor error: {0}")]
    ExecutorError(String),
}
