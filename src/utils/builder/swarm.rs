use crate::utils::{
    docker::DockerCli,
    exec::{CommandExecutor, ExecError, ExecResult, detect_advertise_addr_cancelled},
};
use tokio_util::sync::CancellationToken;

pub(crate) const RUSTPLOY_NETWORK: &str = "rustploy-network";

pub(crate) async fn ensure_swarm_manager(
    executor: &CommandExecutor,
    docker: &DockerCli,
    cancel: &CancellationToken,
) -> ExecResult<()> {
    let info = docker.swarm().inspect_cancelled(cancel).await?;
    let state = info.local_node_state.to_lowercase();
    let control_available = info.control_available;

    if state == "active" && control_available {
        return Ok(());
    }
    if state == "active" && !control_available {
        return Err(command_error(
            "docker swarm is active but this node is not a manager; deploy STACK workloads on a swarm manager or promote this node",
        ));
    }

    let advertise_addr = detect_advertise_addr_cancelled(executor, cancel).await;
    docker
        .swarm()
        .init()
        .advertise_addr(&advertise_addr)
        .listen_addr("0.0.0.0:2377")
        .cancel_with(cancel.clone())
        .run()
        .await?;
    Ok(())
}

pub(crate) async fn ensure_overlay_network(
    docker: &DockerCli,
    name: &str,
    cancel: &CancellationToken,
) -> ExecResult<()> {
    if docker
        .networks()
        .inspect_cmd(name)
        .cancel_with(cancel.clone())
        .run()
        .await
        .is_ok()
    {
        return Ok(());
    }
    match docker
        .networks()
        .create(name)
        .driver(crate::utils::docker::NetworkDriver::Overlay)
        .opt("encrypted", "")
        .attachable()
        .cancel_with(cancel.clone())
        .run()
        .await
    {
        Ok(_) => Ok(()),
        Err(error) if error.to_string().contains("already exists") => Ok(()),
        Err(error) => Err(error),
    }
}

fn command_error(message: impl Into<String>) -> ExecError {
    ExecError::CommandFailed {
        code: None,
        stderr: message.into(),
    }
}
