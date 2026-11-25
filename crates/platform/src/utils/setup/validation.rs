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
    pub rsync: ToolState,
    pub nixpacks: ToolState,
    pub railpack: ToolState,
    pub buildpacks: ToolState,
    pub swarm_active: bool,
    pub network_exists: bool,
    pub base_directory_exists: bool,
    pub docker_group_member: bool,
    pub ports: Vec<PortAvailability>,
}

pub(crate) fn normalized_os_id(os_release: &str) -> String {
    let raw = os_release
        .lines()
        .find_map(|line| line.strip_prefix("ID="))
        .unwrap_or_default()
        .trim_matches('"')
        .to_owned();
    match raw.as_str() {
        "manjaro" | "manjaro-arm" => "arch".into(),
        "pop" | "linuxmint" | "zorin" => "ubuntu".into(),
        "fedora-asahi-remix" => "fedora".into(),
        _ => raw,
    }
}

pub(crate) fn supported_os(os_id: &str) -> bool {
    matches!(
        os_id,
        "arch"
            | "ubuntu"
            | "debian"
            | "raspbian"
            | "centos"
            | "fedora"
            | "rhel"
            | "ol"
            | "rocky"
            | "sles"
            | "opensuse-leap"
            | "opensuse-tumbleweed"
            | "almalinux"
            | "opencloudos"
            | "amzn"
            | "alpine"
    )
}

/// Whether a running container already publishes host `port`, judged from
/// `docker ps --format '{{.Ports}}'` (`0.0.0.0:80->80/tcp, ...`). A container
/// holding the port makes Traefik start "successfully" with nothing bound.
pub(crate) fn port_held_by_docker(docker_ps_ports: &str, port: u16) -> bool {
    let needle = format!(":{port}->");
    docker_ps_ports.contains(&needle)
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
    let docker = crate::utils::docker::DockerCli::from_executor(executor.clone());
    // Every probe is independent, and the remote executor multiplexes
    // channels over one SSH session: run them concurrently instead of paying
    // two network round trips per probe back to back.
    let (
        os_release,
        architecture,
        docker_state,
        git,
        rclone,
        rsync,
        nixpacks,
        railpack,
        buildpacks,
        swarm,
        groups,
        network_inspect,
        base_dir,
        listening,
    ) = tokio::join!(
        async { os.file("/etc/os-release").read().execute().await },
        async { os.system().arch().run().await },
        tool(executor, "docker"),
        tool(executor, "git"),
        tool(executor, "rclone"),
        tool(executor, "rsync"),
        tool(executor, "nixpacks"),
        tool(executor, "railpack"),
        tool(executor, "pack"),
        async { docker.swarm().active().run().await },
        async { os.system().current_groups().run().await },
        async { docker.networks().inspect(network).await },
        async { os.dir(base).exists().run().await },
        async { os.network().listen_ports_detailed().run().await },
    );
    let os_id = normalized_os_id(&os_release?.stdout);
    let architecture = architecture?.stdout_trimmed().to_owned();
    let swarm_active = swarm
        .map(|output| output.stdout_trimmed() == "active")
        .unwrap_or(false);
    let docker_group_member = groups
        .map(|output| {
            output
                .stdout
                .split_whitespace()
                .any(|group| group == "docker")
        })
        .unwrap_or(false);
    let network_exists = network_inspect.is_ok();
    let base_directory_exists = base_dir.is_ok();
    let listening = listening.map(|output| output.stdout).unwrap_or_default();
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
        rsync,
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
    #[test]
    fn detects_docker_published_ports() {
        let ps = "0.0.0.0:80->80/tcp, [::]:80->80/tcp, 0.0.0.0:443->443/udp";
        assert!(super::port_held_by_docker(ps, 80));
        assert!(super::port_held_by_docker(ps, 443));
        assert!(!super::port_held_by_docker(ps, 8080));
        assert!(!super::port_held_by_docker("0.0.0.0:8080->8080/tcp", 80));
    }

    use super::*;

    #[test]
    fn parses_quoted_and_unquoted_os_ids() {
        assert_eq!(normalized_os_id("NAME=Ubuntu\nID=ubuntu\n"), "ubuntu");
        assert_eq!(normalized_os_id("NAME=Alpine\nID=\"alpine\"\n"), "alpine");
        assert_eq!(normalized_os_id("ID=linuxmint\n"), "ubuntu");
        assert!(supported_os("ubuntu"));
        assert!(!supported_os("plan9"));
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
