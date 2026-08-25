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
    spawn_agent_shell(
        socket,
        sessions,
        server_id,
        input,
        start_token,
        TerminalKind::Docker,
    )
    .await;
}

async fn spawn_agent_shell(
    socket: SocketRef,
    sessions: &SessionMap,
    server_id: i64,
    input: DockerTerminalStart,
    start_token: tokio_util::sync::CancellationToken,
    kind: TerminalKind,
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
            emit_error(&socket, format!("could not start Agent shell: {error}"));
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
            kind,
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

pub async fn spawn_agent_server_terminal(
    socket: SocketRef,
    sessions: &SessionMap,
    server_id: i64,
    input: ServerTerminalStart,
    start_token: tokio_util::sync::CancellationToken,
) {
    let docker_input = DockerTerminalStart {
        container: String::new(),
        shell: Some(input.shell().as_str().to_owned()),
        server_id: Some(server_id),
        cols: input.cols,
        rows: input.rows,
    };
    spawn_agent_shell(
        socket,
        sessions,
        server_id,
        docker_input,
        start_token,
        TerminalKind::Server,
    )
    .await;
}
