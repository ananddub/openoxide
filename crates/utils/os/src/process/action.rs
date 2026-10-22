use crate::escape_arg;
use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct ProcessActionBuilder<'a> {
    executor: &'a CommandExecutor,
    cmd_parts: Vec<String>,
}

impl<'a> ProcessActionBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, cmd_parts: Vec<String>) -> Self {
        Self {
            executor,
            cmd_parts,
        }
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor
            .run(&self.cmd_parts[0], &self.cmd_parts[1..])
            .await
    }
}

impl<'a> IntoCommand for ProcessActionBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = Vec::new();
        for part in &self.cmd_parts {
            parts.push(escape_arg(part));
        }
        parts.join(" ")
    }
}
