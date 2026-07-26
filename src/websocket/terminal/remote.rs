use std::sync::Arc;

use pty_process::{Command as PtyCommand, Size};
use socketioxide::extract::SocketRef;
use tokio::sync::Mutex;

use super::helpers::{emit_error, socket_key, spawn_pty_reader};
use super::types::{ServerTerminalStart, SessionMap, TerminalExit, TerminalSession, TerminalStarted};

pub async fn spawn_remote_terminal(
    socket: SocketRef,
    sessions: &SessionMap,
    db: &sqlx::SqlitePool,
    server_id: i64,
    input: ServerTerminalStart,
) {
    let key = socket_key(&socket);

    let executor = match crate::services::compose::remote::remote_executor(db, server_id).await {
        Ok(executor) => executor,
        Err(error) => {
            emit_error(&socket, format!("could not create remote executor: {error}"));
            return;
        }
    };

    let shell = input.shell.unwrap_or_else(|| "sh".into());
    let builder = crate::utils::ssh::SshBuilder::new(
        executor.host().to_string(),
        executor.username().to_string(),
        executor.auth().clone(),
        executor.host_key().clone(),
    )
    .port(executor.port())
    .tty(crate::utils::ssh::TtyMode::ForceTty);

    let (mut args, temp_key, temp_askpass, agent_socket) = match builder.build_args() {
        Ok(res) => res,
        Err(error) => {
            emit_error(&socket, format!("could not build SSH args: {error}"));
            return;
        }
    };
    args.push(shell);

    let (pty, pts) = match pty_process::open() {
        Ok(res) => res,
        Err(error) => {
            emit_error(&socket, format!("could not open PTY: {error}"));
            return;
        }
    };
    let _ = pty.resize(Size::new(24, 80));

    let mut cmd = PtyCommand::new("ssh")
        .args(&args)
        .env("TERM", "xterm-256color");
    if let Some(socket_path) = agent_socket {
        cmd = cmd.env("SSH_AUTH_SOCK", socket_path);
    }
    if let Some(ref askpass) = temp_askpass {
        cmd = cmd
            .env("SSH_ASKPASS", askpass.as_os_str())
            .env("SSH_ASKPASS_REQUIRE", "force")
            .env("DISPLAY", ":0");
    }

    let mut child = match cmd.spawn(pts) {
        Ok(child) => child,
        Err(error) => {
            emit_error(&socket, format!("could not start SSH terminal: {error}"));
            return;
        }
    };

    let (reader, writer) = pty.into_split();

    sessions.insert(
        key.clone(),
        TerminalSession::Pty {
            writer: Arc::new(Mutex::new(writer)),
        },
    );

    let _ = socket.emit(
        "started",
        &TerminalStarted {
            kind: "remote-server",
        },
    );

    spawn_pty_reader(socket.clone(), reader);

    let sessions_clone = sessions.clone();
    let socket_clone = socket.clone();
    tokio::spawn(async move {
        let _keep_alive_key = temp_key;
        let _keep_alive_askpass = temp_askpass;
        let status = child.wait().await;
        sessions_clone.remove(&key);
        let code = status.ok().and_then(|s| s.code());
        let _ = socket_clone.emit("exit", &TerminalExit { code });
    });
}
