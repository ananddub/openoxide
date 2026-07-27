use crate::utils::exec::script::{IntoCommand, sh};
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct EnvSetBuilder<'a> {
    executor: &'a CommandExecutor,
    key: String,
    val: String,
}

impl<'a> EnvSetBuilder<'a> {
    pub fn new(
        executor: &'a CommandExecutor,
        key: impl IntoCommand,
        val: impl IntoCommand,
    ) -> Self {
        Self {
            executor,
            key: key.build_str(),
            val: val.build_str(),
        }
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.script().execute(self.executor).await
    }
    fn script(&self) -> Vec<crate::utils::exec::script::ShellIR> {
        let key = self.key.as_str();
        let val = self.val.as_str();
        sh!(cmd("export", word![dynamic!(key), "=", dynamic!(val)]);)
    }
}

impl<'a> IntoCommand for EnvSetBuilder<'a> {
    fn build_str(&self) -> String {
        self.script().build_str()
    }
}
