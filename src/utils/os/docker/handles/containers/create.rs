use crate::utils::docker::{
    DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent,
    core::{ArgBuilder, Cpu, Memory, Mount, Platform, Port},
    handles::containers::query::RestartPolicy,
};
use tokio::sync::mpsc;

pub struct ContainerCreate<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder, // all options
    pub(crate) image: String,
    pub(crate) command: Vec<String>,
}

impl<'a> ContainerCreate<'a> {
    pub(crate) fn new(cli: &'a DockerCli, image: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::default(),
            image: image.into(),
            command: vec![],
        }
    }

    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.args.pair("--name", v.into());
        self
    }
    pub fn hostname(mut self, v: impl Into<String>) -> Self {
        self.args.pair("--hostname", v.into());
        self
    }
    pub fn workdir(mut self, v: impl Into<String>) -> Self {
        self.args.pair("--workdir", v.into());
        self
    }
    pub fn user(mut self, v: impl Into<String>) -> Self {
        self.args.pair("--user", v.into());
        self
    }
    pub fn network(mut self, v: impl Into<String>) -> Self {
        self.args.pair("--network", v.into());
        self
    }

    pub fn env(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args.env_var(k, v);
        self
    }
    pub fn envs(
        mut self,
        iter: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
    ) -> Self {
        for (k, v) in iter {
            self.args.env_var(k, v);
        }
        self
    }

    pub fn label(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args.label(k, v);
        self
    }
    pub fn labels(
        mut self,
        iter: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
    ) -> Self {
        for (k, v) in iter {
            self.args.label(k, v);
        }
        self
    }

    pub fn publish(mut self, port: Port) -> Self {
        self.args.pair("--publish", port.to_string());
        self
    }

    pub fn mount(mut self, m: Mount) -> Self {
        self.args.pair(m.flag_name(), m.to_string());
        self
    }

    pub fn memory(mut self, m: Memory) -> Self {
        self.args.pair("--memory", m.to_string());
        self
    }
    pub fn cpus(mut self, c: Cpu) -> Self {
        self.args.pair("--cpus", c.to_string());
        self
    }
    pub fn restart(mut self, p: RestartPolicy) -> Self {
        self.args.pair("--restart", p.to_string());
        self
    }
    pub fn platform(mut self, p: Platform) -> Self {
        self.args.pair("--platform", p.to_string());
        self
    }

    pub fn entrypoint(mut self, ep: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.args.pair(
            "--entrypoint",
            ep.into_iter().map(Into::into).collect::<Vec<_>>().join(" "),
        );
        self
    }
    pub fn command(mut self, cmd: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.command = cmd.into_iter().map(Into::into).collect();
        self
    }

    /// Allocate a pseudo-TTY. Pass `std::io::IsTerminal::is_terminal(&std::io::stdin())`.
    pub fn tty(mut self, enabled: bool) -> Self {
        self.args.flag_if("--tty", enabled);
        self
    }
    pub fn interactive(mut self) -> Self {
        self.args.flag("--interactive");
        self
    }
    pub fn detach(mut self) -> Self {
        self.args.flag("--detach");
        self
    }
    pub fn rm(mut self) -> Self {
        self.args.flag("--rm");
        self
    }
    pub fn privileged(mut self) -> Self {
        self.args.flag("--privileged");
        self
    }
    pub fn cap_add(mut self, cap: impl Into<String>) -> Self {
        self.args.pair("--cap-add", cap.into());
        self
    }
    pub fn cap_drop(mut self, cap: impl Into<String>) -> Self {
        self.args.pair("--cap-drop", cap.into());
        self
    }
    pub fn add_host(mut self, host_ip: impl Into<String>) -> Self {
        self.args.pair("--add-host", host_ip.into());
        self
    }
    pub fn dns(mut self, dns: impl Into<String>) -> Self {
        self.args.pair("--dns", dns.into());
        self
    }
    pub fn shm_size(mut self, size: impl Into<String>) -> Self {
        self.args.pair("--shm-size", size.into());
        self
    }
    pub fn security_opt(mut self, opt: impl Into<String>) -> Self {
        self.args.pair("--security-opt", opt.into());
        self
    }
    pub fn init(mut self) -> Self {
        self.args.flag("--init");
        self
    }
    pub fn device(mut self, dev: impl Into<String>) -> Self {
        self.args.pair("--device", dev.into());
        self
    }
    pub fn network_alias(mut self, alias: impl Into<String>) -> Self {
        self.args.pair("--network-alias", alias.into());
        self
    }
    /// Pass any raw flag not covered by the builder.

    fn finalize(mut self, subcmd: &str) -> (ArgBuilder, &'a DockerCli) {
        let mut full = ArgBuilder::cmd(&["container", subcmd]);
        full.inherit_meta(&self.args);
        full.push_all(self.args.build());
        full.push(&self.image);
        full.push_all(self.command.drain(..));
        (full, self.cli)
    }

    /// Dry-run: print the full `docker container run …` command.
    pub fn print_run(&self) -> String {
        let mut a = ArgBuilder::cmd(&["container", "run"]);
        let opts = self.args.clone().build();
        a.push_all(opts);
        a.push(&self.image);
        a.push_all(self.command.clone());
        a.preview()
    }

    /// `docker container create …` — returns the new container ID.
    pub async fn create(self) -> DockerResult<String> {
        let cli = self.cli;
        let (args, _) = self.finalize("create");
        Ok(cli.execute(&args).await?.stdout.trim().to_string())
    }

    /// `docker container run …`
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let (args, cli) = self.finalize("run");
        cli.execute(&args).await
    }

    /// `docker container run …` — streaming output.
    pub async fn run_stream(
        self,
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let (args, cli) = self.finalize("run");
        cli.execute_stream(&args, sender).await
    }
}
crate::impl_builder_opts!(ContainerCreate);

impl crate::utils::exec::script::IntoCommand for ContainerCreate<'_> {
    fn build_str(&self) -> String {
        self.print_run()
    }
}
