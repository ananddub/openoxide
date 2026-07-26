use super::{ExecError, ExecResult, LocalExecutor, RemoteExecutor};
use std::{ffi::OsStr, process::ExitStatus};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub enum ExecExitStatus {
    Local(ExitStatus),
    Remote(u32),
}
impl ExecExitStatus {
    pub fn success(&self) -> bool {
        match self {
            Self::Local(s) => s.success(),
            Self::Remote(c) => *c == 0,
        }
    }
    pub fn code(&self) -> Option<i32> {
        match self {
            Self::Local(s) => s.code(),
            Self::Remote(c) => Some(*c as i32),
        }
    }
}

#[derive(Debug)]
pub struct ExecOutput {
    pub status: ExecExitStatus,
    pub stdout: String,
    pub stderr: String,
}
impl ExecOutput {
    pub fn success(&self) -> bool {
        self.status.success()
    }
    pub fn stdout_trimmed(&self) -> &str {
        self.stdout.trim()
    }
    pub fn combined_output(&self) -> String {
        let stdout = self.stdout.trim();
        let stderr = self.stderr.trim();
        match (stdout.is_empty(), stderr.is_empty()) {
            (true, true) => String::new(),
            (true, false) => stderr.to_owned(),
            (false, true) => stdout.to_owned(),
            (false, false) => format!("{stdout}\n{stderr}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExecStreamEvent {
    Stdout(Vec<u8>),
    Stderr(Vec<u8>),
}

#[derive(Clone, Debug)]
pub enum SshAuth {
    Password(String),
    KeyPair {
        private_key: String,
        public_key: Option<String>,
        passphrase: Option<String>,
    },
    Agent,
    AgentWithSocket(std::path::PathBuf),
    KeyFile(std::path::PathBuf),
}

impl zeroize::Zeroize for SshAuth {
    fn zeroize(&mut self) {
        match self {
            SshAuth::Password(p) => p.zeroize(),
            SshAuth::KeyPair { private_key, public_key, passphrase } => {
                private_key.zeroize();
                if let Some(pk) = public_key { pk.zeroize(); }
                if let Some(pp) = passphrase { pp.zeroize(); }
            }
            _ => {}
        }
    }
}

impl Drop for SshAuth {
    fn drop(&mut self) {
        zeroize::Zeroize::zeroize(self);
    }
}
impl SshAuth {
    pub fn password(value: impl Into<String>) -> Self {
        Self::Password(value.into())
    }
    pub fn key_pair(private_key: impl Into<String>, public_key: impl Into<String>) -> Self {
        Self::KeyPair {
            private_key: private_key.into(),
            public_key: Some(public_key.into()),
            passphrase: None,
        }
    }
    pub fn private_key(private_key: impl Into<String>) -> Self {
        Self::KeyPair {
            private_key: private_key.into(),
            public_key: None,
            passphrase: None,
        }
    }
    pub fn key_file(path: impl Into<std::path::PathBuf>) -> Self {
        Self::KeyFile(path.into())
    }
    pub fn agent_with_socket(path: impl Into<std::path::PathBuf>) -> Self {
        Self::AgentWithSocket(path.into())
    }
    pub fn with_passphrase(mut self, value: impl Into<String>) -> Self {
        if let Self::KeyPair { passphrase, .. } = &mut self {
            *passphrase = Some(value.into());
        }
        self
    }
}

#[derive(Clone, Debug)]
pub enum SshHostKey {
    PinnedSha256(String),
    InsecureAcceptAny,
}

#[derive(Clone, Debug)]
pub enum CommandExecutor {
    Local(LocalExecutor),
    Remote(RemoteExecutor),
}
impl CommandExecutor {
    pub async fn run<I, S>(&self, program: &str, args: I) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        match self {
            Self::Local(e) => e.run(program, args).await,
            Self::Remote(e) => e.run(program, args).await,
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
        let args = args
            .into_iter()
            .map(|v| v.as_ref().to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        match self {
            Self::Local(e) => e.run_with_stdin(program, &args, stdin).await,
            Self::Remote(e) => e.run_with_stdin(program, &args, stdin).await,
        }
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
        let args = args
            .into_iter()
            .map(|v| v.as_ref().to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        match self {
            Self::Local(e) => e.run_cancelled(program, &args, cancel).await,
            Self::Remote(e) => e.run_cancelled(program, &args, cancel).await,
        }
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
        let args = args
            .into_iter()
            .map(|v| v.as_ref().to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        match self {
            Self::Local(e) => {
                e.run_with_stdin_cancelled(program, &args, stdin, cancel)
                    .await
            }
            Self::Remote(e) => {
                e.run_with_stdin_cancelled(program, &args, stdin, cancel)
                    .await
            }
        }
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
        let args = args
            .into_iter()
            .map(|v| v.as_ref().to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        match self {
            Self::Local(e) => e.run_stream(program, &args, sender).await,
            Self::Remote(e) => e.run_stream(program, &args, sender).await,
        }
    }

    pub async fn run_cancelled_in_cgroup<I, S>(
        &self,
        cgroup_path: Option<&str>,
        program: &str,
        args: I,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let args = args
            .into_iter()
            .map(|v| v.as_ref().to_string_lossy().into_owned())
            .collect::<Vec<String>>();

        let Some(path) = cgroup_path else {
            return self.run_cancelled(program, &args, cancel).await;
        };

        let procs_path = format!("{}/cgroup.procs", path);
        let wrapper = format!(
            "echo $$ > {procs} || {{ echo \"CGROUP_ASSIGN_FAILED\" >&2; exit 1; }}; exec \"$@\"",
            procs = procs_path
        );

        let mut sh_args = vec![
            "-c".to_string(),
            wrapper,
            "--".to_string(),
            program.to_string(),
        ];
        sh_args.extend(args);

        match self.run_cancelled("sh", &sh_args, cancel).await {
            Ok(output) => Ok(output),
            Err(e) => {
                let err_str = e.to_string();
                if err_str.contains("CGROUP_ASSIGN_FAILED") {
                    Err(ExecError::CommandFailed {
                        code: Some(1),
                        stderr: "CGROUP_ASSIGN_FAILED: cgroup assignment failed (permission denied or cgroup missing)".into(),
                    })
                } else {
                    Err(e)
                }
            }
        }
    }

    pub async fn write_file(&self, path: &str, content: &str) -> ExecResult<ExecOutput> {
        self.run_with_stdin("tee", &[path], content).await
    }

    pub async fn read_file(&self, path: &str) -> ExecResult<ExecOutput> {
        self.run("cat", &[path]).await
    }
}
