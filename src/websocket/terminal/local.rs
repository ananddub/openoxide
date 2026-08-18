use std::sync::Arc;

use socketioxide::extract::SocketRef;
use tokio::process::Command;
use tokio::sync::Mutex;

use super::helpers::{emit_error, emit_terminal_bytes, socket_key};
use super::types::{
    DockerTerminalStart, SessionMap, TerminalKind, TerminalSession, TerminalStarted,
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
        emit_error(&socket, "remote docker terminal should be opened with server:start");
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

    let nano = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let exec_id = format!("exec-{nano}");

    let mut cmd = Command::new("ctr");
    cmd.args(["-n", "moby", "task", "exec", "--exec-id", &exec_id, "-t", &target_container, &shell_req]);

    cmd.env("TERM", "xterm-256color")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    spawn_local_terminal(socket, sessions, TerminalKind::Docker, cmd).await;
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
            let err_msg = format!("\r\n\x1b[31m[Error] Could not start local process: {error}\x1b[0m\r\n");
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
