use crate::exec::script::{IntoCommand, sh};
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct EnvUnsetBuilder<'a> {
    executor: &'a CommandExecutor,
    key: String,
}

impl<'a> EnvUnsetBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, key: impl IntoCommand) -> Self {
        Self {
            executor,
            key: key.build_str(),
        }
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.script().execute(self.executor).await
    }
    fn script(&self) -> Vec<crate::exec::script::ShellIR> {
        let key = self.key.as_str();
        sh!(cmd("unset", dynamic!(key));)
    }
}

impl<'a> IntoCommand for EnvUnsetBuilder<'a> {
    fn build_str(&self) -> String {
        self.script().build_str()
    }
}
