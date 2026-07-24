use super::types::{Mount, MountKind, PortBinding, Protocol, RestartPolicy};
use crate::utils::docker::{
    DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent,
};
use tokio::sync::mpsc;

/// Fluent builder for `docker container create` / `docker container run`.
///
/// Terminal methods: [`.create()`], [`.run()`], [`.run_stream()`].
pub struct ContainerCreate<'a> {
    cli: &'a DockerCli,
    image: String,
    name: Option<String>,
    hostname: Option<String>,
    env: Vec<(String, String)>,
    labels: Vec<(String, String)>,
    networks: Vec<String>,
    ports: Vec<PortBinding>,
    mounts: Vec<Mount>,
    memory: Option<String>,
    cpus: Option<String>,
    restart: Option<RestartPolicy>,
    command: Vec<String>,
    entrypoint: Vec<String>,
    workdir: Option<String>,
    user: Option<String>,
    platform: Option<String>,
    tty: bool,
    interactive: bool,
    detach: bool,
    rm: bool,
    privileged: bool,
    cap_add: Vec<String>,
    cap_drop: Vec<String>,
    add_hosts: Vec<String>,
    dns: Vec<String>,
    shm_size: Option<String>,
    security_opts: Vec<String>,
    init: bool,
    devices: Vec<String>,
    network_aliases: Vec<String>,
    extra: Vec<String>,
}

impl<'a> ContainerCreate<'a> {
    pub(crate) fn new(cli: &'a DockerCli, image: impl Into<String>) -> Self {
        Self {
            cli,
            image: image.into(),
            name: None,
            hostname: None,
            env: vec![],
            labels: vec![],
            networks: vec![],
            ports: vec![],
            mounts: vec![],
            memory: None,
            cpus: None,
            restart: None,
            command: vec![],
            entrypoint: vec![],
            workdir: None,
            user: None,
            platform: None,
            tty: false,
            interactive: false,
            detach: false,
            rm: false,
            privileged: false,
            cap_add: vec![],
            cap_drop: vec![],
            add_hosts: vec![],
            dns: vec![],
            shm_size: None,
            security_opts: vec![],
            init: false,
            devices: vec![],
            network_aliases: vec![],
            extra: vec![],
        }
    }

    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    pub fn hostname(mut self, v: impl Into<String>) -> Self {
        self.hostname = Some(v.into());
        self
    }

    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.push((key.into(), value.into()));
        self
    }
    pub fn envs(
        mut self,
        iter: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        self.env
            .extend(iter.into_iter().map(|(k, v)| (k.into(), v.into())));
        self
    }

    pub fn label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.labels.push((key.into(), value.into()));
        self
    }
    pub fn labels(
        mut self,
        iter: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        self.labels
            .extend(iter.into_iter().map(|(k, v)| (k.into(), v.into())));
        self
    }

    pub fn network(mut self, net: impl Into<String>) -> Self {
        self.networks.push(net.into());
        self
    }

    /// Publish `host_port:container_port/tcp`.
    pub fn publish(mut self, host_port: u16, container_port: u16) -> Self {
        self.ports.push(PortBinding {
            host: host_port,
            container: container_port,
            proto: Protocol::Tcp,
        });
        self
    }
    /// Publish with an explicit protocol.
    pub fn publish_proto(mut self, host_port: u16, container_port: u16, proto: Protocol) -> Self {
        self.ports.push(PortBinding {
            host: host_port,
            container: container_port,
            proto,
        });
        self
    }

    /// Mount a named volume at `target`.
    pub fn volume(mut self, name: impl Into<String>, target: impl Into<String>) -> Self {
        self.mounts.push(Mount {
            source: name.into(),
            target: target.into(),
            read_only: false,
            kind: MountKind::Volume,
        });
        self
    }
    /// Bind-mount a host path at `target`.
    pub fn bind(mut self, host_path: impl Into<String>, target: impl Into<String>) -> Self {
        self.mounts.push(Mount {
            source: host_path.into(),
            target: target.into(),
            read_only: false,
            kind: MountKind::Bind,
        });
        self
    }
    /// Bind-mount read-only.
    pub fn bind_ro(mut self, host_path: impl Into<String>, target: impl Into<String>) -> Self {
        self.mounts.push(Mount {
            source: host_path.into(),
            target: target.into(),
            read_only: true,
            kind: MountKind::Bind,
        });
        self
    }
    /// tmpfs mount at `target`.
    pub fn tmpfs(mut self, target: impl Into<String>) -> Self {
        self.mounts.push(Mount {
            source: String::new(),
            target: target.into(),
            read_only: false,
            kind: MountKind::Tmpfs,
        });
        self
    }

    pub fn memory(mut self, v: impl Into<String>) -> Self {
        self.memory = Some(v.into());
        self
    }
    pub fn cpus(mut self, v: impl Into<String>) -> Self {
        self.cpus = Some(v.into());
        self
    }
    pub fn restart(mut self, policy: RestartPolicy) -> Self {
        self.restart = Some(policy);
        self
    }
    pub fn workdir(mut self, v: impl Into<String>) -> Self {
        self.workdir = Some(v.into());
        self
    }
    pub fn user(mut self, v: impl Into<String>) -> Self {
        self.user = Some(v.into());
        self
    }
    pub fn platform(mut self, v: impl Into<String>) -> Self {
        self.platform = Some(v.into());
        self
    }

    pub fn command(mut self, cmd: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.command = cmd.into_iter().map(Into::into).collect();
        self
    }
    pub fn entrypoint(mut self, ep: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.entrypoint = ep.into_iter().map(Into::into).collect();
        self
    }

    /// Allocate a pseudo-TTY (`-t`).
    pub fn tty(mut self, enabled: bool) -> Self {
        self.tty = enabled;
        self
    }

    /// Keep STDIN open (`-i`).
    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }
    /// Run in the background (`-d`).
    pub fn detach(mut self) -> Self {
        self.detach = true;
        self
    }
    /// Remove the container when it exits (`--rm`).
    pub fn rm(mut self) -> Self {
        self.rm = true;
        self
    }
    /// Give extended privileges to this container (`--privileged`).
    pub fn privileged(mut self) -> Self {
        self.privileged = true;
        self
    }
    pub fn cap_add(mut self, cap: impl Into<String>) -> Self {
        self.cap_add.push(cap.into());
        self
    }
    pub fn cap_drop(mut self, cap: impl Into<String>) -> Self {
        self.cap_drop.push(cap.into());
        self
    }
    pub fn add_host(mut self, host_ip: impl Into<String>) -> Self {
        self.add_hosts.push(host_ip.into());
        self
    }
    pub fn dns(mut self, server: impl Into<String>) -> Self {
        self.dns.push(server.into());
        self
    }
    pub fn shm_size(mut self, size: impl Into<String>) -> Self {
        self.shm_size = Some(size.into());
        self
    }
    pub fn security_opt(mut self, opt: impl Into<String>) -> Self {
        self.security_opts.push(opt.into());
        self
    }
    pub fn init(mut self) -> Self {
        self.init = true;
        self
    }
    pub fn device(mut self, dev: impl Into<String>) -> Self {
        self.devices.push(dev.into());
        self
    }
    pub fn network_alias(mut self, alias: impl Into<String>) -> Self {
        self.network_aliases.push(alias.into());
        self
    }

    /// Pass any raw flag not yet covered by the builder.
    pub fn arg(mut self, v: impl Into<String>) -> Self {
        self.extra.push(v.into());
        self
    }

    pub(crate) fn build_opts(&self) -> Vec<String> {
        let mut a: Vec<String> = vec![];

        if let Some(n) = &self.name {
            a.extend(["--name".into(), n.clone()]);
        }
        if let Some(h) = &self.hostname {
            a.extend(["--hostname".into(), h.clone()]);
        }
        for (k, v) in &self.env {
            a.extend(["--env".into(), format!("{k}={v}")]);
        }
        for (k, v) in &self.labels {
            a.extend(["--label".into(), format!("{k}={v}")]);
        }
        for net in &self.networks {
            a.extend(["--network".into(), net.clone()]);
        }

        for p in &self.ports {
            a.extend([
                "--publish".into(),
                format!("{}:{}/{}", p.host, p.container, p.proto),
            ]);
        }

        for m in &self.mounts {
            match m.kind {
                MountKind::Volume => {
                    let spec = if m.read_only {
                        format!("{}:{}:ro", m.source, m.target)
                    } else {
                        format!("{}:{}", m.source, m.target)
                    };
                    a.extend(["--volume".into(), spec]);
                }
                MountKind::Bind => {
                    let spec = if m.read_only {
                        format!("type=bind,source={},target={},readonly", m.source, m.target)
                    } else {
                        format!("type=bind,source={},target={}", m.source, m.target)
                    };
                    a.extend(["--mount".into(), spec]);
                }
                MountKind::Tmpfs => {
                    a.extend(["--mount".into(), format!("type=tmpfs,target={}", m.target)]);
                }
            }
        }

        if let Some(m) = &self.memory {
            a.extend(["--memory".into(), m.clone()]);
        }
        if let Some(c) = &self.cpus {
            a.extend(["--cpus".into(), c.clone()]);
        }
        if let Some(r) = &self.restart {
            a.extend(["--restart".into(), r.to_string()]);
        }
        if let Some(w) = &self.workdir {
            a.extend(["--workdir".into(), w.clone()]);
        }
        if let Some(u) = &self.user {
            a.extend(["--user".into(), u.clone()]);
        }
        if let Some(p) = &self.platform {
            a.extend(["--platform".into(), p.clone()]);
        }

        if !self.entrypoint.is_empty() {
            a.push("--entrypoint".into());
            a.push(self.entrypoint.join(" "));
        }

        if self.tty {
            a.push("--tty".into());
        }
        if self.interactive {
            a.push("--interactive".into());
        }
        if self.detach {
            a.push("--detach".into());
        }
        if self.rm {
            a.push("--rm".into());
        }
        if self.privileged {
            a.push("--privileged".into());
        }

        for cap in &self.cap_add {
            a.extend(["--cap-add".into(), cap.clone()]);
        }
        for cap in &self.cap_drop {
            a.extend(["--cap-drop".into(), cap.clone()]);
        }
        for host in &self.add_hosts {
            a.extend(["--add-host".into(), host.clone()]);
        }
        for server in &self.dns {
            a.extend(["--dns".into(), server.clone()]);
        }
        if let Some(s) = &self.shm_size {
            a.extend(["--shm-size".into(), s.clone()]);
        }
        for opt in &self.security_opts {
            a.extend(["--security-opt".into(), opt.clone()]);
        }
        if self.init {
            a.push("--init".into());
        }
        for dev in &self.devices {
            a.extend(["--device".into(), dev.clone()]);
        }
        for alias in &self.network_aliases {
            a.extend(["--network-alias".into(), alias.clone()]);
        }

        a.extend(self.extra.clone());
        a.push(self.image.clone());
        a.extend(self.command.clone());
        a
    }

    /// `docker container create …` — returns the new container ID.
    pub async fn create(self) -> DockerResult<String> {
        let cli = self.cli;
        let mut args = vec!["container".to_string(), "create".to_string()];
        args.extend(self.build_opts());
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        Ok(cli.run(refs).await?.stdout.trim().to_string())
    }

    /// `docker container run …` — returns the combined output.
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let cli = self.cli;
        let mut args = vec!["container".to_string(), "run".to_string()];
        args.extend(self.build_opts());
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        cli.run(refs).await
    }

    /// `docker container run …` — streams output events.
    pub async fn run_stream(
        self,
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let cli = self.cli;
        let mut args = vec!["container".to_string(), "run".to_string()];
        args.extend(self.build_opts());
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        cli.run_stream(refs, sender).await
    }
}
