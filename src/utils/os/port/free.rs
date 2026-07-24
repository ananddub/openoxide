use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct PortFreeBuilder<'a> {
    executor: &'a CommandExecutor,
    start: String,
}

impl<'a> PortFreeBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, start: impl IntoCommand) -> Self {
        Self {
            executor,
            start: start.build_str(),
        }
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run("sh", &["-c", "port=$1; while ss -tuln | grep -q \":$port \"; do port=$((port+1)); done; echo $port", "dummy", &self.start]).await
    }
}

impl<'a> IntoCommand for PortFreeBuilder<'a> {
    fn build_str(&self) -> String {
        format!(
            "sh -c 'port=$1; while ss -tuln | grep -q \":$port \"; do port=$((port+1)); done; echo $port' dummy {}",
            escape_arg(&self.start)
        )
    }
}
