use std::borrow::Cow;
use dashmap::DashMap;
use pty_process::{OwnedWritePty, Size};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::{
    process::{Child, ChildStdin},
    sync::{Mutex, mpsc},
};

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
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TerminalInputPayload {
    Direct(String),
    Object {
        data: String,
    },
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
pub struct TerminalError {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct TerminalExit {
    pub code: Option<i32>,
}

pub type SessionMap = Arc<DashMap<String, TerminalSession>>;

#[derive(Debug, Clone)]
pub enum TerminalSession {
    Pty {
        writer: Arc<Mutex<OwnedWritePty>>,
        child: Arc<Mutex<Child>>,
        session_id: SessionId,
        cancel: tokio_util::sync::CancellationToken,
    },
    Local {
        stdin: Arc<Mutex<ChildStdin>>,
        child: Arc<Mutex<Child>>,
    },
    Remote {
        input: mpsc::Sender<Vec<u8>>,
        cancel: tokio_util::sync::CancellationToken,
    },
}
