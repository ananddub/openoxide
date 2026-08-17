use super::{ExecError, ExecExitStatus, ExecOutput, ExecResult, ExecStreamEvent};
use std::{ffi::OsStr, process::Stdio};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt},
    process::Command,
    sync::mpsc,
};
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug, Default)]
pub struct LocalExecutor {
    non_interactive_sudo: bool,
}
impl LocalExecutor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Runs commands through `sudo -n` without making the parent process root.
    /// The host must grant the OpenOxide user narrowly scoped NOPASSWD rules.
    pub fn with_non_interactive_sudo(mut self) -> Self {
        self.non_interactive_sudo = true;
        self
    }
    pub fn command<I, S>(&self, program: &str, args: I) -> Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let resolved = resolve_binary_path(program);
        let is_root = unsafe { libc::getuid() == 0 };
        let has_sudo = std::path::Path::new("/usr/bin/sudo").exists()
            || std::path::Path::new("/bin/sudo").exists()
            || std::path::Path::new("/usr/sbin/sudo").exists();

        let use_sudo = self.non_interactive_sudo && !is_root && has_sudo;

        let mut c = if use_sudo {
            let sudo_bin = if std::path::Path::new("/usr/bin/sudo").exists() {
                "/usr/bin/sudo"
            } else if std::path::Path::new("/bin/sudo").exists() {
                "/bin/sudo"
            } else {
                "sudo"
            };
            let mut command = Command::new(sudo_bin);
            command.args(["-n", "--", &resolved]);
            command
        } else {
            Command::new(&resolved)
        };
        c.args(args);
        c
    }
    pub async fn run<I, S>(&self, program: &str, args: I) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let res = checked(
            self.command(program, args)
                .output()
                .await
                .map_err(|source| io_command(program, source))?,
        );
        if let Ok(ref out) = res {
            for line in out.stdout.lines() {
                tracing::info!("{line}");
            }
            for line in out.stderr.lines() {
                tracing::info!("{line}");
            }
        }
        res
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
        let mut c = self.command(program, args);
        c.stdout(Stdio::piped()).stderr(Stdio::piped());
        tracing::debug!(program, "starting local command");
        let mut child = c.spawn().map_err(|source| io_command(program, source))?;
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        let stdout_task = tokio::spawn(read_pipe_with_trace(program.to_owned(), "stdout", stdout));
        let stderr_task = tokio::spawn(read_pipe_with_trace(program.to_owned(), "stderr", stderr));
        tokio::select! {
            status = child.wait() => {
                let stdout = join_pipe(stdout_task).await?;
                let stderr = join_pipe(stderr_task).await?;
                checked(std::process::Output { status: status?, stdout, stderr })
            },
            _ = cancel.cancelled() => {
                let _ = child.kill().await;
                let _ = child.wait().await;
                Err(ExecError::StreamCancelled)
            }
        }
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
        let mut c = self.command(program, args);
        c.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = c.spawn().map_err(|source| io_command(program, source))?;
        child
            .stdin
            .take()
            .expect("piped stdin")
            .write_all(stdin.as_ref())
            .await?;
        let res = checked(
            child
                .wait_with_output()
                .await
                .map_err(|source| io_command(program, source))?,
        );
        if let Ok(ref out) = res {
            for line in out.stdout.lines() {
                tracing::info!("{line}");
            }
            for line in out.stderr.lines() {
                tracing::info!("{line}");
            }
        }
        res
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
        let mut c = self.command(program, args);
        c.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = c.spawn().map_err(|source| io_command(program, source))?;
        if let Some(mut child_stdin) = child.stdin.take() {
            child_stdin.write_all(stdin.as_ref()).await?;
        }
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();
        tracing::debug!(program, "starting local command with stdin");

        let stdout_task = tokio::spawn(read_pipe_with_trace(program.to_owned(), "stdout", stdout));
        let stderr_task = tokio::spawn(read_pipe_with_trace(program.to_owned(), "stderr", stderr));
        tokio::select! {
            status = child.wait() => {
                let stdout = join_pipe(stdout_task).await?;
                let stderr = join_pipe(stderr_task).await?;
                checked(std::process::Output { status: status?, stdout, stderr })
            },
            _ = cancel.cancelled() => {
                let _ = child.kill().await;
                let _ = child.wait().await;
                Err(ExecError::StreamCancelled)
            }
        }
    }
    pub async fn run_stream(
        &self,
        program: &str,
        args: &[String],
        sender: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<ExecExitStatus> {
        let mut c = self.command(program, args);
        c.stdout(Stdio::piped()).stderr(Stdio::piped());
        let mut child = c.spawn().map_err(|source| io_command(program, source))?;
        let mut stdout = child.stdout.take().unwrap();
        let mut stderr = child.stderr.take().unwrap();
        let tx = sender.clone();
        let out = tokio::spawn(async move {
            let mut b = vec![0; 8192];
            loop {
                let n = stdout.read(&mut b).await?;
                if n == 0 {
                    break;
                }
                if tx
                    .send(ExecStreamEvent::Stdout(b[..n].to_vec()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
            Ok::<_, std::io::Error>(())
        });
        let tx = sender.clone();
        let err = tokio::spawn(async move {
            let mut b = vec![0; 8192];
            loop {
                let n = stderr.read(&mut b).await?;
                if n == 0 {
                    break;
                }
                if tx
                    .send(ExecStreamEvent::Stderr(b[..n].to_vec()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
            Ok::<_, std::io::Error>(())
        });
        let status = tokio::select! {s=child.wait()=>s?,_=sender.closed()=>{child.kill().await?;let _=child.wait().await;return Err(ExecError::StreamCancelled);}};
        out.await
            .map_err(|e| ExecError::Io(std::io::Error::other(e)))??;
        err.await
            .map_err(|e| ExecError::Io(std::io::Error::other(e)))??;
        let status = ExecExitStatus::Local(status);
        if !status.success() {
            return Err(ExecError::CommandFailed {
                code: status.code(),
                stderr: "streamed command failed".into(),
            });
        }
        Ok(status)
    }
}
async fn read_pipe_with_trace(
    program: String,
    stream: &'static str,
    pipe: Option<impl AsyncRead + Unpin>,
) -> Result<Vec<u8>, std::io::Error> {
    let mut data = Vec::new();
    if let Some(mut pipe) = pipe {
        let mut buffer = vec![0; 8192];
        loop {
            let read = pipe.read(&mut buffer).await?;
            if read == 0 {
                break;
            }
            let chunk = &buffer[..read];
            for line in String::from_utf8_lossy(chunk).lines() {
                if !line.trim().is_empty() {
                    tracing::info!(program = %program, stream, line = %line, "command output");
                }
            }
            data.extend_from_slice(chunk);
        }
    }
    Ok(data)
}

async fn join_pipe(
    task: tokio::task::JoinHandle<Result<Vec<u8>, std::io::Error>>,
) -> Result<Vec<u8>, std::io::Error> {
    task.await.map_err(|error| std::io::Error::other(error))?
}
fn checked(output: std::process::Output) -> ExecResult<ExecOutput> {
    let result = ExecOutput {
        status: ExecExitStatus::Local(output.status),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    };
    if !result.success() {
        return Err(ExecError::CommandFailed {
            code: result.status.code(),
            stderr: result.combined_output(),
        });
    }
    Ok(result)
}

fn io_command(program: &str, source: std::io::Error) -> ExecError {
    ExecError::IoCommand {
        program: program.into(),
        source,
    }
}

fn resolve_binary_path(program: &str) -> String {
    if std::path::Path::new(program).is_absolute() {
        return program.to_string();
    }
    let candidates = [
        program.to_string(),
        format!("/usr/bin/{program}"),
        format!("/usr/local/bin/{program}"),
        format!("/sbin/{program}"),
        format!("/usr/sbin/{program}"),
    ];
    for candidate in &candidates {
        if std::path::Path::new(candidate).exists() {
            return candidate.clone();
        }
    }
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path_var) {
            let full_path = dir.join(program);
            if full_path.exists() {
                return full_path.to_string_lossy().to_string();
            }
        }
    }
    program.to_string()
}
