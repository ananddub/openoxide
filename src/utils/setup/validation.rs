use crate::utils::exec::{CommandExecutor, ExecResult};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ToolState {
    pub installed: bool,
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PortAvailability {
    pub port: u16,
    pub available: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ServerAudit {
    pub os_id: String,
    pub architecture: String,
    pub docker: ToolState,
    pub git: ToolState,
    pub rclone: ToolState,
    pub nixpacks: ToolState,
    pub railpack: ToolState,
    pub buildpacks: ToolState,
    pub swarm_active: bool,
    pub network_exists: bool,
    pub base_directory_exists: bool,
    pub docker_group_member: bool,
    pub ports: Vec<PortAvailability>,
}

fn parse_os_id(os_release: &str) -> String {
    os_release
        .lines()
        .find_map(|line| line.strip_prefix("ID="))
        .unwrap_or_default()
        .trim_matches('"')
        .to_owned()
}

fn port_is_listening(ss_output: &str, port: u16) -> bool {
    let suffix = format!(":{port}");
    ss_output.lines().any(|line| {
        line.split_whitespace()
            .any(|field| field.ends_with(&suffix))
    })
}

pub(crate) async fn audit(
    executor: &CommandExecutor,
    base: &str,
    network: &str,
    ports: &[u16],
) -> ExecResult<ServerAudit> {
    async fn tool(executor: &CommandExecutor, binary: &str) -> ToolState {
        match crate::utils::os::OsCli::new(executor)
            .system()
            .tool_version(binary)
            .run()
            .await
        {
            Ok(output) => ToolState {
                installed: true,
                version: output
                    .combined_output()
                    .lines()
                    .next()
                    .map(str::trim)
                    .filter(|line| !line.is_empty())
                    .map(str::to_owned),
            },
            Err(_) => ToolState::default(),
        }
    }

    let os = crate::utils::os::OsCli::new(executor);
    let os_release = os.file("/etc/os-release").read().execute().await?.stdout;
    let os_id = parse_os_id(&os_release);
    let architecture = os.system().arch().run().await?.stdout_trimmed().to_owned();
    let docker_state = tool(executor, "docker").await;
    let git = tool(executor, "git").await;
    let rclone = tool(executor, "rclone").await;
    let nixpacks = tool(executor, "nixpacks").await;
    let railpack = tool(executor, "railpack").await;
    let buildpacks = tool(executor, "pack").await;
    let docker = crate::utils::docker::DockerCli::from_executor(executor.clone());
    let swarm_active = docker
        .swarm()
        .active()
        .run()
        .await
        .map(|output| output.stdout_trimmed() == "active")
        .unwrap_or(false);
    let docker_group_member = os
        .system()
        .current_groups()
        .run()
        .await
        .map(|output| {
            output
                .stdout
                .split_whitespace()
                .any(|group| group == "docker")
        })
        .unwrap_or(false);
    let network_exists = docker.networks().inspect(network).await.is_ok();
    let base_directory_exists = os.dir(base).exists().run().await.is_ok();
    let listening = os
        .network()
        .listen_ports_detailed()
        .run()
        .await
        .map(|output| output.stdout)
        .unwrap_or_default();
    let mut checked_ports = Vec::new();
    for port in ports {
        let in_use = port_is_listening(&listening, *port);
        checked_ports.push(PortAvailability {
            port: *port,
            available: !in_use,
        });
    }
    Ok(ServerAudit {
        os_id,
        architecture,
        docker: docker_state,
        git,
        rclone,
        nixpacks,
        railpack,
        buildpacks,
        swarm_active,
        docker_group_member,
        network_exists,
        base_directory_exists,
        ports: checked_ports,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_quoted_and_unquoted_os_ids() {
        assert_eq!(parse_os_id("NAME=Ubuntu\nID=ubuntu\n"), "ubuntu");
        assert_eq!(parse_os_id("NAME=Alpine\nID=\"alpine\"\n"), "alpine");
    }

    #[test]
    fn detects_exact_listening_port_without_prefix_collisions() {
        let output = "tcp LISTEN 0 4096 0.0.0.0:8080 0.0.0.0:*\n\
                      tcp LISTEN 0 4096 [::]:443 [::]:*\n";
        assert!(port_is_listening(output, 8080));
        assert!(port_is_listening(output, 443));
        assert!(!port_is_listening(output, 80));
    }
}
