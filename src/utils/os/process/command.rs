use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct ProcessCommandBuilder<'a> {
    executor: &'a CommandExecutor,
    cmd: String,
    args: Vec<String>,
    is_shell: bool,
}

impl<'a> ProcessCommandBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, cmd: &str, args: Vec<String>) -> Self {
        Self {
            executor,
            cmd: cmd.to_string(),
            args,
            is_shell: false,
        }
    }
    pub(crate) fn new_shell(
        executor: &'a CommandExecutor,
        shell_cmd: &str,
        args: Vec<String>,
    ) -> Self {
        Self {
            executor,
            cmd: shell_cmd.to_string(),
            args,
            is_shell: true,
        }
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        if self.is_shell {
            let mut run_args = vec!["-c", &self.cmd];
            for arg in &self.args {
                run_args.push(arg);
            }
            self.executor.run("sh", &run_args).await
        } else {
            self.executor.run(&self.cmd, &self.args).await
        }
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
