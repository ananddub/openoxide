use crate::utils::{
    docker::DockerCli,
    exec::{CommandExecutor, ExecError, ExecResult},
};
use tokio_util::sync::CancellationToken;

pub(crate) const RUSTPLOY_NETWORK: &str = "rustploy-network";

pub(crate) async fn ensure_swarm_manager(
    executor: &CommandExecutor,
    docker: &DockerCli,
    cancel: &CancellationToken,
) -> ExecResult<()> {
    let info = docker
        .run_cancelled(
            [
                "info",
                "--format",
                "{{.Swarm.LocalNodeState}}\t{{.Swarm.ControlAvailable}}",
            ],
            cancel,
        )
        .await?;
    let mut parts = info.stdout_trimmed().split_whitespace();
    let state = parts.next().unwrap_or_default();
    let control_available = parts.next().unwrap_or_default() == "true";

    if state == "active" && control_available {
        return Ok(());
    }
    if state == "active" && !control_available {
        return Err(command_error(
            "docker swarm is active but this node is not a manager; deploy STACK workloads on a swarm manager or promote this node",
        ));
    }

    let advertise_addr = detect_advertise_addr(executor, cancel).await;
    docker
        .run_cancelled(
            [
                "swarm",
                "init",
                "--advertise-addr",
                advertise_addr.as_str(),
                "--listen-addr",
                "0.0.0.0:2377",
            ],
            cancel,
        )
        .await?;
    Ok(())
}

pub(crate) async fn ensure_overlay_network(
    docker: &DockerCli,
    name: &str,
    cancel: &CancellationToken,
) -> ExecResult<()> {
    if docker
        .run_cancelled(["network", "inspect", name], cancel)
        .await
        .is_ok()
    {
        return Ok(());
    }
    match docker
        .run_cancelled(
            [
                "network",
                "create",
                "--driver",
                "overlay",
                "--attachable",
                name,
            ],
            cancel,
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(error) if error.to_string().contains("already exists") => Ok(()),
        Err(error) => Err(error),
    }
}

async fn detect_advertise_addr(executor: &CommandExecutor, cancel: &CancellationToken) -> String {
    let output = executor
        .run_cancelled("hostname", ["-I"], cancel)
        .await
        .ok();
    output
        .as_ref()
        .and_then(|output| {
            output
                .stdout
                .split_whitespace()
                .find(|ip| !ip.starts_with("127."))
                .map(str::to_owned)
        })
        .unwrap_or_else(|| "127.0.0.1".into())
}

fn command_error(message: impl Into<String>) -> ExecError {
    ExecError::CommandFailed {
        code: None,
        stderr: message.into(),
    }
}
