use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::Arc;
use tokio::{
    process::{Child, ChildStdin},
    sync::Mutex,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub rows: u16,
    pub cols: u16,
}

impl Size {
    pub fn new(rows: u16, cols: u16) -> Self {
        Self { rows, cols }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SocketKey(pub String);

impl std::fmt::Display for SocketKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SanitizedShell {
    Sh,
    Bash,
    Custom(String),
}

impl SanitizedShell {
    pub fn parse(s: Option<&str>, default_shell: &'static str) -> Self {
        let str_val = s.unwrap_or(default_shell).trim();
        match str_val {
            "sh" | "/bin/sh" => Self::Sh,
            "bash" | "/bin/bash" => Self::Bash,
            other => {
                let clean: String = other
                    .chars()
                    .filter(|c| c.is_alphanumeric() || *c == '/' || *c == '-' || *c == '_')
                    .collect();
                if clean.is_empty() {
                    if default_shell == "bash" {
                        Self::Bash
                    } else {
                        Self::Sh
                    }
                } else {
                    Self::Custom(clean)
                }
            }
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Sh => "sh",
            Self::Bash => "bash",
            Self::Custom(s) => s.as_str(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionId(pub u64);

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TerminalKind {
    Docker,
    RemoteServer,
    Server,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct TerminalSize {
    #[serde(default = "default_cols")]
    pub cols: u16,
    #[serde(default = "default_rows")]
    pub rows: u16,
}

fn default_cols() -> u16 {
    80
}

fn default_rows() -> u16 {
    24
}

impl TerminalSize {
    pub fn sanitize(&self) -> Size {
        Size::new(self.rows.clamp(1, 1000), self.cols.clamp(1, 1000))
    }
}

#[derive(Debug, Deserialize)]
pub struct DockerTerminalStart {
    #[serde(default = "default_container")]
    pub container: String,
    pub shell: Option<String>,
    #[serde(alias = "serverId")]
    pub server_id: Option<i64>,
    pub cols: Option<u16>,
    pub rows: Option<u16>,
}

fn default_container() -> String {
    "app".to_string()
}

impl DockerTerminalStart {
    pub fn size(&self) -> Size {
        TerminalSize {
            cols: self.cols.unwrap_or(80),
            rows: self.rows.unwrap_or(24),
        }
        .sanitize()
    }

    pub fn shell(&self) -> SanitizedShell {
        SanitizedShell::parse(self.shell.as_deref(), "sh")
    }
}

#[derive(Debug, Deserialize)]
pub struct ServerTerminalStart {
    pub shell: Option<String>,
    #[serde(alias = "command")]
    pub command: Option<String>,
    #[serde(alias = "serverId")]
    pub server_id: Option<i64>,
    pub cols: Option<u16>,
    pub rows: Option<u16>,
}

impl ServerTerminalStart {
    pub fn size(&self) -> Size {
        TerminalSize {
            cols: self.cols.unwrap_or(80),
            rows: self.rows.unwrap_or(24),
        }
        .sanitize()
    }

    pub fn shell(&self) -> SanitizedShell {
        let req = self.shell.as_deref().or(self.command.as_deref());
        SanitizedShell::parse(req, "bash")
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TerminalInputPayload {
    Direct(String),
    Object { data: String },
}

#[derive(Debug, Deserialize)]
pub struct TerminalResize {
    pub cols: Option<u16>,
    pub rows: Option<u16>,
}

impl TerminalResize {
    pub fn size(&self) -> Size {
        TerminalSize {
            cols: self.cols.unwrap_or(80),
            rows: self.rows.unwrap_or(24),
        }
        .sanitize()
    }
}

#[derive(Debug, Serialize)]
pub struct TerminalOutput<'a> {
    pub stream: &'a str,
    pub data: Cow<'a, str>,
}

#[derive(Debug, Serialize)]
pub struct TerminalStarted<'a> {
    pub kind: TerminalKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct TerminalError<'a> {
    pub message: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct TerminalExit {
    pub code: Option<i32>,
}

pub type SessionMap = Arc<DashMap<SocketKey, TerminalSession>>;

#[derive(Debug, Clone)]
pub enum TerminalSession {
    Local {
        stdin: Arc<Mutex<ChildStdin>>,
        child: Arc<Mutex<Child>>,
    },
    InMemorySsh {
        terminal: Arc<os::ssh::RusshTerminal>,
        session_id: SessionId,
        cancel: tokio_util::sync::CancellationToken,
    },
    DockerSocket {
        writer: Arc<Mutex<tokio::net::unix::OwnedWriteHalf>>,
        cancel: tokio_util::sync::CancellationToken,
        container: Option<String>,
        exec_id: String,
        socket_path: String,
    },
}
