use std::sync::Arc;

use pty_process::{Command as PtyCommand, Size};
use socketioxide::extract::SocketRef;
use tokio::sync::Mutex;

use super::helpers::{emit_error, socket_key, spawn_pty_reader};
use super::types::{ServerTerminalStart, SessionMap, TerminalSession, TerminalStarted};

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
    .connect_timeout(executor.connect_timeout().as_secs() as u32)
    .tty(crate::utils::ssh::TtyMode::ForceTty);

    let (mut args, temp_key, temp_askpass, _env_file) = match builder.build_args() {
        Ok(res) => res,
        Err(e) => {
            emit_error(&socket, format!("could not build ssh command: {e}"));
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

    let cmd = PtyCommand::new("ssh").args(&args);

    let mut child = match cmd.spawn(pts) {
        Ok(child) => child,
        Err(error) => {
            emit_error(&socket, format!("could not start remote PTY terminal: {error}"));
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

    tokio::spawn(async move {
        let _temp_key = temp_key;
        let _temp_askpass = temp_askpass;
        let _ = child.wait().await;
    });

    spawn_pty_reader(socket.clone(), reader);
}
