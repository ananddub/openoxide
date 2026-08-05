use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;
use tokio_util::sync::CancellationToken;

pub struct NetworkCommandBuilder<'a> {
    executor: &'a CommandExecutor,
    cmd: &'static str,
    args: Vec<String>,
}

impl<'a> NetworkCommandBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, cmd: &'static str, args: Vec<String>) -> Self {
        Self {
            executor,
            cmd,
            args,
        }
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run(self.cmd, &self.args).await
    }
    pub async fn run_cancelled(self, cancel: &CancellationToken) -> ExecResult<ExecOutput> {
        self.executor
            .run_cancelled(self.cmd, &self.args, cancel)
            .await
    }
}

impl<'a> IntoCommand for NetworkCommandBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec![self.cmd.to_string()];
        for arg in &self.args {
            parts.push(escape_arg(arg));
        }
        parts.join(" ")
    }
}
