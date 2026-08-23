use socketioxide::extract::SocketRef;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::helpers::{emit_error, emit_terminal_bytes, next_session_id, socket_key};
use super::types::{
    DockerTerminalStart, ServerTerminalStart, SessionMap, TerminalExit, TerminalKind,
    TerminalSession, TerminalStarted,
};

pub async fn spawn_remote_docker_terminal_api(
    socket: SocketRef,
    sessions: &SessionMap,
    server_id: i64,
    input: DockerTerminalStart,
    start_token: tokio_util::sync::CancellationToken,
) {
    let key = socket_key(&socket);
    if start_token.is_cancelled() {
        return;
    }
    let monitoring = match auto_di::resolve::<
        crate::services::monitoring::monitoring_service::MonitoringService,
    >()
    .await
    {
        Ok(service) => service,
        Err(error) => {
            emit_error(&socket, format!("could not resolve Agent service: {error}"));
            return;
        }
    };
    let size = input.size();
    let open = crate::services::monitoring::proto::ShellMessage {
        server_id,
        action: crate::services::monitoring::proto::ShellAction::Open as i32,
        container: input.container.clone(),
        shell: input.shell().as_str().to_owned(),
        cols: size.cols as u32,
        rows: size.rows as u32,
        ..Default::default()
    };
    let (sender, mut stream) = match monitoring.open_agent_shell(server_id, open).await {
        Ok(session) => session,
        Err(error) => {
            emit_error(
                &socket,
                format!("could not start Agent Docker shell: {error}"),
            );
            return;
        }
    };
    let cancel = start_token;
    let sender = Arc::new(Mutex::new(sender));
    let session_id = next_session_id();
    sessions.insert(
        key.clone(),
        TerminalSession::AgentShell {
            sender,
            session_id,
            cancel: cancel.clone(),
        },
    );
    let _ = socket.emit(
        "started",
        &TerminalStarted {
            kind: TerminalKind::Docker,
            host: Some("agent"),
        },
    );
    let socket_clone = socket.clone();
    let sessions_clone = sessions.clone();
    tokio::spawn(async move {
        let mut exit_code = Some(0);
        loop {
            tokio::select! {
                _ = cancel.cancelled() => break,
                result = stream.message() => match result {
                    Ok(Some(message)) => match crate::services::monitoring::proto::ShellAction::try_from(message.action).unwrap_or_default() {
                        crate::services::monitoring::proto::ShellAction::Stdout => emit_terminal_bytes(&socket_clone, "stdout", &message.data),
                        crate::services::monitoring::proto::ShellAction::Stderr => emit_terminal_bytes(&socket_clone, "stderr", &message.data),
                        crate::services::monitoring::proto::ShellAction::Exit => {
                            exit_code = Some(message.exit_code);
                            break;
                        }
                        crate::services::monitoring::proto::ShellAction::Error => {
                            emit_error(&socket_clone, message.error);
                            exit_code = Some(-1);
                            break;
                        }
                        _ => {}
                    },
                    Ok(None) => break,
                    Err(error) => {
                        emit_error(&socket_clone, format!("Agent shell stream failed: {error}"));
                        exit_code = Some(-1);
                        break;
                    }
                }
            }
        }
        let current = sessions_clone.get(&key).is_some_and(|entry| {
            matches!(entry.value(), TerminalSession::AgentShell { session_id: id, .. } if *id == session_id)
        });
        if current {
            sessions_clone.remove(&key);
            let _ = socket_clone.emit("exit", &TerminalExit { code: exit_code });
        }
    });
}

pub async fn spawn_remote_terminal(
    socket: SocketRef,
    sessions: &SessionMap,
    db: &sqlx::SqlitePool,
    server_id: i64,
    input: ServerTerminalStart,
    start_token: tokio_util::sync::CancellationToken,
) {
    let key = socket_key(&socket);
    if start_token.is_cancelled() {
        return;
    }

    let executor = match crate::services::compose::remote::remote_executor(db, server_id).await {
        Ok(executor) => executor,
        Err(error) => {
            if start_token.is_cancelled() {
                return;
            }
            tracing::error!(server_id, %error, "remote_executor failed in spawn_remote_terminal");
            let err_msg = format!(
                "\r\n\x1b[31m[Error] Could not create remote SSH executor: {error}\x1b[0m\r\n"
            );
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes());
            emit_error(
                &socket,
                format!("could not create remote executor: {error}"),
            );
            return;
        }
    };

    if start_token.is_cancelled() {
        return;
    }

    let actual_host = executor.host().to_string();
    let cols = input.cols.unwrap_or(80);
    let rows = input.rows.unwrap_or(24);

    let russh_session = match executor.connect_fresh_session().await {
        Ok(s) => s,
        Err(error) => {
            if start_token.is_cancelled() {
                return;
            }
            let _ = socket.emit(
                "started",
                &TerminalStarted {
                    kind: TerminalKind::RemoteServer,
                    host: Some(&actual_host),
                },
            );
            let err_msg = format!(
                "\r\n\x1b[31m[Error] Could not connect SSH session to {actual_host}: {error}\x1b[0m\r\n"
            );
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes());
            emit_error(&socket, format!("could not connect SSH: {error}"));
            return;
        }
    };

    if start_token.is_cancelled() {
        return;
    }

    // For host SSH terminals, pass None so russh requests the native interactive login shell (request_shell)
    let shell_cmd = match input.shell.as_deref() {
        Some("sh") | Some("bash") | Some("") | None => None,
        Some(cmd) => Some(cmd),
    };
    let terminal =
        match os::ssh::RusshTerminal::connect(&russh_session, cols, rows, shell_cmd).await {
            Ok(t) => Arc::new(t),
            Err(error) => {
                if start_token.is_cancelled() {
                    return;
                }
                let err_msg = format!(
                    "\r\n\x1b[31m[Error] Could not open in-memory SSH terminal: {error}\x1b[0m\r\n"
                );
                emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes());
                emit_error(
                    &socket,
                    format!("could not start in-memory SSH terminal: {error}"),
                );
                return;
            }
        };

    if start_token.is_cancelled() {
        return;
    }

    let session_id = next_session_id();
    let cancel = start_token.clone();

    sessions.insert(
        key.clone(),
        TerminalSession::InMemorySsh {
            terminal: terminal.clone(),
            session_id,
            cancel: cancel.clone(),
        },
    );

    let _ = socket.emit(
        "started",
        &TerminalStarted {
            kind: TerminalKind::RemoteServer,
            host: Some(&actual_host),
        },
    );

    let term_read = terminal.clone();
    let socket_clone = socket.clone();
    let sessions_clone = sessions.clone();
    let server_host = actual_host.clone();
    let cancel_clone = cancel.clone();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancel_clone.cancelled() => {
                    tracing::info!(host = %server_host, "in-memory SSH remote terminal session cancelled");
                    break;
                }
                next_chunk = term_read.read_next() => {
                    match next_chunk {
                        Some(data) => emit_terminal_bytes(&socket_clone, "stdout", &data),
                        None => break,
                    }
                }
            }
        }

        let is_current = match sessions_clone.get(&key) {
            Some(entry) => match entry.value() {
                TerminalSession::InMemorySsh {
                    session_id: sid, ..
                } => *sid == session_id,
                _ => false,
            },
            None => false,
        };

        if is_current {
            sessions_clone.remove(&key);
            tracing::info!(host = %server_host, "in-memory russh remote terminal session exited");
            let _ = socket_clone.emit("exit", &TerminalExit { code: Some(0) });
        }
    });
}
