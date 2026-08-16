pub use crate::exec::{
    ExecError as DockerError, ExecExitStatus as DockerExitStatus, ExecOutput as DockerOutput,
    ExecResult as DockerResult, ExecStreamEvent as DockerStreamEvent,
};

pub fn is_transient_docker_error(message: &str) -> bool {
    let message = message.to_ascii_lowercase();
    [
        "cannot connect to the docker daemon",
        "docker daemon",
        "connection refused",
        "connection reset",
        "service unavailable",
        "temporarily unavailable",
        "context deadline exceeded",
        "command output parsing failed",
        "expected a sequence",
        "no such service",
        "not found",
    ]
    .iter()
    .any(|needle| message.contains(needle))
}
