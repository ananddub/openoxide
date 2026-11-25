use super::{
    ExecError, ExecExitStatus, ExecOutput, ExecResult, ExecStreamEvent, SshAuth, SshHostKey,
};
use std::ffi::OsStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

/// Shared, lazily-opened SSH session reused by every command of one executor
/// (and its clones). Opening a session costs a full key exchange plus
/// authentication (~3.5 s on a 300 ms link), so a flow that runs a dozen small
/// commands — a setup audit, a status probe — used to spend most of its time
/// reconnecting.
#[derive(Clone)]
struct SessionCache {
    slot: Arc<tokio::sync::Mutex<Option<Arc<crate::ssh::RusshSession>>>>,
    /// sshd caps channels per session (`MaxSessions`, default 10); stay under it.
    channels: Arc<tokio::sync::Semaphore>,
}

impl std::fmt::Debug for SessionCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionCache").finish_non_exhaustive()
    }
}

const MAX_CONCURRENT_CHANNELS: usize = 8;

#[derive(Clone, Debug)]
pub struct RemoteExecutor {
    host: String,
    port: u16,
    username: String,
    auth: SshAuth,
    host_key: SshHostKey,
    sudo_password: Option<String>,
    command_timeout: Duration,
    connect_timeout: Duration,
    terminal_timeout: Duration,
    job_pid_file: Option<String>,
    multiplexing: bool,
    cache: SessionCache,
}

#[derive(Debug)]
pub struct RemoteTerminal {
    pub input: mpsc::Sender<Vec<u8>>,
    pub resize: mpsc::Sender<(u16, u16)>,
    pub cancel: CancellationToken,
    task: JoinHandle<ExecResult<()>>,
}

impl RemoteTerminal {
    pub fn cancel(&self) {
        self.cancel.cancel();
    }

    pub async fn wait(self) -> ExecResult<()> {
        match self.task.await {
            Ok(result) => result,
            Err(error) => Err(ExecError::Ssh(error.to_string())),
        }
    }
}

impl RemoteExecutor {
    pub fn new(
        host: impl Into<String>,
        port: u16,
        username: impl Into<String>,
        auth: SshAuth,
        host_key: SshHostKey,
    ) -> Self {
        Self {
            host: host.into(),
            port,
            username: username.into(),
            auth,
            host_key,
            sudo_password: None,
            command_timeout: Duration::from_secs(300),
            connect_timeout: Duration::from_secs(15),
            terminal_timeout: Duration::from_secs(30 * 60),
            job_pid_file: None,
            multiplexing: true,
            cache: SessionCache {
                slot: Arc::default(),
                channels: Arc::new(tokio::sync::Semaphore::new(MAX_CONCURRENT_CHANNELS)),
            },
        }
    }

    pub fn with_sudo(mut self) -> Self {
        self.sudo_password = match &self.auth {
            SshAuth::Password(p) => Some(p.clone()),
            _ => Some(String::new()),
        };
        self
    }
    pub fn with_sudo_password(mut self, password: impl Into<String>) -> Self {
        self.sudo_password = Some(password.into());
        self
    }
    pub fn with_command_timeout(mut self, timeout: Duration) -> Self {
        self.command_timeout = timeout;
        self
    }
    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }
    pub fn with_terminal_timeout(mut self, timeout: Duration) -> Self {
        self.terminal_timeout = timeout.max(Duration::from_secs(30));
        self
    }
    pub fn with_job_pid_file(mut self, pid_file: impl Into<String>) -> Self {
        self.job_pid_file = Some(pid_file.into());
        self
    }
    pub fn with_multiplexing(mut self, enabled: bool) -> Self {
        self.multiplexing = enabled;
        self
    }

    pub fn host(&self) -> &str {
        &self.host
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn auth(&self) -> &SshAuth {
        &self.auth
    }
    pub fn host_key(&self) -> &SshHostKey {
        &self.host_key
    }
    pub fn connect_timeout(&self) -> Duration {
        self.connect_timeout
    }

    /// The executor's shared session, reconnecting when it is missing or has
    /// been closed (keepalive failure, sshd restart, network drop). With
    /// multiplexing disabled every call is a fresh, private session.
    pub async fn connect_session(&self) -> ExecResult<Arc<crate::ssh::RusshSession>> {
        if !self.multiplexing {
            return self.connect_fresh_session().await;
        }
        let mut slot = self.cache.slot.lock().await;
        if let Some(session) = slot.as_ref()
            && !session.is_closed()
        {
            return Ok(session.clone());
        }
        let session = self.connect_fresh_session().await?;
        *slot = Some(session.clone());
        Ok(session)
    }

    /// Forget the shared session (only if it is still `stale`), so the next
    /// command dials a new one.
    async fn discard_session(&self, stale: &Arc<crate::ssh::RusshSession>) {
        let mut slot = self.cache.slot.lock().await;
        if slot.as_ref().is_some_and(|s| Arc::ptr_eq(s, stale)) {
            *slot = None;
        }
    }

    pub async fn connect_fresh_session(&self) -> ExecResult<Arc<crate::ssh::RusshSession>> {
        let session = Arc::new(
            crate::ssh::connect_russh(
                &self.host,
                self.port,
                &self.username,
                &self.auth,
                &self.host_key,
                self.connect_timeout,
            )
            .await?,
        );
        Ok(session)
    }

    /// Observe this endpoint's SSH host-key fingerprint via a throwaway
    /// handshake (no authentication). Used for trust-on-first-capture when the
    /// panel has not yet stored a pin for the server.
    pub async fn capture_host_key_fingerprint(&self) -> ExecResult<String> {
        crate::ssh::capture_host_key_fingerprint(&self.host, self.port, self.connect_timeout).await
    }

    /// Capture the full observed host key (fingerprint + OpenSSH `known_hosts`
    /// form). The OpenSSH form feeds the rsync subprocess's `known_hosts` file
    /// so bulk transfers verify the target too, not just russh sessions.
    pub async fn capture_host_key(&self) -> ExecResult<crate::ssh::ServerHostKey> {
        crate::ssh::capture_host_key(&self.host, self.port, self.connect_timeout).await
    }

    pub async fn open_terminal(
        &self,
        output: mpsc::Sender<ExecStreamEvent>,
        _term: impl Into<String>,
        _cols: u16,
        _rows: u16,
    ) -> ExecResult<RemoteTerminal> {
        let builder = crate::ssh::SshBuilder::new(
            self.host.clone(),
            self.username.clone(),
            self.auth.clone(),
            self.host_key.clone(),
        )
        .port(self.port)
        .connect_timeout(self.connect_timeout.as_secs() as u32)
        .tty(crate::ssh::TtyMode::ForceTty);

        let ssh_cmd = builder
            .build_command("sh", &[])
            .await
            .map_err(|e| ExecError::Ssh(e.to_string()))?;

        let mut tokio_command = ssh_cmd.command;
        let _agent_session = ssh_cmd.agent_session;
        let temp_askpass = ssh_cmd.temp_askpass_file;

        tokio_command
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());
        // Terminal sessions must not outlive their owner: dropping the
        // RemoteTerminal/child here would otherwise orphan the ssh process
        // and its remote pty.
        tokio_command.kill_on_drop(true);

        let mut child = tokio_command
            .spawn()
            .map_err(|e| ExecError::Ssh(e.to_string()))?;

        let mut child_stdin = child
            .stdin
            .take()
            .ok_or_else(|| ExecError::Ssh("stdin failed".into()))?;
        let mut child_stdout = child
            .stdout
            .take()
            .ok_or_else(|| ExecError::Ssh("stdout failed".into()))?;
        let mut child_stderr = child
            .stderr
            .take()
            .ok_or_else(|| ExecError::Ssh("stderr failed".into()))?;

        let (input_tx, mut input_rx) = mpsc::channel::<Vec<u8>>(128);
        let (resize_tx, mut resize_rx) = mpsc::channel::<(u16, u16)>(16);
        let cancel = CancellationToken::new();
        let task_cancel = cancel.clone();
        let terminal_timeout = self.terminal_timeout;

        let task = tokio::spawn(async move {
            // Held for the life of the session: dropping the agent kills it and
            // the connection loses its key.
            let _keep_alive_agent = _agent_session;
            let _keep_alive_askpass = temp_askpass;
            let mut stdout_buf = [0u8; 4096];
            let mut stderr_buf = [0u8; 4096];
            let mut stdout_done = false;
            let mut stderr_done = false;
            let idle_timeout = tokio::time::sleep(terminal_timeout);
            tokio::pin!(idle_timeout);

            loop {
                tokio::select! {
                    _ = &mut idle_timeout => {
                        let _ = child.kill().await;
                        return Err(ExecError::Timeout { seconds: terminal_timeout.as_secs() });
                    }
                    _ = task_cancel.cancelled() => {
                        let _ = child.kill().await;
                        return Ok(());
                    }
                    input = input_rx.recv() => {
                        let Some(input) = input else {
                            break;
                        };
                        if child_stdin.write_all(&input).await.is_err() || child_stdin.flush().await.is_err() {
                            break;
                        }
                        idle_timeout.as_mut().reset(tokio::time::Instant::now() + terminal_timeout);
                    }
                    resize = resize_rx.recv() => {
                        let _ = resize;
                        idle_timeout.as_mut().reset(tokio::time::Instant::now() + terminal_timeout);
                    }
                    res = child_stdout.read(&mut stdout_buf), if !stdout_done => {
                        match res {
                            Ok(0) => stdout_done = true,
                            Ok(n) => {
                                if output.send(ExecStreamEvent::Stdout(stdout_buf[..n].to_vec())).await.is_err() {
                                    break;
                                }
                                idle_timeout.as_mut().reset(tokio::time::Instant::now() + terminal_timeout);
                            }
                            Err(_) => break,
                        }
                    }
                    res = child_stderr.read(&mut stderr_buf), if !stderr_done => {
                        match res {
                            Ok(0) => stderr_done = true,
                            Ok(n) => {
                                if output.send(ExecStreamEvent::Stderr(stderr_buf[..n].to_vec())).await.is_err() {
                                    break;
                                }
                                idle_timeout.as_mut().reset(tokio::time::Instant::now() + terminal_timeout);
                            }
                            Err(_) => break,
                        }
                    }
                    status = child.wait() => {
                        let _ = status;
                        break;
                    }
                }
            }
            let _ = child.kill().await;
            Ok(())
        });

        Ok(RemoteTerminal {
            input: input_tx,
            resize: resize_tx,
            cancel,
            task,
        })
    }

    pub async fn kill_pid_file(&self, pid_file: impl AsRef<str>) -> ExecResult<()> {
        let cancel_script = remote_cancel_script(pid_file.as_ref());
        let _ = self.run("sh", &["-c", &cancel_script]).await;
        Ok(())
    }
    pub async fn run<I, S>(&self, program: &str, args: I) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let args = collect(args);
        self.execute(program, &args, &[], None, None).await
    }
    pub async fn run_cancelled<I, S>(
        &self,
        program: &str,
        args: I,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let args = collect(args);
        self.execute(program, &args, &[], None, Some(cancel)).await
    }
    pub async fn run_with_stdin<I, S>(
        &self,
        program: &str,
        args: I,
        stdin: impl AsRef<[u8]>,
    ) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let args = collect(args);
        self.execute(program, &args, stdin.as_ref(), None, None)
            .await
    }
    pub async fn run_with_stdin_cancelled<I, S>(
        &self,
        program: &str,
        args: I,
        stdin: impl AsRef<[u8]>,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let args = collect(args);
        self.execute(program, &args, stdin.as_ref(), None, Some(cancel))
            .await
    }
    pub async fn run_stream(
        &self,
        program: &str,
        args: &[String],
        sender: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<ExecExitStatus> {
        Ok(self
            .execute(program, args, &[], Some(sender), None)
            .await?
            .status)
    }

    async fn execute(
        &self,
        program: &str,
        args: &[String],
        stdin: &[u8],
        stream: Option<mpsc::Sender<ExecStreamEvent>>,
        cancel: Option<&CancellationToken>,
    ) -> ExecResult<ExecOutput> {
        let has_pwd = self
            .sudo_password
            .as_deref()
            .map(|p| !p.is_empty())
            .unwrap_or(false);
        let base_command = remote_command(program, args, self.sudo_password.is_some(), has_pwd);
        let cancel_job = cancel.map(|_| {
            self.job_pid_file
                .as_ref()
                .map(|pid_file| RemoteCancelJob::from_pid_file(pid_file.clone()))
                .unwrap_or_else(RemoteCancelJob::new)
        });
        let command_to_run = if let Some(job) = &cancel_job {
            if stdin.is_empty() {
                cancellable_remote_command(&base_command, &job.pid_file)
            } else {
                base_command.clone()
            }
        } else {
            base_command
        };

        let full_command = format!("sh -c {}", quote(&command_to_run));
        // Stay under sshd's per-session channel cap when many commands overlap.
        let _slot = self
            .cache
            .channels
            .acquire()
            .await
            .map_err(|_| ExecError::Ssh("SSH channel limiter closed".into()))?;
        let mut session = self.connect_session().await?;
        let mut retried = false;
        let run = async {
            loop {
                match crate::ssh::execute_russh_cmd_stream(
                    &session,
                    &full_command,
                    stdin,
                    stream.as_ref(),
                )
                .await
                {
                    Err(ExecError::ChannelOpen(reason)) if self.multiplexing && !retried => {
                        // The command never started, so redialing and retrying
                        // cannot run it twice (a cached session can die between
                        // commands without any notice).
                        tracing::debug!(%reason, "cached SSH session unusable; reconnecting");
                        retried = true;
                        self.discard_session(&session).await;
                        session = self.connect_session().await?;
                    }
                    Err(error) => {
                        if matches!(error, ExecError::Ssh(_)) && self.multiplexing {
                            // Died mid-command: do not reuse this session.
                            self.discard_session(&session).await;
                        }
                        return Err(error);
                    }
                    Ok(done) => return Ok(done),
                }
            }
        };
        // `command_timeout` was stored but never enforced, so a command on a
        // dead connection waited forever. Streams (`logs -f`) are open-ended by
        // design and rely on SSH keepalive to notice a dead peer instead.
        let (code, stdout_bytes, stderr_bytes) = if stream.is_some() {
            run.await?
        } else {
            tokio::time::timeout(self.command_timeout, run)
                .await
                .map_err(|_| ExecError::Timeout {
                    seconds: self.command_timeout.as_secs(),
                })??
        };

        let status = ExecExitStatus::Remote(code);
        let result = ExecOutput {
            status,
            stdout: String::from_utf8_lossy(&stdout_bytes).into_owned(),
            stderr: String::from_utf8_lossy(&stderr_bytes).into_owned(),
        };

        if !result.success() {
            return Err(ExecError::CommandFailed {
                code: result.status.code(),
                stderr: result.combined_output(),
            });
        }

        Ok(result)
    }

    #[allow(dead_code)]
    async fn cancel_remote_job(&self, job: Option<&RemoteCancelJob>) {
        let Some(job) = job else {
            return;
        };
        if let Err(error) = self.kill_pid_file(&job.pid_file).await {
            tracing::warn!(
                error = %error,
                pid_file = %job.pid_file,
                "failed to kill remote cancellable SSH job"
            );
        }
    }
}

#[derive(Clone, Debug)]
struct RemoteCancelJob {
    pid_file: String,
}
impl RemoteCancelJob {
    fn new() -> Self {
        Self {
            pid_file: format!("/tmp/openoxide-ssh-job-{}.pid", Uuid::new_v4()),
        }
    }

    fn from_pid_file(pid_file: String) -> Self {
        Self { pid_file }
    }
}

fn collect<I, S>(args: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    args.into_iter()
        .map(|v| v.as_ref().to_string_lossy().into_owned())
        .collect()
}
fn quote(value: &str) -> String {
    if value.is_empty() {
        return "''".into();
    }
    format!("'{}'", value.replace('\'', "'\\''"))
}
fn remote_command(program: &str, args: &[String], sudo: bool, has_password: bool) -> String {
    let prefix = if sudo {
        if has_password {
            vec![
                "sudo".into(),
                "-S".into(),
                "-p".into(),
                String::new(),
                "--".into(),
                program.into(),
            ]
        } else {
            vec!["sudo".into(), "-n".into(), "--".into(), program.into()]
        }
    } else {
        vec![program.into()]
    };
    prefix
        .into_iter()
        .chain(args.iter().cloned())
        .map(|v| quote(&v))
        .collect::<Vec<_>>()
        .join(" ")
}

fn cancellable_remote_command(command: &str, pid_file: &str) -> String {
    let script = format!(
        r#"rm -f {pid_file}
setsid sh -c {command} &
child="$!"
printf '%s\n' "$child" > {pid_file}
wait "$child"
status="$?"
rm -f {pid_file}
exit "$status""#,
        pid_file = quote(pid_file),
        command = quote(command),
    );
    remote_command("sh", &["-c".into(), script], false, false)
}

fn remote_cancel_script(pid_file: &str) -> String {
    format!(
        r#"pid="$(cat {pid_file} 2>/dev/null || true)"
if [ -n "$pid" ]; then
  kill -TERM -- "-$pid" 2>/dev/null || true
  sleep 2
  kill -KILL -- "-$pid" 2>/dev/null || true
fi
rm -f {pid_file}"#,
        pid_file = quote(pid_file),
    )
}
