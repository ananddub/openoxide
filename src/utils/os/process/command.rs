use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct ProcessCommandBuilder<'a> {
    executor: &'a CommandExecutor,
    cmd: String,
    args: Vec<String>,
}

impl<'a> ProcessCommandBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, cmd: &str, args: Vec<String>) -> Self {
        Self {
            executor,
            cmd: cmd.to_string(),
            args,
        }
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run(&self.cmd, &self.args).await
    }
}

impl<'a> IntoCommand for ProcessCommandBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec![self.cmd.clone()];
        for arg in &self.args {
            parts.push(escape_arg(arg));
        }
        parts.join(" ")
    }
}
