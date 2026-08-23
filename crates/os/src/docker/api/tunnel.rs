use crate::docker::DockerError;
use crate::exec::RemoteExecutor;
use crate::ssh::connect_russh;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::copy_bidirectional;
use tokio::net::UnixListener;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

const IDLE_TIMEOUT: Duration = Duration::from_secs(180);

#[derive(Debug)]
struct RunningTunnel {
    socket: String,
    cancel: tokio_util::sync::CancellationToken,
    task: JoinHandle<()>,
    users: usize,
    generation: u64,
}

#[derive(Clone, Debug, Default)]
pub struct RemoteApiTunnel {
    state: Arc<Mutex<Option<RunningTunnel>>>,
}

pub struct TunnelLease {
    state: Arc<Mutex<Option<RunningTunnel>>>,
    socket: String,
    generation: u64,
}

impl std::fmt::Debug for TunnelLease {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TunnelLease")
            .field("socket", &self.socket)
            .field("generation", &self.generation)
            .finish()
    }
}

impl TunnelLease {
    pub fn socket(&self) -> &str {
        &self.socket
    }
}

impl Drop for TunnelLease {
    fn drop(&mut self) {
        let state = self.state.clone();
        let generation = self.generation;
        tokio::spawn(async move {
            {
                let mut guard = state.lock().await;
                let Some(tunnel) = guard.as_mut() else { return };
                tunnel.users = tunnel.users.saturating_sub(1);
                if tunnel.users != 0 || tunnel.generation != generation {
                    return;
                }
            }
            tokio::time::sleep(IDLE_TIMEOUT).await;
            let mut guard = state.lock().await;
            let should_close = guard
                .as_ref()
                .is_some_and(|t| t.users == 0 && t.generation == generation);
            if should_close {
                if let Some(tunnel) = guard.take() {
                    tunnel.cancel.cancel();
                    tunnel.task.abort();
                    let _ = tokio::fs::remove_file(&tunnel.socket).await;
                }
            }
        });
    }
}

impl RemoteApiTunnel {
    pub async fn acquire(&self, remote: &RemoteExecutor) -> Result<TunnelLease, DockerError> {
        let mut guard = self.state.lock().await;
        let needs_start = match guard.as_mut() {
            Some(tunnel) => tunnel.task.is_finished(),
            None => true,
        };
        if needs_start {
            if let Some(old) = guard.take() {
                old.cancel.cancel();
                old.task.abort();
                let _ = tokio::fs::remove_file(&old.socket).await;
            }
            *guard = Some(start_tunnel(remote).await?);
        }
        let tunnel = guard.as_mut().expect("russh Docker API tunnel initialized");
        tunnel.users += 1;
        tunnel.generation = tunnel.generation.wrapping_add(1);
        Ok(TunnelLease {
            state: self.state.clone(),
            socket: tunnel.socket.clone(),
            generation: tunnel.generation,
        })
    }
}

async fn start_tunnel(remote: &RemoteExecutor) -> Result<RunningTunnel, DockerError> {
    let socket = format!("/tmp/openoxide-docker-api-{}.sock", uuid::Uuid::new_v4());
    let _ = tokio::fs::remove_file(&socket).await;
    let listener = UnixListener::bind(&socket)
        .map_err(|e| DockerError::Ssh(format!("could not bind Docker API socket: {e}")))?;
    let session = connect_russh(
        remote.host(),
        remote.port(),
        remote.username(),
        remote.auth(),
        remote.host_key(),
        remote.connect_timeout(),
    )
    .await
    .map_err(|e| DockerError::Ssh(e.to_string()))?;
    let cancel = tokio_util::sync::CancellationToken::new();
    let task_cancel = cancel.clone();
    let session_for_task = Arc::new(session);
    let task = tokio::spawn(async move {
        loop {
            let accepted = tokio::select! {
                _ = task_cancel.cancelled() => break,
                result = listener.accept() => result,
            };
            let Ok((mut local, _)) = accepted else { break };
            let session = session_for_task.clone();
            let cancelled = task_cancel.clone();
            tokio::spawn(async move {
                let Ok(channel) = session.channel_open_session().await else {
                    return;
                };
                if channel
                    .exec(true, "docker system dial-stdio")
                    .await
                    .is_err()
                {
                    return;
                }
                let mut remote_stream = channel.into_stream();
                tokio::select! {
                    _ = cancelled.cancelled() => {}
                    _ = copy_bidirectional(&mut local, &mut remote_stream) => {}
                }
            });
        }
    });
    Ok(RunningTunnel {
        socket,
        cancel,
        task,
        users: 0,
        generation: 0,
    })
}
