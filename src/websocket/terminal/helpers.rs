use pty_process::OwnedReadPty;
use socketioxide::extract::SocketRef;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::io::AsyncReadExt;

use super::types::{SessionId, SocketKey, TerminalError, TerminalOutput};

static NEXT_SESSION_ID: AtomicU64 = AtomicU64::new(1);

pub fn next_session_id() -> SessionId {
    SessionId(NEXT_SESSION_ID.fetch_add(1, Ordering::SeqCst))
}

pub fn socket_key(socket: &SocketRef) -> SocketKey {
    SocketKey(socket.id.to_string())
}

pub fn emit_terminal_bytes(socket: &SocketRef, stream: &'static str, bytes: &[u8]) {
    let data = String::from_utf8_lossy(bytes);
    let _ = socket.emit("output", &TerminalOutput { stream, data });
}

pub fn emit_error(socket: &SocketRef, message: impl Into<String>) {
    let message_str = message.into();
    tracing::warn!("Terminal socket error emitted: {message_str}");
    let _ = socket.emit(
        "error",
        &TerminalError {
            message: std::borrow::Cow::Owned(message_str),
            code: None,
        },
    );
}

pub fn spawn_output_task(
    socket: SocketRef,
    stream: &'static str,
    mut reader: impl tokio::io::AsyncRead + Unpin + Send + 'static,
) {
    tokio::spawn(async move {
        let mut buffer = vec![0u8; 8192];
        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => return,
                Ok(n) => emit_terminal_bytes(&socket, stream, &buffer[..n]),
                Err(error) => {
                    tracing::warn!("Terminal output read error: {error}");
                    emit_error(&socket, format!("terminal read failed: {error}"));
                    return;
                }
            }
        }
    });
}

pub fn spawn_pty_reader(socket: SocketRef, mut reader: OwnedReadPty) {
    tokio::spawn(async move {
        let mut buffer = vec![0u8; 8192];
        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => emit_terminal_bytes(&socket, "stdout", &buffer[..n]),
                Err(error) => {
                    let is_pty_closed = error
                        .raw_os_error()
                        .map(|code| code == 5)
                        .unwrap_or(false);
                    if !is_pty_closed {
                        tracing::warn!("PTY reader stream error: {error}");
                        emit_error(&socket, format!("PTY stream read error: {error}"));
                    }
                    break;
                }
            }
        }
    });
}
