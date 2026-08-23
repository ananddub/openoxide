use socketioxide::extract::SocketRef;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;

use super::helpers::{emit_error, emit_terminal_bytes, next_session_id, socket_key};
use super::types::{
    DockerTerminalStart, ServerTerminalStart, SessionMap, TerminalExit, TerminalKind,
    TerminalSession, TerminalStarted,
};

pub async fn spawn_remote_docker_terminal_api(
    socket: SocketRef,
    sessions: &SessionMap,
    db: &sqlx::SqlitePool,
    server_id: i64,
    input: DockerTerminalStart,
    start_token: tokio_util::sync::CancellationToken,
) {
    let key = socket_key(&socket);
    if start_token.is_cancelled() {
        return;
    }
    let executor = match crate::services::compose::remote::remote_executor(db, server_id).await {
        Ok(executor) => executor,
        Err(error) => {
            emit_error(
                &socket,
                format!("could not create remote executor: {error}"),
            );
            return;
        }
    };
    let docker = crate::utils::docker::DockerCli::from_remote_executor(executor);
    let shell = input.shell.as_deref().unwrap_or("sh").to_owned();
    let mut target = input.container.clone();
    if let Ok(containers) = docker.containers().ps().all().api().list().await {
        let search = target.to_lowercase();
        if let Some(found) = containers.iter().find(|container| {
            let name = container.names.to_lowercase();
            name.contains(&search)
                || name.trim_start_matches('/').starts_with(&search)
                || name.contains(&format!("{}_", search))
                || name.contains(&format!("{}.", search))
        }) {
            target = if found.id.is_empty() {
                found.names.trim_start_matches('/').to_owned()
            } else {
                found.id.clone()
            };
        }
    }
    let key_str = socket_key(&socket).to_string();
    let stream = match docker
        .containers()
        .exec(&target)
        .interactive()
        .tty(true)
        .api()
        .env("TERM", "xterm-256color")
        .env("OPENOXIDE_SOCKET_ID", &key_str)
        .shell([shell.as_str()])
        .await
    {
        Ok(stream) => stream,
        Err(error) => {
            emit_error(
                &socket,
                format!("could not start remote Docker API shell: {error}"),
            );
            return;
        }
    };
    let cols = input.cols.unwrap_or(80);
    let rows = input.rows.unwrap_or(24);
    let _ = stream.resize(cols, rows).await;
    let cancel = start_token;
    let writer = Arc::new(Mutex::new(stream.writer));
    let exec_id = stream.exec_id.clone();
    let socket_path = stream.socket_path.clone();
    let api_lease = stream.lease.map(Arc::new);
    sessions.insert(
        key.clone(),
        TerminalSession::DockerSocket {
            writer,
            cancel: cancel.clone(),
            container: Some(target.clone()),
            exec_id: exec_id.clone(),
            socket_path: socket_path.clone(),
            api_lease,
        },
    );
    let _ = socket.emit(
        "started",
        &TerminalStarted {
            kind: TerminalKind::Docker,
            host: Some("remote"),
        },
    );
    let mut reader = stream.reader;
    let socket_clone = socket.clone();
    let sessions_clone = sessions.clone();
    tokio::spawn(async move {
        let mut buf = [0u8; 4096];
        loop {
            tokio::select! {
                _ = cancel.cancelled() => break,
                result = reader.read(&mut buf) => match result {
                    Ok(0) | Err(_) => break,
                    Ok(n) => emit_terminal_bytes(&socket_clone, "stdout", &buf[..n]),
                }
            }
        }
        let current = sessions_clone.get(&key).is_some_and(|entry| {
            matches!(entry.value(), TerminalSession::DockerSocket { exec_id: id, .. } if id == &exec_id)
        });
        if current {
            sessions_clone.remove(&key);
            let _ = socket_clone.emit("exit", &TerminalExit { code: Some(0) });
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

pub async fn spawn_remote_docker_terminal(
    socket: SocketRef,
    sessions: &SessionMap,
    db: &sqlx::SqlitePool,
    server_id: i64,
    input: super::types::DockerTerminalStart,
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
            tracing::error!(server_id, %error, "remote_executor failed in spawn_remote_docker_terminal");
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
    let shell_req = input.shell.as_deref().unwrap_or("sh").to_string();
    let container_req = input.container.clone();

    let docker_cli = crate::utils::docker::DockerCli::from_remote_executor(executor.clone());
    let mut target_container = input.container.clone();

    if let Ok(containers) = docker_cli.containers().ps().all().api().list().await {
        let search = input.container.to_lowercase();
        let clean_search = search.trim_end_matches("_db").trim_end_matches("-db");
        if let Some(matching) = containers.iter().find(|c| {
            let n = c.names.to_lowercase();
            n.contains(&search)
                || n.trim_start_matches('/').starts_with(&search)
                || n.contains(&format!("{}_", search))
                || n.contains(&format!("{}.", search))
                || (!clean_search.is_empty()
                    && (n.contains(clean_search)
                        || n.trim_start_matches('/').starts_with(clean_search)))
        }) {
            target_container = matching.names.trim_start_matches('/').to_string();
        }
    }

    if start_token.is_cancelled() {
        return;
    }

    let key_str = socket_key(&socket).to_string();
    let docker_cmd = format!(
        "docker exec -it --env OPENOXIDE_SOCKET_ID={} {} {}",
        key_str, target_container, shell_req
    );

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
                    kind: TerminalKind::Docker,
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

    let terminal = match os::ssh::RusshTerminal::connect(
        &russh_session,
        cols,
        rows,
        Some(&docker_cmd),
    )
    .await
    {
        Ok(t) => Arc::new(t),
        Err(error) => {
            if start_token.is_cancelled() {
                return;
            }
            let err_msg = format!(
                "\r\n\x1b[31m[Error] Could not open in-memory SSH container terminal: {error}\x1b[0m\r\n"
            );
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes());
            emit_error(
                &socket,
                format!("could not start in-memory SSH container terminal: {error}"),
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
                TerminalSession::InMemorySsh {
                    session_id: sid, ..
                } => *sid == session_id,
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
