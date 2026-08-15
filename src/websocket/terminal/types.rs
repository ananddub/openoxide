use dashmap::DashMap;
use pty_process::OwnedWritePty;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::{
    process::{Child, ChildStdin},
    sync::{Mutex, mpsc},
};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Deserialize)]
pub struct DockerTerminalStart {
    pub container: String,
    pub shell: Option<String>,
    pub server_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ServerTerminalStart {
    pub shell: Option<String>,
    pub server_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct TerminalInput {
    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct TerminalResize {
    pub cols: Option<u16>,
    pub rows: Option<u16>,
}

#[derive(Debug, Serialize)]
pub struct TerminalOutput<'a> {
    pub stream: &'a str,
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct TerminalStarted<'a> {
    pub kind: &'a str,
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
        session_id: u64,
    },
    Local {
        stdin: Arc<Mutex<ChildStdin>>,
        child: Arc<Mutex<Child>>,
    },
    Remote {
        input: mpsc::Sender<Vec<u8>>,
        cancel: CancellationToken,
    },
}
