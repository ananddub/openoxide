use std::sync::Arc;

use portable_pty::{CommandBuilder, PtySize, native_pty_system};
use socketioxide::extract::SocketRef;
use tokio::sync::Mutex;

use super::helpers::{emit_error, socket_key, spawn_blocking_pty_reader};
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

    let pty_system = native_pty_system();
    let pair = match pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    }) {
        Ok(pair) => pair,
        Err(error) => {
            emit_error(&socket, format!("could not open PTY: {error}"));
            return;
        }
    };

    let mut cmd = CommandBuilder::new("ssh");
    cmd.args(&args);

    let _child = match pair.slave.spawn_command(cmd) {
        Ok(child) => child,
        Err(error) => {
            emit_error(&socket, format!("could not start remote PTY terminal: {error}"));
            return;
        }
    };

    let reader = match pair.master.try_clone_reader() {
        Ok(r) => r,
        Err(error) => {
            emit_error(&socket, format!("could not clone PTY reader: {error}"));
            return;
        }
    };

    let writer = match pair.master.take_writer() {
        Ok(w) => w,
        Err(error) => {
            emit_error(&socket, format!("could not take PTY writer: {error}"));
            return;
        }
    };

    sessions.insert(
        key.clone(),
        TerminalSession::Pty {
            writer: Arc::new(Mutex::new(writer)),
            master: Arc::new(Mutex::new(pair.master)),
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
    });

    spawn_blocking_pty_reader(socket.clone(), reader);
}
