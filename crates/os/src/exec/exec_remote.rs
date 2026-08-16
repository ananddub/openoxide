use super::{
    ExecError, ExecExitStatus, ExecOutput, ExecResult, ExecStreamEvent, SshAuth, SshHostKey,
};
use std::ffi::OsStr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

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
        let has_pwd = self.sudo_password.as_deref().map(|p| !p.is_empty()).unwrap_or(false);
        let command = remote_command(
            "sh",
            &["-c".into(), remote_cancel_script(pid_file.as_ref())],
            self.sudo_password.is_some(),
            has_pwd,
        );
        self.execute_raw_once(command, true, Duration::from_secs(8))
            .await
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
        let mut builder = crate::ssh::SshBuilder::new(
            self.host.clone(),
            self.username.clone(),
            self.auth.clone(),
            self.host_key.clone(),
        )
        .port(self.port)
        .connect_timeout(self.connect_timeout.as_secs() as u32);

        if !self.multiplexing {
            builder = builder.disable_multiplexing();
        }

        let has_pwd = self.sudo_password.as_deref().map(|p| !p.is_empty()).unwrap_or(false);
        let base_command = remote_command(program, args, self.sudo_password.is_some(), has_pwd);
        let cancel_job = cancel.map(|_| {
            self.job_pid_file
                .as_ref()
                .map(|pid_file| RemoteCancelJob::from_pid_file(pid_file.clone()))
                .unwrap_or_else(RemoteCancelJob::new)
        });
        let command = if let Some(job) = &cancel_job {
            cancellable_remote_command(&base_command, &job.pid_file)
        } else {
            base_command
        };

        let ssh_cmd = builder
            .build_command("sh", &["-c".to_string(), command])
            .await
            .map_err(|e| ExecError::Ssh(e.to_string()))?;

        let mut tokio_command = ssh_cmd.command;
        let _agent_session = ssh_cmd.agent_session;
        let _temp_askpass_file = ssh_cmd.temp_askpass_file;

        tokio_command
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        let mut child = tokio_command
            .spawn()
            .map_err(|e| ExecError::Ssh(e.to_string()))?;

        let mut child_stdin = child
            .stdin
            .take()
            .ok_or_else(|| ExecError::Ssh("Failed to open stdin".into()))?;
        let mut child_stdout = child
            .stdout
            .take()
            .ok_or_else(|| ExecError::Ssh("Failed to open stdout".into()))?;
        let mut child_stderr = child
            .stderr
            .take()
            .ok_or_else(|| ExecError::Ssh("Failed to open stderr".into()))?;

        let mut input_data = Vec::new();
        if let Some(password) = &self.sudo_password {
            if !password.is_empty() {
                input_data.extend_from_slice(password.as_bytes());
                input_data.push(b'\n');
            }
        }
        input_data.extend_from_slice(stdin);

        tokio::spawn(async move {
            if !input_data.is_empty() {
                let _ = child_stdin.write_all(&input_data).await;
            }
            drop(child_stdin);
        });

        let mut stdout_buf = [0u8; 4096];
        let mut stderr_buf = [0u8; 4096];
        let mut stdout_accum = Vec::new();
        let mut stderr_accum = Vec::new();
        let mut stdout_done = false;
        let mut stderr_done = false;
        let exit_status;

        let timeout_fut = tokio::time::sleep(self.command_timeout);
        tokio::pin!(timeout_fut);

        loop {
            tokio::select! {
                res = child_stdout.read(&mut stdout_buf), if !stdout_done => {
                    match res {
                        Ok(0) => stdout_done = true,
                        Ok(n) => {
                            let data = &stdout_buf[..n];
                            if let Some(tx) = &stream {
                                if tx.send(ExecStreamEvent::Stdout(data.to_vec())).await.is_err() {
                                    let _ = child.kill().await;
                                    self.cancel_remote_job(cancel_job.as_ref()).await;
                                    return Err(ExecError::StreamCancelled);
                                }
                            } else {
                                stdout_accum.extend_from_slice(data);
                                 let s = String::from_utf8_lossy(data);
                                for line in s.lines() {
                                    tracing::info!("{line}");
                                }
                            }
                        }
                        Err(e) => return Err(ExecError::Ssh(e.to_string())),
                    }
                }
                res = child_stderr.read(&mut stderr_buf), if !stderr_done => {
                    match res {
                        Ok(0) => stderr_done = true,
                        Ok(n) => {
                            let data = &stderr_buf[..n];
                            if let Some(tx) = &stream {
                                if tx.send(ExecStreamEvent::Stderr(data.to_vec())).await.is_err() {
                                    let _ = child.kill().await;
                                    self.cancel_remote_job(cancel_job.as_ref()).await;
                                    return Err(ExecError::StreamCancelled);
                                }
                            } else {
                                stderr_accum.extend_from_slice(data);
                                let s = String::from_utf8_lossy(data);
                                for line in s.lines() {
                                    tracing::info!("{line}");
                                }
                            }
                        }
                        Err(e) => return Err(ExecError::Ssh(e.to_string())),
                    }
                }
                _ = &mut timeout_fut => {
                    let _ = child.kill().await;
                    self.cancel_remote_job(cancel_job.as_ref()).await;
                    return Err(ExecError::Timeout {
                        seconds: self.command_timeout.as_secs(),
                    });
                }
                _ = async {
                    if let Some(c) = cancel {
                        c.cancelled().await
                    } else {
                        std::future::pending().await
                    }
                } => {
                    let _ = child.kill().await;
                    self.cancel_remote_job(cancel_job.as_ref()).await;
                    return Err(ExecError::StreamCancelled);
                }
                status = child.wait() => {
                    match status {
                        Ok(s) => {
                            exit_status = Some(s);
                            break;
                        }
                        Err(e) => return Err(ExecError::Ssh(e.to_string())),
                    }
                }
            }
        }

        // Drain any remaining output from streams
        if !stdout_done {
            let mut buf = Vec::new();
            if child_stdout.read_to_end(&mut buf).await.is_ok() && !buf.is_empty() {
                if let Some(tx) = &stream {
                    let _ = tx.send(ExecStreamEvent::Stdout(buf)).await;
                } else {
                    stdout_accum.extend_from_slice(&buf);
                    let s = String::from_utf8_lossy(&buf);
                    for line in s.lines() {
                        tracing::info!("{line}");
                    }
                }
            }
        }
        if !stderr_done {
            let mut buf = Vec::new();
            if child_stderr.read_to_end(&mut buf).await.is_ok() && !buf.is_empty() {
                if let Some(tx) = &stream {
                    let _ = tx.send(ExecStreamEvent::Stderr(buf)).await;
                } else {
                    stderr_accum.extend_from_slice(&buf);
                    let s = String::from_utf8_lossy(&buf);
                    for line in s.lines() {
                        tracing::info!("{line}");
                    }
                }
            }
        }

        let status_code = exit_status
            .and_then(|s| s.code())
            .ok_or_else(|| ExecError::Ssh("remote command ended without an exit status".into()))?;

        if status_code == 255 {
            return Err(ExecError::Ssh(format!(
                "SSH connection/authentication failed: {}",
                String::from_utf8_lossy(&stderr_accum)
            )));
        }

        let status = ExecExitStatus::Remote(status_code as u32);
        let result = ExecOutput {
            status,
            stdout: String::from_utf8_lossy(&stdout_accum).into_owned(),
            stderr: String::from_utf8_lossy(&stderr_accum).into_owned(),
        };

        if !result.success() {
            return Err(ExecError::CommandFailed {
                code: result.status.code(),
                stderr: result.combined_output(),
            });
        }

        Ok(result)
    }

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

    async fn execute_raw_once(
        &self,
        command: String,
        send_sudo_password: bool,
        timeout: Duration,
    ) -> ExecResult<()> {
        let builder = crate::ssh::SshBuilder::new(
            self.host.clone(),
            self.username.clone(),
            self.auth.clone(),
            self.host_key.clone(),
        )
        .port(self.port)
        .connect_timeout(timeout.as_secs() as u32);

        let ssh_cmd = builder
            .build_command("sh", &["-c".to_string(), command])
            .await
            .map_err(|e| ExecError::Ssh(e.to_string()))?;

        let mut tokio_command = ssh_cmd.command;
        let _agent_session = ssh_cmd.agent_session;

        tokio_command
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        let mut child = tokio_command
            .spawn()
            .map_err(|e| ExecError::Ssh(e.to_string()))?;

        if send_sudo_password {
            if let Some(password) = &self.sudo_password {
                if let Some(mut child_stdin) = child.stdin.take() {
                    let mut input = password.as_bytes().to_vec();
                    input.push(b'\n');
                    let _ = child_stdin.write_all(&input).await;
                }
            }
        }

        let res = tokio::time::timeout(timeout, child.wait()).await;
        match res {
            Ok(Ok(status)) => {
                let code = status.code().unwrap_or(0);
                if code == 255 {
                    return Err(ExecError::Ssh(
                        "SSH connection/authentication failed".into(),
                    ));
                }
                if code == 0 {
                    Ok(())
                } else {
                    Err(ExecError::CommandFailed {
                        code: Some(code),
                        stderr: "remote cancel command failed".into(),
                    })
                }
            }
            Ok(Err(e)) => Err(ExecError::Ssh(e.to_string())),
            Err(_) => {
                let _ = child.kill().await;
                Err(ExecError::Timeout {
                    seconds: timeout.as_secs(),
                })
            }
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
            vec![
                "sudo".into(),
                "-n".into(),
                "--".into(),
                program.into(),
            ]
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
