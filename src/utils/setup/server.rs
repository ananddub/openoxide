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
        script::{
            IntoCommand, ScriptPipeline,
            dsl::{ArgToken, CaptureSource, Command, Expr, ShellIR, Statement},
        },
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
        let mut script = ScriptPipeline::new();

        if install_dependencies {
            script = self.append_dependency_steps(script, &os);
            script = self.append_build_tool_steps(script, &os);
        }

        script = self.append_directory_steps(script, &os);
        script = self.append_swarm_step(script, &docker);
        script = self.append_network_step(script, &docker);
        script = self.append_traefik_config_steps(script, &os);
        script = self.append_traefik_step(script, &docker);
        self.append_monitoring_step(script, &docker)
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

    fn append_dependency_steps(
        &self,
        mut script: ScriptPipeline,
        os: &OsCli<'_>,
    ) -> ScriptPipeline {
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
            script = script.cmd(
                os.package(package)
                    .install()
                    .update(index == 0)
                    .no_cache(true),
            );
        }

        script = script.if_cmd_succeeds(
            os.has_command("docker"),
            ScriptPipeline::new(),
            Some(ScriptPipeline::new().cmd(os.shell_installer("https://get.docker.com"))),
        );
        script.if_cmd_succeeds(
            os.has_command("systemctl"),
            ScriptPipeline::new()
                .cmd(os.service("docker").enable())
                .cmd(os.service("docker").start()),
            None,
        )
    }

    fn append_build_tool_steps(&self, script: ScriptPipeline, os: &OsCli<'_>) -> ScriptPipeline {
        script
            .if_cmd_succeeds(
                os.has_command("rclone"),
                ScriptPipeline::new(),
                Some(
                    ScriptPipeline::new().cmd(os.shell_installer("https://rclone.org/install.sh")),
                ),
            )
            .if_cmd_succeeds(
                os.has_command("nixpacks"),
                ScriptPipeline::new(),
                Some(
                    ScriptPipeline::new()
                        .cmd(
                            os.shell_installer("https://nixpacks.com/install.sh")
                                .env("NIXPACKS_VERSION", "1.41.0"),
                        )
                        .cmd(command("nixpacks", ["--version"])),
                ),
            )
            .if_cmd_succeeds(
                os.has_command("railpack"),
                ScriptPipeline::new(),
                Some(
                    ScriptPipeline::new()
                        .cmd(
                            os.shell_installer("https://railpack.com/install.sh")
                                .env("RAILPACK_VERSION", "0.15.4"),
                        )
                        .cmd(command("railpack", ["--version"])),
                ),
            )
            .if_cmd_succeeds(
                os.has_command("pack"),
                ScriptPipeline::new(),
                Some(
                    ScriptPipeline::new()
                        .cmd(pack_url_detection())
                        .cmd(
                            os.tarball_installer("$_rustploy_pack_url", "/usr/local/bin")
                                .member("pack"),
                        )
                        .cmd(command("pack", ["--version"])),
                ),
            )
    }

    fn append_directory_steps(&self, mut script: ScriptPipeline, os: &OsCli<'_>) -> ScriptPipeline {
        for path in self.config.paths.all() {
            script = script.cmd(os.dir(path).create().parents(true));
        }
        let acme = format!("{}/acme.json", self.config.paths.traefik_dynamic);
        script
            .cmd(os.file(&self.config.paths.ssh).chmod("700"))
            .if_file_exists(
                &acme,
                ScriptPipeline::new(),
                Some(ScriptPipeline::new().cmd(os.file(&acme).write(""))),
            )
            .cmd(os.file(&acme).chmod("600"))
    }

    fn append_swarm_step(&self, script: ScriptPipeline, docker: &DockerCli) -> ScriptPipeline {
        let init = match &self.config.advertise_addr {
            Some(value) => docker
                .swarm()
                .init()
                .advertise_addr(value)
                .listen_addr("0.0.0.0:2377")
                .build_str(),
            None => swarm_init_with_detected_advertise(),
        };

        script.if_cmd_succeeds(
            format!(
                "{} | {}",
                command("docker", ["info", "--format", "{{.Swarm.LocalNodeState}}"]),
                command("grep", ["-q", "^active$"])
            ),
            ScriptPipeline::new(),
            Some(ScriptPipeline::new().cmd(init)),
        )
    }

    fn append_network_step(&self, script: ScriptPipeline, docker: &DockerCli) -> ScriptPipeline {
        script.if_cmd_succeeds(
            quiet(command(
                "docker",
                ["network", "inspect", self.config.network_name.as_str()],
            )),
            ScriptPipeline::new(),
            Some(
                ScriptPipeline::new().cmd(
                    docker
                        .networks()
                        .create(&self.config.network_name)
                        .driver(crate::utils::docker::NetworkDriver::Overlay)
                        .attachable(),
                ),
            ),
        )
    }

    fn append_traefik_config_steps(
        &self,
        script: ScriptPipeline,
        os: &OsCli<'_>,
    ) -> ScriptPipeline {
        let static_path = format!("{}/traefik.yml", self.config.paths.traefik);
        let middleware_path = format!("{}/middlewares.yml", self.config.paths.traefik_dynamic);
        let static_config = super::traefik::static_config(&self.config);
        let middleware_config = super::traefik::default_middlewares();

        script
            .if_file_exists(
                &static_path,
                ScriptPipeline::new(),
                Some(
                    ScriptPipeline::new()
                        .cmd(os.file(&static_path).write(static_config))
                        .cmd(os.file(&static_path).chmod("600")),
                ),
            )
            .if_file_exists(
                &middleware_path,
                ScriptPipeline::new(),
                Some(
                    ScriptPipeline::new()
                        .cmd(os.file(&middleware_path).write(middleware_config))
                        .cmd(os.file(&middleware_path).chmod("600")),
                ),
            )
    }

    fn append_traefik_step(&self, script: ScriptPipeline, docker: &DockerCli) -> ScriptPipeline {
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
        let containers = docker.containers();
        let create = containers
            .create(&image)
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

        script.if_cmd_succeeds(
            quiet(command("docker", ["container", "inspect", name])),
            ScriptPipeline::new().cmd(docker.containers().start(name)),
            Some(
                ScriptPipeline::new()
                    .if_cmd_succeeds(
                        quiet(command("docker", ["service", "inspect", name])),
                        ScriptPipeline::new().cmd(docker.services().remove(name)),
                        None,
                    )
                    .cmd(docker.images().pull(&image))
                    .cmd(create),
            ),
        )
    }

    fn append_monitoring_step(&self, script: ScriptPipeline, docker: &DockerCli) -> ScriptPipeline {
        let name = "rustploy-monitor";
        let image = "dubeyanand/rustploy-monitor:latest";
        let containers = docker.containers();
        let create = containers
            .create(image)
            .detach()
            .name(name)
            .restart(RestartPolicy::Always)
            .mount(Mount::bind_ro(
                "/var/run/docker.sock",
                "/var/run/docker.sock",
            ))
            .publish(Port::tcp(50051, 50051));

        script.if_cmd_succeeds(
            quiet(command("docker", ["container", "inspect", name])),
            ScriptPipeline::new().cmd(docker.containers().start(name)),
            Some(
                ScriptPipeline::new()
                    .cmd(docker.images().pull(image))
                    .cmd(create),
            ),
        )
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

fn command<const N: usize>(name: &str, args: [&str; N]) -> String {
    ShellIR::Command(Command {
        name: name.to_owned(),
        args: args
            .into_iter()
            .map(|value| ArgToken::Literal(value.to_owned()))
            .collect(),
    })
    .build_str()
}

fn quiet(cmd: String) -> String {
    format!("{cmd} >/dev/null 2>&1")
}

fn pack_url_detection() -> String {
    ShellIR::Sequence(vec![
        ShellIR::Statement(Statement::VarAssign {
            name: "_rustploy_pack_arch".to_owned(),
            val: Box::new(ShellIR::Capture {
                cmd: Box::new(ShellIR::Command(Command {
                    name: "uname".to_owned(),
                    args: vec![ArgToken::Literal("-m".to_owned())],
                })),
                source: CaptureSource::Stdout,
            }),
            default: None,
        }),
        ShellIR::If {
            cond: Box::new(test_var_eq("_rustploy_pack_arch", "aarch64")),
            then_branch: vec![var_literal("_rustploy_pack_suffix", "-arm64")],
            else_branch: Some(vec![ShellIR::If {
                cond: Box::new(test_var_eq("_rustploy_pack_arch", "arm64")),
                then_branch: vec![var_literal("_rustploy_pack_suffix", "-arm64")],
                else_branch: Some(vec![var_literal("_rustploy_pack_suffix", "")]),
            }]),
        },
        ShellIR::Statement(Statement::VarAssign {
            name: "_rustploy_pack_url".to_owned(),
            val: Box::new(ShellIR::Expr(Expr::Word(vec![
                Expr::Literal(
                    "https://github.com/buildpacks/pack/releases/download/v0.39.1/pack-v0.39.1-linux"
                        .to_owned(),
                ),
                Expr::Variable("_rustploy_pack_suffix".to_owned()),
                Expr::Literal(".tgz".to_owned()),
            ]))),
            default: None,
        }),
    ])
    .build_str()
}

fn swarm_init_with_detected_advertise() -> String {
    ShellIR::Sequence(vec![
        ShellIR::Statement(Statement::VarAssign {
            name: "_rustploy_advertise_addr".to_owned(),
            val: Box::new(ShellIR::Capture {
                cmd: Box::new(ShellIR::Pipeline(vec![
                    Command {
                        name: "hostname".to_owned(),
                        args: vec![ArgToken::Literal("-I".to_owned())],
                    },
                    Command {
                        name: "awk".to_owned(),
                        args: vec![ArgToken::Literal(
                            "{ for (i=1; i<=NF; i++) if ($i != \"127.0.0.1\") { print $i; exit } }"
                                .to_owned(),
                        )],
                    },
                ])),
                source: CaptureSource::Stdout,
            }),
            default: None,
        }),
        ShellIR::If {
            cond: Box::new(test_var_eq("_rustploy_advertise_addr", "")),
            then_branch: vec![var_literal("_rustploy_advertise_addr", "127.0.0.1")],
            else_branch: None,
        },
        ShellIR::Command(Command {
            name: "docker".to_owned(),
            args: vec![
                ArgToken::Literal("swarm".to_owned()),
                ArgToken::Literal("init".to_owned()),
                ArgToken::Literal("--advertise-addr".to_owned()),
                ArgToken::Variable("_rustploy_advertise_addr".to_owned()),
                ArgToken::Literal("--listen-addr".to_owned()),
                ArgToken::Literal("0.0.0.0:2377".to_owned()),
            ],
        }),
    ])
    .build_str()
}

fn test_var_eq(name: &str, value: &str) -> ShellIR {
    ShellIR::Command(Command {
        name: "test".to_owned(),
        args: vec![
            ArgToken::Variable(name.to_owned()),
            ArgToken::Literal("=".to_owned()),
            ArgToken::Literal(value.to_owned()),
        ],
    })
}

fn var_literal(name: &str, value: &str) -> ShellIR {
    ShellIR::Statement(Statement::VarAssign {
        name: name.to_owned(),
        val: Box::new(ShellIR::Expr(Expr::Literal(value.to_owned()))),
        default: None,
    })
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
        assert!(
            script
                .contains("docker 'swarm' 'init' '--advertise-addr' \"$_rustploy_advertise_addr\"")
        );
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
        assert!(script.contains("docker 'swarm' 'init'"));
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
