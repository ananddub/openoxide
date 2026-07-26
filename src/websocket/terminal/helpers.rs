use pty_process::OwnedReadPty;
use socketioxide::extract::SocketRef;
use tokio::io::AsyncReadExt;

use super::types::{TerminalError, TerminalOutput};

pub fn socket_key(socket: &SocketRef) -> String {
    socket.id.to_string()
}

pub fn emit_terminal_bytes(socket: &SocketRef, stream: &'static str, bytes: Vec<u8>) {
    let data = String::from_utf8_lossy(&bytes).into_owned();
    let _ = socket.emit("output", &TerminalOutput { stream, data });
}

pub fn emit_error(socket: &SocketRef, message: impl Into<String>) {
    let message_str = message.into();
    tracing::warn!("Terminal socket error emitted: {message_str}");
    let _ = socket.emit(
        "error",
        &TerminalError {
            message: message_str,
        },
    );
}

pub fn spawn_output_task(
    socket: SocketRef,
    stream: &'static str,
    mut reader: impl tokio::io::AsyncRead + Unpin + Send + 'static,
) {
    tokio::spawn(async move {
        let mut buffer = vec![0; 8192];
        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => return,
                Ok(n) => emit_terminal_bytes(&socket, stream, buffer[..n].to_vec()),
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
                Ok(n) => emit_terminal_bytes(&socket, "stdout", buffer[..n].to_vec()),
                Err(error) => {
                    tracing::warn!("PTY reader stream error: {error}");
                    emit_error(&socket, format!("PTY stream read error: {error}"));
                    break;
                }
            }
        }
    });
}
