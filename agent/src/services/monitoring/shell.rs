use crate::api::proto::{
    ExecTaskOutput, ExecTaskOutputKind, ExecTaskRequest, ShellError, ShellExit, ShellInput,
    ShellMessage, ShellOpen, ShellOutput, shell_message,
};
use crate::docker::api::DockerApi;
use std::pin::Pin;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio_stream::Stream;
use tonic::{Response, Status};

pub type ShellStream = Pin<Box<dyn Stream<Item = Result<ShellMessage, Status>> + Send + 'static>>;

/// Releases the agent cgroup's reclaimable page cache (file-backed pages that
/// pile up during docker pull/compose). Best-effort and non-blocking: writes to
/// cgroup-v2 `memory.reclaim`, falling back to `MADV`-style `sync` when that
/// interface is unavailable. Anonymous (real) memory is never touched.
pub fn reclaim_own_page_cache() {
    // cgroup v2: writing a byte budget to memory.reclaim asks the kernel to
    // reclaim that much from THIS cgroup. A large value reclaims all it can
    // (only cache; anon stays). The agent runs in its own container cgroup.
    for path in [
        "/sys/fs/cgroup/memory.reclaim",
        "/sys/fs/cgroup/memory/memory.reclaim",
    ] {
        if std::path::Path::new(path).exists() {
            // 512 MiB is far above the agent's footprint; the kernel simply
            // reclaims whatever cache exists and ignores the rest.
            let _ = std::fs::write(path, b"536870912");
            return;
        }
    }
}

pub struct ShellService;

impl ShellService {
    pub async fn execute(
        server_id: i64,
        request: ExecTaskRequest,
    ) -> Result<impl Stream<Item = Result<ExecTaskOutput, Status>> + Send + 'static, Status> {
        if request.server_id <= 0 || (server_id != -1 && request.server_id != server_id) {
            return Err(Status::not_found(
                "task server_id does not match this agent",
            ));
        }
        if request.command.trim().is_empty() {
            return Err(Status::invalid_argument("task command cannot be empty"));
        }

        let mut command = if request.container.is_empty() {
            let mut command = Command::new("sh");
            command.args(["-lc", &request.command]);
            command
        } else {
            let mut command = Command::new("docker");
            command.args(["exec", "-i"]);
            if !request.working_directory.is_empty() {
                command.args(["-w", &request.working_directory]);
            }
            command.args([&request.container, "sh", "-lc", &request.command]);
            command
        };
        if request.container.is_empty() && !request.working_directory.is_empty() {
            command.current_dir(&request.working_directory);
        }
        let mut child = command
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .stdin(if request.stdin.is_empty() {
                std::process::Stdio::null()
            } else {
                std::process::Stdio::piped()
            })
            .spawn()
            .map_err(|error| Status::internal(format!("could not start task: {error}")))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| Status::internal("task stdout unavailable"))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| Status::internal("task stderr unavailable"))?;
        let (tx, rx) = tokio::sync::mpsc::channel(32);
        let timeout = request.timeout_seconds;
        let stdin_data = request.stdin;
        tokio::spawn(async move {
            let stdout_task = tokio::spawn(stream_task_output(
                stdout,
                ExecTaskOutputKind::ExecTaskOutputStdout,
                tx.clone(),
            ));
            let stderr_task = tokio::spawn(stream_task_output(
                stderr,
                ExecTaskOutputKind::ExecTaskOutputStderr,
                tx.clone(),
            ));
            if let Some(mut stdin) = child.stdin.take() {
                tokio::spawn(async move {
                    let _ = stdin.write_all(&stdin_data).await;
                    let _ = stdin.shutdown().await;
                });
            }
            let status = tokio::select! {
                status = child.wait() => status,
                _ = tokio::time::sleep(std::time::Duration::from_secs(timeout.max(1) as u64)), if timeout > 0 => {
                    let _ = child.kill().await;
                    let status = child.wait().await;
                    let _ = tx.send(Ok(ExecTaskOutput {
                        kind: ExecTaskOutputKind::ExecTaskOutputError as i32,
                        error: format!("task timed out after {timeout} seconds"),
                        ..Default::default()
                    })).await;
                    status
                },
                _ = tx.closed() => {
                    let _ = child.kill().await;
                    child.wait().await
                }
            };
            let _ = stdout_task.await;
            let _ = stderr_task.await;
            let code = status.ok().and_then(|status| status.code()).unwrap_or(-1);
            let _ = tx
                .send(Ok(ExecTaskOutput {
                    kind: ExecTaskOutputKind::ExecTaskOutputExit as i32,
                    exit_code: code,
                    ..Default::default()
                }))
                .await;
            // A deploy (docker pull/compose) fills this cgroup's page cache
            // with image-layer and file reads. That cache is reclaimable but
            // still shows up as the container's memory in `docker stats`, so
            // release it once the task is done to keep the idle footprint tiny.
            crate::services::monitoring::shell::reclaim_own_page_cache();
        });
        Ok(tokio_stream::wrappers::ReceiverStream::new(rx))
    }

    pub async fn open(
        server_id: i64,
        mut input: tonic::Streaming<ShellMessage>,
        expected_server_id: i64,
        _docker: DockerApi,
    ) -> Result<Response<ShellStream>, Status> {
        let first = input
            .message()
            .await?
            .ok_or_else(|| Status::invalid_argument("missing shell open message"))?;
        let open = match first.payload {
            Some(shell_message::Payload::Open(open)) => open,
            _ => return Err(Status::invalid_argument("first shell message must be open")),
        };
        if open.server_id <= 0
            || server_id != expected_server_id
            || (expected_server_id != -1 && open.server_id != expected_server_id)
        {
            return Err(Status::not_found(
                "shell server_id does not match this agent",
            ));
        }

        if open.host {
            return Self::open_host_shell(input, open).await;
        }
        if !open.container.is_empty() {
            return Self::open_container(input, open).await;
        }

        let shell = if open.shell.is_empty() {
            "sh"
        } else {
            open.shell.as_str()
        };
        let mut command = Command::new(shell);
        let mut child = command
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|error| Status::internal(format!("could not start shell: {error}")))?;
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| Status::internal("shell stdin unavailable"))?;
        let mut stdout = child
            .stdout
            .take()
            .ok_or_else(|| Status::internal("shell stdout unavailable"))?;
        let mut stderr = child
            .stderr
            .take()
            .ok_or_else(|| Status::internal("shell stderr unavailable"))?;
        let (tx, rx) = tokio::sync::mpsc::channel(32);

        tokio::spawn(async move {
            let mut out = [0u8; 4096];
            let mut err = [0u8; 4096];
            loop {
                tokio::select! {
                    message = input.message() => match message {
                        Ok(Some(ShellMessage { payload: Some(shell_message::Payload::Input(ShellInput { data })) })) => {
                            if stdin.write_all(&data).await.is_err() { break; }
                            let _ = stdin.flush().await;
                        }
                        Ok(Some(ShellMessage { payload: Some(shell_message::Payload::Close(_)) })) => {
                            let _ = stdin.shutdown().await;
                            break;
                        }
                        Ok(Some(_)) => {}
                        _ => break,
                    },
                    result = stdout.read(&mut out) => match result {
                        Ok(0) => {},
                        Ok(size) => { if tx.send(Ok(stdout_message(&out[..size]))).await.is_err() { break; } },
                        Err(error) => { let _ = tx.send(Ok(error_message(error.to_string()))).await; break; }
                    },
                    result = stderr.read(&mut err) => match result {
                        Ok(0) => {},
                        Ok(size) => { if tx.send(Ok(stderr_message(&err[..size]))).await.is_err() { break; } },
                        Err(error) => { let _ = tx.send(Ok(error_message(error.to_string()))).await; break; }
                    },
                    status = child.wait() => {
                        let code = status.ok().and_then(|status| status.code()).unwrap_or(-1);
                        let _ = tx.send(Ok(exit_message(code))).await;
                        break;
                    }
                }
            }
            // Reap after kill: without `wait` the SIGKILLed shell stays a
            // zombie forever — this process never spawns a reaper.
            if child.try_wait().ok().flatten().is_none() {
                let _ = child.kill().await;
            }
            let _ = child.wait().await;
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }

    async fn open_container(
        input: tonic::Streaming<ShellMessage>,
        open: ShellOpen,
    ) -> Result<Response<ShellStream>, Status> {
        let docker = os::docker::DockerCli::new_local();
        let mut target = open.container.clone();
        if let Ok(containers) = docker.containers().ps().all().api().list().await {
            let search = target.to_lowercase();
            let clean_search = search.trim_end_matches("_db").trim_end_matches("-db");

            // Prefer an exact compose-service match so selecting a specific
            // service opens that service's container, not the first name
            // substring hit. The CLI `Labels` field is a comma-separated list
            // of `key=value` pairs.
            let service_match = containers.iter().find(|container| {
                container.labels.split(',').any(|pair| {
                    pair.trim()
                        .strip_prefix("com.docker.compose.service=")
                        .is_some_and(|value| value.eq_ignore_ascii_case(&search))
                })
            });

            let found = service_match.or_else(|| {
                containers.iter().find(|container| {
                    let name = container.names.to_lowercase();
                    name.contains(&search)
                        || name.trim_start_matches('/').starts_with(&search)
                        || name.contains(&format!("{}_", search))
                        || name.contains(&format!("{}.", search))
                        || (!clean_search.is_empty()
                            && (name.contains(clean_search)
                                || name.trim_start_matches('/').starts_with(clean_search)))
                })
            });

            if let Some(found) = found {
                target = if found.id.is_empty() {
                    found.names.trim_start_matches('/').to_owned()
                } else {
                    found.id.clone()
                };
            }
        }
        let shell = if open.shell.is_empty() {
            "sh".to_owned()
        } else {
            open.shell.clone()
        };
        let stream = docker
            .containers()
            .exec(&target)
            .interactive()
            .tty(true)
            .api()
            .env("TERM", "xterm-256color")
            .shell([shell])
            .await
            .map_err(|error| Status::internal(format!("could not start Docker shell: {error}")))?;
        let cols = u16::try_from(open.cols.max(1)).unwrap_or(u16::MAX);
        let rows = u16::try_from(open.rows.max(1)).unwrap_or(u16::MAX);
        let _ = stream.resize(cols, rows).await;
        Ok(Self::spawn_pump(input, stream))
    }

    /// Opens an interactive root shell on the agent's own host directly —
    /// no helper container, no `docker exec`, no `nsenter` process chain.
    /// The agent mounts the host root read-only at `/host`, so
    /// `/host/proc/1/ns/*` are the host namespace files; the shell child is
    /// `setns`'d into them in its `pre_exec` window (single-threaded after
    /// fork, which the kernel requires for the mount namespace) and lands on
    /// a host PTY. Requires the agent container to run with `CAP_SYS_ADMIN`;
    /// without it (or without the `/host` mount) this fails with a clear
    /// precondition error.
    async fn open_host_shell(
        input: tonic::Streaming<ShellMessage>,
        open: ShellOpen,
    ) -> Result<Response<ShellStream>, Status> {
        use std::os::fd::AsRawFd;

        let shell = if open.shell.is_empty() {
            "sh".to_owned()
        } else {
            open.shell.clone()
        };
        let cols = u16::try_from(open.cols.max(1)).unwrap_or(u16::MAX);
        let rows = u16::try_from(open.rows.max(1)).unwrap_or(u16::MAX);

        let namespaces = open_host_namespaces()?;
        let (master, slave) = open_host_pty(rows, cols)?;

        // The raw fds captured for `pre_exec` are the parent's — valid in the
        // forked child, and O_CLOEXEC so they are gone once the shell execs.
        let pid_fd = namespaces.pid.as_raw_fd();
        let mnt_fd = namespaces.mnt.as_raw_fd();
        let uts_fd = namespaces.uts.as_raw_fd();
        let ipc_fd = namespaces.ipc.as_raw_fd();
        let net_fd = namespaces.net.as_raw_fd();
        let slave_fd = slave.as_raw_fd();

        let mut command = tokio::process::Command::new(&shell);
        command
            .stdin(std::process::Stdio::from(
                slave.try_clone().map_err(host_shell_error)?,
            ))
            .stdout(std::process::Stdio::from(
                slave.try_clone().map_err(host_shell_error)?,
            ))
            .stderr(std::process::Stdio::from(slave))
            .env("TERM", "xterm-256color");
        unsafe {
            command.pre_exec(move || {
                use std::io;
                // New session so the PTY slave can be claimed as the child's
                // controlling terminal — exit status, job control and ^C all
                // behave like a real login shell.
                if libc::setsid() < 0 {
                    return Err(io::Error::last_os_error());
                }
                if libc::ioctl(slave_fd, libc::TIOCSCTTY, 0) < 0 {
                    return Err(io::Error::last_os_error());
                }
                let targets = [
                    (pid_fd, libc::CLONE_NEWPID),
                    (uts_fd, libc::CLONE_NEWUTS),
                    (ipc_fd, libc::CLONE_NEWIPC),
                    (net_fd, libc::CLONE_NEWNET),
                    (mnt_fd, libc::CLONE_NEWNS),
                ];
                for (fd, flag) in targets {
                    if libc::setns(fd, flag) != 0 {
                        return Err(io::Error::last_os_error());
                    }
                }
                // The container's cwd does not exist in the host mount ns.
                if libc::chdir(c"/".as_ptr()) != 0 {
                    return Err(io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let child = command
            .spawn()
            .map_err(|error| Status::internal(format!("could not start host shell: {error}")))?;
        Ok(Self::spawn_pump_pty(input, child, master))
    }

    /// Bridges a host PTY master to the gRPC shell stream: reader output to
    /// the client, keystrokes to the PTY, resize to `TIOCSWINSZ`. When the
    /// session ends the child is killed and reaped, and dropping the master
    /// fds hangs up the PTY so the shell gets SIGHUP even if it ignores the
    /// kill.
    fn spawn_pump_pty(
        mut input: tonic::Streaming<ShellMessage>,
        mut child: tokio::process::Child,
        master: std::fs::File,
    ) -> Response<ShellStream> {
        use std::io::{Read, Write};
        use std::os::fd::AsRawFd;

        let (tx, rx) = tokio::sync::mpsc::channel(32);
        let mut writer = master.try_clone().expect("dup of PTY master fd");
        let mut reader = master;
        let (data_tx, mut data_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(64);
        tokio::spawn(async move {
            let _ = tokio::task::spawn_blocking(move || {
                let mut buffer = [0u8; 4096];
                loop {
                    match reader.read(&mut buffer) {
                        Ok(0) => break,
                        Ok(size) => {
                            if data_tx.blocking_send(buffer[..size].to_vec()).is_err() {
                                break;
                            }
                        }
                        Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                        Err(_) => break,
                    }
                }
            })
            .await;
        });

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    message = input.message() => match message {
                        Ok(Some(ShellMessage { payload: Some(shell_message::Payload::Input(ShellInput { data })) })) => {
                            if writer.write_all(&data).and_then(|_| writer.flush()).is_err() { break; }
                        }
                        Ok(Some(ShellMessage { payload: Some(shell_message::Payload::Resize(resize)) })) => {
                            let window = libc::winsize {
                                ws_row: u16::try_from(resize.rows.max(1)).unwrap_or(u16::MAX),
                                ws_col: u16::try_from(resize.cols.max(1)).unwrap_or(u16::MAX),
                                ws_xpixel: 0,
                                ws_ypixel: 0,
                            };
                            unsafe { libc::ioctl(writer.as_raw_fd(), libc::TIOCSWINSZ, &window) };
                        }
                        Ok(Some(ShellMessage { payload: Some(shell_message::Payload::Close(_)) })) => break,
                        Ok(Some(_)) => {}
                        _ => break,
                    },
                    chunk = data_rx.recv() => match chunk {
                        Some(bytes) => {
                            if tx.send(Ok(stdout_message(&bytes))).await.is_err() { break; }
                        }
                        // PTY hung up: the shell exited.
                        None => break,
                    },
                }
            }
            // Reap: without `wait` the killed shell becomes a zombie — this
            // process never spawns a reaper.
            if child.try_wait().ok().flatten().is_none() {
                let _ = child.kill().await;
            }
            let code = child
                .wait()
                .await
                .ok()
                .and_then(|status| status.code())
                .unwrap_or(-1);
            let _ = tx.send(Ok(exit_message(code))).await;
        });

        Response::new(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }

    fn spawn_pump(
        mut input: tonic::Streaming<ShellMessage>,
        stream: os::docker::handles::containers::socat::ContainerSocketExecStream,
    ) -> Response<ShellStream> {
        let mut reader = stream.reader;
        let mut writer = stream.writer;
        let exec_id = stream.exec_id;
        let socket_path = stream.socket_path;
        let (tx, rx) = tokio::sync::mpsc::channel(32);

        tokio::spawn(async move {
            let mut output = [0u8; 4096];
            loop {
                tokio::select! {
                    message = input.message() => match message {
                        Ok(Some(ShellMessage { payload: Some(shell_message::Payload::Input(ShellInput { data })) })) => {
                            if writer.write_all(&data).await.is_err() { break; }
                            let _ = writer.flush().await;
                        }
                        Ok(Some(ShellMessage { payload: Some(shell_message::Payload::Resize(resize)) })) => {
                            let cols = u16::try_from(resize.cols.max(1)).unwrap_or(u16::MAX);
                            let rows = u16::try_from(resize.rows.max(1)).unwrap_or(u16::MAX);
                            let _ = os::docker::handles::containers::socat::resize_container_exec(
                                &socket_path, &exec_id, cols, rows,
                            ).await;
                        }
                        Ok(Some(ShellMessage { payload: Some(shell_message::Payload::Close(_)) })) => {
                            let _ = writer.shutdown().await;
                            break;
                        }
                        Ok(Some(_)) => {}
                        _ => break,
                    },
                    result = reader.read(&mut output) => match result {
                        Ok(0) => {
                            let _ = tx.send(Ok(exit_message(0))).await;
                            break;
                        }
                        Ok(size) => {
                            if tx.send(Ok(stdout_message(&output[..size]))).await.is_err() { break; }
                        }
                        Err(error) => {
                            let _ = tx.send(Ok(error_message(error.to_string()))).await;
                            break;
                        }
                    }
                }
            }
        });

        Response::new(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}

/// Host namespace file descriptors for the machine's PID 1, opened through
/// the `/host` bind of the host root. Held open until the shell has been
/// spawned; `pre_exec` uses the raw numbers in the forked child.
struct HostNamespaces {
    pid: std::fs::File,
    mnt: std::fs::File,
    uts: std::fs::File,
    ipc: std::fs::File,
    net: std::fs::File,
}

fn open_host_namespaces() -> Result<HostNamespaces, Status> {
    let open = |kind: &str| {
        std::fs::File::open(format!("/host/proc/1/ns/{kind}")).map_err(|error| {
            Status::failed_precondition(format!(
                "host namespaces unavailable at /host/proc/1/ns/{kind} ({error}); the agent \
                 container needs the /host bind mount and CAP_SYS_ADMIN"
            ))
        })
    };
    Ok(HostNamespaces {
        pid: open("pid")?,
        mnt: open("mnt")?,
        uts: open("uts")?,
        ipc: open("ipc")?,
        net: open("net")?,
    })
}

/// Create a PTY pair sized for the terminal the panel opened. Built from
/// plain `posix_openpt`/`grantpt`/`unlockpt`/`ptsname` syscalls so the
/// static musl binary needs no extra libutil linkage.
fn open_host_pty(rows: u16, cols: u16) -> Result<(std::fs::File, std::fs::File), Status> {
    use std::ffi::CStr;
    use std::os::fd::{AsRawFd, FromRawFd};

    let master_fd = unsafe { libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY) };
    if master_fd < 0 {
        return Err(Status::internal("could not open a PTY for the host shell"));
    }
    let master = unsafe { std::fs::File::from_raw_fd(master_fd) };
    let pending = unsafe {
        if libc::grantpt(master.as_raw_fd()) != 0 {
            Some("grantpt")
        } else if libc::unlockpt(master.as_raw_fd()) != 0 {
            Some("unlockpt")
        } else {
            None
        }
    };
    if let Some(stage) = pending {
        return Err(Status::internal(format!(
            "could not prepare PTY slave: {stage} failed"
        )));
    }
    let slave_name = unsafe {
        let ptr = libc::ptsname(master.as_raw_fd());
        if ptr.is_null() {
            return Err(Status::internal("could not resolve PTY slave name"));
        }
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    };
    let slave = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&slave_name)
        .map_err(|error| {
            Status::internal(format!("could not open PTY slave {slave_name}: {error}"))
        })?;
    let window = libc::winsize {
        ws_row: rows,
        ws_col: cols,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    unsafe { libc::ioctl(master.as_raw_fd(), libc::TIOCSWINSZ, &window) };
    Ok((master, slave))
}

fn host_shell_error(error: std::io::Error) -> Status {
    Status::internal(format!("could not wire host shell PTY: {error}"))
}

fn stdout_message(data: &[u8]) -> ShellMessage {
    ShellMessage {
        payload: Some(shell_message::Payload::Stdout(ShellOutput {
            data: data.to_vec(),
        })),
    }
}

fn stderr_message(data: &[u8]) -> ShellMessage {
    ShellMessage {
        payload: Some(shell_message::Payload::Stderr(ShellOutput {
            data: data.to_vec(),
        })),
    }
}

fn exit_message(code: i32) -> ShellMessage {
    ShellMessage {
        payload: Some(shell_message::Payload::Exit(ShellExit { exit_code: code })),
    }
}

fn error_message(message: String) -> ShellMessage {
    ShellMessage {
        payload: Some(shell_message::Payload::Error(ShellError { message })),
    }
}

async fn stream_task_output<R>(
    mut reader: R,
    kind: ExecTaskOutputKind,
    tx: tokio::sync::mpsc::Sender<Result<ExecTaskOutput, Status>>,
) where
    R: tokio::io::AsyncRead + Unpin,
{
    let mut buffer = [0_u8; 4096];
    loop {
        match reader.read(&mut buffer).await {
            Ok(0) => break,
            Ok(size) => {
                if tx
                    .send(Ok(ExecTaskOutput {
                        kind: kind as i32,
                        data: buffer[..size].to_vec(),
                        ..Default::default()
                    }))
                    .await
                    .is_err()
                {
                    break;
                }
            }
            Err(error) => {
                let _ = tx
                    .send(Ok(ExecTaskOutput {
                        kind: ExecTaskOutputKind::ExecTaskOutputError as i32,
                        error: error.to_string(),
                        ..Default::default()
                    }))
                    .await;
                break;
            }
        }
    }
}
