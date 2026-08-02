use tokio::process::Command;
use tracing::error;

/// Reads the last `tail_lines` lines of a container's logs.
///
/// One-shot, not a live stream: the panel calls this to backfill a log view.
/// stdout and stderr are merged because docker interleaves them and callers
/// only need the combined view.
pub async fn tail_container_logs(container_id: &str, tail_lines: usize) -> Vec<String> {
    let output = match Command::new("docker")
        .args(["logs", "--tail", &tail_lines.to_string(), container_id])
        .output()
        .await
    {
        Ok(out) => out,
        Err(err) => {
            error!(error = %err, container_id, "could not run docker logs");
            return Vec::new();
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    stdout
        .lines()
        .chain(stderr.lines())
        .map(str::to_string)
        .collect()
}
