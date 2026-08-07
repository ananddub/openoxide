use crate::escape_arg;
use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct ServiceActionBuilder<'a> {
    executor: &'a CommandExecutor,
    action: &'static str,
    name: String,
}

impl<'a> ServiceActionBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, action: &'static str, name: String) -> Self {
        Self {
            executor,
            action,
            name,
        }
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor
            .run("systemctl", &[self.action, &self.name])
            .await
    }
}

impl<'a> IntoCommand for ServiceActionBuilder<'a> {
    fn build_str(&self) -> String {
        format!("systemctl {} {}", self.action, escape_arg(&self.name))
    }
}
