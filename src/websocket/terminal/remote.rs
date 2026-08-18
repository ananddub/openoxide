use std::sync::Arc;
use socketioxide::extract::SocketRef;

use super::helpers::{emit_error, emit_terminal_bytes, next_session_id, socket_key};
use super::types::{
    ServerTerminalStart, SessionMap, TerminalExit, TerminalKind, TerminalSession, TerminalStarted,
};

pub async fn spawn_remote_terminal(
    socket: SocketRef,
    sessions: &SessionMap,
    db: &sqlx::SqlitePool,
    server_id: i64,
    input: ServerTerminalStart,
) {
    let key = socket_key(&socket);
    if let Some((_, old_session)) = sessions.remove(&key) {
        let cancel = match old_session {
            TerminalSession::InMemorySsh { cancel, .. } => cancel,
            TerminalSession::DockerSocket { cancel, .. } => cancel,
            _ => tokio_util::sync::CancellationToken::new(),
        };
        cancel.cancel();
    }

    let executor = match crate::services::compose::remote::remote_executor(db, server_id).await {
        Ok(executor) => executor,
        Err(error) => {
            tracing::error!(server_id, %error, "remote_executor failed in spawn_remote_terminal");
            let err_msg = format!("\r\n\x1b[31m[Error] Could not create remote SSH executor: {error}\x1b[0m\r\n");
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes());
            emit_error(
                &socket,
                format!("could not create remote executor: {error}"),
            );
            return;
        }
    };

    let actual_host = executor.host().to_string();
    let cols = input.cols.unwrap_or(80);
    let rows = input.rows.unwrap_or(24);

    let russh_session = match executor.connect_session().await {
        Ok(s) => s,
        Err(error) => {
            let err_msg = format!("\r\n\x1b[31m[Error] Could not connect SSH session: {error}\x1b[0m\r\n");
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes());
            emit_error(&socket, format!("could not connect SSH: {error}"));
            return;
        }
    };

    let terminal = match os::ssh::RusshTerminal::connect(&russh_session, cols, rows, None).await {
        Ok(t) => Arc::new(t),
        Err(error) => {
            let err_msg = format!("\r\n\x1b[31m[Error] Could not open in-memory SSH terminal: {error}\x1b[0m\r\n");
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes());
            emit_error(&socket, format!("could not start in-memory SSH terminal: {error}"));
            return;
        }
    };

    let session_id = next_session_id();
    let cancel = tokio_util::sync::CancellationToken::new();

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
                TerminalSession::InMemorySsh { session_id: sid, .. } => *sid == session_id,
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

pub async fn spawn_remote_docker_terminal(
    socket: SocketRef,
    sessions: &SessionMap,
    db: &sqlx::SqlitePool,
    server_id: i64,
    input: super::types::DockerTerminalStart,
) {
    let key = socket_key(&socket);
    if let Some((_, old_session)) = sessions.remove(&key) {
        let cancel = match old_session {
            TerminalSession::InMemorySsh { cancel, .. } => cancel,
            TerminalSession::DockerSocket { cancel, .. } => cancel,
            _ => tokio_util::sync::CancellationToken::new(),
        };
        cancel.cancel();
    }

    let executor = match crate::services::compose::remote::remote_executor(db, server_id).await {
        Ok(executor) => executor,
        Err(error) => {
            tracing::error!(server_id, %error, "remote_executor failed in spawn_remote_docker_terminal");
            let err_msg = format!("\r\n\x1b[31m[Error] Could not create remote SSH executor: {error}\x1b[0m\r\n");
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes());
            emit_error(
                &socket,
                format!("could not create remote executor: {error}"),
            );
            return;
        }
    };

    let actual_host = executor.host().to_string();
    let shell_req = input.shell.as_deref().unwrap_or("sh").to_string();
    let container_req = input.container.clone();

    let docker_cli = crate::utils::docker::DockerCli::from_remote_executor(executor.clone());
    let mut target_container = input.container.clone();

    if let Ok(containers) = docker_cli.containers().ps().all().list().await {
        let search = input.container.to_lowercase();
        let clean_search = search.trim_end_matches("_db").trim_end_matches("-db");
        if let Some(matching) = containers.iter().find(|c| {
            let n = c.names.to_lowercase();
            n.contains(&search)
                || n.trim_start_matches('/').starts_with(&search)
                || n.contains(&format!("{}_", search))
                || n.contains(&format!("{}.", search))
                || (!clean_search.is_empty() && (n.contains(clean_search) || n.trim_start_matches('/').starts_with(clean_search)))
        }) {
            target_container = matching.names.trim_start_matches('/').to_string();
        }
    }

    let key_str = socket_key(&socket).to_string();
    let docker_cmd = format!("docker exec -it --env OPENOXIDE_SOCKET_ID={} {} {}", key_str, target_container, shell_req);

    let cols = input.cols.unwrap_or(80);
    let rows = input.rows.unwrap_or(24);

    let russh_session = match executor.connect_session().await {
        Ok(s) => s,
        Err(error) => {
            let err_msg = format!("\r\n\x1b[31m[Error] Could not connect SSH session: {error}\x1b[0m\r\n");
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes());
            emit_error(&socket, format!("could not connect SSH: {error}"));
            return;
        }
    };

    let terminal = match os::ssh::RusshTerminal::connect(&russh_session, cols, rows, Some(&docker_cmd)).await {
        Ok(t) => Arc::new(t),
        Err(error) => {
            let err_msg = format!("\r\n\x1b[31m[Error] Could not open in-memory SSH container terminal: {error}\x1b[0m\r\n");
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes());
            emit_error(&socket, format!("could not start in-memory SSH container terminal: {error}"));
            return;
        }
    };

    let session_id = next_session_id();
    let cancel = tokio_util::sync::CancellationToken::new();

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
            kind: TerminalKind::Docker,
            host: Some(&actual_host),
        },
    );

    let term_read = terminal.clone();
    let socket_clone = socket.clone();
    let sessions_clone = sessions.clone();
    let server_host = actual_host.clone();
    let container_log = container_req.clone();
    let cancel_clone = cancel.clone();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancel_clone.cancelled() => {
                    tracing::info!(container = %container_log, host = %server_host, "remote docker terminal session cancelled");
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
                TerminalSession::InMemorySsh { session_id: sid, .. } => *sid == session_id,
                _ => false,
            },
            None => false,
        };

        if is_current {
            sessions_clone.remove(&key);
            tracing::info!(container = %container_log, host = %server_host, "remote docker terminal session exited");
            let _ = socket_clone.emit("exit", &TerminalExit { code: Some(0) });
        }
    });
}
