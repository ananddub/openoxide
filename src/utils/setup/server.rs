use super::{ServerAudit, SetupConfig, validation};
use crate::utils::{
    builder::packs::{nixpacks::NixpacksCli, paketo::PackCli, railpack::RailpackCli},
    docker::{
        DockerCli,
        core::{Mount, Port},
        handles::containers::RestartPolicy,
    },
    exec::{CommandExecutor, ExecResult},
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
            // Docker's installer is intentionally kept as one pipeline because the
            // downloaded script must be passed to a privileged shell over SSH.
            self.executor
                .run("sh", ["-c", "curl -fsSL https://get.docker.com | sh"])
                .await?;
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
            self.executor.run("touch", [acme.as_str()]).await?;
        }
        os.file(&acme).chmod("600").run().await?;
        Ok(())
    }
    pub async fn install_build_tools(&self) -> ExecResult<()> {
        let os = OsCli::new(&self.executor);
        if os.has_command("rclone").run().await.is_err() {
            self.executor
                .run(
                    "sh",
                    ["-c", "curl -fsSL https://rclone.org/install.sh | sh"],
                )
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
        let script = "umask 077; cat > \"$1\"";
        self.executor
            .run_with_stdin("sh", ["-c", script, "rustploy-write", path], contents)
            .await?;
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
}
