use crate::utils::{
    builder::{
        custom_type::{DeployEvent, IdType},
        hash_state::ApplicationState,
        queue::common::builder_event_state_opt,
        spec::BuilderEvent,
    },
    cgroup::Cgroup,
    docker::DockerCli,
    exec::{CommandExecutor, ExecError, ExecResult},
};
use std::sync::Arc;
use tokio::{sync::mpsc, time::Duration};
use tokio_util::sync::CancellationToken;

/// Shared infrastructure for both `ApplicationBuilder` and `ComposeBuilder`.
///
/// Owns the executor, docker client, event channel, cancellation state,
/// and health-check timeout — everything that was previously duplicated
/// across the two builder structs.
#[derive(Clone, Debug)]
pub struct BuilderContext {
    pub executor: CommandExecutor,
    pub docker: DockerCli,
    pub cgroup: Option<Cgroup>,
    events: Option<mpsc::Sender<BuilderEvent>>,
    state: Option<(Arc<ApplicationState>, IdType)>,
    pub health_timeout: Duration,
}

impl BuilderContext {
    pub fn new(executor: CommandExecutor) -> Self {
        Self {
            docker: DockerCli::from_executor(executor.clone()),
            executor,
            cgroup: None,
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

    pub fn with_cgroup(mut self, cg: Cgroup) -> Self {
        self.cgroup = Some(cg);
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
                ["-c", "umask 077; cat > \"$1\"", "rustploy-write", path],
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

    pub fn apply_cgroup<T: CgroupApplier>(&self, target: T) -> ExecResult<T> {
        if let Some(ref cg) = self.cgroup {
            target.apply_cgroup(cg)
        } else {
            Ok(target)
        }
    }
}

pub trait CgroupApplier {
    fn apply_cgroup(self, cg: &Cgroup) -> ExecResult<Self>
    where
        Self: Sized;
}

impl CgroupApplier for crate::utils::exec::ScriptPipeline {
    fn apply_cgroup(mut self, cg: &Cgroup) -> ExecResult<Self> {
        for cmd in cg
            .to_apply_commands()
            .map_err(|e| ExecError::CommandFailed {
                code: None,
                stderr: e.to_string(),
            })?
        {
            self = self.cmd(cmd);
        }
        self = self.cmd(cg.to_add_current_process_command());
        Ok(self)
    }
}

impl CgroupApplier for Vec<crate::utils::exec::script::dsl::ShellIR> {
    fn apply_cgroup(self, cg: &Cgroup) -> ExecResult<Self> {
        let mut prefix = Vec::new();
        for cmd in cg
            .to_apply_commands()
            .map_err(|e| ExecError::CommandFailed {
                code: None,
                stderr: e.to_string(),
            })?
        {
            prefix.push(crate::utils::exec::script::dsl::ShellIR::Raw(cmd));
        }
        prefix.push(crate::utils::exec::script::dsl::ShellIR::Raw(
            cg.to_add_current_process_command(),
        ));
        prefix.extend(self);
        Ok(prefix)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::cgroup::builder::CgroupBuilder;
    use crate::utils::exec::LocalExecutor;
    use crate::utils::exec::script::sh;

    #[test]
    fn test_cgroup_applier_sh_dsl() {
        let executor = CommandExecutor::Local(LocalExecutor::new());
        let cg = CgroupBuilder::new("test/cgroup", executor.clone())
            .with_base_path("/tmp/cgroup")
            .build();

        let ctx = BuilderContext::new(executor).with_cgroup(cg);

        let script = sh!(
            echo("hello inside cgroup");
        );

        let cgrouped_script = ctx.apply_cgroup(script).unwrap();
        let bash = cgrouped_script
            .iter()
            .map(|s| s.to_bash())
            .collect::<Vec<_>>()
            .join("\n");

        assert!(bash.contains("mkdir -p '/tmp/cgroup/test/cgroup'"));
        assert!(bash.contains("cgroup.procs"));
        assert!(bash.contains("hello inside cgroup"));
    }
}
