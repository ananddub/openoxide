use crate::docker::{
    DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent, core::ArgBuilder,
};
use tokio::sync::mpsc;

pub struct ExecBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) id: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ExecBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self {
            cli,
            id: id.into(),
            args: ArgBuilder::default(),
        }
    }
    pub fn user(mut self, v: impl Into<String>) -> Self {
        self.args.pair("-u", v.into());
        self
    }
    pub fn workdir(mut self, v: impl Into<String>) -> Self {
        self.args.pair("-w", v.into());
        self
    }
    pub fn env(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args.env_var(k, v);
        self
    }
    pub fn tty(mut self, enabled: bool) -> Self {
        self.args.flag_if("-t", enabled);
        self
    }
    pub fn interactive(mut self) -> Self {
        self.args.flag("-i");
        self
    }
    pub fn detach(mut self) -> Self {
        self.args.flag("-d");
        self
    }

    pub fn print(&self, cmd: &[impl AsRef<str>]) -> String {
        let mut a = ArgBuilder::cmd(&["container", "exec"]);
        a.push_all(self.args.clone().build());
        a.push(&self.id);
        a.push_all(cmd.iter().map(|s| s.as_ref().to_string()));
        a.preview()
    }

    pub async fn run(
        self,
        cmd: impl IntoIterator<Item = impl Into<String>>,
    ) -> DockerResult<DockerOutput> {
        let mut a = ArgBuilder::cmd(&["container", "exec"]);
        a.inherit_meta(&self.args);
        a.push_all(self.args.build());
        a.push(&self.id);
        a.push_all(cmd.into_iter().map(Into::into));
        self.cli.execute(&a).await
    }

    pub async fn run_stream(
        self,
        cmd: impl IntoIterator<Item = impl Into<String>>,
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let mut a = ArgBuilder::cmd(&["container", "exec"]);
        a.inherit_meta(&self.args);
        a.push_all(self.args.build());
        a.push(&self.id);
        a.push_all(cmd.into_iter().map(Into::into));
        self.cli.execute_stream(&a, sender).await
    }

    /// Open an interactive shell directly through Docker Engine's exec API.
    pub async fn shell(
        self,
        cmd: impl IntoIterator<Item = impl Into<String>>,
    ) -> DockerResult<super::socat::ContainerSocketExecStream> {
        if !self.args.api {
            return Err(crate::docker::DockerError::Ssh(
                "ExecBuilder::shell requires .api(); use socket_stream() for the separate raw socket transport".into(),
            ));
        }
        let options = self.args.clone().build();
        let mut builder = super::socat::ContainerSocketStreamBuilder::new(self.cli, self.id)
            .cmd(cmd)
            .interactive(
                options
                    .iter()
                    .any(|arg| arg == "-i" || arg == "--interactive"),
            )
            .tty(options.iter().any(|arg| arg == "-t" || arg == "--tty"));
        for pair in options.windows(2) {
            match pair[0].as_str() {
                "-u" | "--user" => builder = builder.user(pair[1].clone()),
                "-w" | "--workdir" => builder = builder.workdir(pair[1].clone()),
                "--env" | "-e" => {
                    let (key, value) = pair[1].split_once('=').unwrap_or((&pair[1], ""));
                    builder = builder.env(key.to_owned(), value.to_owned());
                }
                _ => {}
            }
        }
        builder.connect().await.map_err(Into::into)
    }

    pub fn build_args(self, cmd: impl IntoIterator<Item = impl Into<String>>) -> Vec<String> {
        let mut a = ArgBuilder::cmd(&["exec"]);
        a.inherit_meta(&self.args);
        a.push_all(self.args.build());
        a.push(&self.id);
        a.push_all(cmd.into_iter().map(Into::into));
        a.build()
    }

    pub fn socket_stream(self) -> super::socat::ContainerSocketStreamBuilder<'a> {
        super::socat::ContainerSocketStreamBuilder::new(self.cli, self.id)
    }
}
crate::impl_builder_opts!(ExecBuilder);
