use super::query::DockerQuery;
use super::{DockerError, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent};
use crate::exec::{CommandExecutor, LocalExecutor, RemoteExecutor, SshAuth, SshHostKey};
use serde::de::DeserializeOwned;
use std::{
    ffi::OsStr,
    path::PathBuf,
    sync::{Arc, RwLock},
};
use tokio::{process::Command, sync::mpsc};
use tokio_util::sync::CancellationToken;

pub type RemoteDockerConfig = RemoteExecutor;
pub type RemoteHostKey = SshHostKey;

#[derive(Clone, Debug)]
pub struct DockerCli {
    executor: CommandExecutor,
    executable: String,
    global_args: Vec<String>,
    api_tunnel: super::api::RemoteApiTunnel,
    registry_auth: Arc<RwLock<Option<String>>>,
}
impl Default for DockerCli {
    fn default() -> Self {
        Self::new_local()
    }
}
impl DockerCli {
    pub(crate) fn set_registry_auth(&self, auth: String) {
        if let Ok(mut value) = self.registry_auth.write() {
            *value = Some(auth);
        }
    }
    pub(crate) async fn api_socket_lease(&self) -> DockerResult<Option<super::api::TunnelLease>> {
        match &self.executor {
            crate::exec::CommandExecutor::Remote(remote) => {
                Ok(Some(self.api_tunnel.acquire(remote).await?))
            }
            crate::exec::CommandExecutor::Local(_) => Ok(None),
        }
    }
    pub(crate) async fn api_json_request(
        &self,
        path: &str,
        body: serde_json::Value,
    ) -> DockerResult<DockerOutput> {
        let lease = match &self.executor {
            crate::exec::CommandExecutor::Remote(remote) => {
                Some(self.api_tunnel.acquire(remote).await?)
            }
            crate::exec::CommandExecutor::Local(_) => None,
        };
        let api = super::DockerApi::new(
            lease
                .as_ref()
                .map(|lease| lease.socket())
                .unwrap_or("/var/run/docker.sock"),
        );
        let payload = api
            .request_with_auth(
                "POST",
                path,
                Some(serde_json::to_vec(&body).unwrap().into()),
                None,
            )
            .await?;
        Ok(DockerOutput {
            status: crate::exec::ExecExitStatus::Api(200),
            stdout: String::from_utf8_lossy(&payload).into_owned(),
            stderr: String::new(),
        })
    }
    pub(crate) async fn api_bytes_request(
        &self,
        method: &str,
        path: &str,
        body: Option<Vec<u8>>,
        content_type: &str,
    ) -> DockerResult<Vec<u8>> {
        let lease = match &self.executor {
            crate::exec::CommandExecutor::Remote(remote) => {
                Some(self.api_tunnel.acquire(remote).await?)
            }
            crate::exec::CommandExecutor::Local(_) => None,
        };
        let api = super::DockerApi::new(
            lease
                .as_ref()
                .map(|lease| lease.socket())
                .unwrap_or("/var/run/docker.sock"),
        );
        Ok(api
            .request_with_type(method, path, body.map(Into::into), content_type)
            .await?
            .to_vec())
    }
    pub(crate) async fn api_get_json(&self, path: &str) -> DockerResult<serde_json::Value> {
        let bytes = self
            .api_bytes_request("GET", path, None, "application/json")
            .await?;
        Ok(serde_json::from_slice(&bytes)?)
    }
    pub fn new() -> Self {
        Self::new_local()
    }

    pub fn system(&self) -> super::handles::SystemHandle<'_> {
        super::handles::SystemHandle(self)
    }

    pub fn new_local() -> Self {
        Self {
            executor: CommandExecutor::Local(LocalExecutor::new()),
            executable: "docker".into(),
            global_args: vec![],
            api_tunnel: Default::default(),
            registry_auth: Default::default(),
        }
    }
    pub fn from_executor(executor: CommandExecutor) -> Self {
        Self {
            executor,
            executable: "docker".into(),
            global_args: vec![],
            api_tunnel: Default::default(),
            registry_auth: Default::default(),
        }
    }
    pub fn from_optional_executor(executor: Option<CommandExecutor>) -> Self {
        match executor {
            Some(exec) => Self::from_executor(exec),
            None => Self::new_local(),
        }
    }

    pub fn with_executable(executable: impl Into<PathBuf>) -> Self {
        Self {
            executor: CommandExecutor::Local(LocalExecutor::new()),
            executable: executable.into().to_string_lossy().into_owned(),
            global_args: vec![],
            api_tunnel: Default::default(),
            registry_auth: Default::default(),
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
            api_tunnel: Default::default(),
            registry_auth: Default::default(),
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
    pub async fn run_bytes<I, S>(&self, args: I) -> DockerResult<crate::exec::ExecBytesOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let args = self.arguments(args);
        self.executor.run_bytes(&self.executable, args).await
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
    pub(crate) async fn json_cancelled<T: DeserializeOwned>(
        &self,
        args: &[&str],
        cancel: &CancellationToken,
    ) -> DockerResult<T> {
        Ok(serde_json::from_str(
            &self.run_cancelled(args, cancel).await?.stdout,
        )?)
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

    pub async fn execute(&self, builder: &crate::exec::ArgBuilder) -> DockerResult<DockerOutput> {
        let mut attempts = 0;
        let args = builder.clone().build();
        if builder.api {
            return self.execute_api(&args).await;
        }
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        tracing::debug!(command = ?refs, "running docker command");
        loop {
            attempts += 1;
            if let Some(cancel) = &builder.cancel_token {
                if cancel.is_cancelled() {
                    return Err(crate::exec::ExecError::StreamCancelled.into());
                }
                match self.run_cancelled(&refs, cancel).await {
                    Ok(out) => return Ok(out),
                    Err(e)
                        if attempts <= builder.retry_limit.unwrap_or(0)
                            && crate::docker::error::is_transient_docker_error(&e.to_string()) =>
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
                            && crate::docker::error::is_transient_docker_error(&e.to_string()) =>
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

    async fn execute_api(&self, args: &[String]) -> DockerResult<DockerOutput> {
        if !super::api::supports_command(args) {
            return Err(crate::docker::DockerError::Ssh(format!(
                "Docker Engine API v1.52 does not support: docker {} (use the default CLI mode)",
                args.join(" ")
            )));
        }
        let lease = match &self.executor {
            crate::exec::CommandExecutor::Remote(remote) => {
                Some(self.api_tunnel.acquire(remote).await?)
            }
            crate::exec::CommandExecutor::Local(_) => None,
        };
        let api = super::DockerApi::new(
            lease
                .as_ref()
                .map(|l| l.socket())
                .unwrap_or("/var/run/docker.sock"),
        );
        if args.first().map(String::as_str) == Some("container")
            && args.get(1).map(String::as_str) == Some("exec")
        {
            return super::api::exec::execute(&api, args).await;
        }
        if args.first().map(String::as_str) == Some("image")
            && args.get(1).map(String::as_str) == Some("build")
        {
            return self.execute_api_build(&api, args).await;
        }
        if super::api::mutations::handles(args) {
            return super::api::mutations::execute(&api, args).await;
        }
        let request = super::api::map_cli_args(args).ok_or_else(|| {
            crate::docker::DockerError::Ssh(format!(
                "no Docker API v1.52 mapping for: docker {}",
                args.join(" ")
            ))
        })?;
        let mut body = request.body;
        if let Some(value) = body.as_mut() {
            if let Some(files) = value
                .get("__openoxide_env_files")
                .and_then(serde_json::Value::as_array)
                .cloned()
            {
                if let Some(env) = value
                    .get_mut("Env")
                    .and_then(serde_json::Value::as_array_mut)
                {
                    for file in files {
                        let path = file.as_str().ok_or_else(|| {
                            crate::docker::DockerError::Ssh("invalid env file path".into())
                        })?;
                        let content = tokio::fs::read_to_string(path)
                            .await
                            .map_err(crate::docker::DockerError::Io)?;
                        for line in content
                            .lines()
                            .map(str::trim)
                            .filter(|line| !line.is_empty() && !line.starts_with('#'))
                        {
                            env.push(serde_json::json!(line));
                        }
                    }
                }
                if let Some(object) = value.as_object_mut() {
                    object.remove("__openoxide_env_files");
                }
            }
        }
        let registry_request = request.path.starts_with("/images/create")
            || request.path.contains("/push")
            || request.path.starts_with("/build");
        let auth = self
            .registry_auth
            .read()
            .ok()
            .and_then(|value| value.clone());
        let payload = api
            .request_with_auth(
                request.method,
                &request.path,
                body.map(|body| serde_json::to_vec(&body).unwrap().into()),
                if registry_request {
                    auth.as_deref()
                } else {
                    None
                },
            )
            .await?;
        if request.start_created {
            let created: serde_json::Value = serde_json::from_slice(&payload).map_err(|e| {
                crate::docker::DockerError::Ssh(format!("Docker create returned invalid JSON: {e}"))
            })?;
            let id = created
                .get("Id")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| {
                    crate::docker::DockerError::Ssh(
                        "Docker create response did not include container Id".into(),
                    )
                })?;
            api.request("POST", &format!("/containers/{id}/start"), None)
                .await?;
        }
        let stdout = match request.response_shape {
            super::api::ResponseShape::Array => format!("[{}]", String::from_utf8_lossy(&payload)),
            super::api::ResponseShape::Raw => String::from_utf8_lossy(&payload).into_owned(),
            super::api::ResponseShape::CreatedId => {
                let value: serde_json::Value = serde_json::from_slice(&payload).map_err(|e| {
                    crate::docker::DockerError::Ssh(format!(
                        "Docker create returned invalid JSON: {e}"
                    ))
                })?;
                value
                    .get("Id")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or_default()
                    .to_owned()
            }
        };
        Ok(DockerOutput {
            status: crate::exec::ExecExitStatus::Api(200),
            stdout,
            stderr: String::new(),
        })
    }

    async fn execute_api_build(
        &self,
        api: &super::DockerApi,
        args: &[String],
    ) -> DockerResult<DockerOutput> {
        let context = args.last().ok_or_else(|| {
            crate::docker::DockerError::Ssh("image build requires context".into())
        })?;
        let mut query = Vec::new();
        for pair in args.windows(2) {
            match pair[0].as_str() {
                "--tag" | "-t" => {
                    query.push(format!("t={}", super::api::encode_query_public(&pair[1])))
                }
                "--file" | "-f" => query.push(format!(
                    "dockerfile={}",
                    super::api::encode_query_public(&pair[1])
                )),
                "--target" => query.push(format!(
                    "target={}",
                    super::api::encode_query_public(&pair[1])
                )),
                "--platform" => query.push(format!(
                    "platform={}",
                    super::api::encode_query_public(&pair[1])
                )),
                "--build-arg" => query.push(format!(
                    "buildargs={}",
                    super::api::encode_query_public(&format!(
                        "{{\"{}\":\"{}\"}}",
                        pair[1].split_once('=').map(|p| p.0).unwrap_or(&pair[1]),
                        pair[1].split_once('=').map(|p| p.1).unwrap_or("")
                    ))
                )),
                _ => {}
            }
        }
        query.push(format!(
            "nocache={}",
            args.iter().any(|arg| arg == "--no-cache")
        ));
        query.push(format!("pull={}", args.iter().any(|arg| arg == "--pull")));
        query.push("forcerm=true".into());
        let path = format!("/build?{}", query.join("&"));
        let archive = tokio::process::Command::new("tar")
            .args(["-C", context, "-cf", "-", "."])
            .output()
            .await
            .map_err(crate::docker::DockerError::Io)?;
        if !archive.status.success() {
            return Err(crate::docker::DockerError::Ssh(
                "failed to archive image build context".into(),
            ));
        }
        let payload = api
            .request_with_type(
                "POST",
                &path,
                Some(archive.stdout.into()),
                "application/x-tar",
            )
            .await?;
        Ok(DockerOutput {
            status: crate::exec::ExecExitStatus::Api(200),
            stdout: String::from_utf8_lossy(&payload).into_owned(),
            stderr: String::from_utf8_lossy(&archive.stderr).into_owned(),
        })
    }

    pub async fn execute_stream(
        &self,
        builder: &crate::exec::ArgBuilder,
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        if builder.api {
            let args = builder.clone().build();
            if !super::api::supports_command(&args) {
                return Err(crate::docker::DockerError::Ssh(format!(
                    "Docker Engine API v1.52 does not support: docker {} (use the default CLI mode)",
                    args.join(" ")
                )));
            }
            let lease = match &self.executor {
                crate::exec::CommandExecutor::Remote(remote) => {
                    Some(self.api_tunnel.acquire(remote).await?)
                }
                crate::exec::CommandExecutor::Local(_) => None,
            };
            if args.first().map(String::as_str) == Some("container")
                && args.get(1).map(String::as_str) == Some("exec")
            {
                return super::api::exec::execute_stream(
                    &super::DockerApi::new(
                        lease
                            .as_ref()
                            .map(|l| l.socket())
                            .unwrap_or("/var/run/docker.sock"),
                    ),
                    &args,
                    sender,
                )
                .await;
            }
            if args.first().map(String::as_str) == Some("image")
                && args.get(1).map(String::as_str) == Some("build")
            {
                return super::api::build::execute_stream(
                    &super::DockerApi::new(
                        lease
                            .as_ref()
                            .map(|l| l.socket())
                            .unwrap_or("/var/run/docker.sock"),
                    ),
                    &args,
                    sender,
                )
                .await;
            }
            let request = super::api::map_cli_args(&args).ok_or_else(|| {
                crate::docker::DockerError::Ssh(format!(
                    "no Docker API v1.52 stream mapping for: docker {}",
                    args.join(" ")
                ))
            })?;
            let api = super::DockerApi::new(
                lease
                    .as_ref()
                    .map(|l| l.socket())
                    .unwrap_or("/var/run/docker.sock"),
            );
            let body = request
                .body
                .map(|body| serde_json::to_vec(&body).unwrap().into());
            let stream = api.request_stream(request.method, &request.path, body, sender);
            let status = if let Some(cancel) = &builder.cancel_token {
                tokio::select! { result = stream => result?, _ = cancel.cancelled() => return Err(crate::exec::ExecError::StreamCancelled.into()) }
            } else {
                stream.await?
            };
            return Ok(crate::exec::ExecExitStatus::Api(status));
        }
        let mut attempts = 0;
        let args = builder.clone().build();
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        loop {
            attempts += 1;
            if let Some(cancel) = &builder.cancel_token {
                if cancel.is_cancelled() {
                    return Err(crate::exec::ExecError::StreamCancelled.into());
                }
            }
            match self.run_stream(&refs, sender.clone()).await {
                Ok(out) => return Ok(out),
                Err(e)
                    if attempts <= builder.retry_limit.unwrap_or(0)
                        && crate::docker::error::is_transient_docker_error(&e.to_string()) =>
                {
                    tokio::time::sleep(tokio::time::Duration::from_secs(2 * attempts as u64)).await;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
    }

    pub(crate) async fn execute_json_lines<T: DeserializeOwned>(
        &self,
        builder: &crate::exec::ArgBuilder,
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

    /// Return a typesafe fluent query / command builder backed by this client.
    ///
    /// # Example
    /// ```rust,no_run
    /// use crate::docker::query::filter::{ContainerFilter, ContainerStatus};
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

    pub fn swarm(&self) -> crate::docker::handles::SwarmHandle<'_> {
        crate::docker::handles::SwarmHandle::new(self)
    }

    pub fn nodes(&self) -> crate::docker::handles::NodesHandle<'_> {
        crate::docker::handles::NodesHandle::new(self)
    }

    pub fn stacks(&self) -> crate::docker::handles::StacksHandle<'_> {
        crate::docker::handles::StacksHandle::new(self)
    }

    pub fn secrets(&self) -> crate::docker::handles::SecretsHandle<'_> {
        crate::docker::handles::SecretsHandle::new(self)
    }

    pub fn configs(&self) -> crate::docker::handles::ConfigsHandle<'_> {
        crate::docker::handles::ConfigsHandle::new(self)
    }

    pub fn services(&self) -> crate::docker::handles::ServicesHandle<'_> {
        crate::docker::handles::ServicesHandle::new(self)
    }

    pub fn containers(&self) -> crate::docker::handles::ContainerHandle<'_> {
        crate::docker::handles::ContainerHandle(self)
    }

    pub fn container(
        &self,
        id: impl Into<String>,
    ) -> crate::docker::handles::ContainerResource<'_> {
        crate::docker::handles::ContainerResource::new(self, id)
    }

    pub fn images(&self) -> crate::docker::handles::ImageHandle<'_> {
        crate::docker::handles::ImageHandle(self)
    }

    pub fn image(&self, id: impl Into<String>) -> crate::docker::handles::ImageResource<'_> {
        crate::docker::handles::ImageResource::new(self, id)
    }

    pub fn networks(&self) -> crate::docker::handles::NetworkHandle<'_> {
        crate::docker::handles::NetworkHandle(self)
    }

    pub fn network(&self, name: impl Into<String>) -> crate::docker::handles::NetworkResource<'_> {
        crate::docker::handles::NetworkResource::new(self, name)
    }

    pub fn volumes(&self) -> crate::docker::handles::VolumeHandle<'_> {
        crate::docker::handles::VolumeHandle(self)
    }

    pub fn volume(&self, name: impl Into<String>) -> crate::docker::handles::VolumeResource<'_> {
        crate::docker::handles::VolumeResource::new(self, name)
    }

    pub fn compose(&self) -> crate::docker::handles::ComposeHandle<'_> {
        crate::docker::handles::ComposeHandle(self)
    }
}
