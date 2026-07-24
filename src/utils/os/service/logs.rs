use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct ServiceLogsBuilder<'a> {
    executor: &'a CommandExecutor,
    name: String,
    limit: String,
}

impl<'a> ServiceLogsBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, name: String, limit: impl IntoCommand) -> Self {
        Self {
            executor,
            name,
            limit: limit.build_str(),
        }
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor
            .run("journalctl", &["-u", &self.name, "-n", &self.limit])
            .await
    }
}

impl<'a> IntoCommand for ServiceLogsBuilder<'a> {
    fn build_str(&self) -> String {
        format!(
            "journalctl -u {} -n {}",
            escape_arg(&self.name),
            escape_arg(&self.limit)
        )
    }
}
