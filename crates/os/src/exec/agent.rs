use super::{ExecError, ExecExitStatus, ExecOutput, ExecResult, ExecStreamEvent};
use async_trait::async_trait;
use std::{ffi::OsStr, sync::Arc};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
pub struct AgentCommandRequest {
    pub program: String,
    pub args: Vec<String>,
    pub stdin: Vec<u8>,
    pub timeout_seconds: u32,
}

#[async_trait]
pub trait AgentTransport: Send + Sync + std::fmt::Debug {
    async fn execute(
        &self,
        request: AgentCommandRequest,
        output: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<i32>;
}

#[derive(Clone)]
pub struct AgentExecutor {
    transport: Arc<dyn AgentTransport>,
    timeout_seconds: u32,
}

impl std::fmt::Debug for AgentExecutor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AgentExecutor")
            .field("timeout_seconds", &self.timeout_seconds)
            .finish_non_exhaustive()
    }
}

impl AgentExecutor {
    pub fn new(transport: Arc<dyn AgentTransport>) -> Self {
        Self {
            transport,
            timeout_seconds: 300,
        }
    }

    pub fn with_timeout_seconds(mut self, timeout_seconds: u32) -> Self {
        self.timeout_seconds = timeout_seconds;
        self
    }

    pub async fn run<I, S>(&self, program: &str, args: I) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.execute(program, args, &[], CancellationToken::new(), None)
            .await
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
        self.execute(
            program,
            args,
            stdin.as_ref(),
            CancellationToken::new(),
            None,
        )
        .await
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
        self.execute(program, args, &[], cancel.clone(), None).await
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
        self.execute(program, args, stdin.as_ref(), cancel.clone(), None)
            .await
    }

    pub async fn run_stream<I, S>(
        &self,
        program: &str,
        args: I,
        sender: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<ExecExitStatus>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let request = self.request(program, args, &[]);
        let code = self.transport.execute(request, sender).await?;
        Ok(ExecExitStatus::Agent(code))
    }

    /// Stop a deployment process recorded by the panel on the Agent host.
    /// This keeps stale-job cleanup on the Agent transport instead of opening
    /// an SSH Docker/API tunnel.
    pub async fn kill_pid_file(&self, pid_file: impl AsRef<OsStr>) -> ExecResult<()> {
        let path = pid_file.as_ref().to_string_lossy().into_owned();
        let command = format!(
            "if test -s \"{path}\"; then kill -TERM \"$(cat \"{path}\")\" 2>/dev/null || true; rm -f \"{path}\"; fi"
        );
        self.run("sh", ["-lc", command.as_str()]).await.map(|_| ())
    }

    async fn execute<I, S>(
        &self,
        program: &str,
        args: I,
        stdin: &[u8],
        cancel: CancellationToken,
        _working_directory: Option<&str>,
    ) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let request = self.request(program, args, stdin);
        let (sender, mut receiver) = mpsc::channel(128);
        let transport = self.transport.clone();
        let task = tokio::spawn(async move { transport.execute(request, sender).await });
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        loop {
            tokio::select! {
                _ = cancel.cancelled() => {
                    task.abort();
                    return Err(ExecError::StreamCancelled);
                }
                event = receiver.recv() => match event {
                    Some(ExecStreamEvent::Stdout(data)) => stdout.extend(data),
                    Some(ExecStreamEvent::Stderr(data)) => stderr.extend(data),
                    None => break,
                }
            }
        }
        let exit_code = tokio::select! {
            _ = cancel.cancelled() => return Err(ExecError::StreamCancelled),
            result = task => result.map_err(|error| ExecError::Agent(error.to_string()))??,
        };
        let output = ExecOutput {
            status: ExecExitStatus::Agent(exit_code),
            stdout: String::from_utf8_lossy(&stdout).into_owned(),
            stderr: String::from_utf8_lossy(&stderr).into_owned(),
        };
        if output.status.success() {
            Ok(output)
        } else {
            Err(ExecError::CommandFailed {
                code: output.status.code(),
                stderr: output.stderr,
            })
        }
    }

    fn request<I, S>(&self, program: &str, args: I, stdin: &[u8]) -> AgentCommandRequest
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        AgentCommandRequest {
            program: program.to_owned(),
            args: args
                .into_iter()
                .map(|value| value.as_ref().to_string_lossy().into_owned())
                .collect(),
            stdin: stdin.to_vec(),
            timeout_seconds: self.timeout_seconds,
        }
    }
}
