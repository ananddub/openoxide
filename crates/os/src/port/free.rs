use crate::exec::script::{IntoCommand, sh};
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

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
        self.script().execute(self.executor).await
    }

    fn script(&self) -> Vec<crate::exec::script::ShellIR> {
        let start = self.start.as_str();
        sh!(
            let port = dynamic!(start);
            while pipe![
                cmd("ss", "-tuln"),
                cmd("grep", "-q", word![":", port, " "])
            ] {
                let port = capture_stdout! { cmd("expr", port, "+", "1"); };
            }
            echo(port);
        )
    }
}

impl<'a> IntoCommand for PortFreeBuilder<'a> {
    fn build_str(&self) -> String {
        self.script().build_str()
    }
}
