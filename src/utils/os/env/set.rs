use crate::utils::exec::script::IntoCommand;
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
        self.executor
            .run("sh", &["-c", &format!("export {}={}", self.key, self.val)])
            .await
    }
}

impl<'a> IntoCommand for EnvSetBuilder<'a> {
    fn build_str(&self) -> String {
        format!("export {}={}", self.key, self.val)
    }
}
