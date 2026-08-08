use super::{ServerAudit, SetupConfig, validation};
use crate::utils::{
    builder::packs::{nixpacks::NixpacksCli, paketo::PackCli, railpack::RailpackCli},
    docker::{
        DockerCli,
        core::{Mount, Port},
        handles::containers::RestartPolicy,
    },
    exec::{
        CommandExecutor, ExecResult, ExecStreamEvent, detect_advertise_addr,
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
    Monitoring,
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

    pub async fn install_dependencies(&self) -> ExecResult<()> {
        let os = OsCli::new(&self.executor);
        for (index, package) in [
            "curl",
            "wget",
            "git",
            "git-lfs",
            "jq",
            "openssl",
            "unzip",
            "tar",
            "ca-certificates",
            "bash",
        ]
        .into_iter()
        .enumerate()
        {
            os.package(package)
                .install()
                .update(index == 0)
                .no_cache(true)
                .run()
                .await?;
        }

        if os.has_command("docker").run().await.is_err() {
            os.shell_installer("https://get.docker.com").run().await?;
        }
        if os.has_command("systemctl").run().await.is_ok() {
            os.service("docker").enable().run().await?;
            os.service("docker").start().run().await?;
        }
        Ok(())
    }
    pub async fn setup_directories(&self) -> ExecResult<()> {
        let os = OsCli::new(&self.executor);
        for path in self.config.paths.all() {
            os.dir(path).create().parents(true).run().await?;
        }
        os.file(&self.config.paths.ssh)
            .chmod(crate::utils::os::file::FileMode::OwnerReadWriteExecute)
            .run()
            .await?;
        let acme = format!("{}/acme.json", self.config.paths.traefik_dynamic);
        if os.file(&acme).exists().run().await.is_err() {
            os.file(&acme).write("").execute().await?;
        }
        os.file(&acme)
            .chmod(crate::utils::os::file::FileMode::OwnerReadWrite)
            .run()
            .await?;
        Ok(())
    }
    pub async fn install_build_tools(&self) -> ExecResult<()> {
        let os = OsCli::new(&self.executor);
        if os.has_command("rclone").run().await.is_err() {
            os.shell_installer("https://rclone.org/install.sh")
                .shell("bash")
                .run()
                .await?;
        }

        NixpacksCli::new(&self.executor)
            .if_not_exist_install()
            .await?;
        RailpackCli::new(&self.executor)
            .if_not_exist_install()
            .await?;
        PackCli::new(&self.executor).if_not_exist_install().await?;
        Ok(())
    }
    pub async fn ensure_swarm(&self) -> ExecResult<()> {
        let docker = DockerCli::from_executor(self.executor.clone());
        if docker.swarm().inspect().await.is_ok() {
            return Ok(());
        }
        let advertise = match &self.config.advertise_addr {
            Some(value) => value.clone(),
            None => detect_advertise_addr(&self.executor).await,
        };
        docker
            .swarm()
            .init()
            .advertise_addr(&advertise)
            .listen_addr("0.0.0.0:2377")
            .run()
            .await?;
        Ok(())
    }
    pub async fn ensure_network(&self) -> ExecResult<()> {
        let docker = DockerCli::from_executor(self.executor.clone());
        if docker
            .networks()
            .inspect(&self.config.network_name)
            .await
            .is_ok()
        {
            return Ok(());
        }
        docker
            .networks()
            .create(&self.config.network_name)
            .driver(crate::utils::docker::NetworkDriver::Overlay)
            .attachable()
            .run()
            .await?;
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
        if !overwrite && os.file(path).exists().run().await.is_ok() {
            return Ok(());
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
        if docker.container(name).inspect().await.is_ok() {
            docker.container(name).start().run().await?;
            return Ok(());
        }
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
            "/etc/rustploy/traefik/dynamic",
        );
        let docker_socket_mount = Mount::bind_ro("/var/run/docker.sock", "/var/run/docker.sock");

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
        Ok(())
    }

    pub async fn ensure_monitoring(&self) -> ExecResult<()> {
        let docker = DockerCli::from_executor(self.executor.clone());
        let name = "rustploy-monitor";
        if docker.container(name).inspect().await.is_ok() {
            if self.config.monitoring_server_id.is_some() {
                // Recreate so a setup run cannot keep stale SERVER_ID/token env.
                let _ = docker.container(name).remove().force().run().await;
            } else {
                docker.container(name).start().run().await?;
                return Ok(());
            }
        }

        let architecture = OsCli::new(&self.executor)
            .system()
            .arch()
            .run()
            .await
            .map(|output| output.stdout_trimmed().to_owned())
            .unwrap_or_default();
        if monitoring_image_unsupported(&architecture) {
            return Ok(());
        }

        let image = monitoring_image();
        docker.images().pull(image).pull().await?;

        let docker_socket_mount = Mount::bind_ro("/var/run/docker.sock", "/var/run/docker.sock");

        let p_grpc = Port::tcp(50051, 50051);

        let containers = docker.containers();
        let mut create = containers
            .create(image)
            .detach()
            .name(name)
            .restart(RestartPolicy::Always)
            .mount(docker_socket_mount)
            .publish(p_grpc);
        if let (Some(server_id), Some(panel_url), Some(token)) = (
            self.config.monitoring_server_id,
            self.config.monitoring_panel_url.as_deref(),
            self.config.monitoring_token.as_deref(),
        ) {
            create = create
                .env("SERVER_ID", server_id.to_string())
                .env("RUSTPLOY_SERVER_URL", panel_url)
                .env("METRICS_TOKEN", token)
                .env("MONITOR_DATABASE_URL", "sqlite:///data/monitor.db")
                .env("REFRESH_RATE", "10")
                .env(
                    "RETENTION_DAYS",
                    self.config.monitoring_retention_days.to_string(),
                )
                .mount(Mount::volume("rustploy-monitor-data", "/data"));
        }
        create.run().await?;
        Ok(())
    }

    pub async fn setup_all(&self, install_dependencies: bool) -> ExecResult<SetupOutcome> {
        self.preflight().await?;
        let mut completed = Vec::new();
        if install_dependencies {
            self.install_dependencies().await?;
            completed.push(SetupStep::Dependencies);
            self.install_build_tools().await?;
            completed.push(SetupStep::BuildTools);
        }
        self.setup_directories().await?;
        completed.push(SetupStep::Directories);
        if self.config.build_server {
            let audit = self.audit().await?;
            return Ok(SetupOutcome { completed, audit });
        }
        self.ensure_swarm().await?;
        completed.push(SetupStep::Swarm);
        self.ensure_network().await?;
        completed.push(SetupStep::Network);
        self.write_traefik_config().await?;
        completed.push(SetupStep::TraefikConfig);
        self.ensure_traefik().await?;
        completed.push(SetupStep::Traefik);
        self.ensure_monitoring().await?;
        completed.push(SetupStep::Monitoring);
        let audit = self.audit().await?;
        Ok(SetupOutcome { completed, audit })
    }

    pub fn oneshot_script(&self, install_dependencies: bool) -> ScriptPipeline {
        let os = OsCli::new(&self.executor);
        let docker = DockerCli::from_executor(self.executor.clone());
        let mut steps = Vec::new();

        if install_dependencies {
            self.append_dependency_steps(&mut steps, &os);
            self.append_build_tool_steps(&mut steps, &os);
        }

        self.append_directory_steps(&mut steps, &os);
        if self.config.build_server {
            return ScriptPipeline::new().cmd(steps);
        }
        self.append_swarm_step(&mut steps, &docker);
        self.append_network_step(&mut steps, &docker);
        self.append_traefik_config_steps(&mut steps, &os);
        self.append_traefik_step(&mut steps, &docker);
        self.append_monitoring_step(&mut steps, &docker);

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
            completed: setup_steps(install_dependencies),
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
                            if !line.is_empty() {
                                let formatted_line = format!("[STDERR] {line}");
                                if log_tx.send(formatted_line).await.is_err() {
                                    return;
                                }
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
                let _ = log_tx.send(format!("[STDERR] {line_err}")).await;
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
            completed: setup_steps(install_dependencies),
            audit,
        })
    }

    fn append_dependency_steps(&self, steps: &mut Vec<ShellIR>, os: &OsCli<'_>) {
        steps.extend(sh!(info!("Installing base dependencies");));
        for (index, package) in [
            "curl",
            "wget",
            "git",
            "git-lfs",
            "jq",
            "openssl",
            "unzip",
            "tar",
            "ca-certificates",
            "bash",
        ]
        .into_iter()
        .enumerate()
        {
            steps.extend(sh!(
                os.package(package)
                    .install()
                    .update(index == 0)
                    .no_cache(true);
            ));
        }

        steps.extend(sh!(
            info!("Checking Docker installation");
            if !os.has_command("docker") {
                info!("Installing Docker");
                os.shell_installer("https://get.docker.com");
            }
            if os.has_command("systemctl") {
                info!("Ensuring Docker service is enabled and running");
                os.service("docker").enable();
                os.service("docker").start();
            }
            cmd("docker", "info");
        ));
    }

    fn append_build_tool_steps(&self, steps: &mut Vec<ShellIR>, os: &OsCli<'_>) {
        steps.extend(sh!(
            info!("Checking build tools");
            if !os.has_command("rclone") {
                info!("Installing rclone");
                os.shell_installer("https://rclone.org/install.sh")
                    .shell("bash");
            }
            if !os.has_command("nixpacks") {
                info!("Installing nixpacks");
                os.shell_installer("https://nixpacks.com/install.sh")
                    .shell("bash")
                    .env("NIXPACKS_VERSION", "1.41.0");
                cmd("nixpacks", "--version");
            }
            if !os.has_command("railpack") {
                info!("Installing railpack");
                os.shell_installer("https://railpack.com/install.sh")
                    .shell("bash")
                    .env("RAILPACK_VERSION", "0.15.4");
                cmd("railpack", "--version");
            }
            if !os.has_command("pack") {
                info!("Installing pack CLI");
                os.pack_installer("0.39.1");
                cmd("pack", "--version");
            }
        ));
    }

    fn append_directory_steps(&self, steps: &mut Vec<ShellIR>, os: &OsCli<'_>) {
        steps.extend(sh!(info!("Creating Rustploy directories");));
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
                    let _rustploy_vpn_addr = capture_stdout! {
                        pipe![
                            cmd("ip", "-4", "-o", "addr", "show"),
                            grep!("-E", "^[0-9]+: (tailscale|wg|zt|wt|nebula|ipsec|ppp|tun|tap)"),
                            grep!("-oE", "([0-9]+\\.){3}[0-9]+"),
                            head!("-n", "1")
                        ];
                    };
                    if cmd("test", "-n", _rustploy_vpn_addr) {
                        info!("Using VPN/overlay interface for swarm advertise address");
                        docker
                            .swarm()
                            .init()
                            .advertise_addr(_rustploy_vpn_addr)
                            .listen_addr("0.0.0.0:2377");
                    } else {
                        let _rustploy_cgnat_addr = capture_stdout! {
                            pipe![
                                cmd("ip", "-4", "-o", "addr", "show"),
                                grep!("-oE", "([0-9]+\\.){3}[0-9]+"),
                                grep!("-E", "^100\\.(6[4-9]|[7-9][0-9]|1[01][0-9]|12[0-7])\\."),
                                head!("-n", "1")
                            ];
                        };
                        if cmd("test", "-n", _rustploy_cgnat_addr) {
                            info!("Using CGNAT-range VPN address for swarm advertise address");
                            docker
                                .swarm()
                                .init()
                                .advertise_addr(_rustploy_cgnat_addr)
                                .listen_addr("0.0.0.0:2377");
                        } else {
                            let _rustploy_advertise_addr = capture_stdout! {
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
                                .advertise_addr(_rustploy_advertise_addr)
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
            info!("Checking Rustploy Docker network");
            if !docker
                .networks()
                .inspect_cmd(network_name)
                .stdout(crate::utils::exec::script::dsl::OutputTarget::Null)
                .stderr(crate::utils::exec::script::dsl::OutputTarget::Null)
            {
                info!("Creating Rustploy Docker network");
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
        let image = format!("traefik:v{}", self.config.traefik_version);
        let static_mount = Mount::bind_ro(
            format!("{}/traefik.yml", self.config.paths.traefik),
            "/etc/traefik/traefik.yml",
        );
        let dynamic_mount = Mount::bind(
            &self.config.paths.traefik_dynamic,
            "/etc/rustploy/traefik/dynamic",
        );
        let docker_socket_mount = Mount::bind_ro("/var/run/docker.sock", "/var/run/docker.sock");
        steps.extend(sh!(
            info!("Checking Traefik container");
            if docker
                .containers()
                .inspect_cmd(name)
                .stdout(crate::utils::exec::script::dsl::OutputTarget::Null)
                .stderr(crate::utils::exec::script::dsl::OutputTarget::Null)
            {
                info!("Starting existing Traefik container");
                docker.container(name).start();
            } else {
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
            }
        ));
    }

    fn append_monitoring_step(&self, steps: &mut Vec<ShellIR>, docker: &DockerCli) {
        let name = "rustploy-monitor";
        let image = monitoring_image();

        steps.extend(sh!(
            info!("Checking Rustploy monitor container");
            if docker
                .containers()
                .inspect_cmd(name)
                .stdout(crate::utils::exec::script::dsl::OutputTarget::Null)
                .stderr(crate::utils::exec::script::dsl::OutputTarget::Null)
            {
                info!("Starting existing Rustploy monitor container");
                docker.container(name).start();
            } else {
                let _rustploy_arch = capture_stdout! {
                    cmd("uname", "-m");
                };
                if cmd("test", _rustploy_arch, "=", "aarch64") {
                    echo("Skipping rustploy monitor on ARM64; image has no arm64 manifest")
                        .stderr(crate::utils::exec::script::dsl::OutputTarget::StandardError);
                } else if cmd("test", _rustploy_arch, "=", "arm64") {
                    echo("Skipping rustploy monitor on ARM64; image has no arm64 manifest")
                        .stderr(crate::utils::exec::script::dsl::OutputTarget::StandardError);
                } else {
                    info!("Pulling Rustploy monitor image");
                    docker.images().pull(image);
                    info!("Creating Rustploy monitor container");
                    docker
                        .containers()
                        .create(image)
                        .detach()
                        .name(name)
                        .restart(RestartPolicy::Always)
                        .mount(Mount::bind_ro(
                            "/var/run/docker.sock",
                            "/var/run/docker.sock",
                        ))
                        .env("SERVER_ID", self.config.monitoring_server_id.unwrap_or(0).to_string())
                        .env("RUSTPLOY_SERVER_URL", self.config.monitoring_panel_url.clone().unwrap_or_default())
                        .env("METRICS_TOKEN", self.config.monitoring_token.clone().unwrap_or_default())
                        .env("MONITOR_DATABASE_URL", "sqlite:///app/data/monitor.db")
                        .env("REFRESH_RATE", "10")
                        .env("RETENTION_DAYS", self.config.monitoring_retention_days.to_string())
                        .mount(Mount::volume("rustploy-monitor-data", "/app/data"))
                        .publish(Port::tcp(50051, 50051));
                }
            }
        ));
    }
}

fn monitoring_image() -> &'static str {
    "dubeyanand/rustploy-monitor:latest"
}

fn monitoring_image_unsupported(architecture: &str) -> bool {
    matches!(architecture, "aarch64" | "arm64")
}

fn setup_steps(install_dependencies: bool) -> Vec<SetupStep> {
    let mut completed = Vec::new();
    if install_dependencies {
        completed.push(SetupStep::Dependencies);
        completed.push(SetupStep::BuildTools);
    }
    completed.extend([
        SetupStep::Directories,
        SetupStep::Swarm,
        SetupStep::Network,
        SetupStep::TraefikConfig,
        SetupStep::Traefik,
        SetupStep::Monitoring,
    ]);
    completed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oneshot_setup_script_contains_expected_steps() {
        let setup = ServerSetup::new_local(SetupConfig::default());
        let script = setup.compile_oneshot_script(true);

        assert!(script.starts_with("set -e\n"));
        assert!(script.contains("apt-get 'install' '-y' 'jq'"));
        assert!(script.contains("apt-get 'install' '-y' 'bash'"));
        assert!(script.contains("echo '[INFO] Checking Docker installation'"));
        assert!(script.contains("echo '[INFO] Checking Traefik container'"));
        assert!(script.contains("Removing directory created at Traefik config file path"));
        assert!(script.contains("https://get.docker.com"));
        assert!(script.contains("env 'bash' \"$_rustploy_installer\""));
        assert!(script.contains("https://nixpacks.com/install.sh"));
        assert!(script.contains("'NIXPACKS_VERSION=1.41.0' 'bash' \"$_rustploy_installer\""));
        assert!(script.contains("_rustploy_pack_url="));
        assert!(script.contains(
            "_rustploy_vpn_addr=$(ip '-4' '-o' 'addr' 'show' | grep '-E' '^[0-9]+: (tailscale|wg|zt|wt|nebula|ipsec|ppp|tun|tap)' | grep '-oE' '([0-9]+\\.){3}[0-9]+' | head '-n' '1')"
        ));
        assert!(script.contains("test '-n' \"$_rustploy_vpn_addr\""));
        assert!(script.contains("docker swarm init --advertise-addr $_rustploy_vpn_addr"));
        assert!(script.contains("test '-n' \"$_rustploy_cgnat_addr\""));
        assert!(script.contains("docker swarm init --advertise-addr $_rustploy_cgnat_addr"));
        assert!(script.contains("^100\\.(6[4-9]|[7-9][0-9]|1[01][0-9]|12[0-7])\\."));
        assert!(script.contains("hostname '-I' | awk"));
        assert!(
            script.contains("[ -z \"$_rustploy_advertise_addr\" ] && _rustploy_advertise_addr=")
        );
        assert!(!script.contains("https://ifconfig.io"));
        assert!(script.contains("docker swarm init --advertise-addr $_rustploy_advertise_addr"));
        assert!(script.contains("docker network create --driver overlay --attachable rustploy"));
        assert!(script.contains("traefik:v"));
        assert!(script.contains("dubeyanand/rustploy-monitor:latest"));
        assert!(script.contains("uname '-m'"));
        assert!(script.contains("Skipping rustploy monitor on ARM64; image has no arm64 manifest"));
    }

    #[test]
    fn monitoring_image_is_skipped_on_arm64() {
        assert!(monitoring_image_unsupported("aarch64"));
        assert!(monitoring_image_unsupported("arm64"));
        assert!(!monitoring_image_unsupported("x86_64"));
    }

    #[test]
    fn oneshot_setup_script_can_skip_dependency_steps() {
        let setup = ServerSetup::new_local(SetupConfig::default());
        let script = setup.compile_oneshot_script(false);

        assert!(!script.contains("https://get.docker.com"));
        assert!(!script.contains("https://nixpacks.com/install.sh"));
        assert!(script.contains("docker swarm init"));
        assert!(script.contains("docker network create --driver overlay --attachable rustploy"));
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
}
