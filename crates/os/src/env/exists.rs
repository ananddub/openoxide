use crate::exec::script::{IntoCommand, sh};
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct EnvExistsBuilder<'a> {
    executor: &'a CommandExecutor,
    key: String,
}

impl<'a> EnvExistsBuilder<'a> {
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
        sh!(cmd("printenv", dynamic!(key)).stdout(crate::exec::script::dsl::OutputTarget::Null);)
    }
}

impl<'a> IntoCommand for EnvExistsBuilder<'a> {
    fn build_str(&self) -> String {
        self.script().build_str()
    }
}
