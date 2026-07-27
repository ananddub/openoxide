use super::{ServerAudit, SetupConfig, validation};
use crate::utils::{
    builder::packs::{nixpacks::NixpacksCli, paketo::PackCli, railpack::RailpackCli},
    docker::{
        DockerCli,
        core::{Mount, Port},
        handles::containers::RestartPolicy,
    },
    exec::{
        CommandExecutor, ExecResult,
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
        os.file(&self.config.paths.ssh).chmod("700").run().await?;
        let acme = format!("{}/acme.json", self.config.paths.traefik_dynamic);
        if os.file(&acme).exists().run().await.is_err() {
            os.file(&acme).write("").execute().await?;
        }
        os.file(&acme).chmod("600").run().await?;
        Ok(())
    }
    pub async fn install_build_tools(&self) -> ExecResult<()> {
        let os = OsCli::new(&self.executor);
        if os.has_command("rclone").run().await.is_err() {
            os.shell_installer("https://rclone.org/install.sh")
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
            None => self
                .executor
                .run("hostname", ["-I"])
                .await?
                .stdout
                .split_whitespace()
                .find(|ip| *ip != "127.0.0.1")
                .unwrap_or("127.0.0.1")
                .to_owned(),
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
        if !overwrite && os.file(path).exists().run().await.is_ok() {
            return Ok(());
        }
        self.executor
            .run_with_stdin("tee", [path], contents)
            .await?;
        os.file(path).chmod("600").run().await?;
        Ok(())
    }
    pub async fn ensure_traefik(&self) -> ExecResult<()> {
        let docker = DockerCli::from_executor(self.executor.clone());
        let name = self.config.traefik_name.as_str();
        if docker.containers().inspect(name).await.is_ok() {
            docker.containers().start(name).run().await?;
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
        if docker.containers().inspect(name).await.is_ok() {
            docker.containers().start(name).run().await?;
            return Ok(());
        }

        let image = "dubeyanand/rustploy-monitor:latest";
        docker.images().pull(image).pull().await?;

        let docker_socket_mount = Mount::bind_ro("/var/run/docker.sock", "/var/run/docker.sock");

        let p_grpc = Port::tcp(50051, 50051);

        docker
            .containers()
            .create(image)
            .detach()
            .name(name)
            .restart(RestartPolicy::Always)
            .mount(docker_socket_mount)
            .publish(p_grpc)
            .run()
            .await?;
        Ok(())
    }

    pub async fn setup_all(&self, install_dependencies: bool) -> ExecResult<SetupOutcome> {
        let mut completed = Vec::new();
        if install_dependencies {
            self.install_dependencies().await?;
            completed.push(SetupStep::Dependencies);
            self.install_build_tools().await?;
            completed.push(SetupStep::BuildTools);
        }
        self.setup_directories().await?;
        completed.push(SetupStep::Directories);
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
        self.oneshot_script(install_dependencies)
            .execute(&self.executor)
            .await?;
        let audit = self.audit().await?;
        Ok(SetupOutcome {
            completed: setup_steps(install_dependencies),
            audit,
        })
    }

    fn append_dependency_steps(&self, steps: &mut Vec<ShellIR>, os: &OsCli<'_>) {
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
            if !os.has_command("docker") {
                os.shell_installer("https://get.docker.com");
            }
            if os.has_command("systemctl") {
                os.service("docker").enable();
                os.service("docker").start();
            }
        ));
    }

    fn append_build_tool_steps(&self, steps: &mut Vec<ShellIR>, os: &OsCli<'_>) {
        steps.extend(sh!(
            if !os.has_command("rclone") {
                os.shell_installer("https://rclone.org/install.sh");
            }
            if !os.has_command("nixpacks") {
                os.shell_installer("https://nixpacks.com/install.sh")
                    .env("NIXPACKS_VERSION", "1.41.0");
                cmd("nixpacks", "--version");
            }
            if !os.has_command("railpack") {
                os.shell_installer("https://railpack.com/install.sh")
                    .env("RAILPACK_VERSION", "0.15.4");
                cmd("railpack", "--version");
            }
            if !os.has_command("pack") {
                os.pack_installer("0.39.1");
                cmd("pack", "--version");
            }
        ));
    }

    fn append_directory_steps(&self, steps: &mut Vec<ShellIR>, os: &OsCli<'_>) {
        for path in self.config.paths.all() {
            steps.extend(sh!(
                os.dir(path).create().parents(true);
            ));
        }
        let acme = format!("{}/acme.json", self.config.paths.traefik_dynamic);
        let ssh_path = self.config.paths.ssh.as_str();
        steps.extend(sh!(
            os.file(ssh_path).chmod("700");
            if !os.file(acme.as_str()).exists() {
                os.file(acme.as_str()).write("");
            }
            os.file(acme.as_str()).chmod("600");
        ));
    }

    fn append_swarm_step(&self, steps: &mut Vec<ShellIR>, docker: &DockerCli) {
        if let Some(advertise_addr) = &self.config.advertise_addr {
            steps.extend(sh!(if !docker.swarm().active() {
                docker
                    .swarm()
                    .init()
                    .advertise_addr(advertise_addr)
                    .listen_addr("0.0.0.0:2377");
            }));
        } else {
            steps.extend(sh!(if !docker.swarm().active() {
                let _rustploy_advertise_addr = capture_stdout! {
                    pipe![
                        cmd("hostname", "-I"),
                        awk(awk_for_fields! {
                            if field != "127.0.0.1" {
                                print(field);
                                exit;
                            }
                        })
                    ];
                }
                .default("127.0.0.1");
                docker
                    .swarm()
                    .init()
                    .advertise_addr(_rustploy_advertise_addr)
                    .listen_addr("0.0.0.0:2377");
            }));
        }
    }

    fn append_network_step(&self, steps: &mut Vec<ShellIR>, docker: &DockerCli) {
        let network_name = self.config.network_name.as_str();

        steps.extend(sh!(if !docker
            .networks()
            .inspect_cmd(network_name)
            .stdout("/dev/null")
            .stderr("/dev/null")
        {
            docker
                .networks()
                .create(network_name)
                .driver(crate::utils::docker::NetworkDriver::Overlay)
                .attachable();
        }));
    }

    fn append_traefik_config_steps(&self, steps: &mut Vec<ShellIR>, os: &OsCli<'_>) {
        let static_path = format!("{}/traefik.yml", self.config.paths.traefik);
        let middleware_path = format!("{}/middlewares.yml", self.config.paths.traefik_dynamic);
        let static_config = super::traefik::static_config(&self.config);
        let middleware_config = super::traefik::default_middlewares();

        steps.extend(sh!(
            if !os.file(static_path.as_str()).exists() {
                os.file(static_path.as_str()).write(static_config);
                os.file(static_path.as_str()).chmod("600");
            }
            if !os.file(middleware_path.as_str()).exists() {
                os.file(middleware_path.as_str()).write(middleware_config);
                os.file(middleware_path.as_str()).chmod("600");
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
        steps.extend(sh!(if docker
            .containers()
            .inspect_cmd(name)
            .stdout("/dev/null")
            .stderr("/dev/null")
        {
            docker.containers().start(name);
        } else {
            if docker
                .services()
                .inspect_cmd(name)
                .stdout("/dev/null")
                .stderr("/dev/null")
            {
                docker.services().remove(name);
            }
            docker.images().pull(image.as_str());
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
        }));
    }

    fn append_monitoring_step(&self, steps: &mut Vec<ShellIR>, docker: &DockerCli) {
        let name = "rustploy-monitor";
        let image = "dubeyanand/rustploy-monitor:latest";

        steps.extend(sh!(if docker
            .containers()
            .inspect_cmd(name)
            .stdout("/dev/null")
            .stderr("/dev/null")
        {
            docker.containers().start(name);
        } else {
            docker.images().pull(image);
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
                .publish(Port::tcp(50051, 50051));
        }));
    }
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
        assert!(script.contains("https://get.docker.com"));
        assert!(script.contains("https://nixpacks.com/install.sh"));
        assert!(script.contains("_rustploy_pack_url="));
        assert!(script.contains("docker swarm init --advertise-addr $_rustploy_advertise_addr"));
        assert!(script.contains("docker network create --driver overlay --attachable rustploy"));
        assert!(script.contains("traefik:v"));
        assert!(script.contains("dubeyanand/rustploy-monitor:latest"));
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
