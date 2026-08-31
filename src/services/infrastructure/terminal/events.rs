use bytes::Bytes;
use tokio::sync::mpsc;

use crate::api::connect::generated::openoxide::api::v1::{
    TerminalError, TerminalEvent, TerminalExit, TerminalKind, TerminalOutputStream,
    TerminalOutput, TerminalStarted, terminal_event,
};

/// Client-supplied terminal dimensions clamped to sane PTY bounds.
#[derive(Debug, Clone, Copy)]
pub struct TerminalSize {
    pub cols: u16,
    pub rows: u16,
}

impl TerminalSize {
    pub fn from_proto(cols: Option<u32>, rows: Option<u32>) -> Self {
        let cols = u16::try_from(cols.unwrap_or(80).clamp(1, 1000)).unwrap_or(80);
        let rows = u16::try_from(rows.unwrap_or(24).clamp(1, 1000)).unwrap_or(24);
        Self { cols, rows }
    }
}

/// Event channel handle bound to one session id: every emit stamps the right
/// session id and payload shape, so callers never thread ids around. Emits
/// return `false` once the client-side stream is gone.
#[derive(Clone)]
pub struct SessionEvents {
    session_id: String,
    tx: mpsc::Sender<TerminalEvent>,
}

impl SessionEvents {
    pub(super) fn new(session_id: String, tx: mpsc::Sender<TerminalEvent>) -> Self {
        Self { session_id, tx }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    async fn emit(&self, payload: terminal_event::Payload) -> bool {
        self.tx
            .send(TerminalEvent {
                session_id: self.session_id.clone(),
                payload: Some(payload),
            })
            .await
            .is_ok()
    }

    pub async fn emit_started(&self, kind: TerminalKind, host: Option<String>) -> bool {
        self.emit(terminal_event::Payload::Started(Box::new(TerminalStarted {
            kind: ::buffa::EnumValue::Known(kind),
            host,
        })))
        .await
    }

    pub async fn emit_output(&self, stream: TerminalOutputStream, data: impl Into<Bytes>) -> bool {
        self.emit(terminal_event::Payload::Output(Box::new(TerminalOutput {
            stream: ::buffa::EnumValue::Known(stream),
            data: data.into(),
        })))
        .await
    }

    pub async fn emit_error(&self, message: String) -> bool {
        self.emit(terminal_event::Payload::Error(Box::new(TerminalError {
            message,
            code: None,
        })))
        .await
    }

    pub async fn emit_exit(&self, code: Option<i32>) -> bool {
        self.emit(terminal_event::Payload::Exit(Box::new(TerminalExit { code })))
            .await
    }
}
