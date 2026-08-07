use crate::exec::script::{IntoCommand, sh};
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

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
        self.script().execute(self.executor).await
    }

    fn script(&self) -> Vec<crate::exec::script::ShellIR> {
        let port = self.port.as_str();
        sh!(pipe![
            cmd("ss", "-tuln"),
            cmd("grep", "-q", word![":", dynamic!(port), " "])
        ];)
    }
}

impl<'a> IntoCommand for PortCheckBuilder<'a> {
    fn build_str(&self) -> String {
        self.script().build_str()
    }
}
