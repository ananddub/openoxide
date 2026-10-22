use crate::utils::{
    docker::DockerCli,
    exec::{CommandExecutor, ExecError, ExecResult, detect_advertise_addr_cancelled},
};
use tokio_util::sync::CancellationToken;

pub const OPENOXIDE_NETWORK: &str = "openoxide-network";

pub async fn ensure_swarm_manager(
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

pub async fn ensure_overlay_network(
    docker: &DockerCli,
    name: &str,
    cancel: &CancellationToken,
) -> ExecResult<()> {
    if let Ok(network) = docker.networks().inspect(name).await {
        return ensure_network_compatible(name, &network.driver, &network.scope);
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
        // Lost a race with another creator: validate what now exists instead
        // of assuming it is the overlay network this deployment needs.
        Err(error) if error.to_string().contains("already exists") => {
            let network = docker.networks().inspect(name).await?;
            ensure_network_compatible(name, &network.driver, &network.scope)
        }
        Err(error) => Err(error),
    }
}

fn ensure_network_compatible(name: &str, driver: &str, scope: &str) -> ExecResult<()> {
    if driver.eq_ignore_ascii_case("overlay") && scope.eq_ignore_ascii_case("swarm") {
        return Ok(());
    }
    Err(command_error(format!(
        "network '{name}' already exists with driver '{driver}' and scope '{scope}', but STACK workloads require an attachable overlay network in swarm scope; rename or recreate that network as an overlay, then redeploy"
    )))
}

fn command_error(message: impl Into<String>) -> ExecError {
    ExecError::CommandFailed {
        code: None,
        stderr: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swarm_overlay_network_is_accepted() {
        assert!(ensure_network_compatible("openoxide-network", "overlay", "swarm").is_ok());
        assert!(ensure_network_compatible("openoxide-network", "Overlay", "Swarm").is_ok());
    }

    #[test]
    fn prepopulated_bridge_network_is_rejected() {
        let error = ensure_network_compatible("openoxide-network", "bridge", "local").unwrap_err();
        let message = error.to_string();
        assert!(message.contains("driver 'bridge'"), "{message}");
        assert!(message.contains("scope 'local'"), "{message}");
        assert!(message.contains("overlay network"), "{message}");
    }
}
