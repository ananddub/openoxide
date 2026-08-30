use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use bytes::Bytes;
use connectrpc::ConnectError;
use dashmap::DashMap;
use futures::Stream;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::api::connect::generated::openoxide::api::v1::{
    TerminalError, TerminalEvent, TerminalExit, TerminalOutputStream,
    TerminalOutput, terminal_event,
};
use crate::services::observability::monitoring::proto::{ShellAction, ShellMessage};

use super::init::TerminalController;

/// Input side of a live terminal session, kept in the registry so the unary
/// `ExecutionCommand` RPC can reach the PTY after the streaming RPC opened it.
/// The agent sender is behind a mutex because `mpsc::Sender` is `!Sync` and
/// `DashMap` values must be `Sync`.
#[derive(Clone)]
pub enum TerminalSessionHandle {
    AgentShell {
        sender: Arc<tokio::sync::Mutex<mpsc::Sender<ShellMessage>>>,
    },
    RusshShell {
        terminal: Arc<os::ssh::RusshTerminal>,
    },
}

/// Reader side of a freshly opened session, consumed by the pump task that
/// forwards PTY output into the event channel.
pub enum TerminalPump {
    Agent { stream: tonic::Streaming<ShellMessage> },
    Russh { terminal: Arc<os::ssh::RusshTerminal> },
}

pub struct TerminalSessionEntry {
    pub owner_user_id: i64,
    pub cancel: CancellationToken,
    pub session: TerminalSessionHandle,
}

pub enum SanitizedShell {
    Sh,
    Bash,
    Custom(String),
}

impl SanitizedShell {
    pub fn parse(value: Option<&str>, default_shell: &'static str) -> Self {
        let value = value.unwrap_or(default_shell).trim();
        match value {
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
            Self::Custom(value) => value,
        }
    }
}

pub fn clamp_size(cols: Option<u32>, rows: Option<u32>) -> (u16, u16) {
    let cols = u16::try_from(cols.unwrap_or(80).clamp(1, 1000)).unwrap_or(80);
    let rows = u16::try_from(rows.unwrap_or(24).clamp(1, 1000)).unwrap_or(24);
    (cols, rows)
}

pub fn terminal_event(session_id: &str, payload: terminal_event::Payload) -> TerminalEvent {
    TerminalEvent {
        session_id: session_id.to_owned(),
        payload: Some(payload),
    }
}

pub fn output_event(
    session_id: &str,
    stream: TerminalOutputStream,
    data: impl Into<Bytes>,
) -> Result<TerminalEvent, ConnectError> {
    Ok(terminal_event(
        session_id,
        terminal_event::Payload::Output(Box::new(TerminalOutput {
            stream: ::buffa::EnumValue::Known(stream),
            data: data.into(),
        })),
    ))
}

pub fn error_event(session_id: &str, message: String) -> Result<TerminalEvent, ConnectError> {
    Ok(terminal_event(
        session_id,
        terminal_event::Payload::Error(Box::new(TerminalError { message, code: None })),
    ))
}

pub fn exit_event(session_id: &str, code: Option<i32>) -> Result<TerminalEvent, ConnectError> {
    Ok(terminal_event(
        session_id,
        terminal_event::Payload::Exit(Box::new(TerminalExit { code })),
    ))
}

pub fn spawn_agent_pump(
    session_id: String,
    mut stream: tonic::Streaming<ShellMessage>,
    events: mpsc::Sender<Result<TerminalEvent, ConnectError>>,
    cancel: CancellationToken,
    sessions: Arc<DashMap<String, TerminalSessionEntry>>,
) {
    tokio::spawn(async move {
        let mut exit_code = Some(0);
        loop {
            tokio::select! {
                _ = cancel.cancelled() => break,
                result = stream.message() => match result {
                    Ok(Some(message)) => match ShellAction::try_from(message.action).unwrap_or_default() {
                        ShellAction::Stdout => {
                            if events.send(output_event(&session_id, TerminalOutputStream::Stdout, message.data)).await.is_err() {
                                break;
                            }
                        }
                        ShellAction::Stderr => {
                            if events.send(output_event(&session_id, TerminalOutputStream::Stderr, message.data)).await.is_err() {
                                break;
                            }
                        }
                        ShellAction::Exit => {
                            exit_code = Some(message.exit_code);
                            break;
                        }
                        ShellAction::Error => {
                            let _ = events.send(error_event(&session_id, message.error)).await;
                            exit_code = Some(-1);
                            break;
                        }
                        _ => {}
                    },
                    Ok(None) => break,
                    Err(error) => {
                        let _ = events
                            .send(error_event(&session_id, format!("Agent shell stream failed: {error}")))
                            .await;
                        exit_code = Some(-1);
                        break;
                    }
                }
            }
        }
        sessions.remove(&session_id);
        let _ = events.send(exit_event(&session_id, exit_code)).await;
    });
}

pub fn spawn_russh_pump(
    session_id: String,
    terminal: Arc<os::ssh::RusshTerminal>,
    events: mpsc::Sender<Result<TerminalEvent, ConnectError>>,
    cancel: CancellationToken,
    sessions: Arc<DashMap<String, TerminalSessionEntry>>,
) {
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancel.cancelled() => break,
                output = terminal.read_next() => match output {
                    Some(bytes) => {
                        if events
                            .send(output_event(&session_id, TerminalOutputStream::Stdout, bytes))
                            .await
                            .is_err()
                        {
                            break;
                        }
                    }
                    None => break,
                }
            }
        }
        sessions.remove(&session_id);
        let _ = events.send(exit_event(&session_id, Some(0))).await;
    });
}

/// Response body of `OpenTerminal`. Dropping the stream (client disconnect
/// or natural end) tears the session down: the registry entry is removed,
/// the pumps are cancelled and the Agent channel receives an explicit
/// close message before its sender is dropped.
pub struct TerminalEventStream {
    pub receiver: mpsc::Receiver<Result<TerminalEvent, ConnectError>>,
    pub sessions: Arc<DashMap<String, TerminalSessionEntry>>,
    pub session_id: String,
}

impl Stream for TerminalEventStream {
    type Item = Result<TerminalEvent, ConnectError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut().receiver.poll_recv(cx)
    }
}

impl Drop for TerminalEventStream {
    fn drop(&mut self) {
        let sessions = Arc::clone(&self.sessions);
        let session_id = std::mem::take(&mut self.session_id);
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            let _ = handle.spawn(async move {
                if let Some((_, entry)) = sessions.remove(&session_id) {
                    TerminalController::terminate(entry).await;
                }
            });
        } else {
            // No runtime (process shutdown): dropping the entry still closes
            // the Agent request channel and the SSH terminal.
            sessions.remove(&session_id);
        }
    }
}
