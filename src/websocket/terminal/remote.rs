use socketioxide::extract::SocketRef;
use tokio::sync::mpsc;

use super::helpers::{emit_error, emit_terminal_bytes, socket_key};
use super::types::{ServerTerminalStart, SessionMap, TerminalExit, TerminalSession, TerminalStarted};
use crate::utils::exec::ExecStreamEvent;

pub async fn spawn_remote_terminal(
    socket: SocketRef,
    sessions: &SessionMap,
    db: &sqlx::SqlitePool,
    server_id: i64,
    input: ServerTerminalStart,
) {
    let key = socket_key(&socket);
    let (output_tx, mut output_rx) = mpsc::channel::<ExecStreamEvent>(256);

    let executor = match crate::services::compose::remote::remote_executor(db, server_id).await {
        Ok(executor) => executor,
        Err(error) => {
            emit_error(&socket, format!("could not create remote executor: {error}"));
            return;
        }
    };

    let shell = input.shell.unwrap_or_else(|| "sh".into());
    let terminal = match executor.open_terminal(output_tx, shell, 80, 24).await {
        Ok(terminal) => terminal,
        Err(error) => {
            emit_error(&socket, format!("could not start remote terminal: {error}"));
            return;
        }
    };

    sessions.insert(
        key.clone(),
        TerminalSession::Remote {
            input: terminal.input.clone(),
            resize: terminal.resize.clone(),
            cancel: terminal.cancel.clone(),
        },
    );

    let _ = socket.emit(
        "started",
        &TerminalStarted {
            kind: "remote-server",
        },
    );

    let output_socket = socket.clone();
    tokio::spawn(async move {
        while let Some(event) = output_rx.recv().await {
            match event {
                ExecStreamEvent::Stdout(bytes) => {
                    emit_terminal_bytes(&output_socket, "stdout", bytes)
                }
                ExecStreamEvent::Stderr(bytes) => {
                    emit_terminal_bytes(&output_socket, "stderr", bytes)
                }
            }
        }
    });

    let sessions_clone = sessions.clone();
    let socket_clone = socket.clone();
    tokio::spawn(async move {
        let result = terminal.wait().await;
        sessions_clone.remove(&key);
        match result {
            Ok(()) => {
                let _ = socket_clone.emit("exit", &TerminalExit { code: Some(0) });
            }
            Err(error) => emit_error(&socket_clone, format!("remote terminal exit: {error}")),
        }
    });
}
