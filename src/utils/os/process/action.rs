use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct ProcessActionBuilder<'a> {
    executor: &'a CommandExecutor,
    cmd_parts: Vec<String>,
    is_shell: bool,
}

impl<'a> ProcessActionBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, cmd_parts: Vec<String>) -> Self {
        Self {
            executor,
            cmd_parts,
            is_shell: false,
        }
    }
    pub(crate) fn new_shell(
        executor: &'a CommandExecutor,
        shell_cmd: &str,
        args: Vec<String>,
    ) -> Self {
        let mut parts = vec![shell_cmd.to_string()];
        parts.extend(args);
        Self {
            executor,
            cmd_parts: parts,
            is_shell: true,
        }
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        if self.is_shell {
            let mut sh_args = vec!["-c", &self.cmd_parts[0]];
            for part in &self.cmd_parts[1..] {
                sh_args.push(part);
            }
            self.executor.run("sh", &sh_args).await
        } else {
            self.executor
                .run(&self.cmd_parts[0], &self.cmd_parts[1..])
                .await
        }
    }
}

impl<'a> IntoCommand for ProcessActionBuilder<'a> {
    fn build_str(&self) -> String {
        if self.is_shell {
            let mut parts = vec![self.cmd_parts[0].clone()];
            for arg in &self.cmd_parts[1..] {
                parts.push(escape_arg(arg));
            }
            parts.join(" ")
        } else {
            let mut parts = Vec::new();
            for part in &self.cmd_parts {
                parts.push(escape_arg(part));
            }
            parts.join(" ")
        }
    }
}
