use std::sync::Arc;

use socketioxide::extract::SocketRef;
use tokio::process::Command;
use tokio::sync::Mutex;

use super::helpers::{emit_error, emit_terminal_bytes, socket_key};
use super::types::{
    DockerTerminalStart, SessionMap, TerminalExit, TerminalKind, TerminalSession, TerminalStarted,
};

pub async fn spawn_docker_terminal(
    socket: SocketRef,
    sessions: &SessionMap,
    input: DockerTerminalStart,
) {
    let key = socket_key(&socket);
    if input.server_id.is_some() {
        let msg = "\r\n\x1b[31m[Error] Remote docker terminal should be opened with server:start\x1b[0m\r\n";
        emit_terminal_bytes(&socket, "stdout", msg.as_bytes());
        emit_error(
            &socket,
            "remote docker terminal should be opened with server:start",
        );
        return;
    }

    let shell_req = input.shell.as_deref().unwrap_or("sh").to_string();
    let mut target_container = input.container.clone();

    let docker = crate::utils::docker::DockerCli::new_local();
    let mut container_found = false;

    if let Ok(containers) = docker.containers().ps().list().await {
        let search = input.container.to_lowercase();
        if let Some(matching) = containers.iter().find(|c| {
            let n = c.names.to_lowercase();
            n.contains(&search)
                || n.trim_start_matches('/').starts_with(&search)
                || n.contains(&format!("{}_", search))
                || n.contains(&format!("{}.", search))
        }) {
            target_container = matching.names.trim_start_matches('/').to_string();
            container_found = true;
        }
    }

    if !container_found && !input.container.is_empty() {
        target_container = input.container.clone();
    }

    let key_str = socket_key(&socket).to_string();
    let socket_stream = match docker
        .containers()
        .exec(&target_container)
        .socket_stream()
        .cmd([&shell_req])
        .env("TERM", "xterm-256color")
        .env("OPENOXIDE_SOCKET_ID", &key_str)
        .size(input.cols.unwrap_or(80), input.rows.unwrap_or(24))
        .connect()
        .await
    {
        Ok(stream) => stream,
        Err(error) => {
            let err_msg = format!(
                "\r\n\x1b[31m[Error] Failed to connect to docker exec socket stream for '{target_container}': {error}\x1b[0m\r\n"
            );
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes());
            emit_error(
                &socket,
                format!("could not start docker socket stream: {error}"),
            );
            return;
        }
    };

    let _ = socket_stream
        .resize(input.cols.unwrap_or(80), input.rows.unwrap_or(24))
        .await;

    let cancel = tokio_util::sync::CancellationToken::new();
    let writer_arc = Arc::new(Mutex::new(socket_stream.writer));
    let exec_id = socket_stream.exec_id.clone();
    let socket_path = socket_stream.socket_path.clone();

    sessions.insert(
        key.clone(),
        TerminalSession::DockerSocket {
            writer: writer_arc,
            cancel: cancel.clone(),
            container: Some(target_container.clone()),
            exec_id: exec_id.clone(),
            socket_path: socket_path.clone(),
        },
    );

    let _ = socket.emit(
        "started",
        &TerminalStarted {
            kind: TerminalKind::Docker,
            host: Some(&target_container),
        },
    );

    let mut reader = socket_stream.reader;
    let socket_clone = socket.clone();
    let cancel_clone = cancel.clone();
    let sessions_clone = sessions.clone();
    let key_clone = key.clone();

    tokio::spawn(async move {
        let mut buf = [0u8; 4096];
        loop {
            tokio::select! {
                _ = cancel_clone.cancelled() => break,
                res = tokio::io::AsyncReadExt::read(&mut reader, &mut buf) => {
                    match res {
                        Ok(0) => break,
                        Ok(n) => {
                            emit_terminal_bytes(&socket_clone, "stdout", &buf[..n]);
                        }
                        Err(_) => break,
                    }
                }
            }
        }

        let is_current = match sessions_clone.get(&key_clone) {
            Some(entry) => match entry.value() {
                TerminalSession::DockerSocket { exec_id: sid, .. } => sid == &exec_id,
                _ => false,
            },
            None => false,
        };

        if is_current {
            sessions_clone.remove(&key_clone);
            let _ = socket_clone.emit("exit", &TerminalExit { code: Some(0) });
        }
    });
}

pub async fn spawn_local_terminal(
    socket: SocketRef,
    sessions: &SessionMap,
    kind: TerminalKind,
    mut command: Command,
) {
    let key = socket_key(&socket);

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            let err_msg =
                format!("\r\n\x1b[31m[Error] Could not start local process: {error}\x1b[0m\r\n");
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes());
            emit_error(&socket, format!("could not start terminal: {error}"));
            return;
        }
    };

    let stdin = child
        .stdin
        .take()
        .expect("stdin was configured with piped()");
    let stdout = child
        .stdout
        .take()
        .expect("stdout was configured with piped()");
    let stderr = child
        .stderr
        .take()
        .expect("stderr was configured with piped()");

    sessions.insert(
        key.clone(),
        TerminalSession::Local {
            stdin: Arc::new(Mutex::new(stdin)),
            child: Arc::new(Mutex::new(child)),
        },
    );

    let _ = socket.emit(
        "started",
        &TerminalStarted {
            kind,
            host: Some("local"),
        },
    );

    super::helpers::spawn_output_task(socket.clone(), "stdout", stdout);
    super::helpers::spawn_output_task(socket.clone(), "stderr", stderr);
}
