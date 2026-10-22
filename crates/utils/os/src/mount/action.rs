use crate::escape_arg;
use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct MountActionBuilder<'a> {
    executor: &'a CommandExecutor,
    cmd: &'static str,
    args: Vec<String>,
}

impl<'a> MountActionBuilder<'a> {
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
}

impl<'a> IntoCommand for MountActionBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec![self.cmd.to_string()];
        for arg in &self.args {
            parts.push(escape_arg(arg));
        }
        parts.join(" ")
    }
}
