use super::{ServerAudit, SetupConfig, validation};
use crate::utils::os::file::FileMode;
use crate::utils::{
    docker::{
        DockerCli,
        core::{Mount, Port},
        handles::containers::RestartPolicy,
    },
    exec::{
        CommandExecutor, ExecResult, ExecStreamEvent,
        script::{ScriptPipeline, ShellIR, sh},
    },
    os::OsCli,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SetupStep {
    Dependencies,
    BuildTools,
    Directories,
    Swarm,
    Network,
    TraefikConfig,
    Traefik,
    Agent,
}

#[derive(Clone, Debug, Default)]
pub struct SetupOutcome {
    pub completed: Vec<SetupStep>,
    pub audit: ServerAudit,
}

#[derive(Clone, Debug)]
pub struct ServerSetup {
    executor: CommandExecutor,
    pub config: SetupConfig,
}

impl ServerSetup {
    pub fn new(executor: CommandExecutor, config: SetupConfig) -> Self {
        Self { executor, config }
    }
    pub fn new_local(config: SetupConfig) -> Self {
        Self::new(
            CommandExecutor::Local(crate::utils::exec::LocalExecutor::new()),
            config,
        )
    }
    pub fn new_remote(executor: crate::utils::exec::RemoteExecutor, config: SetupConfig) -> Self {
        Self::new(CommandExecutor::Remote(executor), config)
    }
    pub fn executor(&self) -> &CommandExecutor {
        &self.executor
    }
    pub async fn audit(&self) -> ExecResult<ServerAudit> {
        validation::audit(
            &self.executor,
            &self.config.paths.base,
            &self.config.network_name,
            &[
                self.config.http_port,
                self.config.https_port,
                self.config.http3_port,
            ],
        )
        .await
    }

    pub async fn preflight(&self) -> ExecResult<()> {
        let os = OsCli::new(&self.executor);
        let uid = self
            .executor
            .run("id", ["-u"])
            .await?
            .stdout_trimmed()
            .to_owned();
        if uid != "0" {
            return Err(crate::utils::exec::ExecError::InvalidData(
                "remote setup requires root or working sudo credentials".into(),
            ));
        }

        let os_release = os.file("/etc/os-release").read().execute().await?.stdout;
        let os_id = validation::normalized_os_id(&os_release);
        if !validation::supported_os(&os_id) {
            return Err(crate::utils::exec::ExecError::InvalidData(format!(
                "unsupported remote operating system: {os_id}"
            )));
        }

        if os.has_command("snap").run().await.is_ok()
            && self.executor.run("snap", ["list", "docker"]).await.is_ok()
        {
            return Err(crate::utils::exec::ExecError::InvalidData(
                "Docker installed through Snap is unsupported; install Docker Engine instead"
                    .into(),
            ));
        }
        Ok(())
    }

    pub async fn write_traefik_config(&self) -> ExecResult<()> {
        let static_path = format!("{}/traefik.yml", self.config.paths.traefik);
        let middleware_path = format!("{}/middlewares.yml", self.config.paths.traefik_dynamic);
        self.write_if_missing(
            &static_path,
            super::traefik::static_config(&self.config).as_bytes(),
            false,
        )
        .await?;
        self.write_if_missing(
            &middleware_path,
            super::traefik::default_middlewares().as_bytes(),
            false,
        )
        .await?;
        Ok(())
    }
    async fn write_if_missing(
        &self,
        path: &str,
        contents: &[u8],
        overwrite: bool,
    ) -> ExecResult<()> {
        let os = OsCli::new(&self.executor);
        if os.dir(path).exists().run().await.is_ok() {
            os.dir(path).delete().run().await?;
        }

        if let Some(parent) = std::path::Path::new(path).parent() {
            let _ = os
                .dir(parent.to_string_lossy().as_ref())
                .create()
                .run()
                .await;
        }

        if !overwrite && os.file(path).exists().run().await.is_ok() {
            if is_valid_yaml_file(&os, path).await {
                return Ok(());
            }
        }
        os.file(path)
            .write(String::from_utf8_lossy(contents).as_ref())
            .execute()
            .await?;
        os.file(path)
            .chmod(crate::utils::os::file::FileMode::OwnerReadWrite)
            .run()
            .await?;
        Ok(())
    }
    pub async fn ensure_traefik(&self) -> ExecResult<()> {
        let docker = DockerCli::from_executor(self.executor.clone());
        let name = self.config.traefik_name.as_str();
        let mut started = false;
        if docker.container(name).inspect().await.is_ok() {
            let _ = docker
                .networks()
                .connect(self.config.network_name.as_str(), name)
                .run()
                .await;
            match docker.container(name).start().run().await {
                Ok(_) => started = true,

                Err(_) => {
                    let _ = docker.container(name).remove().force().run().await;
                }
            }
        }
        if !started {
            if docker.services().inspect(name).await.is_ok() {
                docker.services().remove(name).run().await?;
            }
            let image = format!("traefik:v{}", self.config.traefik_version);
            docker.images().pull(&image).pull().await?;

            let static_mount = Mount::bind_ro(
                format!("{}/traefik.yml", self.config.paths.traefik),
                "/etc/traefik/traefik.yml",
            );
            let dynamic_mount = Mount::bind(
                &self.config.paths.traefik_dynamic,
                "/etc/openoxide/traefik/dynamic",
            );
            let docker_socket_mount =
                Mount::bind_ro("/var/run/docker.sock", "/var/run/docker.sock");

            let p_http = Port::tcp(self.config.http_port, self.config.http_port);
            let p_https = Port::tcp(self.config.https_port, self.config.https_port);
            let p_http3 = Port::udp(self.config.http3_port, self.config.http3_port);
            let p_dashboard = Port::tcp(self.config.dashboard_port, 8080);

            docker
                .containers()
                .create(&image)
                .detach()
                .name(name)
                .restart(RestartPolicy::Always)
                .network(self.config.network_name.as_str())
                .mount(static_mount)
                .mount(dynamic_mount)
                .mount(docker_socket_mount)
                .publish(p_http)
                .publish(p_https)
                .publish(p_http3)
                .publish(p_dashboard)
                .run()
                .await?;
        }
        Ok(())
    }

    async fn write_agent_mtls(
        &self,
        dir: &str,
        ca: &str,
        cert: &str,
        key: &str,
    ) -> ExecResult<bool> {
        let os = OsCli::new(&self.executor);
        os.dir(dir).create().parents(true).run().await?;
        let mut changed = false;
        for (name, contents, mode) in [
            ("ca.crt", ca, FileMode::OwnerReadWriteGroupReadWorldRead),
            (
                "agent.crt",
                cert,
                FileMode::OwnerReadWriteGroupReadWorldRead,
            ),
            ("agent.key", key, FileMode::OwnerReadWrite),
        ] {
            let path = format!("{dir}/{name}");
            let current = os.file(&path).read().execute().await.ok();
            let same = current
                .as_ref()
                .map(|o| o.stdout_trimmed() == contents.trim())
                .unwrap_or(false);
            if !same {
                changed = true;
                os.file(&path).write(contents).execute().await?;
                os.file(&path).chmod(mode).run().await?;
            }
        }
        Ok(changed)
    }

    pub async fn ensure_agent(&self) -> ExecResult<()> {
        let docker = DockerCli::from_executor(self.executor.clone());
        let name = AGENT_CONTAINER_NAME;

        let mtls_dir = format!("{}/mtls", self.config.paths.agent);
        let (mtls_ready, mtls_changed) = if let (Some(ca), Some(cert), Some(key)) = (
            self.config.agent_mtls_ca_cert.as_deref(),
            self.config.agent_mtls_cert.as_deref(),
            self.config.agent_mtls_key.as_deref(),
        ) {
            let changed = self.write_agent_mtls(&mtls_dir, ca, cert, key).await?;
            (true, changed)
        } else {
            (false, false)
        };

        if self.config.agent_server_id.is_some()
            && docker
                .container("openoxide-monitor")
                .inspect()
                .await
                .is_ok()
        {
            let _ = docker
                .container("openoxide-monitor")
                .remove()
                .force()
                .run()
                .await;
        }
        if let Ok(existing) = docker.container(name).inspect().await {
            if existing.state.running && self.config.agent_server_id.is_some() {
                if !mtls_changed
                    && agent_supports_direct_host_shell(&existing.mounts, &existing.host_config)
                {
                    return Ok(());
                }
                if mtls_changed {
                    tracing::info!("agent mTLS certs rotated; recreating agent to load them");
                } else {
                    tracing::info!(
                        "agent container predates the direct host shell; recreating it with CAP_SYS_ADMIN, CAP_SYS_PTRACE and the /host bind"
                    );
                }
            } else if self.config.agent_server_id.is_none() {
                if docker.container(name).start().run().await.is_ok() {
                    return Ok(());
                }
                let _ = docker.container(name).remove().force().run().await;
            }
            let _ = docker.container(name).stop().run().await;
            let _ = docker.container(name).remove().force().run().await;
        }

        let architecture = OsCli::new(&self.executor)
            .system()
            .arch()
            .run()
            .await
            .map(|output| output.stdout_trimmed().to_owned())
            .unwrap_or_default();
        if agent_image_unsupported(&architecture) {
            return Ok(());
        }

        let image = agent_image();
        docker.images().pull(image).pull().await?;

        let docker_socket_mount = Mount::bind("/var/run/docker.sock", "/var/run/docker.sock");
        let cgroup_mount = Mount::bind("/sys/fs/cgroup", "/sys/fs/cgroup");
        // Host root, read-only: the live metrics collector resolves container
        // cgroups and per-container network counters through /host/proc, and the
        // direct host shell enters /host/proc/1/ns/* with setns. Reading those
        // namespace symlinks is ptrace-gated, so the shell needs CAP_SYS_PTRACE
        // on top of the CAP_SYS_ADMIN that makes the namespace switch legal.
        let host_mount = Mount::bind_ro("/", "/host");
        let workspace_mount = Mount::bind(&self.config.paths.agent, "/app/workspace");
        let traefik_mount = Mount::bind(&self.config.paths.traefik, "/etc/openoxide/traefik");
        let apps_mount = Mount::bind(
            &self.config.paths.applications,
            &self.config.paths.applications,
        );
        let compose_mount = Mount::bind(&self.config.paths.compose, &self.config.paths.compose);

        let p_grpc = Port::tcp(50051, 50051);

        let containers = docker.containers();
        let mut create = containers
            .create(image)
            .detach()
            .name(name)
            .init()
            .restart(RestartPolicy::Always)
            .network(self.config.network_name.as_str())
            .mount(docker_socket_mount)
            .mount(cgroup_mount)
            .mount(host_mount)
            .cap_add("SYS_ADMIN")
            .cap_add("SYS_PTRACE")
            .mount(workspace_mount)
            .mount(apps_mount)
            .mount(compose_mount)
            .mount(traefik_mount)
            .publish(p_grpc);
        if mtls_ready {
            create = create
                .mount(Mount::bind_ro(&mtls_dir, "/etc/openoxide/mtls"))
                .env("AGENT_TLS_CERT", "/etc/openoxide/mtls/agent.crt")
                .env("AGENT_TLS_KEY", "/etc/openoxide/mtls/agent.key")
                .env("AGENT_TLS_CLIENT_CA", "/etc/openoxide/mtls/ca.crt");
        }
        if let (Some(server_id), Some(panel_url), Some(token)) = (
            self.config.agent_server_id,
            self.config.agent_panel_url.as_deref(),
            self.config.agent_token.as_deref(),
        ) {
            create = create
                .env("SERVER_ID", server_id.to_string())
                .env("OPENOXIDE_SERVER_URL", panel_url)
                .env("METRICS_TOKEN", token)
                .env("OPENOXIDE_BASE_PATH", self.config.paths.base.as_str())
                .env("MONITOR_DATABASE_URL", "memory://metrics")
                .env("REFRESH_RATE", agent_refresh_rate())
                .env(
                    "RETENTION_DAYS",
                    self.config.agent_retention_days.to_string(),
                );
        }
        create.run().await?;

        docker.container(name).start().run().await?;
        Ok(())
    }

    pub fn oneshot_script(&self, install_dependencies: bool) -> ScriptPipeline {
        let os = OsCli::new(&self.executor);
        let docker = DockerCli::from_executor(self.executor.clone());
        let mut steps = Vec::new();

        if install_dependencies {
            self.append_dependency_steps(&mut steps, &os);
            if self.config.build_server {
                self.append_build_tool_steps(&mut steps, &os);
            }
        }

        self.append_directory_steps(&mut steps, &os);
        if self.config.build_server {
            return ScriptPipeline::new().cmd(steps);
        }
        self.append_swarm_step(&mut steps, &docker);
        self.append_network_step(&mut steps, &docker);
        self.append_traefik_config_steps(&mut steps, &os);
        self.append_traefik_step(&mut steps, &docker);
        self.append_agent_step(&mut steps, &docker);

        ScriptPipeline::new().cmd(steps)
    }

    pub fn compile_oneshot_script(&self, install_dependencies: bool) -> String {
        self.oneshot_script(install_dependencies).compile()
    }

    pub async fn setup_all_oneshot(&self, install_dependencies: bool) -> ExecResult<SetupOutcome> {
        self.preflight().await?;
        self.oneshot_script(install_dependencies)
            .execute(&self.executor)
            .await?;
        let audit = self.audit().await?;
        Ok(SetupOutcome {
            completed: setup_steps(install_dependencies, self.config.build_server),
            audit,
        })
    }

    pub async fn setup_all_oneshot_stream(
        &self,
        install_dependencies: bool,
        sender: tokio::sync::mpsc::Sender<String>,
    ) -> ExecResult<SetupOutcome> {
        self.preflight().await?;
        let script = self.compile_oneshot_script(install_dependencies);
        let (exec_tx, mut exec_rx) = tokio::sync::mpsc::channel::<ExecStreamEvent>(128);
        let log_tx = sender.clone();
        let forward_logs = tokio::spawn(async move {
            let mut pending_stdout = String::new();
            let mut pending_stderr = String::new();
            while let Some(event) = exec_rx.recv().await {
                match event {
                    ExecStreamEvent::Stdout(bytes) => {
                        pending_stdout.push_str(&String::from_utf8_lossy(&bytes));
                        while let Some(index) = pending_stdout.find('\n') {
                            let line = pending_stdout[..index].trim_end_matches('\r').to_owned();
                            pending_stdout.drain(..=index);
                            if !line.is_empty() && log_tx.send(line).await.is_err() {
                                return;
                            }
                        }
                    }
                    ExecStreamEvent::Stderr(bytes) => {
                        pending_stderr.push_str(&String::from_utf8_lossy(&bytes));
                        while let Some(index) = pending_stderr.find('\n') {
                            let line = pending_stderr[..index].trim_end_matches('\r').to_owned();
                            pending_stderr.drain(..=index);
                            if !line.is_empty() && log_tx.send(line).await.is_err() {
                                return;
                            }
                        }
                    }
                }
            }
            let line_out = pending_stdout.trim_end_matches('\r').to_owned();
            if !line_out.is_empty() {
                let _ = log_tx.send(line_out).await;
            }
            let line_err = pending_stderr.trim_end_matches('\r').to_owned();
            if !line_err.is_empty() {
                let _ = log_tx.send(line_err).await;
            }
        });

        let result = self
            .executor
            .run_stream("sh", ["-c".to_owned(), script], exec_tx)
            .await;
        let _ = forward_logs.await;
        result?;

        let audit = self.audit().await?;
        Ok(SetupOutcome {
            completed: setup_steps(install_dependencies, self.config.build_server),
            audit,
        })
    }

    fn append_dependency_steps(&self, steps: &mut Vec<ShellIR>, os: &OsCli<'_>) {
        steps.extend(sh!(
            info!("Checking Docker installation");
            if !os.has_command("docker") {
                info!("Installing Docker");
                os.shell_installer("https://get.docker.com").shell("sh");
            }
            if os.has_command("systemctl") {
                info!("Ensuring Docker service is enabled and running");
                os.service("docker").enable().now().non_fatal();
                os.service("docker").start().non_fatal();
            }
            cmd("docker", "info");
        ));
    }

    fn append_build_tool_steps(&self, _steps: &mut Vec<ShellIR>, _os: &OsCli<'_>) {}

    fn append_directory_steps(&self, steps: &mut Vec<ShellIR>, os: &OsCli<'_>) {
        steps.extend(sh!(info!("Creating OpenOxide directories");));
        for path in self.config.paths.all() {
            steps.extend(sh!(
                os.dir(path).create().parents(true);
            ));
        }
        let acme = format!("{}/acme.json", self.config.paths.traefik_dynamic);
        let ssh_path = self.config.paths.ssh.as_str();
        steps.extend(sh!(
            os.file(ssh_path).chmod(crate::utils::os::file::FileMode::OwnerReadWriteExecute);
            if !os.file(acme.as_str()).exists() {
                os.file(acme.as_str()).write("");
            }
            os.file(acme.as_str()).chmod(crate::utils::os::file::FileMode::OwnerReadWrite);
        ));
    }

    fn append_swarm_step(&self, steps: &mut Vec<ShellIR>, docker: &DockerCli) {
        if let Some(advertise_addr) = &self.config.advertise_addr {
            steps.extend(sh!(if !docker.swarm().active() {
                info!("Initializing Docker Swarm");
                docker
                    .swarm()
                    .init()
                    .advertise_addr(advertise_addr)
                    .listen_addr("0.0.0.0:2377");
            }));
        } else {
            steps.extend(sh!(
                info!("Checking Docker Swarm");
                if !docker.swarm().active() {
                    info!("Initializing Docker Swarm");
                    let _openoxide_vpn_addr = capture_stdout! {
                        pipe![
                            cmd("ip", "-4", "-o", "addr", "show"),
                            grep!("-E", "^[0-9]+: (tailscale|wg|zt|wt|nebula|ipsec|ppp|tun|tap)"),
                            grep!("-oE", "([0-9]+\\.){3}[0-9]+"),
                            head!("-n", "1")
                        ];
                    };
                    if cmd("test", "-n", _openoxide_vpn_addr) {
                        info!("Using VPN/overlay interface for swarm advertise address");
                        docker
                            .swarm()
                            .init()
                            .advertise_addr(_openoxide_vpn_addr)
                            .listen_addr("0.0.0.0:2377");
                    } else {
                        let _openoxide_cgnat_addr = capture_stdout! {
                            pipe![
                                cmd("ip", "-4", "-o", "addr", "show"),
                                grep!("-oE", "([0-9]+\\.){3}[0-9]+"),
                                grep!("-E", "^100\\.(6[4-9]|[7-9][0-9]|1[01][0-9]|12[0-7])\\."),
                                head!("-n", "1")
                            ];
                        };
                        if cmd("test", "-n", _openoxide_cgnat_addr) {
                            info!("Using CGNAT-range VPN address for swarm advertise address");
                            docker
                                .swarm()
                                .init()
                                .advertise_addr(_openoxide_cgnat_addr)
                                .listen_addr("0.0.0.0:2377");
                        } else {
                            let _openoxide_advertise_addr = capture_stdout! {
                                pipe![
                                    cmd("hostname", "-I"),
                                    awk! {
                                        for field in fields {
                                            if field != "127.0.0.1" {
                                                print(field);
                                                exit;
                                            }
                                        }
                                    }
                                ];
                            }
                            .default("127.0.0.1");
                            docker
                                .swarm()
                                .init()
                                .advertise_addr(_openoxide_advertise_addr)
                                .listen_addr("0.0.0.0:2377");
                        }
                    }
                }
            ));
        }
    }

    fn append_network_step(&self, steps: &mut Vec<ShellIR>, docker: &DockerCli) {
        let network_name = self.config.network_name.as_str();

        steps.extend(sh!(
            info!("Checking OpenOxide Docker network");
            if !docker
                .networks()
                .inspect_cmd(network_name)
                .stdout(crate::utils::exec::script::dsl::OutputTarget::Null)
                .stderr(crate::utils::exec::script::dsl::OutputTarget::Null)
            {
                info!("Creating OpenOxide Docker network");
                docker
                    .networks()
                    .create(network_name)
                    .driver(crate::utils::docker::NetworkDriver::Overlay)
                    .attachable();
            }
        ));
    }

    fn append_traefik_config_steps(&self, steps: &mut Vec<ShellIR>, os: &OsCli<'_>) {
        let static_path = format!("{}/traefik.yml", self.config.paths.traefik);
        let middleware_path = format!("{}/middlewares.yml", self.config.paths.traefik_dynamic);
        let static_config = super::traefik::static_config(&self.config);
        let middleware_config = super::traefik::default_middlewares();

        steps.extend(sh!(
            info!("Writing Traefik configuration");
            if os.dir(static_path.as_str()).exists() {
                echo("Removing directory created at Traefik config file path")
                    .stderr(crate::utils::exec::script::dsl::OutputTarget::StandardError);
                os.dir(static_path.as_str()).delete();
            }
            if !os.file(static_path.as_str()).exists() {
                os.file(static_path.as_str()).write(static_config);
                os.file(static_path.as_str()).chmod(crate::utils::os::file::FileMode::OwnerReadWrite);
            }
            if os.dir(middleware_path.as_str()).exists() {
                echo("Removing directory created at Traefik middleware file path")
                    .stderr(crate::utils::exec::script::dsl::OutputTarget::StandardError);
                os.dir(middleware_path.as_str()).delete();
            }
            if !os.file(middleware_path.as_str()).exists() {
                os.file(middleware_path.as_str()).write(middleware_config);
                os.file(middleware_path.as_str()).chmod(crate::utils::os::file::FileMode::OwnerReadWrite);
            }
        ));
    }

    fn append_traefik_step(&self, steps: &mut Vec<ShellIR>, docker: &DockerCli) {
        let name = self.config.traefik_name.as_str();
        steps.extend(sh!(
            info!("Checking Traefik container");
            if docker
                .containers()
                .inspect_cmd(name)
                .stdout(crate::utils::exec::script::dsl::OutputTarget::Null)
                .stderr(crate::utils::exec::script::dsl::OutputTarget::Null)
            {
                info!("Starting existing Traefik container");
                if !docker.container(name).start() {

                    info!("Existing Traefik container could not start — recreating it");
                    docker.container(name).remove().force();
                }
            }
        ));
        self.append_traefik_create_if_absent(steps, docker);
    }

    fn append_traefik_create_if_absent(&self, steps: &mut Vec<ShellIR>, docker: &DockerCli) {
        let mut create = Vec::new();
        self.append_traefik_create_block(&mut create, docker);
        steps.push(guarded_by_absent_container(
            docker,
            self.config.traefik_name.as_str(),
            create,
        ));
    }

    fn append_traefik_create_block(&self, steps: &mut Vec<ShellIR>, docker: &DockerCli) {
        let name = self.config.traefik_name.as_str();
        let image = format!("traefik:v{}", self.config.traefik_version);
        let static_mount = Mount::bind_ro(
            format!("{}/traefik.yml", self.config.paths.traefik),
            "/etc/traefik/traefik.yml",
        );
        let dynamic_mount = Mount::bind(
            &self.config.paths.traefik_dynamic,
            "/etc/openoxide/traefik/dynamic",
        );
        let docker_socket_mount = Mount::bind_ro("/var/run/docker.sock", "/var/run/docker.sock");
        steps.extend(sh!(
            if docker
                .services()
                .inspect_cmd(name)
                .stdout(crate::utils::exec::script::dsl::OutputTarget::Null)
                .stderr(crate::utils::exec::script::dsl::OutputTarget::Null)
            {
                info!("Removing existing Traefik service");
                docker.services().remove(name);
            }
            info!("Pulling Traefik image");
            docker.images().pull(image.as_str());
            info!("Creating Traefik container");
            docker
                .containers()
                .create(image.as_str())
                .detach()
                .name(name)
                .restart(RestartPolicy::Always)
                .network(self.config.network_name.as_str())
                .mount(static_mount)
                .mount(dynamic_mount)
                .mount(docker_socket_mount)
                .publish(Port::tcp(self.config.http_port, self.config.http_port))
                .publish(Port::tcp(self.config.https_port, self.config.https_port))
                .publish(Port::udp(self.config.http3_port, self.config.http3_port))
                .publish(Port::tcp(self.config.dashboard_port, 8080));
        ));
    }

    fn append_agent_step(&self, steps: &mut Vec<ShellIR>, docker: &DockerCli) {
        let name = AGENT_CONTAINER_NAME;

        steps.extend(sh!(
            info!("Checking OpenOxide agent container");
            if docker
                .containers()
                .inspect_cmd(name)
                .stdout(crate::utils::exec::script::dsl::OutputTarget::Null)
                .stderr(crate::utils::exec::script::dsl::OutputTarget::Null)
            {
                info!("Starting existing OpenOxide agent container");
                if !docker.container(name).start() {

                    info!("Existing OpenOxide agent container could not start — recreating it");
                    docker.container(name).remove().force();
                }
            }
        ));
        let mut create = Vec::new();
        self.append_agent_create_block(&mut create, docker);
        steps.push(guarded_by_absent_container(
            docker,
            AGENT_CONTAINER_NAME,
            create,
        ));
    }

    fn append_agent_create_block(&self, steps: &mut Vec<ShellIR>, docker: &DockerCli) {
        let name = AGENT_CONTAINER_NAME;
        let image = agent_image();

        let mtls_dir = format!("{}/mtls", self.config.paths.agent);
        let mtls = match (
            self.config.agent_mtls_ca_cert.as_deref(),
            self.config.agent_mtls_cert.as_deref(),
            self.config.agent_mtls_key.as_deref(),
        ) {
            (Some(ca), Some(cert), Some(key)) => {
                let os = OsCli::new(&self.executor);
                steps.extend(sh!(
                    info!("Writing agent mTLS material");
                    os.dir(mtls_dir.as_str()).create().parents(true);
                ));
                for (fname, contents, mode) in [
                    ("ca.crt", ca, FileMode::OwnerReadWriteGroupReadWorldRead),
                    (
                        "agent.crt",
                        cert,
                        FileMode::OwnerReadWriteGroupReadWorldRead,
                    ),
                    ("agent.key", key, FileMode::OwnerReadWrite),
                ] {
                    let path = format!("{mtls_dir}/{fname}");
                    steps.extend(sh!(
                        os.file(path.as_str()).write(contents);
                        os.file(path.as_str()).chmod(mode);
                    ));
                }
                true
            }
            _ => false,
        };
        let containers = docker.containers();
        let mut create_cmd = containers
            .create(image)
            .detach()
            .name(name)
            .init()
            .restart(RestartPolicy::Always)
            .network(self.config.network_name.as_str())
            .mount(Mount::bind("/var/run/docker.sock", "/var/run/docker.sock"))
            .mount(Mount::bind("/sys/fs/cgroup", "/sys/fs/cgroup"))
            .mount(Mount::bind_ro("/", "/host"))
            .cap_add("SYS_ADMIN")
            .cap_add("SYS_PTRACE")
            .mount(Mount::bind(
                self.config.paths.agent.as_str(),
                "/app/workspace",
            ))
            .mount(Mount::bind(
                self.config.paths.traefik.as_str(),
                "/etc/openoxide/traefik",
            ))
            .mount(Mount::bind(
                self.config.paths.applications.as_str(),
                self.config.paths.applications.as_str(),
            ))
            .mount(Mount::bind(
                self.config.paths.compose.as_str(),
                self.config.paths.compose.as_str(),
            ))
            .env(
                "SERVER_ID",
                self.config.agent_server_id.unwrap_or(0).to_string(),
            )
            .env(
                "OPENOXIDE_SERVER_URL",
                self.config.agent_panel_url.clone().unwrap_or_default(),
            )
            .env(
                "METRICS_TOKEN",
                self.config.agent_token.clone().unwrap_or_default(),
            )
            .env("OPENOXIDE_BASE_PATH", self.config.paths.base.as_str())
            .env("MONITOR_DATABASE_URL", "memory://metrics")
            .env("REFRESH_RATE", agent_refresh_rate())
            .env(
                "RETENTION_DAYS",
                self.config.agent_retention_days.to_string(),
            );
        if mtls {
            create_cmd = create_cmd
                .mount(Mount::bind_ro(mtls_dir.as_str(), "/etc/openoxide/mtls"))
                .env("AGENT_TLS_CERT", "/etc/openoxide/mtls/agent.crt")
                .env("AGENT_TLS_KEY", "/etc/openoxide/mtls/agent.key")
                .env("AGENT_TLS_CLIENT_CA", "/etc/openoxide/mtls/ca.crt");
        }
        let create_cmd = create_cmd.publish(Port::tcp(50051, 50051));
        steps.extend(sh!(
            let _openoxide_arch = capture_stdout! {
                cmd("uname", "-m");
            };
            if cmd("test", _openoxide_arch, "=", "aarch64") {
                echo("Skipping OpenOxide agent on ARM64; image has no arm64 manifest")
                    .stderr(crate::utils::exec::script::dsl::OutputTarget::StandardError);
            } else if cmd("test", _openoxide_arch, "=", "arm64") {
                echo("Skipping OpenOxide agent on ARM64; image has no arm64 manifest")
                    .stderr(crate::utils::exec::script::dsl::OutputTarget::StandardError);
            } else {
                info!("Pulling OpenOxide agent image");
                docker.images().pull(image);
                info!("Creating OpenOxide agent container");
                create_cmd;
            }
        ));
    }
}

pub(crate) const AGENT_CONTAINER_NAME: &str = "openoxide-agent";

fn guarded_by_absent_container(docker: &DockerCli, name: &str, body: Vec<ShellIR>) -> ShellIR {
    use crate::utils::exec::script::dsl::{IntoShellIRGeneric, ShellIRWrapper};

    let inspect = ShellIRWrapper(docker.containers().inspect_cmd(name))
        .into_shell_ir()
        .stdout(crate::utils::exec::script::dsl::OutputTarget::Null)
        .stderr(crate::utils::exec::script::dsl::OutputTarget::Null);
    ShellIR::If {
        cond: Box::new(inspect.failure()),
        then_branch: body,
        else_branch: None,
    }
}

pub(crate) fn agent_image() -> &'static str {
    "dubeyanand/openoxide-agent:latest"
}

/// Whether a running agent container can serve the direct host shell: the
/// read-only host-root bind whose `/proc/1/ns/*` the namespaces are opened
/// through, plus CAP_SYS_PTRACE to read those symlinks and CAP_SYS_ADMIN for
/// the `setns` itself. Agents created before any of that are recreated rather
/// than left answering every shell open with `failed_precondition`.
pub(crate) fn agent_supports_direct_host_shell(
    mounts: &[crate::utils::docker::core::types::ContainerInspectMount],
    host_config: &serde_json::Value,
) -> bool {
    let host_bind = mounts
        .iter()
        .any(|mount| mount.destination == "/host" && !mount.rw)
        || string_array(host_config, "Binds")
            .iter()
            .any(|bind| bind.ends_with(":/host:ro"));
    host_shell_caps(&string_array(host_config, "CapAdd")) && host_bind
}

/// Docker reports `--cap-add SYS_ADMIN` back as `CAP_SYS_ADMIN`, so accept
/// either spelling of each required capability.
fn host_shell_caps(caps: &[String]) -> bool {
    ["SYS_ADMIN", "SYS_PTRACE"].iter().all(|needed| {
        caps.iter()
            .any(|cap| cap.strip_prefix("CAP_") == Some(*needed))
    })
}

fn string_array(value: &serde_json::Value, field: &str) -> Vec<String> {
    value
        .get(field)
        .and_then(|arr| arr.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| item.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

/// Metric collection interval for the agent. Metrics are sampled live per
/// request (no history stored), so this only affects the shared stats feed
/// cadence. Default 2s for live-feeling stats; override with
/// `AGENT_REFRESH_RATE`.
fn agent_refresh_rate() -> String {
    std::env::var("AGENT_REFRESH_RATE")
        .ok()
        .and_then(|v| v.trim().parse::<u32>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(2)
        .to_string()
}

fn agent_image_unsupported(architecture: &str) -> bool {
    matches!(architecture, "aarch64" | "arm64")
}

fn setup_steps(install_dependencies: bool, build_server: bool) -> Vec<SetupStep> {
    let mut completed = Vec::new();
    if install_dependencies {
        completed.push(SetupStep::Dependencies);
        if build_server {
            completed.push(SetupStep::BuildTools);
        }
    }
    completed.push(SetupStep::Directories);
    if build_server {
        return completed;
    }
    completed.extend([
        SetupStep::Swarm,
        SetupStep::Network,
        SetupStep::TraefikConfig,
        SetupStep::Traefik,
        SetupStep::Agent,
    ]);
    completed
}

async fn is_valid_yaml_file(os: &crate::utils::os::OsCli<'_>, path: &str) -> bool {
    let Ok(output) = os.file(path).read().execute().await else {
        return false;
    };
    let text = output.stdout;
    if text.trim().is_empty() {
        return false;
    }
    matches!(
        serde_yaml::from_str::<serde_yaml::Value>(&text),
        Ok(serde_yaml::Value::Mapping(_))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oneshot_script_distributes_mtls_when_material_present() {
        let mut config = SetupConfig::default();
        config.agent_server_id = Some(7);
        config.agent_panel_url = Some("http://panel:4000".into());
        config.agent_token = Some("tok".into());
        config.agent_mtls_ca_cert = Some("CA-PEM".into());
        config.agent_mtls_cert = Some("AGENT-PEM".into());
        config.agent_mtls_key = Some("KEY-PEM".into());
        let script = ServerSetup::new_local(config).compile_oneshot_script(false);

        assert!(script.contains("/mtls"));
        assert!(script.contains("ca.crt"));
        assert!(script.contains("agent.key"));
        assert!(script.contains("AGENT_TLS_CERT"));
        assert!(script.contains("AGENT_TLS_CLIENT_CA"));
        assert!(script.contains("/etc/openoxide/mtls"));
    }

    #[test]
    fn oneshot_script_skips_mtls_when_material_absent() {
        let mut config = SetupConfig::default();
        config.agent_server_id = Some(7);
        let script = ServerSetup::new_local(config).compile_oneshot_script(false);
        assert!(!script.contains("AGENT_TLS_CERT"));
    }

    #[test]
    fn agent_create_grants_host_shell_prerequisites() {
        // The direct host shell setns'es into /host/proc/1/ns/* — it needs the
        // host-root bind mount, CAP_SYS_ADMIN for the switch and CAP_SYS_PTRACE
        // to read the namespace symlinks, on every provisioning path.
        let setup = ServerSetup::new_local(SetupConfig::default());
        let script = setup.compile_oneshot_script(false);
        // Every argv token is shell-quoted by the builder, so match the
        // rendered pair rather than a raw `--cap-add SYS_ADMIN`.
        assert!(script.contains("'--cap-add' 'SYS_ADMIN'"), "{script}");
        assert!(script.contains("'--cap-add' 'SYS_PTRACE'"), "{script}");
        assert!(script.contains("/:/host:ro"), "{script}");
    }

    fn host_mount(rw: bool) -> crate::utils::docker::core::types::ContainerInspectMount {
        crate::utils::docker::core::types::ContainerInspectMount {
            destination: "/host".into(),
            source: "/".into(),
            rw,
            ..Default::default()
        }
    }

    #[test]
    fn running_agent_with_caps_and_host_bind_is_left_alone() {
        // A real agent container: Docker reports the added capabilities with
        // their CAP_ prefix, so the predicate has to accept that form.
        let host_config =
            serde_json::json!({ "CapAdd": ["CAP_SYS_ADMIN", "CAP_SYS_PTRACE", "CAP_NET_ADMIN"] });
        assert!(agent_supports_direct_host_shell(
            &[host_mount(false)],
            &host_config
        ));
    }

    #[test]
    fn stale_agent_container_is_recreated_for_the_host_shell() {
        // Deployed before the direct shell: missing capabilities, missing host
        // bind, and a writable host bind is not enough either.
        let no_caps = serde_json::json!({ "CapAdd": ["NET_ADMIN"] });
        assert!(!agent_supports_direct_host_shell(
            &[host_mount(false)],
            &no_caps
        ));

        // SYS_ADMIN alone was the first guess; the namespace symlinks are
        // ptrace-gated, so an agent without SYS_PTRACE still cannot open them.
        let one_cap = serde_json::json!({ "CapAdd": ["CAP_SYS_ADMIN"] });
        assert!(!agent_supports_direct_host_shell(
            &[host_mount(false)],
            &one_cap
        ));

        let granted = serde_json::json!({ "CapAdd": ["SYS_ADMIN", "SYS_PTRACE"] });
        assert!(!agent_supports_direct_host_shell(
            &[],
            &serde_json::json!({})
        ));
        assert!(!agent_supports_direct_host_shell(
            &[host_mount(true)],
            &granted
        ));

        // `-v /:/host:ro` shows up as a HostConfig bind instead of a Mount entry.
        let binds = serde_json::json!({
            "CapAdd": ["CAP_SYS_ADMIN", "CAP_SYS_PTRACE"],
            "Binds": ["/var/run/docker.sock:/var/run/docker.sock", "/:/host:ro"]
        });
        assert!(agent_supports_direct_host_shell(&[], &binds));
    }

    #[test]
    fn oneshot_setup_script_contains_expected_steps() {
        let setup = ServerSetup::new_local(SetupConfig::default());
        let script = setup.compile_oneshot_script(true);

        assert!(script.starts_with("set -e\n"));

        assert!(!script.contains("apt-get 'install' '-y' 'jq'"));
        assert!(!script.contains("apt-get 'install' '-y' 'bash'"));
        assert!(script.contains("echo '[INFO] Checking Docker installation'"));
        assert!(script.contains("echo '[INFO] Checking Traefik container'"));
        assert!(script.contains("Removing directory created at Traefik config file path"));
        assert!(script.contains("https://get.docker.com"));
        assert!(!script.contains("https://nixpacks.com/install.sh"));
        assert!(!script.contains("_openoxide_pack_url="));
        assert!(script.contains(
            "_openoxide_vpn_addr=$(ip '-4' '-o' 'addr' 'show' | grep '-E' '^[0-9]+: (tailscale|wg|zt|wt|nebula|ipsec|ppp|tun|tap)' | grep '-oE' '([0-9]+\\.){3}[0-9]+' | head '-n' '1')"
        ));
        assert!(script.contains("test '-n' \"$_openoxide_vpn_addr\""));
        assert!(script.contains("docker swarm init --advertise-addr $_openoxide_vpn_addr"));
        assert!(script.contains("test '-n' \"$_openoxide_cgnat_addr\""));
        assert!(script.contains("docker swarm init --advertise-addr $_openoxide_cgnat_addr"));
        assert!(script.contains("^100\\.(6[4-9]|[7-9][0-9]|1[01][0-9]|12[0-7])\\."));
        assert!(script.contains("hostname '-I' | awk"));
        assert!(
            script.contains("[ -z \"$_openoxide_advertise_addr\" ] && _openoxide_advertise_addr=")
        );
        assert!(!script.contains("https://ifconfig.io"));
        assert!(script.contains("docker swarm init --advertise-addr $_openoxide_advertise_addr"));
        assert!(script.contains("docker network create --driver overlay --attachable openoxide"));
        assert!(script.contains("traefik:v"));
        assert!(script.contains("dubeyanand/openoxide-agent:latest"));
        assert!(script.contains("uname '-m'"));
        assert!(script.contains("Skipping OpenOxide agent on ARM64; image has no arm64 manifest"));
        assert!(script.contains("Checking OpenOxide agent container"));
        assert!(script.contains("'--name' 'openoxide-agent'"));
    }

    #[test]
    fn agent_image_is_skipped_on_arm64() {
        assert!(agent_image_unsupported("aarch64"));
        assert!(agent_image_unsupported("arm64"));
        assert!(!agent_image_unsupported("x86_64"));
    }

    #[test]
    fn oneshot_setup_script_can_skip_dependency_steps() {
        let setup = ServerSetup::new_local(SetupConfig::default());
        let script = setup.compile_oneshot_script(false);

        assert!(!script.contains("https://get.docker.com"));
        assert!(!script.contains("https://nixpacks.com/install.sh"));
        assert!(script.contains("docker swarm init"));
        assert!(script.contains("docker network create --driver overlay --attachable openoxide"));
    }

    #[test]
    fn oneshot_setup_script_has_valid_shell_syntax() {
        use std::io::Write;
        use std::process::{Command as ProcessCommand, Stdio};

        let setup = ServerSetup::new_local(SetupConfig::default());
        let script = setup.compile_oneshot_script(true);
        let mut child = ProcessCommand::new("sh")
            .arg("-n")
            .stdin(Stdio::piped())
            .spawn()
            .expect("spawn sh -n");

        child
            .stdin
            .as_mut()
            .expect("open sh stdin")
            .write_all(script.as_bytes())
            .expect("write generated script");

        let status = child.wait().expect("wait for sh -n");
        assert!(status.success(), "generated script failed sh -n");
    }

    #[test]
    fn generated_deploy_setup_uses_flat_elif_chains() {
        let setup = ServerSetup::new_local(SetupConfig::default());
        let script = setup.compile_oneshot_script(true);

        println!("\n===== GENERATED DEPLOY SERVER SETUP =====\n{script}");

        assert!(script.contains("elif test \"$_openoxide_arch\" '=' 'arm64'; then"));
        assert!(!script.contains("else\n  if test \"$_openoxide_arch\" '=' 'arm64'; then"));
    }

    #[test]
    fn generated_build_server_setup_stops_before_deploy_only_steps() {
        let mut config = SetupConfig::default();
        config.build_server = true;
        let setup = ServerSetup::new_local(config);
        let script = setup.compile_oneshot_script(true);

        println!("\n===== GENERATED BUILD SERVER SETUP =====\n{script}");

        assert!(script.contains("https://get.docker.com"));

        assert!(!script.contains("https://nixpacks.com/install.sh"));
        assert!(!script.contains("_openoxide_pack_url="));
        assert!(!script.contains("docker swarm init"));
        assert!(!script.contains("docker network create --driver overlay"));
        assert!(!script.contains("traefik:v"));
        assert!(!script.contains("dubeyanand/openoxide-agent:latest"));
    }
}
