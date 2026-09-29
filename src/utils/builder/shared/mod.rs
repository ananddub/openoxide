use crate::utils::{
    builder::{
        custom_type::{DeployEvent, IdType},
        hash_state::ApplicationState,
        queue::common::builder_event_state_opt,
        spec::BuilderEvent,
    },
    docker::DockerCli,
    exec::{CommandExecutor, ExecError, ExecResult},
};
use std::sync::Arc;
use tokio::{sync::mpsc, time::Duration};
use tokio_util::sync::CancellationToken;

pub mod patches;

/// Shared infrastructure for both `ApplicationBuilder` and `ComposeBuilder`.
///
/// Owns the executor, docker client, event channel, cancellation state,
/// and health-check timeout — everything that was previously duplicated
/// across the two builder structs.
#[derive(Clone, Debug)]
pub struct BuilderContext {
    pub executor: CommandExecutor,
    pub docker: DockerCli,
    events: Option<mpsc::Sender<BuilderEvent>>,
    state: Option<(Arc<ApplicationState>, IdType)>,
    pub health_timeout: Duration,
}

impl BuilderContext {
    pub fn new(executor: CommandExecutor) -> Self {
        Self {
            docker: DockerCli::from_executor(executor.clone()),
            executor,
            events: None,
            state: None,
            health_timeout: Duration::from_secs(120),
        }
    }

    pub fn with_events(mut self, events: mpsc::Sender<BuilderEvent>) -> Self {
        self.events = Some(events);
        self
    }

    pub fn with_state(mut self, state: Arc<ApplicationState>, id: IdType) -> Self {
        self.state = Some((state, id));
        self
    }

    pub fn with_health_timeout(mut self, timeout: Duration) -> Self {
        self.health_timeout = timeout;
        self
    }

    // ── Cancellation ────────────────────────────────────────────────────

    pub fn cancelled(&self, token: &CancellationToken) -> ExecResult<()> {
        if token.is_cancelled() {
            Err(ExecError::StreamCancelled)
        } else {
            Ok(())
        }
    }

    // ── File I/O ────────────────────────────────────────────────────────

    pub async fn write_file_cancelled(
        &self,
        path: &str,
        content: &[u8],
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        self.executor
            .run_with_stdin_cancelled(
                "sh",
                ["-c", "umask 077; cat > \"$1\"", "openoxide-write", path],
                content,
                cancel,
            )
            .await?;
        Ok(())
    }

    // ── Event Emission ──────────────────────────────────────────────────

    pub async fn emit(&self, event: BuilderEvent) {
        if let Some(sender) = &self.events {
            let _ = sender.send(event.clone()).await;
        }
        if let Some((state, id)) = &self.state
            && let BuilderEvent::Message(message) = &event
        {
            if let Some(sender) = state.get_broadcast_send(id.clone()) {
                let _ = sender.send(DeployEvent::Message(message.clone()));
            }
        }
        if let Some((state, id)) = &self.state {
            if let Some(deploy_state) = builder_event_state_opt(&event) {
                let _ = state.send_state(id.clone(), deploy_state);
            }
        }
    }

    // ── Streaming command execution ─────────────────────────────────────

    /// Runs a pipeline, emitting its stdout/stderr as `Message` lines in
    /// realtime instead of one dump at the end.
    pub async fn run_pipeline_streamed(
        &self,
        pipeline: &os::exec::script::ScriptPipeline,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        use crate::utils::exec::ExecStreamEvent;

        let (tx, mut rx) = mpsc::channel::<ExecStreamEvent>(256);

        let drain = async {
            let mut buffer: Vec<u8> = Vec::new();
            while let Some(event) = rx.recv().await {
                let bytes = match event {
                    ExecStreamEvent::Stdout(bytes) | ExecStreamEvent::Stderr(bytes) => bytes,
                };
                buffer.extend_from_slice(&bytes);
                while let Some(newline) = buffer.iter().position(|byte| *byte == b'\n') {
                    let line: Vec<u8> = buffer.drain(..=newline).collect();
                    let text = String::from_utf8_lossy(&line);
                    let text = text.trim_end_matches(['\n', '\r']);
                    if !text.is_empty() {
                        self.emit(BuilderEvent::Message(text.to_owned())).await;
                    }
                }
            }
            // Flush a trailing partial line.
            if !buffer.is_empty() {
                let text = String::from_utf8_lossy(&buffer);
                let text = text.trim_end_matches(['\n', '\r']);
                if !text.is_empty() {
                    self.emit(BuilderEvent::Message(text.to_owned())).await;
                }
            }
        };

        let run = pipeline.execute_stream_cancelled(&self.executor, tx, cancel);
        let (status, ()) = tokio::join!(run, drain);
        status.map(|_| ())
    }
}

// ── Shared Validation Helpers ───────────────────────────────────────────────

/// Validate that a name contains only safe characters (alphanumeric, `-`, `_`, `.`).
pub fn validate_name(label: &str, value: &str) -> ExecResult<()> {
    if value.is_empty()
        || !value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
    {
        return Err(ExecError::CommandFailed {
            code: None,
            stderr: format!("invalid {label} name: {value}"),
        });
    }
    Ok(())
}

/// Validate that a domain host string is safe.
pub fn validate_domain_host(host: &str) -> ExecResult<()> {
    if host.is_empty()
        || !host
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '*'))
    {
        return Err(ExecError::CommandFailed {
            code: None,
            stderr: format!("invalid domain host: {host}"),
        });
    }
    Ok(())
}

pub mod mapper;
pub mod mounts;
pub mod source;
pub mod traefik;

