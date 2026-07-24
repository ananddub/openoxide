use super::query::DockerQuery;
use super::{DockerError, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent};
use crate::utils::exec::{CommandExecutor, LocalExecutor, RemoteExecutor, SshAuth, SshHostKey};
use serde::de::DeserializeOwned;
use std::{ffi::OsStr, path::PathBuf};
use tokio::{process::Command, sync::mpsc};
use tokio_util::sync::CancellationToken;

pub type RemoteDockerConfig = RemoteExecutor;
pub type RemoteHostKey = SshHostKey;

#[derive(Clone, Debug)]
pub struct DockerCli {
    executor: CommandExecutor,
    executable: String,
    global_args: Vec<String>,
}
impl Default for DockerCli {
    fn default() -> Self {
        Self::new_local()
    }
}
impl DockerCli {
    pub fn new_local() -> Self {
        Self {
            executor: CommandExecutor::Local(LocalExecutor::new()),
            executable: "docker".into(),
            global_args: vec![],
        }
    }
    pub fn from_executor(executor: CommandExecutor) -> Self {
        Self {
            executor,
            executable: "docker".into(),
            global_args: vec![],
        }
    }

    pub fn with_executable(executable: impl Into<PathBuf>) -> Self {
        Self {
            executor: CommandExecutor::Local(LocalExecutor::new()),
            executable: executable.into().to_string_lossy().into_owned(),
            global_args: vec![],
        }
    }
    pub fn new_remote(
        host: impl Into<String>,
        port: u16,
        username: impl Into<String>,
        auth: SshAuth,
        host_key: SshHostKey,
    ) -> Self {
        Self::from_remote_executor(RemoteExecutor::new(host, port, username, auth, host_key))
    }
    pub fn from_remote_executor(executor: RemoteExecutor) -> Self {
        Self {
            executor: CommandExecutor::Remote(executor),
            executable: "docker".into(),
            global_args: vec![],
        }
    }
    pub fn with_remote_sudo(mut self) -> Self {
        if let CommandExecutor::Remote(remote) = self.executor {
            self.executor = CommandExecutor::Remote(remote.with_sudo());
        }
        self
    }
    pub fn with_remote_sudo_password(mut self, password: impl Into<String>) -> Self {
        if let CommandExecutor::Remote(remote) = self.executor {
            self.executor = CommandExecutor::Remote(remote.with_sudo_password(password));
        }
        self
    }
    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.global_args.extend(["--host".into(), host.into()]);
        self
    }
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.global_args
            .extend(["--context".into(), context.into()]);
        self
    }
    pub fn command<I, S>(&self, args: I) -> DockerResult<Command>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        match &self.executor {
            CommandExecutor::Local(local) => {
                let arguments = self.arguments(args);
                Ok(local.command(&self.executable, arguments))
            }
            CommandExecutor::Remote(_) => Err(DockerError::Ssh(
                "a local process cannot be created for a remote client; use run_stream".into(),
            )),
        }
    }
    pub async fn run<I, S>(&self, args: I) -> DockerResult<DockerOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let args = self.arguments(args);
        self.executor.run(&self.executable, args).await
    }
    pub async fn run_cancelled<I, S>(
        &self,
        args: I,
        cancel: &CancellationToken,
    ) -> DockerResult<DockerOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let args = self.arguments(args);
        self.executor
            .run_cancelled(&self.executable, args, cancel)
            .await
    }
    pub async fn run_with_stdin<I, S>(
        &self,
        args: I,
        stdin: impl AsRef<[u8]>,
    ) -> DockerResult<DockerOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let args = self.arguments(args);
        self.executor
            .run_with_stdin(&self.executable, args, stdin)
            .await
    }
    pub async fn run_stream<I, S>(
        &self,
        args: I,
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let args = self.arguments(args);
        self.executor
            .run_stream(&self.executable, args, sender)
            .await
    }
    fn arguments<I, S>(&self, args: I) -> Vec<String>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.global_args
            .iter()
            .cloned()
            .chain(
                args.into_iter()
                    .map(|v| v.as_ref().to_string_lossy().into_owned()),
            )
            .collect()
    }
    pub(crate) async fn json<T: DeserializeOwned>(&self, args: &[&str]) -> DockerResult<T> {
        Ok(serde_json::from_str(&self.run(args).await?.stdout)?)
    }
    pub(crate) async fn json_lines<T: DeserializeOwned>(
        &self,
        args: &[&str],
    ) -> DockerResult<Vec<T>> {
        let stdout = self.run(args).await?.stdout;
        let trimmed = stdout.trim();
        if trimmed.is_empty() {
            return Ok(Vec::new());
        }
        if trimmed.starts_with('[') {
            if let Ok(vec) = serde_json::from_str::<Vec<T>>(trimmed) {
                return Ok(vec);
            }
        }
        if trimmed.starts_with('{') {
            if let Ok(item) = serde_json::from_str::<T>(trimmed) {
                return Ok(vec![item]);
            }
        }
        let mut items = Vec::new();
        for line in trimmed.lines().filter(|l| !l.trim().is_empty()) {
            if let Ok(item) = serde_json::from_str::<T>(line) {
                items.push(item);
            }
        }
        Ok(items)
    }
    pub(crate) async fn prefixed(
        &self,
        prefix: &[&str],
        args: &[&str],
    ) -> DockerResult<DockerOutput> {
        let mut command = prefix.to_vec();
        command.extend_from_slice(args);
        tracing::debug!(command = ?command, "running docker command");
        self.run(command).await
    }
    pub(crate) async fn prefixed_cancelled(
        &self,
        prefix: &[&str],
        args: &[&str],
        cancel: &CancellationToken,
    ) -> DockerResult<DockerOutput> {
        let mut command = prefix.to_vec();
        command.extend_from_slice(args);
        self.run_cancelled(command, cancel).await
    }

    pub async fn execute(
        &self,
        builder: &crate::utils::exec::ArgBuilder,
    ) -> DockerResult<DockerOutput> {
        let mut attempts = 0;
        let args = builder.clone().build();
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        tracing::debug!(command = ?refs, "running docker command");
        loop {
            attempts += 1;
            if let Some(cancel) = &builder.cancel_token {
                if cancel.is_cancelled() {
                    return Err(crate::utils::exec::ExecError::StreamCancelled.into());
                }
                match self.run_cancelled(&refs, cancel).await {
                    Ok(out) => return Ok(out),
                    Err(e)
                        if attempts <= builder.retry_limit.unwrap_or(0)
                            && crate::utils::docker::error::is_transient_docker_error(
                                &e.to_string(),
                            ) =>
                    {
                        tokio::time::sleep(tokio::time::Duration::from_secs(2 * attempts as u64))
                            .await;
                        continue;
                    }
                    Err(e) => return Err(e),
                }
            } else {
                match self.run(&refs).await {
                    Ok(out) => return Ok(out),
                    Err(e)
                        if attempts <= builder.retry_limit.unwrap_or(0)
                            && crate::utils::docker::error::is_transient_docker_error(
                                &e.to_string(),
                            ) =>
                    {
                        tokio::time::sleep(tokio::time::Duration::from_secs(2 * attempts as u64))
                            .await;
                        continue;
                    }
                    Err(e) => return Err(e),
                }
            }
        }
    }

    pub async fn execute_stream(
        &self,
        builder: &crate::utils::exec::ArgBuilder,
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let mut attempts = 0;
        let args = builder.clone().build();
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        loop {
            attempts += 1;
            if let Some(cancel) = &builder.cancel_token {
                if cancel.is_cancelled() {
                    return Err(crate::utils::exec::ExecError::StreamCancelled.into());
                }
            }
            match self.run_stream(&refs, sender.clone()).await {
                Ok(out) => return Ok(out),
                Err(e)
                    if attempts <= builder.retry_limit.unwrap_or(0)
                        && crate::utils::docker::error::is_transient_docker_error(
                            &e.to_string(),
                        ) =>
                {
                    tokio::time::sleep(tokio::time::Duration::from_secs(2 * attempts as u64)).await;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
    }

    pub(crate) async fn execute_json<T: DeserializeOwned>(
        &self,
        builder: &crate::utils::exec::ArgBuilder,
    ) -> DockerResult<T> {
        Ok(serde_json::from_str(&self.execute(builder).await?.stdout)?)
    }

    pub(crate) async fn execute_json_lines<T: DeserializeOwned>(
        &self,
        builder: &crate::utils::exec::ArgBuilder,
    ) -> DockerResult<Vec<T>> {
        let stdout = self.execute(builder).await?.stdout;
        let trimmed = stdout.trim();
        if trimmed.is_empty() {
            return Ok(Vec::new());
        }
        if trimmed.starts_with('[') {
            if let Ok(vec) = serde_json::from_str::<Vec<T>>(trimmed) {
                return Ok(vec);
            }
        }
        if trimmed.starts_with('{') {
            if let Ok(item) = serde_json::from_str::<T>(trimmed) {
                return Ok(vec![item]);
            }
        }
        let mut items = Vec::new();
        for line in trimmed.lines().filter(|l| !l.trim().is_empty()) {
            if let Ok(item) = serde_json::from_str::<T>(line) {
                items.push(item);
            }
        }
        Ok(items)
    }

    pub(crate) async fn list<T: DeserializeOwned>(
        &self,
        object: &str,
        filters: &[&str],
    ) -> DockerResult<Vec<T>> {
        let mut args = vec![object, "ls", "--format", "{{json .}}"];
        for filter in filters {
            args.extend(["--filter", filter]);
        }
        self.json_lines(&args).await
    }
    pub(crate) async fn prune(&self, object: &str, filters: &[&str]) -> DockerResult<DockerOutput> {
        let mut args = vec![object, "prune", "--force"];
        for filter in filters {
            args.extend(["--filter", filter]);
        }
        self.run(args).await
    }

    /// Return a typesafe fluent query / command builder backed by this client.
    ///
    /// # Example
    /// ```rust,no_run
    /// use crate::utils::docker::query::filter::{ContainerFilter, ContainerStatus};
    ///
    /// let containers = docker.query()
    ///     .containers()
    ///     .all()
    ///     .filter(ContainerFilter::Status(ContainerStatus::Running))
    ///     .list()
    ///     .await?;
    ///
    /// let id = docker.query()
    ///     .create_container("nginx:latest")
    ///     .name("web")
    ///     .network("bridge")
    ///     .publish(8080, 80)
    ///     .tty(std::io::IsTerminal::is_terminal(&std::io::stdin()))
    ///     .create()
    ///     .await?;
    /// ```
    pub fn query(&self) -> DockerQuery<'_> {
        DockerQuery::new(self)
    }

    pub fn swarm(&self) -> crate::utils::docker::handles::SwarmHandle<'_> {
        crate::utils::docker::handles::SwarmHandle::new(self)
    }

    pub fn nodes(&self) -> crate::utils::docker::handles::NodesHandle<'_> {
        crate::utils::docker::handles::NodesHandle::new(self)
    }

    pub fn stacks(&self) -> crate::utils::docker::handles::StacksHandle<'_> {
        crate::utils::docker::handles::StacksHandle::new(self)
    }

    pub fn secrets(&self) -> crate::utils::docker::handles::SecretsHandle<'_> {
        crate::utils::docker::handles::SecretsHandle::new(self)
    }

    pub fn configs(&self) -> crate::utils::docker::handles::ConfigsHandle<'_> {
        crate::utils::docker::handles::ConfigsHandle::new(self)
    }

    pub fn services(&self) -> crate::utils::docker::handles::ServicesHandle<'_> {
        crate::utils::docker::handles::ServicesHandle::new(self)
    }

    pub fn containers(&self) -> crate::utils::docker::handles::ContainerHandle<'_> {
        crate::utils::docker::handles::ContainerHandle(self)
    }

    pub fn images(&self) -> crate::utils::docker::handles::ImageHandle<'_> {
        crate::utils::docker::handles::ImageHandle(self)
    }

    pub fn networks(&self) -> crate::utils::docker::handles::NetworkHandle<'_> {
        crate::utils::docker::handles::NetworkHandle(self)
    }

    pub fn volumes(&self) -> crate::utils::docker::handles::VolumeHandle<'_> {
        crate::utils::docker::handles::VolumeHandle(self)
    }

    pub fn compose(&self) -> crate::utils::docker::handles::ComposeHandle<'_> {
        crate::utils::docker::handles::ComposeHandle(self)
    }
}
