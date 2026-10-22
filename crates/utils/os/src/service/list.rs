use crate::escape_arg;
use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct ServiceCommandBuilder<'a> {
    executor: &'a CommandExecutor,
    args: Vec<String>,
}

impl<'a> ServiceCommandBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, args: Vec<String>) -> Self {
        Self { executor, args }
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run("systemctl", &self.args).await
    }
}

impl<'a> IntoCommand for ServiceCommandBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec!["systemctl".to_string()];
        for arg in &self.args {
            parts.push(escape_arg(arg));
        }
        parts.join(" ")
    }
}
