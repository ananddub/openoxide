use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct PortCheckBuilder<'a> {
    executor: &'a CommandExecutor,
    port: String,
}

impl<'a> PortCheckBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, port: impl IntoCommand) -> Self {
        Self {
            executor,
            port: port.build_str(),
        }
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor
            .run(
                "sh",
                &["-c", "ss -tuln | grep -q \":$1 \"", "dummy", &self.port],
            )
            .await
    }
}

impl<'a> IntoCommand for PortCheckBuilder<'a> {
    fn build_str(&self) -> String {
        format!(
            "sh -c 'ss -tuln | grep -q \":$1 \"' dummy {}",
            escape_arg(&self.port)
        )
    }
}
